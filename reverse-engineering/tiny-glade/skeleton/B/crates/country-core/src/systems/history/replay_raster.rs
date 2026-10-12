// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/history/replay_raster.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 909deae1642da94fc633d2108e23ed7319a440ec181afe7b307816a920c8cb66
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: replay_raster.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 536, 568, 569, 572, 1595, 1602, 1611, 1623, 1682, 1708
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
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::impl$6::eq
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7831
// 首条位置: {"module":536,"inlinee":30769}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::impl$0::from
// 记录形式: inline_instance, inlinee_declaration; instances=25
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11F94
// 首条位置: {"module":1602,"inlinee":73620}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::Arc::as_ptr
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1602,"record_offset":106076,"parent_record_offset":106052,"ancestor_record_offset":106052,"inlinee":7985,"source_line":1606,"archive":"_pdb/B/modules/01602.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::RasterReplayCmd::forward
// 记录形式: compiled_procedure; instances=1
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1682,"record_offset":159024,"rva":24118320,"va":5392827440}
// 编译实例 RVAs: 0x1700430
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12203604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392827440,"raw_hex":"e8acc945ff","size":5,"soft_fail":false,"target_va":5380624320,"va":5392827919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18714433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392827440,"raw_hex":"e8418f1d01","size":5,"soft_fail":false,"target_va":5411542296,"va":5392827858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1682,"record_offset":159716,"parent_record_offset":159024,"ancestor_record_offset":159024,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":1682,"record_offset":159192,"parent_record_offset":159024,"ancestor_record_offset":159024,"inlinee":32054,"source_line":70,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::RasterReplayCmd::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1682,"record_offset":159688,"parent_record_offset":159024,"ancestor_record_offset":159024,"inlinee":32711,"source_line":115,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::duration_based_on_stroke_length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1682,"record_offset":159632,"parent_record_offset":159024,"ancestor_record_offset":159024,"inlinee":76774,"source_line":472,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1682,"record_offset":159740,"parent_record_offset":159716,"ancestor_record_offset":159024,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1682,"record_offset":159304,"parent_record_offset":159276,"ancestor_record_offset":159024,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1682,"record_offset":159656,"parent_record_offset":159632,"ancestor_record_offset":159024,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1682,"record_offset":159276,"parent_record_offset":159248,"ancestor_record_offset":159024,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1682,"record_offset":159248,"parent_record_offset":159192,"ancestor_record_offset":159024,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1682,"record_offset":159444,"parent_record_offset":159380,"ancestor_record_offset":159024,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1682,"record_offset":159776,"parent_record_offset":159740,"ancestor_record_offset":159024,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1682,"record_offset":159800,"parent_record_offset":159776,"ancestor_record_offset":159024,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1682,"record_offset":159832,"parent_record_offset":159740,"ancestor_record_offset":159024,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1682,"record_offset":159336,"parent_record_offset":159192,"ancestor_record_offset":159024,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1682,"record_offset":159516,"parent_record_offset":159380,"ancestor_record_offset":159024,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1682,"record_offset":159380,"parent_record_offset":159336,"ancestor_record_offset":159024,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1682,"record_offset":159580,"parent_record_offset":159516,"ancestor_record_offset":159024,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::RasterReplayCmd::forward_instant
// 记录形式: inline_instance, inlinee_declaration; instances=74
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7FC8
// 首条位置: {"module":572,"inlinee":32712}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::history::replay_raster::RasterReplayCmd::new
//     owner=private_source_ownership_candidate; count=72
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1602,"record_offset":106432,"parent_record_offset":106408,"ancestor_record_offset":106408,"inlinee":32711,"source_line":115,"archive":"_pdb/B/modules/01602.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::RasterReplayCmd::backward
// 记录形式: compiled_procedure; instances=1
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1682,"record_offset":159872,"rva":24118816,"va":5392827936}
// 编译实例 RVAs: 0x1700620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12204100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392827936,"raw_hex":"e8bcc745ff","size":5,"soft_fail":false,"target_va":5380624320,"va":5392828415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18713937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392827936,"raw_hex":"e8518d1d01","size":5,"soft_fail":false,"target_va":5411542296,"va":5392828354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1682,"record_offset":160564,"parent_record_offset":159872,"ancestor_record_offset":159872,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":1682,"record_offset":160040,"parent_record_offset":159872,"ancestor_record_offset":159872,"inlinee":32054,"source_line":70,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::RasterReplayCmd::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1682,"record_offset":160536,"parent_record_offset":159872,"ancestor_record_offset":159872,"inlinee":32711,"source_line":115,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::duration_based_on_stroke_length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1682,"record_offset":160480,"parent_record_offset":159872,"ancestor_record_offset":159872,"inlinee":76774,"source_line":472,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1682,"record_offset":160588,"parent_record_offset":160564,"ancestor_record_offset":159872,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1682,"record_offset":160152,"parent_record_offset":160124,"ancestor_record_offset":159872,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1682,"record_offset":160504,"parent_record_offset":160480,"ancestor_record_offset":159872,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1682,"record_offset":160124,"parent_record_offset":160096,"ancestor_record_offset":159872,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1682,"record_offset":160096,"parent_record_offset":160040,"ancestor_record_offset":159872,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1682,"record_offset":160292,"parent_record_offset":160228,"ancestor_record_offset":159872,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1682,"record_offset":160624,"parent_record_offset":160588,"ancestor_record_offset":159872,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1682,"record_offset":160648,"parent_record_offset":160624,"ancestor_record_offset":159872,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1682,"record_offset":160680,"parent_record_offset":160588,"ancestor_record_offset":159872,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1682,"record_offset":160184,"parent_record_offset":160040,"ancestor_record_offset":159872,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1682,"record_offset":160364,"parent_record_offset":160228,"ancestor_record_offset":159872,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1682,"record_offset":160228,"parent_record_offset":160184,"ancestor_record_offset":159872,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1682,"record_offset":160428,"parent_record_offset":160364,"ancestor_record_offset":159872,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::RasterReplayCmd::backward_instant
// 记录形式: inline_instance, inlinee_declaration; instances=74
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7FC6
// 首条位置: {"module":572,"inlinee":32710}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::history::replay_raster::RasterReplayCmd::new
//     owner=private_source_ownership_candidate; count=72
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1602,"record_offset":106348,"parent_record_offset":106324,"ancestor_record_offset":106324,"inlinee":32711,"source_line":115,"archive":"_pdb/B/modules/01602.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::RasterReplayCmd::new
// 记录形式: inline_instance, inlinee_declaration; instances=149
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7FC7
// 首条位置: {"module":572,"inlinee":32711}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::_::impl$0::get_param
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7F36
// 首条位置: {"module":569,"record_offset":116116,"rva":11425840,"va":5380134960}
// 编译实例 RVAs: 0xae5830, 0x1507610
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31406339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380134960,"raw_hex":"e80339df01","size":5,"soft_fail":false,"target_va":5411541904,"va":5380135560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$105::get_param
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":569,"record_offset":116268,"parent_record_offset":116116,"ancestor_record_offset":116116,"inlinee":5390,"source_line":2106,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":569,"record_offset":116724,"parent_record_offset":116700,"ancestor_record_offset":116116,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":569,"record_offset":116772,"parent_record_offset":116748,"ancestor_record_offset":116116,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":569,"record_offset":116748,"parent_record_offset":116724,"ancestor_record_offset":116116,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":569,"record_offset":116700,"parent_record_offset":116676,"ancestor_record_offset":116116,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":569,"record_offset":116676,"parent_record_offset":116636,"ancestor_record_offset":116116,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":569,"record_offset":116636,"parent_record_offset":116612,"ancestor_record_offset":116116,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":569,"record_offset":119280,"parent_record_offset":119256,"ancestor_record_offset":116116,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":569,"record_offset":119256,"parent_record_offset":119220,"ancestor_record_offset":116116,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":569,"record_offset":116956,"parent_record_offset":116920,"ancestor_record_offset":116116,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":569,"record_offset":116892,"parent_record_offset":116360,"ancestor_record_offset":116116,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":569,"record_offset":116920,"parent_record_offset":116892,"ancestor_record_offset":116116,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":569,"record_offset":116392,"parent_record_offset":116360,"ancestor_record_offset":116116,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":569,"record_offset":116984,"parent_record_offset":116920,"ancestor_record_offset":116116,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":569,"record_offset":116440,"parent_record_offset":116416,"ancestor_record_offset":116116,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":569,"record_offset":116416,"parent_record_offset":116392,"ancestor_record_offset":116116,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":569,"record_offset":116612,"parent_record_offset":116588,"ancestor_record_offset":116116,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$12::get_param
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":569,"record_offset":118668,"parent_record_offset":116268,"ancestor_record_offset":116116,"inlinee":4173,"source_line":1020,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$12::get_param::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":569,"record_offset":119464,"parent_record_offset":119440,"ancestor_record_offset":116116,"inlinee":4208,"source_line":1028,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$9::get_param
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":569,"record_offset":116324,"parent_record_offset":116268,"ancestor_record_offset":116116,"inlinee":4134,"source_line":905,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$9::get_param::closure$0
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":569,"record_offset":117052,"parent_record_offset":117028,"ancestor_record_offset":116116,"inlinee":4199,"source_line":914,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":569,"record_offset":119348,"parent_record_offset":119324,"ancestor_record_offset":116116,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":569,"record_offset":119324,"parent_record_offset":118712,"ancestor_record_offset":116116,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":569,"record_offset":118712,"parent_record_offset":118668,"ancestor_record_offset":116116,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_with_ticks
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":569,"record_offset":116360,"parent_record_offset":116324,"ancestor_record_offset":116116,"inlinee":4135,"source_line":616,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":569,"record_offset":117164,"parent_record_offset":117052,"ancestor_record_offset":116116,"inlinee":4206,"source_line":812,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":569,"record_offset":119608,"parent_record_offset":119464,"ancestor_record_offset":116116,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::rt::Argument::new
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\rt.rs
//     evidence={"module":569,"record_offset":117104,"parent_record_offset":117080,"ancestor_record_offset":116116,"inlinee":4202,"source_line":99,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::rt::Argument::new_display
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\rt.rs
//     evidence={"module":569,"record_offset":117080,"parent_record_offset":117052,"ancestor_record_offset":116116,"inlinee":4201,"source_line":113,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":569,"record_offset":116528,"parent_record_offset":116468,"ancestor_record_offset":116116,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":569,"record_offset":116548,"parent_record_offset":116528,"ancestor_record_offset":116116,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":569,"record_offset":116588,"parent_record_offset":116416,"ancestor_record_offset":116116,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":569,"record_offset":117028,"parent_record_offset":116324,"ancestor_record_offset":116116,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::from_ref
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":569,"record_offset":117128,"parent_record_offset":117104,"ancestor_record_offset":116116,"inlinee":4204,"source_line":268,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":569,"record_offset":116468,"parent_record_offset":116440,"ancestor_record_offset":116116,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":569,"record_offset":116840,"parent_record_offset":116816,"ancestor_record_offset":116116,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":569,"record_offset":116500,"parent_record_offset":116468,"ancestor_record_offset":116116,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":569,"record_offset":116816,"parent_record_offset":116636,"ancestor_record_offset":116116,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":119372,"parent_record_offset":119348,"ancestor_record_offset":116116,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":119396,"parent_record_offset":119372,"ancestor_record_offset":116116,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":569,"record_offset":117512,"parent_record_offset":117488,"ancestor_record_offset":116116,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::_::impl$0::init_state
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7F26
// 首条位置: {"module":569,"inlinee":32550}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$105::init_state
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":569,"record_offset":984,"parent_record_offset":964,"ancestor_record_offset":964,"inlinee":5383,"source_line":2064,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$22::init_state
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":569,"record_offset":1012,"parent_record_offset":984,"ancestor_record_offset":964,"inlinee":7795,"source_line":1268,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$8::from_world
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":569,"record_offset":1036,"parent_record_offset":1012,"ancestor_record_offset":964,"inlinee":5705,"source_line":3758,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::impl$8::default
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":569,"record_offset":1060,"parent_record_offset":1036,"ancestor_record_offset":964,"inlinee":30758,"source_line":145,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::_::impl$0::validate_param
// 记录形式: inline_instance, inlinee_declaration; instances=15
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7F29
// 首条位置: {"module":569,"inlinee":32553}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$12::validate_param
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":569,"record_offset":56496,"parent_record_offset":55032,"ancestor_record_offset":55032,"inlinee":4225,"source_line":1000,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$9::validate_param
//     owner=registry_dependency_definition; count=36
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":569,"record_offset":55056,"parent_record_offset":55032,"ancestor_record_offset":55032,"inlinee":4223,"source_line":885,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":569,"record_offset":55408,"parent_record_offset":55384,"ancestor_record_offset":55032,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":569,"record_offset":55456,"parent_record_offset":55432,"ancestor_record_offset":55032,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":569,"record_offset":55432,"parent_record_offset":55408,"ancestor_record_offset":55032,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":569,"record_offset":55384,"parent_record_offset":55360,"ancestor_record_offset":55032,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":569,"record_offset":55360,"parent_record_offset":55328,"ancestor_record_offset":55032,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":569,"record_offset":55328,"parent_record_offset":55304,"ancestor_record_offset":55032,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":569,"record_offset":55588,"parent_record_offset":55564,"ancestor_record_offset":55032,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":569,"record_offset":55564,"parent_record_offset":55544,"ancestor_record_offset":55032,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":569,"record_offset":55084,"parent_record_offset":55056,"ancestor_record_offset":55032,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":569,"record_offset":55132,"parent_record_offset":55108,"ancestor_record_offset":55032,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":569,"record_offset":55108,"parent_record_offset":55084,"ancestor_record_offset":55032,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":569,"record_offset":55304,"parent_record_offset":55280,"ancestor_record_offset":55032,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=96
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":569,"record_offset":55220,"parent_record_offset":55160,"ancestor_record_offset":55032,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":569,"record_offset":55240,"parent_record_offset":55220,"ancestor_record_offset":55032,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":569,"record_offset":55520,"parent_record_offset":55056,"ancestor_record_offset":55032,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":569,"record_offset":55280,"parent_record_offset":55108,"ancestor_record_offset":55032,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":569,"record_offset":55160,"parent_record_offset":55132,"ancestor_record_offset":55032,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":569,"record_offset":55192,"parent_record_offset":55160,"ancestor_record_offset":55032,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=36
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":569,"record_offset":55900,"parent_record_offset":55876,"ancestor_record_offset":55032,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00569.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::impl$8::default
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 145
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7826
// 首条位置: {"module":536,"inlinee":30758}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 165
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":536,"record_offset":70408,"rva":10470032,"va":5379179152}
// 编译实例 RVAs: 0x9fc290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14014ba20
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9120350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e8a2d574ff","size":5,"soft_fail":false,"target_va":5370067488,"va":5379187833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4100001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e85f70c1ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379181484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=25
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4100575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e8216ec1ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379182074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408772e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1601678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e8728fe7ff","size":5,"soft_fail":false,"target_va":5377585888,"va":5379187561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140893b10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1483891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e88d5be9ff","size":5,"soft_fail":false,"target_va":5377702672,"va":5379186558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969720
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-602802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e84ecdf6ff","size":5,"soft_fail":false,"target_va":5378578208,"va":5379181005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a15a10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t103926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e8f6950100","size":5,"soft_fail":false,"target_va":5379283472,"va":5379179541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a161a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t99869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e81d860100","size":5,"soft_fail":false,"target_va":5379285408,"va":5379185534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a16580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t100474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e87a880100","size":5,"soft_fail":false,"target_va":5379286400,"va":5379185921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a16960
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t107491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e8e3a30100","size":5,"soft_fail":false,"target_va":5379287392,"va":5379179896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a172d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t108667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e87ba80100","size":5,"soft_fail":false,"target_va":5379289808,"va":5379181136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ade620
//     owner=unresolved_namespace; count=3
//     target_aliases=9 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t925553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e8711f0e00","size":5,"soft_fail":false,"target_va":5380105760,"va":5379180202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141466300
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10917219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e86395a600","size":5,"soft_fail":false,"target_va":5390099200,"va":5379181976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141469230
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10929453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e82dc5a600","size":5,"soft_fail":false,"target_va":5390111280,"va":5379181822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32666072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e8d871f201","size":5,"soft_fail":false,"target_va":5411847152,"va":5379181075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32660759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e8175df201","size":5,"soft_fail":false,"target_va":5411847184,"va":5379186420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32667686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e82678f201","size":5,"soft_fail":false,"target_va":5411847200,"va":5379179509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1589402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e89a401800","size":5,"soft_fail":false,"target_va":5380775408,"va":5379186001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32160250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e8fab9ea01","size":5,"soft_fail":false,"target_va":5411347040,"va":5379186785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32353678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e88eaded01","size":5,"soft_fail":false,"target_va":5411540963,"va":5379187280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32354557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e8fdb0ed01","size":5,"soft_fail":false,"target_va":5411541225,"va":5379186663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::send_event
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9315912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e848268e00","size":5,"soft_fail":false,"target_va":5388496048,"va":5379180131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32354584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e818b1ed01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379187187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32295716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e824cbec01","size":5,"soft_fail":false,"target_va":5411482433,"va":5379186712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32355311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e8efb3ed01","size":5,"soft_fail":false,"target_va":5411542064,"va":5379186748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32355083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e80bb3ed01","size":5,"soft_fail":false,"target_va":5411542296,"va":5379187208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32354955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e88bb2ed01","size":5,"soft_fail":false,"target_va":5411541904,"va":5379186944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-44355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e8bd52ffff","size":5,"soft_fail":false,"target_va":5379135376,"va":5379179726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32355470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e88eb4ed01","size":5,"soft_fail":false,"target_va":5411542880,"va":5379187405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32355940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e864b6ed01","size":5,"soft_fail":false,"target_va":5411543088,"va":5379187143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::cubic_in_out
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2725637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379179152,"raw_hex":"e805972900","size":5,"soft_fail":false,"target_va":5381906912,"va":5379181270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*%r12","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5379179152,"raw_hex":"41ffd4","size":3,"soft_fail":false,"va":5379179446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":536,"record_offset":82740,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":72516,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":70944,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":83328,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":83768,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":6099,"source_line":2040,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":536,"record_offset":83628,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":536,"record_offset":70972,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":536,"record_offset":73952,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":536,"record_offset":71528,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":9843,"source_line":63,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":536,"record_offset":81636,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":536,"record_offset":82708,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":536,"record_offset":73888,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":83068,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":71916,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":4614,"source_line":134,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":81440,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":71444,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":82680,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":83508,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":7307,"source_line":990,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":72084,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":536,"record_offset":73648,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":536,"record_offset":73924,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":26195,"source_line":297,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::MultiEdit::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":536,"record_offset":72252,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":30766,"source_line":475,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::history_chain::HistoryEditChain::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":536,"record_offset":71972,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":30764,"source_line":412,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":536,"record_offset":71028,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":29439,"source_line":198,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::iter_points_xz
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":536,"record_offset":73116,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":30770,"source_line":59,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$2::extend_dirty
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":536,"record_offset":73592,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":30773,"source_line":405,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_path::replay_path::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_path.rs
//     evidence={"module":536,"record_offset":73396,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":30772,"source_line":35,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::get_stroke_from_0_to_t
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":74012,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":30775,"source_line":435,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":73088,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":30769,"source_line":39,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":536,"record_offset":71944,"parent_record_offset":70408,"ancestor_record_offset":70408,"inlinee":28304,"source_line":156,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":74820,"parent_record_offset":74796,"ancestor_record_offset":70408,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":74844,"parent_record_offset":74820,"ancestor_record_offset":70408,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":75816,"parent_record_offset":75792,"ancestor_record_offset":70408,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":74796,"parent_record_offset":74612,"ancestor_record_offset":70408,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":75792,"parent_record_offset":75700,"ancestor_record_offset":70408,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":71180,"parent_record_offset":71152,"ancestor_record_offset":70408,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":71056,"parent_record_offset":71028,"ancestor_record_offset":70408,"inlinee":28183,"source_line":1632,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":71084,"parent_record_offset":71056,"ancestor_record_offset":70408,"inlinee":26750,"source_line":715,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":71280,"parent_record_offset":71084,"ancestor_record_offset":70408,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":71124,"parent_record_offset":71084,"ancestor_record_offset":70408,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":71152,"parent_record_offset":71124,"ancestor_record_offset":70408,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":71212,"parent_record_offset":71180,"ancestor_record_offset":70408,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":71316,"parent_record_offset":71280,"ancestor_record_offset":70408,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":82916,"parent_record_offset":82888,"ancestor_record_offset":70408,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":74544,"parent_record_offset":74512,"ancestor_record_offset":70408,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":71236,"parent_record_offset":71212,"ancestor_record_offset":70408,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":75728,"parent_record_offset":75700,"ancestor_record_offset":70408,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":75700,"parent_record_offset":75676,"ancestor_record_offset":70408,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":82976,"parent_record_offset":82944,"ancestor_record_offset":70408,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":71364,"parent_record_offset":71340,"ancestor_record_offset":70408,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":71340,"parent_record_offset":71316,"ancestor_record_offset":70408,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":82944,"parent_record_offset":82916,"ancestor_record_offset":70408,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":74612,"parent_record_offset":74576,"ancestor_record_offset":70408,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":74576,"parent_record_offset":74544,"ancestor_record_offset":70408,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":75676,"parent_record_offset":75652,"ancestor_record_offset":70408,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":74656,"parent_record_offset":74612,"ancestor_record_offset":70408,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":536,"record_offset":73224,"parent_record_offset":73116,"ancestor_record_offset":70408,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":72580,"parent_record_offset":72556,"ancestor_record_offset":70408,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":72556,"parent_record_offset":72516,"ancestor_record_offset":70408,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":74928,"parent_record_offset":74900,"ancestor_record_offset":70408,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":83816,"parent_record_offset":83768,"ancestor_record_offset":70408,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":75600,"parent_record_offset":75560,"ancestor_record_offset":70408,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":82888,"parent_record_offset":82852,"ancestor_record_offset":70408,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":84032,"parent_record_offset":83768,"ancestor_record_offset":70408,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":74480,"parent_record_offset":74288,"ancestor_record_offset":70408,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":74512,"parent_record_offset":74480,"ancestor_record_offset":70408,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":74208,"parent_record_offset":74168,"ancestor_record_offset":70408,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":75108,"parent_record_offset":75080,"ancestor_record_offset":70408,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":84408,"parent_record_offset":84380,"ancestor_record_offset":70408,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":536,"record_offset":84088,"parent_record_offset":84064,"ancestor_record_offset":70408,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":536,"record_offset":84136,"parent_record_offset":84112,"ancestor_record_offset":70408,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":536,"record_offset":75176,"parent_record_offset":75108,"ancestor_record_offset":70408,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":536,"record_offset":74900,"parent_record_offset":74288,"ancestor_record_offset":70408,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":536,"record_offset":74248,"parent_record_offset":74208,"ancestor_record_offset":70408,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":536,"record_offset":74288,"parent_record_offset":74248,"ancestor_record_offset":70408,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":536,"record_offset":70996,"parent_record_offset":70972,"ancestor_record_offset":70408,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":536,"record_offset":82824,"parent_record_offset":82796,"ancestor_record_offset":70408,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":536,"record_offset":82852,"parent_record_offset":82824,"ancestor_record_offset":70408,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::send_event
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":536,"record_offset":82796,"parent_record_offset":82768,"ancestor_record_offset":70408,"inlinee":30776,"source_line":1146,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":536,"record_offset":74680,"parent_record_offset":74656,"ancestor_record_offset":70408,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":536,"record_offset":74704,"parent_record_offset":74680,"ancestor_record_offset":70408,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":536,"record_offset":75436,"parent_record_offset":75412,"ancestor_record_offset":70408,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":536,"record_offset":75460,"parent_record_offset":75436,"ancestor_record_offset":70408,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":536,"record_offset":73716,"parent_record_offset":73676,"ancestor_record_offset":70408,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":536,"record_offset":75500,"parent_record_offset":75348,"ancestor_record_offset":70408,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":536,"record_offset":75528,"parent_record_offset":75348,"ancestor_record_offset":70408,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::typed_swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":536,"record_offset":81660,"parent_record_offset":81636,"ancestor_record_offset":70408,"inlinee":6104,"source_line":4004,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":536,"record_offset":74752,"parent_record_offset":74728,"ancestor_record_offset":70408,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":536,"record_offset":75052,"parent_record_offset":75012,"ancestor_record_offset":70408,"inlinee":9582,"source_line":30,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":536,"record_offset":74984,"parent_record_offset":74956,"ancestor_record_offset":70408,"inlinee":9580,"source_line":71,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":536,"record_offset":74340,"parent_record_offset":74288,"ancestor_record_offset":70408,"inlinee":11428,"source_line":58,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":536,"record_offset":71576,"parent_record_offset":71552,"ancestor_record_offset":70408,"inlinee":6091,"source_line":73,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":536,"record_offset":71784,"parent_record_offset":71600,"ancestor_record_offset":70408,"inlinee":6094,"source_line":85,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":74168,"parent_record_offset":74012,"ancestor_record_offset":70408,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":71552,"parent_record_offset":71528,"ancestor_record_offset":70408,"inlinee":6090,"source_line":2852,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":71812,"parent_record_offset":71784,"ancestor_record_offset":70408,"inlinee":6096,"source_line":2859,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":74956,"parent_record_offset":74928,"ancestor_record_offset":70408,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":75080,"parent_record_offset":75052,"ancestor_record_offset":70408,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":71600,"parent_record_offset":71576,"ancestor_record_offset":70408,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":536,"record_offset":83588,"parent_record_offset":83540,"ancestor_record_offset":70408,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":536,"record_offset":74728,"parent_record_offset":74704,"ancestor_record_offset":70408,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":536,"record_offset":76368,"parent_record_offset":74012,"ancestor_record_offset":70408,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":536,"record_offset":75760,"parent_record_offset":75728,"ancestor_record_offset":70408,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":536,"record_offset":71836,"parent_record_offset":71812,"ancestor_record_offset":70408,"inlinee":4194,"source_line":293,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":536,"record_offset":75412,"parent_record_offset":75388,"ancestor_record_offset":70408,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":536,"record_offset":75388,"parent_record_offset":75348,"ancestor_record_offset":70408,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":536,"record_offset":74436,"parent_record_offset":74412,"ancestor_record_offset":70408,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=55
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":72696,"parent_record_offset":72516,"ancestor_record_offset":70408,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":536,"record_offset":71492,"parent_record_offset":71468,"ancestor_record_offset":70408,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":536,"record_offset":74412,"parent_record_offset":74388,"ancestor_record_offset":70408,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":71744,"parent_record_offset":71720,"ancestor_record_offset":70408,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":83708,"parent_record_offset":83628,"ancestor_record_offset":70408,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":74388,"parent_record_offset":74364,"ancestor_record_offset":70408,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":71668,"parent_record_offset":71632,"ancestor_record_offset":70408,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":81896,"parent_record_offset":81740,"ancestor_record_offset":70408,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":81684,"parent_record_offset":81660,"ancestor_record_offset":70408,"inlinee":6105,"source_line":1071,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":536,"record_offset":81712,"parent_record_offset":81684,"ancestor_record_offset":70408,"inlinee":6107,"source_line":3874,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_simple_untyped
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":81740,"parent_record_offset":81712,"ancestor_record_offset":70408,"inlinee":6108,"source_line":1139,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":75144,"parent_record_offset":75108,"ancestor_record_offset":70408,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":536,"record_offset":75308,"parent_record_offset":75280,"ancestor_record_offset":70408,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":536,"record_offset":73800,"parent_record_offset":73772,"ancestor_record_offset":70408,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":536,"record_offset":73828,"parent_record_offset":73800,"ancestor_record_offset":70408,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":71996,"parent_record_offset":71972,"ancestor_record_offset":70408,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":83540,"parent_record_offset":83508,"ancestor_record_offset":70408,"inlinee":7309,"source_line":1013,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":536,"record_offset":76396,"parent_record_offset":74012,"ancestor_record_offset":70408,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":536,"record_offset":76424,"parent_record_offset":76396,"ancestor_record_offset":70408,"inlinee":4362,"source_line":559,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":536,"record_offset":72020,"parent_record_offset":71996,"ancestor_record_offset":70408,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":72188,"parent_record_offset":72164,"ancestor_record_offset":70408,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":536,"record_offset":71468,"parent_record_offset":71444,"ancestor_record_offset":70408,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":71720,"parent_record_offset":71696,"ancestor_record_offset":70408,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":75012,"parent_record_offset":74984,"ancestor_record_offset":70408,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":74364,"parent_record_offset":74340,"ancestor_record_offset":70408,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":72164,"parent_record_offset":72084,"ancestor_record_offset":70408,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":72840,"parent_record_offset":72816,"ancestor_record_offset":70408,"inlinee":28520,"source_line":313,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":71632,"parent_record_offset":71600,"ancestor_record_offset":70408,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":71696,"parent_record_offset":71632,"ancestor_record_offset":70408,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":536,"record_offset":73744,"parent_record_offset":73716,"ancestor_record_offset":70408,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":536,"record_offset":73676,"parent_record_offset":73648,"ancestor_record_offset":70408,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":71860,"parent_record_offset":71836,"ancestor_record_offset":70408,"inlinee":30763,"source_line":200,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":73048,"parent_record_offset":73024,"ancestor_record_offset":70408,"inlinee":30767,"source_line":249,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":536,"record_offset":73616,"parent_record_offset":73592,"ancestor_record_offset":70408,"inlinee":29556,"source_line":189,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":536,"record_offset":75280,"parent_record_offset":74012,"ancestor_record_offset":70408,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::try_slice
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":536,"record_offset":75560,"parent_record_offset":74012,"ancestor_record_offset":70408,"inlinee":9594,"source_line":859,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":536,"record_offset":75348,"parent_record_offset":74012,"ancestor_record_offset":70408,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 165
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":536,"record_offset":6732,"rva":10427632,"va":5379136752}
// 编译实例 RVAs: 0x9f1cf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14014ba20
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9077950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e8427b75ff","size":5,"soft_fail":false,"target_va":5370067488,"va":5379145433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4057601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e8ff15c2ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379139084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=25
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4058175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e8c113c2ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379139674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408772e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1559278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e81235e8ff","size":5,"soft_fail":false,"target_va":5377585888,"va":5379145161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140893b10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1441491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e82d01eaff","size":5,"soft_fail":false,"target_va":5377702672,"va":5379144158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969720
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-560402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e8ee72f7ff","size":5,"soft_fail":false,"target_va":5378578208,"va":5379138605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a15a10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t146326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e8963b0200","size":5,"soft_fail":false,"target_va":5379283472,"va":5379137141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a161a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t142269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e8bd2b0200","size":5,"soft_fail":false,"target_va":5379285408,"va":5379143134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a16580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t142874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e81a2e0200","size":5,"soft_fail":false,"target_va":5379286400,"va":5379143521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a16960
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t149891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e883490200","size":5,"soft_fail":false,"target_va":5379287392,"va":5379137496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a172d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t151067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e81b4e0200","size":5,"soft_fail":false,"target_va":5379289808,"va":5379138736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ade620
//     owner=unresolved_namespace; count=3
//     target_aliases=9 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t967953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e811c50e00","size":5,"soft_fail":false,"target_va":5380105760,"va":5379137802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141466300
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10959619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e8033ba700","size":5,"soft_fail":false,"target_va":5390099200,"va":5379139576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141469230
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10971853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e8cd6aa700","size":5,"soft_fail":false,"target_va":5390111280,"va":5379139422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32708472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e87817f301","size":5,"soft_fail":false,"target_va":5411847152,"va":5379138675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32703159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e8b702f301","size":5,"soft_fail":false,"target_va":5411847184,"va":5379144020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32710086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e8c61df301","size":5,"soft_fail":false,"target_va":5411847200,"va":5379137109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1631802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e83ae61800","size":5,"soft_fail":false,"target_va":5380775408,"va":5379143601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32202650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e89a5feb01","size":5,"soft_fail":false,"target_va":5411347040,"va":5379144385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32396078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e82e53ee01","size":5,"soft_fail":false,"target_va":5411540963,"va":5379144880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32396957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e89d56ee01","size":5,"soft_fail":false,"target_va":5411541225,"va":5379144263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::send_event
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9358312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e8e8cb8e00","size":5,"soft_fail":false,"target_va":5388496048,"va":5379137731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32396984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e8b856ee01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379144787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32338116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e8c470ed01","size":5,"soft_fail":false,"target_va":5411482433,"va":5379144312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32397711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e88f59ee01","size":5,"soft_fail":false,"target_va":5411542064,"va":5379144348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32397483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e8ab58ee01","size":5,"soft_fail":false,"target_va":5411542296,"va":5379144808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32397355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e82b58ee01","size":5,"soft_fail":false,"target_va":5411541904,"va":5379144544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-1955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e85df8ffff","size":5,"soft_fail":false,"target_va":5379135376,"va":5379137326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32397870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e82e5aee01","size":5,"soft_fail":false,"target_va":5411542880,"va":5379145005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32398340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e8045cee01","size":5,"soft_fail":false,"target_va":5411543088,"va":5379144743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::cubic_in_out
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2768037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379136752,"raw_hex":"e8a53c2a00","size":5,"soft_fail":false,"target_va":5381906912,"va":5379138870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*%r12","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5379136752,"raw_hex":"41ffd4","size":3,"soft_fail":false,"va":5379137046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":536,"record_offset":19064,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":8840,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":7268,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":19652,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":20092,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":6099,"source_line":2040,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":536,"record_offset":19952,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":536,"record_offset":7296,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":536,"record_offset":10276,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":536,"record_offset":7852,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":9843,"source_line":63,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":536,"record_offset":17960,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":536,"record_offset":19032,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":536,"record_offset":10212,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":19392,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":8240,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":4614,"source_line":134,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":17764,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":7768,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":19004,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":19832,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":7307,"source_line":990,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":8408,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":536,"record_offset":9972,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":536,"record_offset":10248,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":26195,"source_line":297,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::MultiEdit::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":536,"record_offset":8576,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":30766,"source_line":475,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::history_chain::HistoryEditChain::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":536,"record_offset":8296,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":30764,"source_line":412,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":536,"record_offset":7352,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":29439,"source_line":198,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::iter_points_xz
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":536,"record_offset":9440,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":30770,"source_line":59,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$2::extend_dirty
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":536,"record_offset":9916,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":30773,"source_line":405,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_path::replay_path::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_path.rs
//     evidence={"module":536,"record_offset":9720,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":30772,"source_line":35,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::get_stroke_from_0_to_t
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":10336,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":30775,"source_line":435,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":9412,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":30769,"source_line":39,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":536,"record_offset":8268,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":28304,"source_line":156,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":11144,"parent_record_offset":11120,"ancestor_record_offset":6732,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":11168,"parent_record_offset":11144,"ancestor_record_offset":6732,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":12140,"parent_record_offset":12116,"ancestor_record_offset":6732,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":11120,"parent_record_offset":10936,"ancestor_record_offset":6732,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":12116,"parent_record_offset":12024,"ancestor_record_offset":6732,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":7504,"parent_record_offset":7476,"ancestor_record_offset":6732,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":7380,"parent_record_offset":7352,"ancestor_record_offset":6732,"inlinee":28183,"source_line":1632,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":7408,"parent_record_offset":7380,"ancestor_record_offset":6732,"inlinee":26750,"source_line":715,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":7604,"parent_record_offset":7408,"ancestor_record_offset":6732,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":7448,"parent_record_offset":7408,"ancestor_record_offset":6732,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":7476,"parent_record_offset":7448,"ancestor_record_offset":6732,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":7536,"parent_record_offset":7504,"ancestor_record_offset":6732,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":7640,"parent_record_offset":7604,"ancestor_record_offset":6732,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":19240,"parent_record_offset":19212,"ancestor_record_offset":6732,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":10868,"parent_record_offset":10836,"ancestor_record_offset":6732,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":7560,"parent_record_offset":7536,"ancestor_record_offset":6732,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":12052,"parent_record_offset":12024,"ancestor_record_offset":6732,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":12024,"parent_record_offset":12000,"ancestor_record_offset":6732,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":19300,"parent_record_offset":19268,"ancestor_record_offset":6732,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":7688,"parent_record_offset":7664,"ancestor_record_offset":6732,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":7664,"parent_record_offset":7640,"ancestor_record_offset":6732,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":19268,"parent_record_offset":19240,"ancestor_record_offset":6732,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":10936,"parent_record_offset":10900,"ancestor_record_offset":6732,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":10900,"parent_record_offset":10868,"ancestor_record_offset":6732,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":12000,"parent_record_offset":11976,"ancestor_record_offset":6732,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":10980,"parent_record_offset":10936,"ancestor_record_offset":6732,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":536,"record_offset":9548,"parent_record_offset":9440,"ancestor_record_offset":6732,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":8904,"parent_record_offset":8880,"ancestor_record_offset":6732,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":8880,"parent_record_offset":8840,"ancestor_record_offset":6732,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":11252,"parent_record_offset":11224,"ancestor_record_offset":6732,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":20140,"parent_record_offset":20092,"ancestor_record_offset":6732,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":11924,"parent_record_offset":11884,"ancestor_record_offset":6732,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":19212,"parent_record_offset":19176,"ancestor_record_offset":6732,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":20356,"parent_record_offset":20092,"ancestor_record_offset":6732,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":10804,"parent_record_offset":10612,"ancestor_record_offset":6732,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":10836,"parent_record_offset":10804,"ancestor_record_offset":6732,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":10532,"parent_record_offset":10492,"ancestor_record_offset":6732,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":11432,"parent_record_offset":11404,"ancestor_record_offset":6732,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":20732,"parent_record_offset":20704,"ancestor_record_offset":6732,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":536,"record_offset":20412,"parent_record_offset":20388,"ancestor_record_offset":6732,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":536,"record_offset":20460,"parent_record_offset":20436,"ancestor_record_offset":6732,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":536,"record_offset":11500,"parent_record_offset":11432,"ancestor_record_offset":6732,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":536,"record_offset":11224,"parent_record_offset":10612,"ancestor_record_offset":6732,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":536,"record_offset":10572,"parent_record_offset":10532,"ancestor_record_offset":6732,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":536,"record_offset":10612,"parent_record_offset":10572,"ancestor_record_offset":6732,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":536,"record_offset":7320,"parent_record_offset":7296,"ancestor_record_offset":6732,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":536,"record_offset":19148,"parent_record_offset":19120,"ancestor_record_offset":6732,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":536,"record_offset":19176,"parent_record_offset":19148,"ancestor_record_offset":6732,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::send_event
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":536,"record_offset":19120,"parent_record_offset":19092,"ancestor_record_offset":6732,"inlinee":30776,"source_line":1146,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":536,"record_offset":11004,"parent_record_offset":10980,"ancestor_record_offset":6732,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":536,"record_offset":11028,"parent_record_offset":11004,"ancestor_record_offset":6732,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":536,"record_offset":11760,"parent_record_offset":11736,"ancestor_record_offset":6732,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":536,"record_offset":11784,"parent_record_offset":11760,"ancestor_record_offset":6732,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":536,"record_offset":10040,"parent_record_offset":10000,"ancestor_record_offset":6732,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":536,"record_offset":11824,"parent_record_offset":11672,"ancestor_record_offset":6732,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":536,"record_offset":11852,"parent_record_offset":11672,"ancestor_record_offset":6732,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::typed_swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":536,"record_offset":17984,"parent_record_offset":17960,"ancestor_record_offset":6732,"inlinee":6104,"source_line":4004,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":536,"record_offset":11076,"parent_record_offset":11052,"ancestor_record_offset":6732,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":536,"record_offset":11376,"parent_record_offset":11336,"ancestor_record_offset":6732,"inlinee":9582,"source_line":30,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":536,"record_offset":11308,"parent_record_offset":11280,"ancestor_record_offset":6732,"inlinee":9580,"source_line":71,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":536,"record_offset":10664,"parent_record_offset":10612,"ancestor_record_offset":6732,"inlinee":11428,"source_line":58,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":536,"record_offset":7900,"parent_record_offset":7876,"ancestor_record_offset":6732,"inlinee":6091,"source_line":73,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":536,"record_offset":8108,"parent_record_offset":7924,"ancestor_record_offset":6732,"inlinee":6094,"source_line":85,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":10492,"parent_record_offset":10336,"ancestor_record_offset":6732,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":7876,"parent_record_offset":7852,"ancestor_record_offset":6732,"inlinee":6090,"source_line":2852,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":8136,"parent_record_offset":8108,"ancestor_record_offset":6732,"inlinee":6096,"source_line":2859,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":11280,"parent_record_offset":11252,"ancestor_record_offset":6732,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":11404,"parent_record_offset":11376,"ancestor_record_offset":6732,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":7924,"parent_record_offset":7900,"ancestor_record_offset":6732,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":536,"record_offset":19912,"parent_record_offset":19864,"ancestor_record_offset":6732,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":536,"record_offset":11052,"parent_record_offset":11028,"ancestor_record_offset":6732,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":536,"record_offset":12692,"parent_record_offset":10336,"ancestor_record_offset":6732,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":536,"record_offset":12084,"parent_record_offset":12052,"ancestor_record_offset":6732,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":536,"record_offset":8160,"parent_record_offset":8136,"ancestor_record_offset":6732,"inlinee":4194,"source_line":293,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":536,"record_offset":11736,"parent_record_offset":11712,"ancestor_record_offset":6732,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":536,"record_offset":11712,"parent_record_offset":11672,"ancestor_record_offset":6732,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":536,"record_offset":10760,"parent_record_offset":10736,"ancestor_record_offset":6732,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=55
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":9020,"parent_record_offset":8840,"ancestor_record_offset":6732,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":536,"record_offset":7816,"parent_record_offset":7792,"ancestor_record_offset":6732,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":536,"record_offset":10736,"parent_record_offset":10712,"ancestor_record_offset":6732,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":8068,"parent_record_offset":8044,"ancestor_record_offset":6732,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":20032,"parent_record_offset":19952,"ancestor_record_offset":6732,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":10712,"parent_record_offset":10688,"ancestor_record_offset":6732,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":7992,"parent_record_offset":7956,"ancestor_record_offset":6732,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":18220,"parent_record_offset":18064,"ancestor_record_offset":6732,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":18008,"parent_record_offset":17984,"ancestor_record_offset":6732,"inlinee":6105,"source_line":1071,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":536,"record_offset":18036,"parent_record_offset":18008,"ancestor_record_offset":6732,"inlinee":6107,"source_line":3874,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_simple_untyped
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":18064,"parent_record_offset":18036,"ancestor_record_offset":6732,"inlinee":6108,"source_line":1139,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":11468,"parent_record_offset":11432,"ancestor_record_offset":6732,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":536,"record_offset":11632,"parent_record_offset":11604,"ancestor_record_offset":6732,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":536,"record_offset":10124,"parent_record_offset":10096,"ancestor_record_offset":6732,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":536,"record_offset":10152,"parent_record_offset":10124,"ancestor_record_offset":6732,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":8320,"parent_record_offset":8296,"ancestor_record_offset":6732,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":19864,"parent_record_offset":19832,"ancestor_record_offset":6732,"inlinee":7309,"source_line":1013,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":536,"record_offset":12720,"parent_record_offset":10336,"ancestor_record_offset":6732,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":536,"record_offset":12748,"parent_record_offset":12720,"ancestor_record_offset":6732,"inlinee":4362,"source_line":559,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":536,"record_offset":8344,"parent_record_offset":8320,"ancestor_record_offset":6732,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":8512,"parent_record_offset":8488,"ancestor_record_offset":6732,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":536,"record_offset":7792,"parent_record_offset":7768,"ancestor_record_offset":6732,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":8044,"parent_record_offset":8020,"ancestor_record_offset":6732,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":11336,"parent_record_offset":11308,"ancestor_record_offset":6732,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":10688,"parent_record_offset":10664,"ancestor_record_offset":6732,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":8488,"parent_record_offset":8408,"ancestor_record_offset":6732,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":9164,"parent_record_offset":9140,"ancestor_record_offset":6732,"inlinee":28520,"source_line":313,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":7956,"parent_record_offset":7924,"ancestor_record_offset":6732,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":8020,"parent_record_offset":7956,"ancestor_record_offset":6732,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":536,"record_offset":10068,"parent_record_offset":10040,"ancestor_record_offset":6732,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":536,"record_offset":10000,"parent_record_offset":9972,"ancestor_record_offset":6732,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":8184,"parent_record_offset":8160,"ancestor_record_offset":6732,"inlinee":30763,"source_line":200,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":9372,"parent_record_offset":9348,"ancestor_record_offset":6732,"inlinee":30767,"source_line":249,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":536,"record_offset":9940,"parent_record_offset":9916,"ancestor_record_offset":6732,"inlinee":29556,"source_line":189,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":536,"record_offset":11604,"parent_record_offset":10336,"ancestor_record_offset":6732,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::try_slice
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":536,"record_offset":11884,"parent_record_offset":10336,"ancestor_record_offset":6732,"inlinee":9594,"source_line":859,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":536,"record_offset":11672,"parent_record_offset":10336,"ancestor_record_offset":6732,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 165
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":536,"record_offset":54592,"rva":10459440,"va":5379168560}
// 编译实例 RVAs: 0x9f9930
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14014ba20
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9109758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e802ff74ff","size":5,"soft_fail":false,"target_va":5370067488,"va":5379177241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4089409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e8bf99c1ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379170892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=25
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4089983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e88197c1ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379171482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408772e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1591086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e8d2b8e7ff","size":5,"soft_fail":false,"target_va":5377585888,"va":5379176969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140893b10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1473299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e8ed84e9ff","size":5,"soft_fail":false,"target_va":5377702672,"va":5379175966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969720
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-592210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e8aef6f6ff","size":5,"soft_fail":false,"target_va":5378578208,"va":5379170413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a15a10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t114518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e856bf0100","size":5,"soft_fail":false,"target_va":5379283472,"va":5379168949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a161a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t110461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e87daf0100","size":5,"soft_fail":false,"target_va":5379285408,"va":5379174942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a16580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t111066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e8dab10100","size":5,"soft_fail":false,"target_va":5379286400,"va":5379175329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a16960
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t118083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e843cd0100","size":5,"soft_fail":false,"target_va":5379287392,"va":5379169304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a172d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t119259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e8dbd10100","size":5,"soft_fail":false,"target_va":5379289808,"va":5379170544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ade620
//     owner=unresolved_namespace; count=3
//     target_aliases=9 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t936145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e8d1480e00","size":5,"soft_fail":false,"target_va":5380105760,"va":5379169610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141466300
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10927811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e8c3bea600","size":5,"soft_fail":false,"target_va":5390099200,"va":5379171384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141469230
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10940045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e88deea600","size":5,"soft_fail":false,"target_va":5390111280,"va":5379171230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32676664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e8389bf201","size":5,"soft_fail":false,"target_va":5411847152,"va":5379170483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32671351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e87786f201","size":5,"soft_fail":false,"target_va":5411847184,"va":5379175828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32678278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e886a1f201","size":5,"soft_fail":false,"target_va":5411847200,"va":5379168917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1599994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e8fa691800","size":5,"soft_fail":false,"target_va":5380775408,"va":5379175409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32170842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e85ae3ea01","size":5,"soft_fail":false,"target_va":5411347040,"va":5379176193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32364270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e8eed6ed01","size":5,"soft_fail":false,"target_va":5411540963,"va":5379176688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32365149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e85ddaed01","size":5,"soft_fail":false,"target_va":5411541225,"va":5379176071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::send_event
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9326504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e8a84f8e00","size":5,"soft_fail":false,"target_va":5388496048,"va":5379169539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32365176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e878daed01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379176595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32306308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e884f4ec01","size":5,"soft_fail":false,"target_va":5411482433,"va":5379176120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32365903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e84fdded01","size":5,"soft_fail":false,"target_va":5411542064,"va":5379176156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32365675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e86bdced01","size":5,"soft_fail":false,"target_va":5411542296,"va":5379176616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32365547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e8ebdbed01","size":5,"soft_fail":false,"target_va":5411541904,"va":5379176352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-33763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e81d7cffff","size":5,"soft_fail":false,"target_va":5379135376,"va":5379169134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32366062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e8eedded01","size":5,"soft_fail":false,"target_va":5411542880,"va":5379176813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32366532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e8c4dfed01","size":5,"soft_fail":false,"target_va":5411543088,"va":5379176551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::cubic_in_out
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2736229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379168560,"raw_hex":"e865c02900","size":5,"soft_fail":false,"target_va":5381906912,"va":5379170678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*%r12","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5379168560,"raw_hex":"41ffd4","size":3,"soft_fail":false,"va":5379168854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":536,"record_offset":66924,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":56700,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":55128,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":67512,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":67952,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":6099,"source_line":2040,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":536,"record_offset":67812,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":536,"record_offset":55156,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":536,"record_offset":58136,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":536,"record_offset":55712,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":9843,"source_line":63,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":536,"record_offset":65820,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":536,"record_offset":66892,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":536,"record_offset":58072,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":67252,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":56100,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":4614,"source_line":134,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":65624,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":55628,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":66864,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":67692,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":7307,"source_line":990,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":56268,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":536,"record_offset":57832,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":536,"record_offset":58108,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":26195,"source_line":297,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::MultiEdit::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":536,"record_offset":56436,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":30766,"source_line":475,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::history_chain::HistoryEditChain::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":536,"record_offset":56156,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":30764,"source_line":412,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":536,"record_offset":55212,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":29439,"source_line":198,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::iter_points_xz
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":536,"record_offset":57300,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":30770,"source_line":59,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$2::extend_dirty
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":536,"record_offset":57776,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":30773,"source_line":405,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_path::replay_path::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_path.rs
//     evidence={"module":536,"record_offset":57580,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":30772,"source_line":35,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::get_stroke_from_0_to_t
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":58196,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":30775,"source_line":435,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":57272,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":30769,"source_line":39,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":536,"record_offset":56128,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":28304,"source_line":156,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":59004,"parent_record_offset":58980,"ancestor_record_offset":54592,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":59028,"parent_record_offset":59004,"ancestor_record_offset":54592,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":60000,"parent_record_offset":59976,"ancestor_record_offset":54592,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":58980,"parent_record_offset":58796,"ancestor_record_offset":54592,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":59976,"parent_record_offset":59884,"ancestor_record_offset":54592,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":55364,"parent_record_offset":55336,"ancestor_record_offset":54592,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":55240,"parent_record_offset":55212,"ancestor_record_offset":54592,"inlinee":28183,"source_line":1632,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":55268,"parent_record_offset":55240,"ancestor_record_offset":54592,"inlinee":26750,"source_line":715,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":55464,"parent_record_offset":55268,"ancestor_record_offset":54592,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":55308,"parent_record_offset":55268,"ancestor_record_offset":54592,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":55336,"parent_record_offset":55308,"ancestor_record_offset":54592,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":55396,"parent_record_offset":55364,"ancestor_record_offset":54592,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":55500,"parent_record_offset":55464,"ancestor_record_offset":54592,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":67100,"parent_record_offset":67072,"ancestor_record_offset":54592,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":58728,"parent_record_offset":58696,"ancestor_record_offset":54592,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":55420,"parent_record_offset":55396,"ancestor_record_offset":54592,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":59912,"parent_record_offset":59884,"ancestor_record_offset":54592,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":59884,"parent_record_offset":59860,"ancestor_record_offset":54592,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":67160,"parent_record_offset":67128,"ancestor_record_offset":54592,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":55548,"parent_record_offset":55524,"ancestor_record_offset":54592,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":55524,"parent_record_offset":55500,"ancestor_record_offset":54592,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":67128,"parent_record_offset":67100,"ancestor_record_offset":54592,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":58796,"parent_record_offset":58760,"ancestor_record_offset":54592,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":58760,"parent_record_offset":58728,"ancestor_record_offset":54592,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":59860,"parent_record_offset":59836,"ancestor_record_offset":54592,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":58840,"parent_record_offset":58796,"ancestor_record_offset":54592,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":536,"record_offset":57408,"parent_record_offset":57300,"ancestor_record_offset":54592,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":56764,"parent_record_offset":56740,"ancestor_record_offset":54592,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":56740,"parent_record_offset":56700,"ancestor_record_offset":54592,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":59112,"parent_record_offset":59084,"ancestor_record_offset":54592,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":68000,"parent_record_offset":67952,"ancestor_record_offset":54592,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":59784,"parent_record_offset":59744,"ancestor_record_offset":54592,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":67072,"parent_record_offset":67036,"ancestor_record_offset":54592,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":68216,"parent_record_offset":67952,"ancestor_record_offset":54592,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":58664,"parent_record_offset":58472,"ancestor_record_offset":54592,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":58696,"parent_record_offset":58664,"ancestor_record_offset":54592,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":58392,"parent_record_offset":58352,"ancestor_record_offset":54592,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":59292,"parent_record_offset":59264,"ancestor_record_offset":54592,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":68592,"parent_record_offset":68564,"ancestor_record_offset":54592,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":536,"record_offset":68272,"parent_record_offset":68248,"ancestor_record_offset":54592,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":536,"record_offset":68320,"parent_record_offset":68296,"ancestor_record_offset":54592,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":536,"record_offset":59360,"parent_record_offset":59292,"ancestor_record_offset":54592,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":536,"record_offset":59084,"parent_record_offset":58472,"ancestor_record_offset":54592,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":536,"record_offset":58432,"parent_record_offset":58392,"ancestor_record_offset":54592,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":536,"record_offset":58472,"parent_record_offset":58432,"ancestor_record_offset":54592,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":536,"record_offset":55180,"parent_record_offset":55156,"ancestor_record_offset":54592,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":536,"record_offset":67008,"parent_record_offset":66980,"ancestor_record_offset":54592,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":536,"record_offset":67036,"parent_record_offset":67008,"ancestor_record_offset":54592,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::send_event
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":536,"record_offset":66980,"parent_record_offset":66952,"ancestor_record_offset":54592,"inlinee":30776,"source_line":1146,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":536,"record_offset":58864,"parent_record_offset":58840,"ancestor_record_offset":54592,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":536,"record_offset":58888,"parent_record_offset":58864,"ancestor_record_offset":54592,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":536,"record_offset":59620,"parent_record_offset":59596,"ancestor_record_offset":54592,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":536,"record_offset":59644,"parent_record_offset":59620,"ancestor_record_offset":54592,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":536,"record_offset":57900,"parent_record_offset":57860,"ancestor_record_offset":54592,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":536,"record_offset":59684,"parent_record_offset":59532,"ancestor_record_offset":54592,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":536,"record_offset":59712,"parent_record_offset":59532,"ancestor_record_offset":54592,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::typed_swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":536,"record_offset":65844,"parent_record_offset":65820,"ancestor_record_offset":54592,"inlinee":6104,"source_line":4004,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":536,"record_offset":58936,"parent_record_offset":58912,"ancestor_record_offset":54592,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":536,"record_offset":59236,"parent_record_offset":59196,"ancestor_record_offset":54592,"inlinee":9582,"source_line":30,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":536,"record_offset":59168,"parent_record_offset":59140,"ancestor_record_offset":54592,"inlinee":9580,"source_line":71,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":536,"record_offset":58524,"parent_record_offset":58472,"ancestor_record_offset":54592,"inlinee":11428,"source_line":58,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":536,"record_offset":55760,"parent_record_offset":55736,"ancestor_record_offset":54592,"inlinee":6091,"source_line":73,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":536,"record_offset":55968,"parent_record_offset":55784,"ancestor_record_offset":54592,"inlinee":6094,"source_line":85,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":58352,"parent_record_offset":58196,"ancestor_record_offset":54592,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":55736,"parent_record_offset":55712,"ancestor_record_offset":54592,"inlinee":6090,"source_line":2852,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":55996,"parent_record_offset":55968,"ancestor_record_offset":54592,"inlinee":6096,"source_line":2859,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":59140,"parent_record_offset":59112,"ancestor_record_offset":54592,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":59264,"parent_record_offset":59236,"ancestor_record_offset":54592,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":55784,"parent_record_offset":55760,"ancestor_record_offset":54592,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":536,"record_offset":67772,"parent_record_offset":67724,"ancestor_record_offset":54592,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":536,"record_offset":58912,"parent_record_offset":58888,"ancestor_record_offset":54592,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":536,"record_offset":60552,"parent_record_offset":58196,"ancestor_record_offset":54592,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":536,"record_offset":59944,"parent_record_offset":59912,"ancestor_record_offset":54592,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":536,"record_offset":56020,"parent_record_offset":55996,"ancestor_record_offset":54592,"inlinee":4194,"source_line":293,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":536,"record_offset":59596,"parent_record_offset":59572,"ancestor_record_offset":54592,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":536,"record_offset":59572,"parent_record_offset":59532,"ancestor_record_offset":54592,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":536,"record_offset":58620,"parent_record_offset":58596,"ancestor_record_offset":54592,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=55
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":56880,"parent_record_offset":56700,"ancestor_record_offset":54592,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":536,"record_offset":55676,"parent_record_offset":55652,"ancestor_record_offset":54592,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":536,"record_offset":58596,"parent_record_offset":58572,"ancestor_record_offset":54592,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":55928,"parent_record_offset":55904,"ancestor_record_offset":54592,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":67892,"parent_record_offset":67812,"ancestor_record_offset":54592,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":58572,"parent_record_offset":58548,"ancestor_record_offset":54592,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":55852,"parent_record_offset":55816,"ancestor_record_offset":54592,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":66080,"parent_record_offset":65924,"ancestor_record_offset":54592,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":65868,"parent_record_offset":65844,"ancestor_record_offset":54592,"inlinee":6105,"source_line":1071,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":536,"record_offset":65896,"parent_record_offset":65868,"ancestor_record_offset":54592,"inlinee":6107,"source_line":3874,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_simple_untyped
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":65924,"parent_record_offset":65896,"ancestor_record_offset":54592,"inlinee":6108,"source_line":1139,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":59328,"parent_record_offset":59292,"ancestor_record_offset":54592,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":536,"record_offset":59492,"parent_record_offset":59464,"ancestor_record_offset":54592,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":536,"record_offset":57984,"parent_record_offset":57956,"ancestor_record_offset":54592,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":536,"record_offset":58012,"parent_record_offset":57984,"ancestor_record_offset":54592,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":56180,"parent_record_offset":56156,"ancestor_record_offset":54592,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":67724,"parent_record_offset":67692,"ancestor_record_offset":54592,"inlinee":7309,"source_line":1013,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":536,"record_offset":60580,"parent_record_offset":58196,"ancestor_record_offset":54592,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":536,"record_offset":60608,"parent_record_offset":60580,"ancestor_record_offset":54592,"inlinee":4362,"source_line":559,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":536,"record_offset":56204,"parent_record_offset":56180,"ancestor_record_offset":54592,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":56372,"parent_record_offset":56348,"ancestor_record_offset":54592,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":536,"record_offset":55652,"parent_record_offset":55628,"ancestor_record_offset":54592,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":55904,"parent_record_offset":55880,"ancestor_record_offset":54592,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":59196,"parent_record_offset":59168,"ancestor_record_offset":54592,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":58548,"parent_record_offset":58524,"ancestor_record_offset":54592,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":56348,"parent_record_offset":56268,"ancestor_record_offset":54592,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":57024,"parent_record_offset":57000,"ancestor_record_offset":54592,"inlinee":28520,"source_line":313,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":55816,"parent_record_offset":55784,"ancestor_record_offset":54592,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":55880,"parent_record_offset":55816,"ancestor_record_offset":54592,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":536,"record_offset":57928,"parent_record_offset":57900,"ancestor_record_offset":54592,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":536,"record_offset":57860,"parent_record_offset":57832,"ancestor_record_offset":54592,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":56044,"parent_record_offset":56020,"ancestor_record_offset":54592,"inlinee":30763,"source_line":200,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":57232,"parent_record_offset":57208,"ancestor_record_offset":54592,"inlinee":30767,"source_line":249,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":536,"record_offset":57800,"parent_record_offset":57776,"ancestor_record_offset":54592,"inlinee":29556,"source_line":189,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":536,"record_offset":59464,"parent_record_offset":58196,"ancestor_record_offset":54592,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::try_slice
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":536,"record_offset":59744,"parent_record_offset":58196,"ancestor_record_offset":54592,"inlinee":9594,"source_line":859,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":536,"record_offset":59532,"parent_record_offset":58196,"ancestor_record_offset":54592,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 165
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":536,"record_offset":38760,"rva":10448880,"va":5379158000}
// 编译实例 RVAs: 0x9f6ff0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14014ba20
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9099166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e8622875ff","size":5,"soft_fail":false,"target_va":5370067488,"va":5379166649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4078840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e808c3c1ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379160323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=25
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4079423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e8c1c0c1ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379160922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408772e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1580494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e832e2e7ff","size":5,"soft_fail":false,"target_va":5377585888,"va":5379166377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140893b10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1462739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e82daee9ff","size":5,"soft_fail":false,"target_va":5377702672,"va":5379165406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969720
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-581641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e8f71ff7ff","size":5,"soft_fail":false,"target_va":5378578208,"va":5379159844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a15a10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t125078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e896e80100","size":5,"soft_fail":false,"target_va":5379283472,"va":5379158389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a161a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t121021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e8bdd80100","size":5,"soft_fail":false,"target_va":5379285408,"va":5379164382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a16580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t121624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e818db0100","size":5,"soft_fail":false,"target_va":5379286400,"va":5379164771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a16960
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t128643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e883f60100","size":5,"soft_fail":false,"target_va":5379287392,"va":5379158744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a172d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t129828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e824fb0100","size":5,"soft_fail":false,"target_va":5379289808,"va":5379159975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ade620
//     owner=unresolved_namespace; count=3
//     target_aliases=9 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t946714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e81a720e00","size":5,"soft_fail":false,"target_va":5380105760,"va":5379159041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141466300
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10938371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e803e8a600","size":5,"soft_fail":false,"target_va":5390099200,"va":5379160824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141469230
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10950605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e8cd17a700","size":5,"soft_fail":false,"target_va":5390111280,"va":5379160670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32687233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e881c4f201","size":5,"soft_fail":false,"target_va":5411847152,"va":5379159914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32681911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e8b7aff201","size":5,"soft_fail":false,"target_va":5411847184,"va":5379165268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32688838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e8c6caf201","size":5,"soft_fail":false,"target_va":5411847200,"va":5379158357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4808171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e815a2b6ff","size":5,"soft_fail":false,"target_va":5374356528,"va":5379164694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32374867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e85300ee01","size":5,"soft_fail":false,"target_va":5411540963,"va":5379166091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32375709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e89d03ee01","size":5,"soft_fail":false,"target_va":5411541225,"va":5379165511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::writer::EventWriter$LT$E$GT$::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9633601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e841ff9200","size":5,"soft_fail":false,"target_va":5388792576,"va":5379158970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32375773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e8dd03ee01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379165998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32316868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e8c41ded01","size":5,"soft_fail":false,"target_va":5411482433,"va":5379165560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32376463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e88f06ee01","size":5,"soft_fail":false,"target_va":5411542064,"va":5379165596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32376272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e8d005ee01","size":5,"soft_fail":false,"target_va":5411542296,"va":5379166019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32376144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e85005ee01","size":5,"soft_fail":false,"target_va":5411541904,"va":5379165755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-23203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e85da5ffff","size":5,"soft_fail":false,"target_va":5379135376,"va":5379158574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32376659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e85307ee01","size":5,"soft_fail":false,"target_va":5411542880,"va":5379166216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32377129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e82909ee01","size":5,"soft_fail":false,"target_va":5411543088,"va":5379165954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::cubic_in_out
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2746798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379158000,"raw_hex":"e8aee92900","size":5,"soft_fail":false,"target_va":5381906912,"va":5379160109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*%r12","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5379158000,"raw_hex":"41ffd4","size":3,"soft_fail":false,"va":5379158294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":536,"record_offset":51048,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":40796,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":39224,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":51696,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":52136,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":6099,"source_line":2040,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":536,"record_offset":51996,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":536,"record_offset":39252,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":536,"record_offset":42232,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":536,"record_offset":39808,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":9843,"source_line":63,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":536,"record_offset":49944,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":536,"record_offset":51016,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":536,"record_offset":42168,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":51436,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":40196,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":4614,"source_line":134,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":49748,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":39724,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":50988,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":51876,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":7307,"source_line":990,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":40364,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":536,"record_offset":41928,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":536,"record_offset":42204,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":26195,"source_line":297,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::MultiEdit::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":536,"record_offset":40532,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":30766,"source_line":475,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::history_chain::HistoryEditChain::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":536,"record_offset":40252,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":30764,"source_line":412,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":536,"record_offset":39308,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":29439,"source_line":198,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::iter_points_xz
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":536,"record_offset":41396,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":30770,"source_line":59,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$2::extend_dirty
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":536,"record_offset":41872,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":30773,"source_line":405,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_garden::replay_garden::closure$2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_garden.rs
//     evidence={"module":536,"record_offset":41676,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":30779,"source_line":39,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::get_stroke_from_0_to_t
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":42292,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":30775,"source_line":435,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":41368,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":30769,"source_line":39,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":536,"record_offset":40224,"parent_record_offset":38760,"ancestor_record_offset":38760,"inlinee":28304,"source_line":156,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":43100,"parent_record_offset":43076,"ancestor_record_offset":38760,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":43124,"parent_record_offset":43100,"ancestor_record_offset":38760,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":44096,"parent_record_offset":44072,"ancestor_record_offset":38760,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":43076,"parent_record_offset":42892,"ancestor_record_offset":38760,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":44072,"parent_record_offset":43980,"ancestor_record_offset":38760,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":39460,"parent_record_offset":39432,"ancestor_record_offset":38760,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":39336,"parent_record_offset":39308,"ancestor_record_offset":38760,"inlinee":28183,"source_line":1632,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":39364,"parent_record_offset":39336,"ancestor_record_offset":38760,"inlinee":26750,"source_line":715,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":39560,"parent_record_offset":39364,"ancestor_record_offset":38760,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":39404,"parent_record_offset":39364,"ancestor_record_offset":38760,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":39432,"parent_record_offset":39404,"ancestor_record_offset":38760,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":39492,"parent_record_offset":39460,"ancestor_record_offset":38760,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":39596,"parent_record_offset":39560,"ancestor_record_offset":38760,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":42824,"parent_record_offset":42792,"ancestor_record_offset":38760,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":39516,"parent_record_offset":39492,"ancestor_record_offset":38760,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":44008,"parent_record_offset":43980,"ancestor_record_offset":38760,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":43980,"parent_record_offset":43956,"ancestor_record_offset":38760,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":39644,"parent_record_offset":39620,"ancestor_record_offset":38760,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":39620,"parent_record_offset":39596,"ancestor_record_offset":38760,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":42892,"parent_record_offset":42856,"ancestor_record_offset":38760,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":42856,"parent_record_offset":42824,"ancestor_record_offset":38760,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":43956,"parent_record_offset":43932,"ancestor_record_offset":38760,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":42936,"parent_record_offset":42892,"ancestor_record_offset":38760,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":536,"record_offset":41504,"parent_record_offset":41396,"ancestor_record_offset":38760,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":40860,"parent_record_offset":40836,"ancestor_record_offset":38760,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":40836,"parent_record_offset":40796,"ancestor_record_offset":38760,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":43208,"parent_record_offset":43180,"ancestor_record_offset":38760,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":52184,"parent_record_offset":52136,"ancestor_record_offset":38760,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":43880,"parent_record_offset":43840,"ancestor_record_offset":38760,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":51224,"parent_record_offset":51188,"ancestor_record_offset":38760,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":52400,"parent_record_offset":52136,"ancestor_record_offset":38760,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":42760,"parent_record_offset":42568,"ancestor_record_offset":38760,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":42792,"parent_record_offset":42760,"ancestor_record_offset":38760,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":42488,"parent_record_offset":42448,"ancestor_record_offset":38760,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":43388,"parent_record_offset":43360,"ancestor_record_offset":38760,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":52776,"parent_record_offset":52748,"ancestor_record_offset":38760,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":536,"record_offset":52456,"parent_record_offset":52432,"ancestor_record_offset":38760,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":536,"record_offset":52504,"parent_record_offset":52480,"ancestor_record_offset":38760,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":536,"record_offset":43456,"parent_record_offset":43388,"ancestor_record_offset":38760,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":536,"record_offset":43180,"parent_record_offset":42568,"ancestor_record_offset":38760,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":536,"record_offset":42528,"parent_record_offset":42488,"ancestor_record_offset":38760,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":536,"record_offset":42568,"parent_record_offset":42528,"ancestor_record_offset":38760,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":536,"record_offset":39276,"parent_record_offset":39252,"ancestor_record_offset":38760,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":536,"record_offset":51124,"parent_record_offset":51100,"ancestor_record_offset":38760,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":536,"record_offset":51188,"parent_record_offset":51124,"ancestor_record_offset":38760,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":536,"record_offset":51100,"parent_record_offset":51076,"ancestor_record_offset":38760,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":536,"record_offset":42960,"parent_record_offset":42936,"ancestor_record_offset":38760,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":536,"record_offset":42984,"parent_record_offset":42960,"ancestor_record_offset":38760,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":536,"record_offset":43716,"parent_record_offset":43692,"ancestor_record_offset":38760,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":536,"record_offset":43740,"parent_record_offset":43716,"ancestor_record_offset":38760,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":536,"record_offset":41996,"parent_record_offset":41956,"ancestor_record_offset":38760,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":536,"record_offset":43780,"parent_record_offset":43628,"ancestor_record_offset":38760,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":536,"record_offset":43808,"parent_record_offset":43628,"ancestor_record_offset":38760,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::typed_swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":536,"record_offset":49968,"parent_record_offset":49944,"ancestor_record_offset":38760,"inlinee":6104,"source_line":4004,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":536,"record_offset":43032,"parent_record_offset":43008,"ancestor_record_offset":38760,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":536,"record_offset":43332,"parent_record_offset":43292,"ancestor_record_offset":38760,"inlinee":9582,"source_line":30,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":536,"record_offset":43264,"parent_record_offset":43236,"ancestor_record_offset":38760,"inlinee":9580,"source_line":71,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":536,"record_offset":42620,"parent_record_offset":42568,"ancestor_record_offset":38760,"inlinee":11428,"source_line":58,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":536,"record_offset":39856,"parent_record_offset":39832,"ancestor_record_offset":38760,"inlinee":6091,"source_line":73,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":536,"record_offset":40064,"parent_record_offset":39880,"ancestor_record_offset":38760,"inlinee":6094,"source_line":85,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":42448,"parent_record_offset":42292,"ancestor_record_offset":38760,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":39832,"parent_record_offset":39808,"ancestor_record_offset":38760,"inlinee":6090,"source_line":2852,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":40092,"parent_record_offset":40064,"ancestor_record_offset":38760,"inlinee":6096,"source_line":2859,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":43236,"parent_record_offset":43208,"ancestor_record_offset":38760,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":43360,"parent_record_offset":43332,"ancestor_record_offset":38760,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":39880,"parent_record_offset":39856,"ancestor_record_offset":38760,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":536,"record_offset":51956,"parent_record_offset":51908,"ancestor_record_offset":38760,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":536,"record_offset":43008,"parent_record_offset":42984,"ancestor_record_offset":38760,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":536,"record_offset":44648,"parent_record_offset":42292,"ancestor_record_offset":38760,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":536,"record_offset":44040,"parent_record_offset":44008,"ancestor_record_offset":38760,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":536,"record_offset":40116,"parent_record_offset":40092,"ancestor_record_offset":38760,"inlinee":4194,"source_line":293,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":536,"record_offset":43692,"parent_record_offset":43668,"ancestor_record_offset":38760,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":536,"record_offset":43668,"parent_record_offset":43628,"ancestor_record_offset":38760,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":536,"record_offset":42716,"parent_record_offset":42692,"ancestor_record_offset":38760,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=55
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":40976,"parent_record_offset":40796,"ancestor_record_offset":38760,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":536,"record_offset":39772,"parent_record_offset":39748,"ancestor_record_offset":38760,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":536,"record_offset":42692,"parent_record_offset":42668,"ancestor_record_offset":38760,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":40024,"parent_record_offset":40000,"ancestor_record_offset":38760,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":52076,"parent_record_offset":51996,"ancestor_record_offset":38760,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":42668,"parent_record_offset":42644,"ancestor_record_offset":38760,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":39948,"parent_record_offset":39912,"ancestor_record_offset":38760,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":50204,"parent_record_offset":50048,"ancestor_record_offset":38760,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":49992,"parent_record_offset":49968,"ancestor_record_offset":38760,"inlinee":6105,"source_line":1071,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":536,"record_offset":50020,"parent_record_offset":49992,"ancestor_record_offset":38760,"inlinee":6107,"source_line":3874,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_simple_untyped
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":50048,"parent_record_offset":50020,"ancestor_record_offset":38760,"inlinee":6108,"source_line":1139,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":43424,"parent_record_offset":43388,"ancestor_record_offset":38760,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":536,"record_offset":43588,"parent_record_offset":43560,"ancestor_record_offset":38760,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":536,"record_offset":42080,"parent_record_offset":42052,"ancestor_record_offset":38760,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":536,"record_offset":42108,"parent_record_offset":42080,"ancestor_record_offset":38760,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":40276,"parent_record_offset":40252,"ancestor_record_offset":38760,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":51908,"parent_record_offset":51876,"ancestor_record_offset":38760,"inlinee":7309,"source_line":1013,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":536,"record_offset":44676,"parent_record_offset":42292,"ancestor_record_offset":38760,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":536,"record_offset":44704,"parent_record_offset":44676,"ancestor_record_offset":38760,"inlinee":4362,"source_line":559,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":536,"record_offset":40300,"parent_record_offset":40276,"ancestor_record_offset":38760,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":40468,"parent_record_offset":40444,"ancestor_record_offset":38760,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":536,"record_offset":39748,"parent_record_offset":39724,"ancestor_record_offset":38760,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":40000,"parent_record_offset":39976,"ancestor_record_offset":38760,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":43292,"parent_record_offset":43264,"ancestor_record_offset":38760,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":42644,"parent_record_offset":42620,"ancestor_record_offset":38760,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":40444,"parent_record_offset":40364,"ancestor_record_offset":38760,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":41120,"parent_record_offset":41096,"ancestor_record_offset":38760,"inlinee":28520,"source_line":313,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":39912,"parent_record_offset":39880,"ancestor_record_offset":38760,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":39976,"parent_record_offset":39912,"ancestor_record_offset":38760,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":536,"record_offset":42024,"parent_record_offset":41996,"ancestor_record_offset":38760,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":536,"record_offset":41956,"parent_record_offset":41928,"ancestor_record_offset":38760,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":40140,"parent_record_offset":40116,"ancestor_record_offset":38760,"inlinee":30763,"source_line":200,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":41328,"parent_record_offset":41304,"ancestor_record_offset":38760,"inlinee":30767,"source_line":249,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":536,"record_offset":41896,"parent_record_offset":41872,"ancestor_record_offset":38760,"inlinee":29556,"source_line":189,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":536,"record_offset":43560,"parent_record_offset":42292,"ancestor_record_offset":38760,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::try_slice
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":536,"record_offset":43840,"parent_record_offset":42292,"ancestor_record_offset":38760,"inlinee":9594,"source_line":859,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":536,"record_offset":43628,"parent_record_offset":42292,"ancestor_record_offset":38760,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 165
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":536,"record_offset":86224,"rva":10480624,"va":5379189744}
// 编译实例 RVAs: 0x9febf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14014ba20
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9130942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e842ac74ff","size":5,"soft_fail":false,"target_va":5370067488,"va":5379198425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4110603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e8f546c1ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379192086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=25
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4111178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e8b644c1ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379192677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408772e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1612270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e81266e7ff","size":5,"soft_fail":false,"target_va":5377585888,"va":5379198153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140893b10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1494483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e82d32e9ff","size":5,"soft_fail":false,"target_va":5377702672,"va":5379197150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969720
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-613378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e8fea3f6ff","size":5,"soft_fail":false,"target_va":5378578208,"va":5379191581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a15a10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t93350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e8a66c0100","size":5,"soft_fail":false,"target_va":5379283472,"va":5379190117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a161a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t89273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e8b95c0100","size":5,"soft_fail":false,"target_va":5379285408,"va":5379196130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a16580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t89876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e8145f0100","size":5,"soft_fail":false,"target_va":5379286400,"va":5379196519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a16960
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t96915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e8937a0100","size":5,"soft_fail":false,"target_va":5379287392,"va":5379190472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a172d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t98091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e82b7f0100","size":5,"soft_fail":false,"target_va":5379289808,"va":5379191712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ade620
//     owner=unresolved_namespace; count=3
//     target_aliases=9 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t914977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e821f60d00","size":5,"soft_fail":false,"target_va":5380105760,"va":5379190778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141466300
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10906616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e8f86ba600","size":5,"soft_fail":false,"target_va":5390099200,"va":5379192579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141469230
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10918847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e8bf9ba600","size":5,"soft_fail":false,"target_va":5390111280,"va":5379192428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32655496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e88848f201","size":5,"soft_fail":false,"target_va":5411847152,"va":5379191651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32650167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e8b733f201","size":5,"soft_fail":false,"target_va":5411847184,"va":5379197012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32657112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e8d84ef201","size":5,"soft_fail":false,"target_va":5411847200,"va":5379190083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1578804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e834171800","size":5,"soft_fail":false,"target_va":5380775408,"va":5379196599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32149658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e89a90ea01","size":5,"soft_fail":false,"target_va":5411347040,"va":5379197377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32343093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e83584ed01","size":5,"soft_fail":false,"target_va":5411540963,"va":5379197865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32343965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e89d87ed01","size":5,"soft_fail":false,"target_va":5411541225,"va":5379197255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::send_event
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9305336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e8f8fc8d00","size":5,"soft_fail":false,"target_va":5388496048,"va":5379190707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32343999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e8bf87ed01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379197772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32285124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e8c4a1ec01","size":5,"soft_fail":false,"target_va":5411482433,"va":5379197304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32344719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e88f8aed01","size":5,"soft_fail":false,"target_va":5411542064,"va":5379197340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32344498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e8b289ed01","size":5,"soft_fail":false,"target_va":5411542296,"va":5379197793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32344363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e82b89ed01","size":5,"soft_fail":false,"target_va":5411541904,"va":5379197536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-54931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e86d29ffff","size":5,"soft_fail":false,"target_va":5379135376,"va":5379190302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32344885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e8358bed01","size":5,"soft_fail":false,"target_va":5411542880,"va":5379197990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32345355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e80b8ded01","size":5,"soft_fail":false,"target_va":5411543088,"va":5379197728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::cubic_in_out
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2715054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379189744,"raw_hex":"e8ae6d2900","size":5,"soft_fail":false,"target_va":5381906912,"va":5379191853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*%r13","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5379189744,"raw_hex":"41ffd5","size":3,"soft_fail":false,"va":5379190022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":536,"record_offset":98544,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":88316,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":86744,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":99132,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":99572,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":6099,"source_line":2040,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":536,"record_offset":99432,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":536,"record_offset":86772,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":536,"record_offset":89752,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":536,"record_offset":87328,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":9843,"source_line":63,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":536,"record_offset":97440,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":536,"record_offset":98512,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":536,"record_offset":89688,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":98872,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":87716,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":4614,"source_line":134,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":97244,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":87244,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":98484,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":99312,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":7307,"source_line":990,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":87884,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":536,"record_offset":89448,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":536,"record_offset":89724,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":26195,"source_line":297,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::MultiEdit::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":536,"record_offset":88052,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":30766,"source_line":475,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::history_chain::HistoryEditChain::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":536,"record_offset":87772,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":30764,"source_line":412,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":536,"record_offset":86828,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":29439,"source_line":198,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::iter_points_xz
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":536,"record_offset":88916,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":30770,"source_line":59,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$2::extend_dirty
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":536,"record_offset":89392,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":30773,"source_line":405,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_path::replay_path::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_path.rs
//     evidence={"module":536,"record_offset":89196,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":30772,"source_line":35,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::get_stroke_from_0_to_t
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":89812,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":30775,"source_line":435,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":88888,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":30769,"source_line":39,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":536,"record_offset":87744,"parent_record_offset":86224,"ancestor_record_offset":86224,"inlinee":28304,"source_line":156,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":90624,"parent_record_offset":90600,"ancestor_record_offset":86224,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":90648,"parent_record_offset":90624,"ancestor_record_offset":86224,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":91620,"parent_record_offset":91596,"ancestor_record_offset":86224,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":90600,"parent_record_offset":90416,"ancestor_record_offset":86224,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":91596,"parent_record_offset":91504,"ancestor_record_offset":86224,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":86980,"parent_record_offset":86952,"ancestor_record_offset":86224,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":86856,"parent_record_offset":86828,"ancestor_record_offset":86224,"inlinee":28183,"source_line":1632,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":86884,"parent_record_offset":86856,"ancestor_record_offset":86224,"inlinee":26750,"source_line":715,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":87080,"parent_record_offset":86884,"ancestor_record_offset":86224,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":86924,"parent_record_offset":86884,"ancestor_record_offset":86224,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":86952,"parent_record_offset":86924,"ancestor_record_offset":86224,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":87012,"parent_record_offset":86980,"ancestor_record_offset":86224,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":87116,"parent_record_offset":87080,"ancestor_record_offset":86224,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":98720,"parent_record_offset":98692,"ancestor_record_offset":86224,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":90348,"parent_record_offset":90316,"ancestor_record_offset":86224,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":87036,"parent_record_offset":87012,"ancestor_record_offset":86224,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":91532,"parent_record_offset":91504,"ancestor_record_offset":86224,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":91504,"parent_record_offset":91480,"ancestor_record_offset":86224,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":98780,"parent_record_offset":98748,"ancestor_record_offset":86224,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":87164,"parent_record_offset":87140,"ancestor_record_offset":86224,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":87140,"parent_record_offset":87116,"ancestor_record_offset":86224,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":98748,"parent_record_offset":98720,"ancestor_record_offset":86224,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":90416,"parent_record_offset":90380,"ancestor_record_offset":86224,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":90380,"parent_record_offset":90348,"ancestor_record_offset":86224,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":91480,"parent_record_offset":91456,"ancestor_record_offset":86224,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":90460,"parent_record_offset":90416,"ancestor_record_offset":86224,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":536,"record_offset":89024,"parent_record_offset":88916,"ancestor_record_offset":86224,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":88380,"parent_record_offset":88356,"ancestor_record_offset":86224,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":88356,"parent_record_offset":88316,"ancestor_record_offset":86224,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":90732,"parent_record_offset":90704,"ancestor_record_offset":86224,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":99620,"parent_record_offset":99572,"ancestor_record_offset":86224,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":91404,"parent_record_offset":91364,"ancestor_record_offset":86224,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":98692,"parent_record_offset":98656,"ancestor_record_offset":86224,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":99836,"parent_record_offset":99572,"ancestor_record_offset":86224,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":90284,"parent_record_offset":90092,"ancestor_record_offset":86224,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":90316,"parent_record_offset":90284,"ancestor_record_offset":86224,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":90012,"parent_record_offset":89972,"ancestor_record_offset":86224,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":90912,"parent_record_offset":90884,"ancestor_record_offset":86224,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":100212,"parent_record_offset":100184,"ancestor_record_offset":86224,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":536,"record_offset":99892,"parent_record_offset":99868,"ancestor_record_offset":86224,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":536,"record_offset":99940,"parent_record_offset":99916,"ancestor_record_offset":86224,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":536,"record_offset":90980,"parent_record_offset":90912,"ancestor_record_offset":86224,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":536,"record_offset":90704,"parent_record_offset":90092,"ancestor_record_offset":86224,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":536,"record_offset":90052,"parent_record_offset":90012,"ancestor_record_offset":86224,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":536,"record_offset":90092,"parent_record_offset":90052,"ancestor_record_offset":86224,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":536,"record_offset":86796,"parent_record_offset":86772,"ancestor_record_offset":86224,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":536,"record_offset":98628,"parent_record_offset":98600,"ancestor_record_offset":86224,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":536,"record_offset":98656,"parent_record_offset":98628,"ancestor_record_offset":86224,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::send_event
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":536,"record_offset":98600,"parent_record_offset":98572,"ancestor_record_offset":86224,"inlinee":30776,"source_line":1146,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":536,"record_offset":90484,"parent_record_offset":90460,"ancestor_record_offset":86224,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":536,"record_offset":90508,"parent_record_offset":90484,"ancestor_record_offset":86224,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":536,"record_offset":91240,"parent_record_offset":91216,"ancestor_record_offset":86224,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":536,"record_offset":91264,"parent_record_offset":91240,"ancestor_record_offset":86224,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":536,"record_offset":89516,"parent_record_offset":89476,"ancestor_record_offset":86224,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":536,"record_offset":91304,"parent_record_offset":91152,"ancestor_record_offset":86224,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":536,"record_offset":91332,"parent_record_offset":91152,"ancestor_record_offset":86224,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::typed_swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":536,"record_offset":97464,"parent_record_offset":97440,"ancestor_record_offset":86224,"inlinee":6104,"source_line":4004,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":536,"record_offset":90556,"parent_record_offset":90532,"ancestor_record_offset":86224,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":536,"record_offset":90856,"parent_record_offset":90816,"ancestor_record_offset":86224,"inlinee":9582,"source_line":30,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":536,"record_offset":90788,"parent_record_offset":90760,"ancestor_record_offset":86224,"inlinee":9580,"source_line":71,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":536,"record_offset":90144,"parent_record_offset":90092,"ancestor_record_offset":86224,"inlinee":11428,"source_line":58,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":536,"record_offset":87376,"parent_record_offset":87352,"ancestor_record_offset":86224,"inlinee":6091,"source_line":73,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":536,"record_offset":87584,"parent_record_offset":87400,"ancestor_record_offset":86224,"inlinee":6094,"source_line":85,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":89972,"parent_record_offset":89812,"ancestor_record_offset":86224,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":87352,"parent_record_offset":87328,"ancestor_record_offset":86224,"inlinee":6090,"source_line":2852,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":87612,"parent_record_offset":87584,"ancestor_record_offset":86224,"inlinee":6096,"source_line":2859,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":90760,"parent_record_offset":90732,"ancestor_record_offset":86224,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":90884,"parent_record_offset":90856,"ancestor_record_offset":86224,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":87400,"parent_record_offset":87376,"ancestor_record_offset":86224,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":536,"record_offset":99392,"parent_record_offset":99344,"ancestor_record_offset":86224,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":536,"record_offset":90532,"parent_record_offset":90508,"ancestor_record_offset":86224,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":536,"record_offset":92172,"parent_record_offset":89812,"ancestor_record_offset":86224,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":536,"record_offset":91564,"parent_record_offset":91532,"ancestor_record_offset":86224,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":536,"record_offset":87636,"parent_record_offset":87612,"ancestor_record_offset":86224,"inlinee":4194,"source_line":293,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":536,"record_offset":91216,"parent_record_offset":91192,"ancestor_record_offset":86224,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":536,"record_offset":91192,"parent_record_offset":91152,"ancestor_record_offset":86224,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":536,"record_offset":90240,"parent_record_offset":90216,"ancestor_record_offset":86224,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=55
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":88496,"parent_record_offset":88316,"ancestor_record_offset":86224,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":536,"record_offset":87292,"parent_record_offset":87268,"ancestor_record_offset":86224,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":536,"record_offset":90216,"parent_record_offset":90192,"ancestor_record_offset":86224,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":87544,"parent_record_offset":87520,"ancestor_record_offset":86224,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":99512,"parent_record_offset":99432,"ancestor_record_offset":86224,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":90192,"parent_record_offset":90168,"ancestor_record_offset":86224,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":87468,"parent_record_offset":87432,"ancestor_record_offset":86224,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":97700,"parent_record_offset":97544,"ancestor_record_offset":86224,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":97488,"parent_record_offset":97464,"ancestor_record_offset":86224,"inlinee":6105,"source_line":1071,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":536,"record_offset":97516,"parent_record_offset":97488,"ancestor_record_offset":86224,"inlinee":6107,"source_line":3874,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_simple_untyped
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":97544,"parent_record_offset":97516,"ancestor_record_offset":86224,"inlinee":6108,"source_line":1139,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":90948,"parent_record_offset":90912,"ancestor_record_offset":86224,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":536,"record_offset":91112,"parent_record_offset":91084,"ancestor_record_offset":86224,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":536,"record_offset":89600,"parent_record_offset":89572,"ancestor_record_offset":86224,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":536,"record_offset":89628,"parent_record_offset":89600,"ancestor_record_offset":86224,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":87796,"parent_record_offset":87772,"ancestor_record_offset":86224,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":99344,"parent_record_offset":99312,"ancestor_record_offset":86224,"inlinee":7309,"source_line":1013,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":536,"record_offset":92200,"parent_record_offset":89812,"ancestor_record_offset":86224,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":536,"record_offset":92228,"parent_record_offset":92200,"ancestor_record_offset":86224,"inlinee":4362,"source_line":559,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":536,"record_offset":87820,"parent_record_offset":87796,"ancestor_record_offset":86224,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":87988,"parent_record_offset":87964,"ancestor_record_offset":86224,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":536,"record_offset":87268,"parent_record_offset":87244,"ancestor_record_offset":86224,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":87520,"parent_record_offset":87496,"ancestor_record_offset":86224,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":90816,"parent_record_offset":90788,"ancestor_record_offset":86224,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":90168,"parent_record_offset":90144,"ancestor_record_offset":86224,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":87964,"parent_record_offset":87884,"ancestor_record_offset":86224,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":88640,"parent_record_offset":88616,"ancestor_record_offset":86224,"inlinee":28520,"source_line":313,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":87432,"parent_record_offset":87400,"ancestor_record_offset":86224,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":87496,"parent_record_offset":87432,"ancestor_record_offset":86224,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":536,"record_offset":89544,"parent_record_offset":89516,"ancestor_record_offset":86224,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":536,"record_offset":89476,"parent_record_offset":89448,"ancestor_record_offset":86224,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":87660,"parent_record_offset":87636,"ancestor_record_offset":86224,"inlinee":30763,"source_line":200,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":88848,"parent_record_offset":88824,"ancestor_record_offset":86224,"inlinee":30767,"source_line":249,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":536,"record_offset":89416,"parent_record_offset":89392,"ancestor_record_offset":86224,"inlinee":29556,"source_line":189,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":536,"record_offset":91084,"parent_record_offset":89812,"ancestor_record_offset":86224,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::try_slice
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":536,"record_offset":91364,"parent_record_offset":89812,"ancestor_record_offset":86224,"inlinee":9594,"source_line":859,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":536,"record_offset":91152,"parent_record_offset":89812,"ancestor_record_offset":86224,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 165
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":536,"record_offset":22548,"rva":10438224,"va":5379147344}
// 编译实例 RVAs: 0x9f4650
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14014ba20
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9088606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e8a25175ff","size":5,"soft_fail":false,"target_va":5370067488,"va":5379156089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4068182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e8aaecc1ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379149665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=25
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4068766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e862eac1ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379150265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408772e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1569934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e8720be8ff","size":5,"soft_fail":false,"target_va":5377585888,"va":5379155817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140893b10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1452179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e86dd7e9ff","size":5,"soft_fail":false,"target_va":5377702672,"va":5379154846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969720
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-570980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e89c49f7ff","size":5,"soft_fail":false,"target_va":5378578208,"va":5379149183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a15a10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t135730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e832120200","size":5,"soft_fail":false,"target_va":5379283472,"va":5379147737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a161a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t131679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e85f020200","size":5,"soft_fail":false,"target_va":5379285408,"va":5379153724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a16580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t132183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e857040200","size":5,"soft_fail":false,"target_va":5379286400,"va":5379154212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a16960
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t139299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e823200200","size":5,"soft_fail":false,"target_va":5379287392,"va":5379148088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a172d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t140489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e8c9240200","size":5,"soft_fail":false,"target_va":5379289808,"va":5379149314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ade620
//     owner=unresolved_namespace; count=3
//     target_aliases=9 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t957375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e8bf9b0e00","size":5,"soft_fail":false,"target_va":5380105760,"va":5379148380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141466300
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10949028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e8a411a700","size":5,"soft_fail":false,"target_va":5390099200,"va":5379150167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141469230
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10961262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e86e41a700","size":5,"soft_fail":false,"target_va":5390111280,"va":5379150013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32697894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e826eef201","size":5,"soft_fail":false,"target_va":5411847152,"va":5379149253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32692471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e8f7d8f201","size":5,"soft_fail":false,"target_va":5411847184,"va":5379154708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32699492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e864f4f201","size":5,"soft_fail":false,"target_va":5411847200,"va":5379147703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-4797521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e8afcbb6ff","size":5,"soft_fail":false,"target_va":5374356528,"va":5379154044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32385427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e89329ee01","size":5,"soft_fail":false,"target_va":5411540963,"va":5379155531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32386269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e8dd2cee01","size":5,"soft_fail":false,"target_va":5411541225,"va":5379154951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32386333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e81d2dee01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379155438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32327428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e80447ed01","size":5,"soft_fail":false,"target_va":5411482433,"va":5379155000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32387023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e8cf2fee01","size":5,"soft_fail":false,"target_va":5411542064,"va":5379155036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32386832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e8102fee01","size":5,"soft_fail":false,"target_va":5411542296,"va":5379155459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32386704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e8902eee01","size":5,"soft_fail":false,"target_va":5411541904,"va":5379155195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-12547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e8fdceffff","size":5,"soft_fail":false,"target_va":5379135376,"va":5379147918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32387219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e89330ee01","size":5,"soft_fail":false,"target_va":5411542880,"va":5379155656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32387689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e86932ee01","size":5,"soft_fail":false,"target_va":5411543088,"va":5379155394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::history::replay_water::replay_water::closure$2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t52038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e846cb0000","size":5,"soft_fail":false,"target_va":5379200352,"va":5379148309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::cubic_in_out
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2757455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379147344,"raw_hex":"e84f132a00","size":5,"soft_fail":false,"target_va":5381906912,"va":5379149452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*%r12","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5379147344,"raw_hex":"41ffd4","size":3,"soft_fail":false,"va":5379147638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":536,"record_offset":34804,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":24580,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":23008,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":35864,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":36304,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":6099,"source_line":2040,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":536,"record_offset":36164,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":536,"record_offset":23036,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":536,"record_offset":25988,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":536,"record_offset":23592,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":9843,"source_line":63,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":536,"record_offset":33700,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":536,"record_offset":34772,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":536,"record_offset":25924,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":35604,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":23980,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":4614,"source_line":134,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":33504,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":23508,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":34744,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":36044,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":7307,"source_line":990,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":24148,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":536,"record_offset":25684,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":536,"record_offset":25960,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":26195,"source_line":297,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::MultiEdit::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":536,"record_offset":24316,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":30766,"source_line":475,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::history_chain::HistoryEditChain::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":536,"record_offset":24036,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":30764,"source_line":412,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":536,"record_offset":23092,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":29439,"source_line":198,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::iter_points_xz
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":536,"record_offset":25180,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":30770,"source_line":59,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$2::extend_dirty
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":536,"record_offset":25628,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":30773,"source_line":405,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::get_stroke_from_0_to_t
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":26048,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":30775,"source_line":435,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":25152,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":30769,"source_line":39,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_water::replay_water::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_water.rs
//     evidence={"module":536,"record_offset":34832,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":30777,"source_line":43,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":536,"record_offset":24008,"parent_record_offset":22548,"ancestor_record_offset":22548,"inlinee":28304,"source_line":156,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":26856,"parent_record_offset":26832,"ancestor_record_offset":22548,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":26880,"parent_record_offset":26856,"ancestor_record_offset":22548,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":27852,"parent_record_offset":27828,"ancestor_record_offset":22548,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":26832,"parent_record_offset":26648,"ancestor_record_offset":22548,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":27828,"parent_record_offset":27736,"ancestor_record_offset":22548,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":23244,"parent_record_offset":23216,"ancestor_record_offset":22548,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":23120,"parent_record_offset":23092,"ancestor_record_offset":22548,"inlinee":28183,"source_line":1632,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":23148,"parent_record_offset":23120,"ancestor_record_offset":22548,"inlinee":26750,"source_line":715,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":23344,"parent_record_offset":23148,"ancestor_record_offset":22548,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":23188,"parent_record_offset":23148,"ancestor_record_offset":22548,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":536,"record_offset":23216,"parent_record_offset":23188,"ancestor_record_offset":22548,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":23276,"parent_record_offset":23244,"ancestor_record_offset":22548,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":23380,"parent_record_offset":23344,"ancestor_record_offset":22548,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":26580,"parent_record_offset":26548,"ancestor_record_offset":22548,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":23300,"parent_record_offset":23276,"ancestor_record_offset":22548,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":27764,"parent_record_offset":27736,"ancestor_record_offset":22548,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":27736,"parent_record_offset":27712,"ancestor_record_offset":22548,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":23428,"parent_record_offset":23404,"ancestor_record_offset":22548,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":23404,"parent_record_offset":23380,"ancestor_record_offset":22548,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":26648,"parent_record_offset":26612,"ancestor_record_offset":22548,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":26612,"parent_record_offset":26580,"ancestor_record_offset":22548,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":27712,"parent_record_offset":27688,"ancestor_record_offset":22548,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":26692,"parent_record_offset":26648,"ancestor_record_offset":22548,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":536,"record_offset":25288,"parent_record_offset":25180,"ancestor_record_offset":22548,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":24644,"parent_record_offset":24620,"ancestor_record_offset":22548,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":24620,"parent_record_offset":24580,"ancestor_record_offset":22548,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":26964,"parent_record_offset":26936,"ancestor_record_offset":22548,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":36352,"parent_record_offset":36304,"ancestor_record_offset":22548,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":27636,"parent_record_offset":27596,"ancestor_record_offset":22548,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":34988,"parent_record_offset":34952,"ancestor_record_offset":22548,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":36568,"parent_record_offset":36304,"ancestor_record_offset":22548,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":26516,"parent_record_offset":26324,"ancestor_record_offset":22548,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":26548,"parent_record_offset":26516,"ancestor_record_offset":22548,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":26244,"parent_record_offset":26204,"ancestor_record_offset":22548,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":27144,"parent_record_offset":27116,"ancestor_record_offset":22548,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":36944,"parent_record_offset":36916,"ancestor_record_offset":22548,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":536,"record_offset":36624,"parent_record_offset":36600,"ancestor_record_offset":22548,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":536,"record_offset":36672,"parent_record_offset":36648,"ancestor_record_offset":22548,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":536,"record_offset":27212,"parent_record_offset":27144,"ancestor_record_offset":22548,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":536,"record_offset":26936,"parent_record_offset":26324,"ancestor_record_offset":22548,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":536,"record_offset":26284,"parent_record_offset":26244,"ancestor_record_offset":22548,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":536,"record_offset":26324,"parent_record_offset":26284,"ancestor_record_offset":22548,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":536,"record_offset":23060,"parent_record_offset":23036,"ancestor_record_offset":22548,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":536,"record_offset":34888,"parent_record_offset":34864,"ancestor_record_offset":22548,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":536,"record_offset":34952,"parent_record_offset":34888,"ancestor_record_offset":22548,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":536,"record_offset":34864,"parent_record_offset":34832,"ancestor_record_offset":22548,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":536,"record_offset":26716,"parent_record_offset":26692,"ancestor_record_offset":22548,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":536,"record_offset":26740,"parent_record_offset":26716,"ancestor_record_offset":22548,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":536,"record_offset":27472,"parent_record_offset":27448,"ancestor_record_offset":22548,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":536,"record_offset":27496,"parent_record_offset":27472,"ancestor_record_offset":22548,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":536,"record_offset":25752,"parent_record_offset":25712,"ancestor_record_offset":22548,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":536,"record_offset":27536,"parent_record_offset":27384,"ancestor_record_offset":22548,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":536,"record_offset":27564,"parent_record_offset":27384,"ancestor_record_offset":22548,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::typed_swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":536,"record_offset":33724,"parent_record_offset":33700,"ancestor_record_offset":22548,"inlinee":6104,"source_line":4004,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":536,"record_offset":26788,"parent_record_offset":26764,"ancestor_record_offset":22548,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":536,"record_offset":27088,"parent_record_offset":27048,"ancestor_record_offset":22548,"inlinee":9582,"source_line":30,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":536,"record_offset":27020,"parent_record_offset":26992,"ancestor_record_offset":22548,"inlinee":9580,"source_line":71,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":536,"record_offset":26376,"parent_record_offset":26324,"ancestor_record_offset":22548,"inlinee":11428,"source_line":58,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":536,"record_offset":23640,"parent_record_offset":23616,"ancestor_record_offset":22548,"inlinee":6091,"source_line":73,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":536,"record_offset":23848,"parent_record_offset":23664,"ancestor_record_offset":22548,"inlinee":6094,"source_line":85,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":26204,"parent_record_offset":26048,"ancestor_record_offset":22548,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":23616,"parent_record_offset":23592,"ancestor_record_offset":22548,"inlinee":6090,"source_line":2852,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":23876,"parent_record_offset":23848,"ancestor_record_offset":22548,"inlinee":6096,"source_line":2859,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":26992,"parent_record_offset":26964,"ancestor_record_offset":22548,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":27116,"parent_record_offset":27088,"ancestor_record_offset":22548,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":23664,"parent_record_offset":23640,"ancestor_record_offset":22548,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":536,"record_offset":36124,"parent_record_offset":36076,"ancestor_record_offset":22548,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":536,"record_offset":26764,"parent_record_offset":26740,"ancestor_record_offset":22548,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":536,"record_offset":28404,"parent_record_offset":26048,"ancestor_record_offset":22548,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":536,"record_offset":27796,"parent_record_offset":27764,"ancestor_record_offset":22548,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":536,"record_offset":23900,"parent_record_offset":23876,"ancestor_record_offset":22548,"inlinee":4194,"source_line":293,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":536,"record_offset":27448,"parent_record_offset":27424,"ancestor_record_offset":22548,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":536,"record_offset":27424,"parent_record_offset":27384,"ancestor_record_offset":22548,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":536,"record_offset":26472,"parent_record_offset":26448,"ancestor_record_offset":22548,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=55
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":24760,"parent_record_offset":24580,"ancestor_record_offset":22548,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":536,"record_offset":23556,"parent_record_offset":23532,"ancestor_record_offset":22548,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":536,"record_offset":26448,"parent_record_offset":26424,"ancestor_record_offset":22548,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":23808,"parent_record_offset":23784,"ancestor_record_offset":22548,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":36244,"parent_record_offset":36164,"ancestor_record_offset":22548,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":26424,"parent_record_offset":26400,"ancestor_record_offset":22548,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":23732,"parent_record_offset":23696,"ancestor_record_offset":22548,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":33960,"parent_record_offset":33804,"ancestor_record_offset":22548,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":33748,"parent_record_offset":33724,"ancestor_record_offset":22548,"inlinee":6105,"source_line":1071,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":536,"record_offset":33776,"parent_record_offset":33748,"ancestor_record_offset":22548,"inlinee":6107,"source_line":3874,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_simple_untyped
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":33804,"parent_record_offset":33776,"ancestor_record_offset":22548,"inlinee":6108,"source_line":1139,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":27180,"parent_record_offset":27144,"ancestor_record_offset":22548,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":536,"record_offset":27344,"parent_record_offset":27316,"ancestor_record_offset":22548,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":536,"record_offset":25836,"parent_record_offset":25808,"ancestor_record_offset":22548,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":536,"record_offset":25864,"parent_record_offset":25836,"ancestor_record_offset":22548,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":24060,"parent_record_offset":24036,"ancestor_record_offset":22548,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":536,"record_offset":36076,"parent_record_offset":36044,"ancestor_record_offset":22548,"inlinee":7309,"source_line":1013,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":536,"record_offset":28432,"parent_record_offset":26048,"ancestor_record_offset":22548,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":536,"record_offset":28460,"parent_record_offset":28432,"ancestor_record_offset":22548,"inlinee":4362,"source_line":559,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":536,"record_offset":24084,"parent_record_offset":24060,"ancestor_record_offset":22548,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":24252,"parent_record_offset":24228,"ancestor_record_offset":22548,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":536,"record_offset":23532,"parent_record_offset":23508,"ancestor_record_offset":22548,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":23784,"parent_record_offset":23760,"ancestor_record_offset":22548,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":27048,"parent_record_offset":27020,"ancestor_record_offset":22548,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":26400,"parent_record_offset":26376,"ancestor_record_offset":22548,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":24228,"parent_record_offset":24148,"ancestor_record_offset":22548,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":24904,"parent_record_offset":24880,"ancestor_record_offset":22548,"inlinee":28520,"source_line":313,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":23696,"parent_record_offset":23664,"ancestor_record_offset":22548,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":23760,"parent_record_offset":23696,"ancestor_record_offset":22548,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":536,"record_offset":25780,"parent_record_offset":25752,"ancestor_record_offset":22548,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":536,"record_offset":35540,"parent_record_offset":35516,"ancestor_record_offset":22548,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":536,"record_offset":35564,"parent_record_offset":35540,"ancestor_record_offset":22548,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":536,"record_offset":25712,"parent_record_offset":25684,"ancestor_record_offset":22548,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":23924,"parent_record_offset":23900,"ancestor_record_offset":22548,"inlinee":30763,"source_line":200,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":536,"record_offset":25112,"parent_record_offset":25088,"ancestor_record_offset":22548,"inlinee":30767,"source_line":249,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback::on_terrain_modified
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/terrain/delayed_terrain_changed_msg.rs
//     evidence={"module":536,"record_offset":35516,"parent_record_offset":34832,"ancestor_record_offset":22548,"inlinee":29405,"source_line":39,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":536,"record_offset":25652,"parent_record_offset":25628,"ancestor_record_offset":22548,"inlinee":29556,"source_line":189,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":536,"record_offset":27316,"parent_record_offset":26048,"ancestor_record_offset":22548,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::try_slice
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":536,"record_offset":27596,"parent_record_offset":26048,"ancestor_record_offset":22548,"inlinee":9594,"source_line":859,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":536,"record_offset":27384,"parent_record_offset":26048,"ancestor_record_offset":22548,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::replay_raster::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 200
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x782B
// 首条位置: {"module":536,"inlinee":30763}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::replay_raster::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 249
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x782F
// 首条位置: {"module":536,"inlinee":30767}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::replay_raster::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=20
// 源码行: 342
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7828
// 首条位置: {"module":536,"inlinee":30760}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::get_stroke_from_0_to_t
// 记录形式: inline_instance, inlinee_declaration; instances=13
// 源码行: 435
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7837
// 首条位置: {"module":536,"inlinee":30775}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":536,"record_offset":10464,"parent_record_offset":10336,"ancestor_record_offset":10336,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":10492,"parent_record_offset":10336,"ancestor_record_offset":10336,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":536,"record_offset":12692,"parent_record_offset":10336,"ancestor_record_offset":10336,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=60
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":12192,"parent_record_offset":10336,"ancestor_record_offset":10336,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":536,"record_offset":12720,"parent_record_offset":10336,"ancestor_record_offset":10336,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":536,"record_offset":11604,"parent_record_offset":10336,"ancestor_record_offset":10336,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::try_slice
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":536,"record_offset":11884,"parent_record_offset":10336,"ancestor_record_offset":10336,"inlinee":9594,"source_line":859,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":536,"record_offset":11672,"parent_record_offset":10336,"ancestor_record_offset":10336,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":11144,"parent_record_offset":11120,"ancestor_record_offset":10336,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":11168,"parent_record_offset":11144,"ancestor_record_offset":10336,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=96
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":12140,"parent_record_offset":12116,"ancestor_record_offset":10336,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":11120,"parent_record_offset":10936,"ancestor_record_offset":10336,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=96
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":536,"record_offset":12116,"parent_record_offset":12024,"ancestor_record_offset":10336,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":10868,"parent_record_offset":10836,"ancestor_record_offset":10336,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=96
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":12052,"parent_record_offset":12024,"ancestor_record_offset":10336,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=96
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":12024,"parent_record_offset":12000,"ancestor_record_offset":10336,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":10936,"parent_record_offset":10900,"ancestor_record_offset":10336,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":10900,"parent_record_offset":10868,"ancestor_record_offset":10336,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=96
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":12000,"parent_record_offset":11976,"ancestor_record_offset":10336,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":536,"record_offset":10980,"parent_record_offset":10936,"ancestor_record_offset":10336,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":11252,"parent_record_offset":11224,"ancestor_record_offset":10336,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":11924,"parent_record_offset":11884,"ancestor_record_offset":10336,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":10804,"parent_record_offset":10612,"ancestor_record_offset":10336,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":10836,"parent_record_offset":10804,"ancestor_record_offset":10336,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":10532,"parent_record_offset":10492,"ancestor_record_offset":10336,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":11432,"parent_record_offset":11404,"ancestor_record_offset":10336,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":536,"record_offset":11500,"parent_record_offset":11432,"ancestor_record_offset":10336,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":536,"record_offset":11224,"parent_record_offset":10612,"ancestor_record_offset":10336,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":536,"record_offset":10572,"parent_record_offset":10532,"ancestor_record_offset":10336,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":536,"record_offset":10612,"parent_record_offset":10572,"ancestor_record_offset":10336,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":536,"record_offset":11004,"parent_record_offset":10980,"ancestor_record_offset":10336,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":536,"record_offset":11028,"parent_record_offset":11004,"ancestor_record_offset":10336,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":536,"record_offset":11760,"parent_record_offset":11736,"ancestor_record_offset":10336,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":536,"record_offset":11784,"parent_record_offset":11760,"ancestor_record_offset":10336,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":536,"record_offset":11824,"parent_record_offset":11672,"ancestor_record_offset":10336,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":536,"record_offset":11852,"parent_record_offset":11672,"ancestor_record_offset":10336,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":536,"record_offset":11076,"parent_record_offset":11052,"ancestor_record_offset":10336,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":536,"record_offset":11376,"parent_record_offset":11336,"ancestor_record_offset":10336,"inlinee":9582,"source_line":30,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":536,"record_offset":11308,"parent_record_offset":11280,"ancestor_record_offset":10336,"inlinee":9580,"source_line":71,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::size_hint
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":536,"record_offset":10664,"parent_record_offset":10612,"ancestor_record_offset":10336,"inlinee":11428,"source_line":58,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":11280,"parent_record_offset":11252,"ancestor_record_offset":10336,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":536,"record_offset":11404,"parent_record_offset":11376,"ancestor_record_offset":10336,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":536,"record_offset":11052,"parent_record_offset":11028,"ancestor_record_offset":10336,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=96
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":536,"record_offset":12084,"parent_record_offset":12052,"ancestor_record_offset":10336,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":536,"record_offset":11736,"parent_record_offset":11712,"ancestor_record_offset":10336,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":536,"record_offset":11712,"parent_record_offset":11672,"ancestor_record_offset":10336,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":536,"record_offset":10760,"parent_record_offset":10736,"ancestor_record_offset":10336,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=168
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":11952,"parent_record_offset":11884,"ancestor_record_offset":10336,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":536,"record_offset":10736,"parent_record_offset":10712,"ancestor_record_offset":10336,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":11540,"parent_record_offset":11336,"ancestor_record_offset":10336,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":536,"record_offset":10712,"parent_record_offset":10688,"ancestor_record_offset":10336,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":536,"record_offset":11468,"parent_record_offset":11432,"ancestor_record_offset":10336,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":536,"record_offset":11632,"parent_record_offset":11604,"ancestor_record_offset":10336,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":536,"record_offset":12748,"parent_record_offset":12720,"ancestor_record_offset":10336,"inlinee":4362,"source_line":559,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":11336,"parent_record_offset":11308,"ancestor_record_offset":10336,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":536,"record_offset":10688,"parent_record_offset":10664,"ancestor_record_offset":10336,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/00536.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::duration_based_on_stroke_length
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 472
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12BE6
// 首条位置: {"module":1682,"inlinee":76774}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1682,"record_offset":159656,"parent_record_offset":159632,"ancestor_record_offset":159632,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01682.jsonl.gz"}
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
// 完整结构及原始类型记录: replay_raster.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::history::replay_raster::RasterReplayCmd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 2
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::history::replay_raster::RasterReplayResources
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 151
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands>", "constructor": "tuple$", "arguments": ["country_core::systems::history::replay_raster::RasterReplayResources", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>", "bevy_ecs::system::commands::Commands"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystemState<tuple$<country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands> >", "constructor": "bevy_ecs::system::function_system::FunctionSystemState", "arguments": ["tuple$<country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystemState<tuple$<country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystemState<tuple$<country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::initialize", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::initialize", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::initialize", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::initialize", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::initialize", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::run_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::run_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::run_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::run_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::run_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::get_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::get_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::get_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::get_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::set_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::set_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::set_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::set_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::apply_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::apply_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::queue_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::queue_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::check_change_tick<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::check_change_tick", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::check_change_tick<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::check_change_tick", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::name<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::name", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::name<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::name", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> > >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)>", "constructor": "tuple$", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::Infallible", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::Infallible", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::Infallible", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::Infallible", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::Infallible", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::Infallible", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)> > >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::ScheduleConfigTupleMarker", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<tuple$<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)> > >,country_core::systems::GameSystemSet>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set", "arguments": ["tuple$<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)>", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)> > >", "country_core::systems::GameSystemSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::default_system_sets<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::default_system_sets", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::default_system_sets<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::default_system_sets", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::default_system_sets<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::default_system_sets", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::default_system_sets<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::default_system_sets", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::default_system_sets<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::default_system_sets", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::default_system_sets<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::default_system_sets", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> > >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> > >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> > >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> > >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> > >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> > >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::initialize", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::run_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::get_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::get_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::set_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::set_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::apply_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::queue_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::check_change_tick<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::check_change_tick", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::name<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::name", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::impl$4::into_configs<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::schedule::config::impl$4::into_configs", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> > >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: replay_raster.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-camera/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::systems::history::replay_raster::RasterReplayResources", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: replay_raster.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 221; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>,bevy_ecs::event::reader::EventReader<country_core::systems::backend::icc::IccProfileChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>", "bevy_ecs::event::reader::EventReader<country_core::systems::backend::icc::IccProfileChanged>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::backend::icc::IccProfileChanged>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::reader::EventReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::history::history_process::DebugHistoryMsg>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::NewReplayCommandEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::timelapse_replay::InterruptTimelapse>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::event::reader::EventReader<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::history::history_process::DebugHistoryMsg>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>", "bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::NewReplayCommandEvent>", "bevy_ecs::event::writer::EventWriter<country_core::systems::timelapse_replay::InterruptTimelapse>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::history::history_process::DebugHistoryMsg>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::NewReplayCommandEvent>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::timelapse_replay::InterruptTimelapse>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::reader::EventReader<country_core::systems::history::history_process::NewReplayCommandEvent>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::event::reader::EventReader<country_core::systems::history::history_process::NewReplayCommandEvent>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::history::history_process::NewReplayCommandEvent>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_draw_terrain::CacheMouseRaycastHeight>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>", "bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>", "bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >", "bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>", "bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_draw_terrain::CacheMouseRaycastHeight>", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_draw_terrain::CacheMouseRaycastHeight>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::RasterReplayCmd::backward
// 信息 ID: 238af0cf909e119c25eeae1d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::RasterReplayCmd::backward_instant
// 信息 ID: c8641c3cf4bf73848dc4e1d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 72 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=106324; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=107040; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=107536; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=108516; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=109012; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=109508; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=111644; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=112844; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::RasterReplayCmd::forward
// 信息 ID: 288d25504dcb4307ff757953
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::RasterReplayCmd::forward_instant
// 信息 ID: 0fee832554ad8d03f6121efc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 72 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=106408; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=107124; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=107620; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=108600; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=109096; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=109592; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=111728; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=112928; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::RasterReplayCmd::new
// 信息 ID: 8ddc7f3a65275243b0b6a669
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 146 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=106348; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=106432; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=107064; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=107148; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=107560; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=107644; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=108540; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=108624; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::_::impl$0::get_param
// 信息 ID: 4f9ce184a9e5af11eb3902bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1dc1859ef49d43f052d66862: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)
// [泛型宿主/闭包/直接引用方参数候选] 724590ef75324dbbc996662f: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 7c5ce92838e852174aa069b7: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)
// [泛型宿主/闭包/直接引用方参数候选] beeb1749b20c1ef79c4b27a8: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] bfbf3f13a58ea0ca42a281f1: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] d55efa96b8c611f52326f04f: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 11 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > @ 0xae166e -> 0xae5830 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> > @ 0xae187e -> 0xae5830 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> > @ 0xae1a8e -> 0xae5830 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> > @ 0xae1c99 -> 0xae5830 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> > @ 0xae217e -> 0xae5830 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> > > @ 0x14c4ae3 -> 0x1507610 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> > > @ 0x14c7111 -> 0x1507610 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> > > @ 0x14daec3 -> 0x1507610 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> > > @ 0x14dfd51 -> 0x1507610 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> > > @ 0x14eab03 -> 0x1507610 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > > @ 0x14ebde3 -> 0x1507610 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >; module=1708, inline_record=35500; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::_::impl$0::init_state
// 信息 ID: 717529a97d2012f204822602
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1dc1859ef49d43f052d66862: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)
// [泛型宿主/闭包/直接引用方参数候选] 724590ef75324dbbc996662f: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 7c5ce92838e852174aa069b7: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)
// [泛型宿主/闭包/直接引用方参数候选] beeb1749b20c1ef79c4b27a8: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] bfbf3f13a58ea0ca42a281f1: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] d55efa96b8c611f52326f04f: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >; module=569, inline_record=964; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >; module=569, inline_record=1988; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >; module=569, inline_record=3012; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >; module=569, inline_record=5572; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >; module=569, inline_record=7228; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >; module=1708, inline_record=1080; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::_::impl$0::validate_param
// 信息 ID: 9abb0c3ec7238a9145b00ead
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1dc1859ef49d43f052d66862: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)
// [泛型宿主/闭包/直接引用方参数候选] 724590ef75324dbbc996662f: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 7c5ce92838e852174aa069b7: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)
// [泛型宿主/闭包/直接引用方参数候选] beeb1749b20c1ef79c4b27a8: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] bfbf3f13a58ea0ca42a281f1: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] d55efa96b8c611f52326f04f: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >; module=569, inline_record=55032; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >; module=569, inline_record=59668; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >; module=569, inline_record=62828; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >; module=569, inline_record=66004; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >; module=569, inline_record=69180; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> > >; module=1611, inline_record=619568; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> > >; module=1611, inline_record=627476; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> > >; module=1611, inline_record=832700; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::duration_based_on_stroke_length
// 信息 ID: 250a7c6456b9e48d0b83b13d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_raster::RasterReplayCmd::forward; module=1682, inline_record=159632; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::RasterReplayCmd::backward; module=1682, inline_record=160480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::get_stroke_from_0_to_t
// 信息 ID: c663249239910d39429b0369
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >; module=536, inline_record=10336; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >; module=536, inline_record=14016; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=536, inline_record=26048; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=536, inline_record=29728; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>; module=536, inline_record=42292; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>; module=536, inline_record=45972; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >; module=536, inline_record=58196; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >; module=536, inline_record=61876; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::impl$0::from
// 信息 ID: 3ed874225a137698949eb828
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 24 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=106052; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=106768; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=107264; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=108240; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=108740; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=109236; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::history_process::replay_edit::closure_env$0>; module=1602, inline_record=123292; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::history_process::replay_edit::closure_env$0>; module=1602, inline_record=124008; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::impl$6::eq
// 信息 ID: a3b0bac62fa6e62c60bf7756
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >; module=536, inline_record=9412; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=536, inline_record=25152; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>; module=536, inline_record=41368; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >; module=536, inline_record=57272; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> >; module=536, inline_record=73088; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >; module=536, inline_record=88888; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::impl$8::default
// 信息 ID: 538460ca940071e8d8fee74c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 724590ef75324dbbc996662f: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] beeb1749b20c1ef79c4b27a8: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] bfbf3f13a58ea0ca42a281f1: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] d55efa96b8c611f52326f04f: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >; module=569, inline_record=1060; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >; module=569, inline_record=2084; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >; module=569, inline_record=3108; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >; module=569, inline_record=5668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::replay_raster::closure$0
// 信息 ID: d0f7d20286f0de57ecd76bad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::history::replay_raster::replay_raster", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >; module=536, inline_record=8184; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=536, inline_record=23924; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>; module=536, inline_record=40140; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >; module=536, inline_record=56044; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> >; module=536, inline_record=71860; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >; module=536, inline_record=87660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::replay_raster::closure$1
// 信息 ID: b99a2cbd04a1549713e7aa45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::history::replay_raster::replay_raster", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >; module=536, inline_record=9372; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=536, inline_record=25112; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>; module=536, inline_record=41328; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >; module=536, inline_record=57232; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> >; module=536, inline_record=73048; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >; module=536, inline_record=88848; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::replay_raster::closure$2
// 信息 ID: 9540a7ddcdf8b3307c235d5c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::history::replay_raster::replay_raster", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::history::replay_raster::replay_raster::closure_env$2 <- country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_raster::replay_raster::closure_env$2 <- country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_raster::replay_raster::closure_env$2 <- country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_raster::replay_raster::closure_env$2 <- country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_raster::replay_raster::closure_env$2 <- country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_raster::replay_raster::closure_env$2 <- country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_raster::replay_raster::closure_env$2 <- conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_raster::replay_raster::closure_env$2 <- conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 8 条。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > > >; module=568, inline_record=39616; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > > >; module=568, inline_record=39904; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > > >; module=568, inline_record=41376; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > > >; module=568, inline_record=41664; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > >; module=568, inline_record=43120; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > >; module=568, inline_record=43408; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> > >; module=568, inline_record=44804; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> > >; module=568, inline_record=45092; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> >
// 信息 ID: 429132c6ef9fdcf6b542d2a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 724590ef75324dbbc996662f: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_path::replay_path::closure_env$1 -> ["country_core::systems::history::replay_path::Cobblestone1PathFilter"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster -> ["country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> > @ 0xae2278 -> 0x9fc290 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> > > @ 0x14dafbd -> 0x9fc290 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >
// 信息 ID: dd1c6671505bceb729d72a5d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] bfbf3f13a58ea0ca42a281f1: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_path::replay_path::closure_env$1 -> ["country_core::systems::history::replay_path::Cobblestone2PathFilter"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster -> ["country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> > @ 0xae1b88 -> 0x9f1cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> > > @ 0x14c4bdd -> 0x9f1cf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >
// 信息 ID: 716f7adf6a59c9210ca26b7b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d55efa96b8c611f52326f04f: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_path::replay_path::closure_env$1 -> ["country_core::systems::history::replay_path::Cobblestone3PathFilter"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster -> ["country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> > @ 0xae1978 -> 0x9f9930 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> > > @ 0x14eabfd -> 0x9f9930 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>
// 信息 ID: 7eabfa431ff78276a93b5cff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 7c5ce92838e852174aa069b7: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster -> ["country_core::resources::world_raster_defs::GardenRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_garden::replay_garden::closure_env$2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> > > @ 0x14c7283 -> 0x9f6ff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> > @ 0x1768d0f -> 0x9f6ff0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >
// 信息 ID: ddb3ac9779b6ede5de4be1a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] beeb1749b20c1ef79c4b27a8: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_path::replay_path::closure_env$1 -> ["country_core::systems::history::replay_path::DirtPathFilter"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster -> ["country_core::resources::world_raster_defs::PathRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > @ 0xae1768 -> 0x9febf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > > @ 0x14ebedd -> 0x9febf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>
// 信息 ID: 08cecc2916e4858df2d632cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1dc1859ef49d43f052d66862: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster -> ["country_core::resources::world_raster_defs::WaterRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_water::replay_water::closure_env$2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> > @ 0xae1f7f -> 0x9f4650 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::writer::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::writer::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> > > @ 0x14e0037 -> 0x9f4650 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

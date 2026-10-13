// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/history/replay_raster.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: b795f7c984b79a10ee74f6679b27cfdabd4cc643317dc4903d07da02286a2129
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: replay_raster.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 416, 421, 484, 485, 491, 1321, 1339, 1426, 1429, 1431
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::RasterReplayCmd::forward
// 记录形式: compiled_procedure; instances=1
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":485,"record_offset":248,"rva":6879664,"va":5375588784}
// 编译实例 RVAs: 0x68f9b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d04d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3929666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375588784,"raw_hex":"e8be09c4ff","size":5,"soft_fail":false,"target_va":5371659472,"va":5375589133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27390274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375588784,"raw_hex":"e842f1a101","size":5,"soft_fail":false,"target_va":5402979344,"va":5375589065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":485,"record_offset":1112,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":485,"record_offset":416,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":26026,"source_line":37,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::RasterReplayCmd::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":485,"record_offset":1084,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":27306,"source_line":56,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::duration_based_on_stroke_length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":485,"record_offset":1028,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":27346,"source_line":397,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":485,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":248,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":485,"record_offset":552,"parent_record_offset":416,"ancestor_record_offset":248,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":485,"record_offset":512,"parent_record_offset":484,"ancestor_record_offset":248,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":485,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":248,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":485,"record_offset":484,"parent_record_offset":456,"ancestor_record_offset":248,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":485,"record_offset":456,"parent_record_offset":416,"ancestor_record_offset":248,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":485,"record_offset":808,"parent_record_offset":768,"ancestor_record_offset":248,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":580,"parent_record_offset":552,"ancestor_record_offset":248,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":608,"parent_record_offset":580,"ancestor_record_offset":248,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":485,"record_offset":1172,"parent_record_offset":1136,"ancestor_record_offset":248,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":485,"record_offset":1196,"parent_record_offset":1172,"ancestor_record_offset":248,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":485,"record_offset":1228,"parent_record_offset":1136,"ancestor_record_offset":248,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":485,"record_offset":744,"parent_record_offset":416,"ancestor_record_offset":248,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":485,"record_offset":836,"parent_record_offset":768,"ancestor_record_offset":248,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":485,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":248,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":485,"record_offset":872,"parent_record_offset":836,"ancestor_record_offset":248,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::RasterReplayCmd::forward_instant
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6AB5
// 首条位置: {"module":484,"inlinee":27317}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::history::replay_raster::RasterReplayCmd::new
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":484,"record_offset":17588,"parent_record_offset":17564,"ancestor_record_offset":17564,"inlinee":27306,"source_line":56,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::RasterReplayCmd::backward
// 记录形式: compiled_procedure; instances=1
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":485,"record_offset":1268,"rva":6880032,"va":5375589152}
// 编译实例 RVAs: 0x68fb20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d04d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3930034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375589152,"raw_hex":"e84e08c4ff","size":5,"soft_fail":false,"target_va":5371659472,"va":5375589501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27389906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375589152,"raw_hex":"e8d2efa101","size":5,"soft_fail":false,"target_va":5402979344,"va":5375589433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":485,"record_offset":2132,"parent_record_offset":1268,"ancestor_record_offset":1268,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":485,"record_offset":1436,"parent_record_offset":1268,"ancestor_record_offset":1268,"inlinee":26026,"source_line":37,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::RasterReplayCmd::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":485,"record_offset":2104,"parent_record_offset":1268,"ancestor_record_offset":1268,"inlinee":27306,"source_line":56,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::duration_based_on_stroke_length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":485,"record_offset":2048,"parent_record_offset":1268,"ancestor_record_offset":1268,"inlinee":27346,"source_line":397,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":485,"record_offset":2156,"parent_record_offset":2132,"ancestor_record_offset":1268,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":485,"record_offset":1572,"parent_record_offset":1436,"ancestor_record_offset":1268,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":485,"record_offset":1532,"parent_record_offset":1504,"ancestor_record_offset":1268,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":485,"record_offset":2072,"parent_record_offset":2048,"ancestor_record_offset":1268,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":485,"record_offset":1504,"parent_record_offset":1476,"ancestor_record_offset":1268,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":485,"record_offset":1476,"parent_record_offset":1436,"ancestor_record_offset":1268,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":485,"record_offset":1828,"parent_record_offset":1788,"ancestor_record_offset":1268,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":1600,"parent_record_offset":1572,"ancestor_record_offset":1268,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":1628,"parent_record_offset":1600,"ancestor_record_offset":1268,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":485,"record_offset":2192,"parent_record_offset":2156,"ancestor_record_offset":1268,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":485,"record_offset":2216,"parent_record_offset":2192,"ancestor_record_offset":1268,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":485,"record_offset":2248,"parent_record_offset":2156,"ancestor_record_offset":1268,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":485,"record_offset":1764,"parent_record_offset":1436,"ancestor_record_offset":1268,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":485,"record_offset":1856,"parent_record_offset":1788,"ancestor_record_offset":1268,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":485,"record_offset":1788,"parent_record_offset":1764,"ancestor_record_offset":1268,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":485,"record_offset":1892,"parent_record_offset":1856,"ancestor_record_offset":1268,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::RasterReplayCmd::backward_instant
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6AA9
// 首条位置: {"module":484,"inlinee":27305}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::history::replay_raster::RasterReplayCmd::new
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":484,"record_offset":17504,"parent_record_offset":17480,"ancestor_record_offset":17480,"inlinee":27306,"source_line":56,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::RasterReplayCmd::new
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6AAA
// 首条位置: {"module":484,"inlinee":27306}
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
// 记录形式: compiled_procedure; instances=1
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":485,"record_offset":49304,"rva":6907952,"va":5375617072}
// 编译实例 RVAs: 0x696830
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27361273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375617072,"raw_hex":"e8f97fa101","size":5,"soft_fail":false,"target_va":5402978944,"va":5375617666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$89::get_param
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":485,"record_offset":49456,"parent_record_offset":49304,"ancestor_record_offset":49304,"inlinee":6811,"source_line":1466,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":485,"record_offset":52072,"parent_record_offset":52048,"ancestor_record_offset":49304,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":485,"record_offset":52048,"parent_record_offset":52012,"ancestor_record_offset":49304,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":485,"record_offset":49996,"parent_record_offset":49960,"ancestor_record_offset":49304,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":485,"record_offset":49932,"parent_record_offset":49544,"ancestor_record_offset":49304,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":485,"record_offset":49960,"parent_record_offset":49932,"ancestor_record_offset":49304,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":485,"record_offset":49576,"parent_record_offset":49544,"ancestor_record_offset":49304,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":485,"record_offset":50024,"parent_record_offset":49960,"ancestor_record_offset":49304,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":49624,"parent_record_offset":49600,"ancestor_record_offset":49304,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":49600,"parent_record_offset":49576,"ancestor_record_offset":49304,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":49716,"parent_record_offset":49652,"ancestor_record_offset":49304,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":485,"record_offset":49800,"parent_record_offset":49776,"ancestor_record_offset":49304,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$4::get_param
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":485,"record_offset":49508,"parent_record_offset":49456,"ancestor_record_offset":49304,"inlinee":4356,"source_line":525,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$4::get_param::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":485,"record_offset":50092,"parent_record_offset":50068,"ancestor_record_offset":49304,"inlinee":4403,"source_line":533,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$7::get_param
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":485,"record_offset":51456,"parent_record_offset":49456,"ancestor_record_offset":49304,"inlinee":4382,"source_line":620,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$7::get_param::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":485,"record_offset":52256,"parent_record_offset":52232,"ancestor_record_offset":49304,"inlinee":4407,"source_line":628,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":485,"record_offset":52140,"parent_record_offset":52116,"ancestor_record_offset":49304,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":52116,"parent_record_offset":51500,"ancestor_record_offset":49304,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":51500,"parent_record_offset":51456,"ancestor_record_offset":49304,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_with_ticks
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":485,"record_offset":49544,"parent_record_offset":49508,"ancestor_record_offset":49304,"inlinee":4357,"source_line":533,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":485,"record_offset":50120,"parent_record_offset":50092,"ancestor_record_offset":49304,"inlinee":4405,"source_line":699,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":485,"record_offset":52332,"parent_record_offset":52256,"ancestor_record_offset":49304,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":49736,"parent_record_offset":49716,"ancestor_record_offset":49304,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":49776,"parent_record_offset":49600,"ancestor_record_offset":49304,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":485,"record_offset":50068,"parent_record_offset":49508,"ancestor_record_offset":49304,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":485,"record_offset":49880,"parent_record_offset":49856,"ancestor_record_offset":49304,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":485,"record_offset":49652,"parent_record_offset":49624,"ancestor_record_offset":49304,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":49688,"parent_record_offset":49652,"ancestor_record_offset":49304,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":485,"record_offset":49856,"parent_record_offset":49824,"ancestor_record_offset":49304,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":485,"record_offset":52164,"parent_record_offset":52140,"ancestor_record_offset":49304,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":485,"record_offset":52188,"parent_record_offset":52164,"ancestor_record_offset":49304,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":485,"record_offset":49824,"parent_record_offset":49800,"ancestor_record_offset":49304,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00485.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::_::impl$0::init_state
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6AF4
// 首条位置: {"module":491,"inlinee":27380}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$89::init_state
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":491,"record_offset":1036,"parent_record_offset":1012,"ancestor_record_offset":1012,"inlinee":6810,"source_line":1443,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$17::init_state
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":491,"record_offset":1064,"parent_record_offset":1036,"ancestor_record_offset":1012,"inlinee":9053,"source_line":839,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$8::from_world
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":491,"record_offset":1092,"parent_record_offset":1064,"ancestor_record_offset":1012,"inlinee":7177,"source_line":2804,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":491,"record_offset":1440,"parent_record_offset":1412,"ancestor_record_offset":1012,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":491,"record_offset":1492,"parent_record_offset":1468,"ancestor_record_offset":1012,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":491,"record_offset":1468,"parent_record_offset":1412,"ancestor_record_offset":1012,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":491,"record_offset":1516,"parent_record_offset":1492,"ancestor_record_offset":1012,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":491,"record_offset":1368,"parent_record_offset":1344,"ancestor_record_offset":1012,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":491,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":1012,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::impl$2::default
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":491,"record_offset":1120,"parent_record_offset":1092,"ancestor_record_offset":1012,"inlinee":23742,"source_line":78,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":491,"record_offset":1148,"parent_record_offset":1120,"ancestor_record_offset":1012,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":491,"record_offset":1204,"parent_record_offset":1176,"ancestor_record_offset":1012,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":491,"record_offset":1292,"parent_record_offset":1260,"ancestor_record_offset":1012,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":491,"record_offset":1412,"parent_record_offset":1260,"ancestor_record_offset":1012,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":491,"record_offset":1176,"parent_record_offset":1148,"ancestor_record_offset":1012,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":491,"record_offset":1316,"parent_record_offset":1292,"ancestor_record_offset":1012,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":491,"record_offset":1344,"parent_record_offset":1316,"ancestor_record_offset":1012,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":491,"record_offset":1260,"parent_record_offset":1232,"ancestor_record_offset":1012,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":491,"record_offset":1232,"parent_record_offset":1204,"ancestor_record_offset":1012,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::impl$2::default
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5CBE
// 首条位置: {"module":416,"inlinee":23742}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":491,"record_offset":1148,"parent_record_offset":1120,"ancestor_record_offset":1120,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":491,"record_offset":1440,"parent_record_offset":1412,"ancestor_record_offset":1120,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":491,"record_offset":1492,"parent_record_offset":1468,"ancestor_record_offset":1120,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":491,"record_offset":1468,"parent_record_offset":1412,"ancestor_record_offset":1120,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":491,"record_offset":1516,"parent_record_offset":1492,"ancestor_record_offset":1120,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":491,"record_offset":1368,"parent_record_offset":1344,"ancestor_record_offset":1120,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":491,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":1120,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":491,"record_offset":1204,"parent_record_offset":1176,"ancestor_record_offset":1120,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":491,"record_offset":1292,"parent_record_offset":1260,"ancestor_record_offset":1120,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":491,"record_offset":1412,"parent_record_offset":1260,"ancestor_record_offset":1120,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":491,"record_offset":1176,"parent_record_offset":1148,"ancestor_record_offset":1120,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":491,"record_offset":1316,"parent_record_offset":1292,"ancestor_record_offset":1120,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":491,"record_offset":1344,"parent_record_offset":1316,"ancestor_record_offset":1120,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":491,"record_offset":1260,"parent_record_offset":1232,"ancestor_record_offset":1120,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":491,"record_offset":1232,"parent_record_offset":1204,"ancestor_record_offset":1120,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00491.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1339,"record_offset":39564,"rva":15516608,"va":5384225728}
// 编译实例 RVAs: 0xecc3c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da5f0
//     owner=unresolved_namespace; count=2
//     target_aliases=15 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12534142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e882be40ff","size":5,"soft_fail":false,"target_va":5371700720,"va":5384234857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11478415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e871da50ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5384228106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=24
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11479079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e8d9d750ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384228786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405a0920
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9617145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e807416dff","size":5,"soft_fail":false,"target_va":5374609696,"va":5384226836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140667370
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8804300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e834a879ff","size":5,"soft_fail":false,"target_va":5375423344,"va":5384227639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e876c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-284832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e860a7fbff","size":5,"soft_fail":false,"target_va":5383943872,"va":5384228699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e89460
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-277089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e89fc5fbff","size":5,"soft_fail":false,"target_va":5383951456,"va":5384228540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e92b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-235854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e8b266fcff","size":5,"soft_fail":false,"target_va":5383990064,"va":5384225913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410315f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1462199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e8b74f1600","size":5,"soft_fail":false,"target_va":5385688560,"va":5384226356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141031ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1458308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e884401600","size":5,"soft_fail":false,"target_va":5385690336,"va":5384232023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141032030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1458674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e8f2411600","size":5,"soft_fail":false,"target_va":5385691184,"va":5384232505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141032380
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1465492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e8945c1600","size":5,"soft_fail":false,"target_va":5385692032,"va":5384226535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141032bb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1466353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e8f15f1600","size":5,"soft_fail":false,"target_va":5385694128,"va":5384227770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9ac0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18725325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e8cdb91d01","size":5,"soft_fail":false,"target_va":5402958528,"va":5384233198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420d45a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18900271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e82f652001","size":5,"soft_fail":false,"target_va":5403133344,"va":5384233068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19365630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e8fe7e2701","size":5,"soft_fail":false,"target_va":5403593344,"va":5384227709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19364857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e8f97b2701","size":5,"soft_fail":false,"target_va":5403593360,"va":5384228498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19360438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e8b66a2701","size":5,"soft_fail":false,"target_va":5403593376,"va":5384232933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19367065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e899842701","size":5,"soft_fail":false,"target_va":5403593392,"va":5384226322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18744595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e813051e01","size":5,"soft_fail":false,"target_va":5402978144,"va":5384233544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-12921496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e868d53aff","size":5,"soft_fail":false,"target_va":5371305232,"va":5384226723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6706404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e81cab99ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5384233519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18745300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e8d4071e01","size":5,"soft_fail":false,"target_va":5402978432,"va":5384233127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18745660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e83c091e01","size":5,"soft_fail":false,"target_va":5402978880,"va":5384233215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t18745896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e8280a1e01","size":5,"soft_fail":false,"target_va":5402979152,"va":5384233251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18746232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e8780b1e01","size":5,"soft_fail":false,"target_va":5402979344,"va":5384233107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18745466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e87a081e01","size":5,"soft_fail":false,"target_va":5402978944,"va":5384233473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-28457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e8d790ffff","size":5,"soft_fail":false,"target_va":5384204496,"va":5384232948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18746732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e86c0d1e01","size":5,"soft_fail":false,"target_va":5402980128,"va":5384233391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18746544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e8b00c1e01","size":5,"soft_fail":false,"target_va":5402980288,"va":5384233739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::cubic_in_out
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7289076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384225728,"raw_hex":"e80cc790ff","size":5,"soft_fail":false,"target_va":5376938816,"va":5384227887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*%r12","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5384225728,"raw_hex":"41ffd4","size":3,"soft_fail":false,"va":5384226262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::ptr_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1339,"record_offset":43676,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":4640,"source_line":1833,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":43028,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":42556,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":54040,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":54164,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":8653,"source_line":1598,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":44012,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1339,"record_offset":53664,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1339,"record_offset":42232,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1339,"record_offset":44928,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":1339,"record_offset":42668,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":8593,"source_line":63,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1339,"record_offset":51296,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":8892,"source_line":728,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1339,"record_offset":52704,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1339,"record_offset":44864,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":53136,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1339,"record_offset":50960,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1339,"record_offset":42584,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1339,"record_offset":52672,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1339,"record_offset":53372,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":9336,"source_line":976,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1339,"record_offset":43196,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1339,"record_offset":44624,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data_mut
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1339,"record_offset":44900,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":23616,"source_line":296,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::MultiEdit::iter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1339,"record_offset":43320,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":25554,"source_line":248,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::history_chain::HistoryEditChain::iter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1339,"record_offset":43084,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":24519,"source_line":194,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":1339,"record_offset":42296,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":23761,"source_line":165,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::iter_points_xz
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":1339,"record_offset":43732,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":26028,"source_line":50,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::radius
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":1339,"record_offset":43844,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":26027,"source_line":46,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$1::extend_dirty
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1339,"record_offset":44568,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":62330,"source_line":354,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_garden::replay_garden::closure$2
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_garden.rs
//     evidence={"module":1339,"record_offset":44108,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":62333,"source_line":37,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::get_stroke_from_0_to_t
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1339,"record_offset":44988,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":62331,"source_line":360,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1339,"record_offset":43056,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":25759,"source_line":132,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::Entry::or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1339,"record_offset":41020,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":6546,"source_line":2798,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1339,"record_offset":40036,"parent_record_offset":39564,"ancestor_record_offset":39564,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1339,"record_offset":45432,"parent_record_offset":45408,"ancestor_record_offset":39564,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1339,"record_offset":45456,"parent_record_offset":45432,"ancestor_record_offset":39564,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1339,"record_offset":46164,"parent_record_offset":46140,"ancestor_record_offset":39564,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1339,"record_offset":45408,"parent_record_offset":45316,"ancestor_record_offset":39564,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1339,"record_offset":46140,"parent_record_offset":46040,"ancestor_record_offset":39564,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1339,"record_offset":42324,"parent_record_offset":42296,"ancestor_record_offset":39564,"inlinee":23762,"source_line":1544,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1339,"record_offset":42352,"parent_record_offset":42324,"ancestor_record_offset":39564,"inlinee":21857,"source_line":697,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1339,"record_offset":42392,"parent_record_offset":42352,"ancestor_record_offset":39564,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1339,"record_offset":42460,"parent_record_offset":42352,"ancestor_record_offset":39564,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1339,"record_offset":42484,"parent_record_offset":42460,"ancestor_record_offset":39564,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::wrap_index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1339,"record_offset":42508,"parent_record_offset":42484,"ancestor_record_offset":39564,"inlinee":11291,"source_line":2679,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1339,"record_offset":46072,"parent_record_offset":46040,"ancestor_record_offset":39564,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1339,"record_offset":42428,"parent_record_offset":42392,"ancestor_record_offset":39564,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1339,"record_offset":45316,"parent_record_offset":45288,"ancestor_record_offset":39564,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1339,"record_offset":45288,"parent_record_offset":45260,"ancestor_record_offset":39564,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1339,"record_offset":45528,"parent_record_offset":45288,"ancestor_record_offset":39564,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1339,"record_offset":46040,"parent_record_offset":46016,"ancestor_record_offset":39564,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1339,"record_offset":45216,"parent_record_offset":45180,"ancestor_record_offset":39564,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1339,"record_offset":45180,"parent_record_offset":45144,"ancestor_record_offset":39564,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1339,"record_offset":45144,"parent_record_offset":45108,"ancestor_record_offset":39564,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":44292,"parent_record_offset":44252,"ancestor_record_offset":39564,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":54212,"parent_record_offset":54164,"ancestor_record_offset":39564,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":45964,"parent_record_offset":45924,"ancestor_record_offset":39564,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":44252,"parent_record_offset":44216,"ancestor_record_offset":39564,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":54372,"parent_record_offset":54164,"ancestor_record_offset":39564,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":45260,"parent_record_offset":45216,"ancestor_record_offset":39564,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":45108,"parent_record_offset":44988,"ancestor_record_offset":39564,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":46704,"parent_record_offset":44988,"ancestor_record_offset":39564,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1339,"record_offset":54428,"parent_record_offset":54404,"ancestor_record_offset":39564,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1339,"record_offset":54476,"parent_record_offset":54452,"ancestor_record_offset":39564,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1339,"record_offset":42260,"parent_record_offset":42232,"ancestor_record_offset":39564,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1339,"record_offset":44156,"parent_record_offset":44132,"ancestor_record_offset":39564,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1339,"record_offset":44132,"parent_record_offset":44108,"ancestor_record_offset":39564,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1339,"record_offset":44216,"parent_record_offset":44132,"ancestor_record_offset":39564,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1339,"record_offset":45352,"parent_record_offset":45316,"ancestor_record_offset":39564,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1339,"record_offset":45376,"parent_record_offset":45352,"ancestor_record_offset":39564,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":1339,"record_offset":40720,"parent_record_offset":40696,"ancestor_record_offset":39564,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1339,"record_offset":45800,"parent_record_offset":45776,"ancestor_record_offset":39564,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1339,"record_offset":40768,"parent_record_offset":40744,"ancestor_record_offset":39564,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1339,"record_offset":45824,"parent_record_offset":45800,"ancestor_record_offset":39564,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1339,"record_offset":44692,"parent_record_offset":44652,"ancestor_record_offset":39564,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1339,"record_offset":41688,"parent_record_offset":41664,"ancestor_record_offset":39564,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1339,"record_offset":40296,"parent_record_offset":40272,"ancestor_record_offset":39564,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1339,"record_offset":40380,"parent_record_offset":40356,"ancestor_record_offset":39564,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1339,"record_offset":45864,"parent_record_offset":45712,"ancestor_record_offset":39564,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1339,"record_offset":45892,"parent_record_offset":45712,"ancestor_record_offset":39564,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1339,"record_offset":40320,"parent_record_offset":40296,"ancestor_record_offset":39564,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1339,"record_offset":42716,"parent_record_offset":42692,"ancestor_record_offset":39564,"inlinee":8626,"source_line":73,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1339,"record_offset":42896,"parent_record_offset":42740,"ancestor_record_offset":39564,"inlinee":8628,"source_line":85,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1339,"record_offset":42692,"parent_record_offset":42668,"ancestor_record_offset":39564,"inlinee":8594,"source_line":2898,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1339,"record_offset":42924,"parent_record_offset":42896,"ancestor_record_offset":39564,"inlinee":8630,"source_line":2905,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1339,"record_offset":42740,"parent_record_offset":42716,"ancestor_record_offset":39564,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1339,"record_offset":53448,"parent_record_offset":53404,"ancestor_record_offset":39564,"inlinee":8892,"source_line":728,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap_simple
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1339,"record_offset":51368,"parent_record_offset":51344,"ancestor_record_offset":39564,"inlinee":8891,"source_line":761,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1339,"record_offset":46676,"parent_record_offset":44988,"ancestor_record_offset":39564,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1339,"record_offset":46104,"parent_record_offset":46072,"ancestor_record_offset":39564,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1339,"record_offset":41948,"parent_record_offset":41920,"ancestor_record_offset":39564,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1339,"record_offset":40492,"parent_record_offset":40468,"ancestor_record_offset":39564,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1339,"record_offset":43636,"parent_record_offset":43612,"ancestor_record_offset":39564,"inlinee":4401,"source_line":166,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1339,"record_offset":42948,"parent_record_offset":42924,"ancestor_record_offset":39564,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1339,"record_offset":45776,"parent_record_offset":45752,"ancestor_record_offset":39564,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1339,"record_offset":45752,"parent_record_offset":45712,"ancestor_record_offset":39564,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::addr_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":43700,"parent_record_offset":43676,"ancestor_record_offset":39564,"inlinee":4641,"source_line":2014,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1339,"record_offset":46844,"parent_record_offset":46820,"ancestor_record_offset":39564,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1339,"record_offset":45568,"parent_record_offset":45216,"ancestor_record_offset":39564,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=53
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":45992,"parent_record_offset":45924,"ancestor_record_offset":39564,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1339,"record_offset":42632,"parent_record_offset":42608,"ancestor_record_offset":39564,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1339,"record_offset":40656,"parent_record_offset":40632,"ancestor_record_offset":39564,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1339,"record_offset":40632,"parent_record_offset":40608,"ancestor_record_offset":39564,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1339,"record_offset":42156,"parent_record_offset":42132,"ancestor_record_offset":39564,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1339,"record_offset":42860,"parent_record_offset":42836,"ancestor_record_offset":39564,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1339,"record_offset":53744,"parent_record_offset":53664,"ancestor_record_offset":39564,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1339,"record_offset":42808,"parent_record_offset":42772,"ancestor_record_offset":39564,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":51644,"parent_record_offset":51368,"ancestor_record_offset":39564,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":45484,"parent_record_offset":45456,"ancestor_record_offset":39564,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":51320,"parent_record_offset":51296,"ancestor_record_offset":39564,"inlinee":8889,"source_line":1002,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_simple_untyped
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":51344,"parent_record_offset":51320,"ancestor_record_offset":39564,"inlinee":8890,"source_line":1059,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":42180,"parent_record_offset":42156,"ancestor_record_offset":39564,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1339,"record_offset":45668,"parent_record_offset":45640,"ancestor_record_offset":39564,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1339,"record_offset":44776,"parent_record_offset":44748,"ancestor_record_offset":39564,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1339,"record_offset":44804,"parent_record_offset":44776,"ancestor_record_offset":39564,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1339,"record_offset":43108,"parent_record_offset":43084,"ancestor_record_offset":39564,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1339,"record_offset":53404,"parent_record_offset":53372,"ancestor_record_offset":39564,"inlinee":9338,"source_line":999,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1339,"record_offset":44040,"parent_record_offset":44012,"ancestor_record_offset":39564,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1339,"record_offset":44068,"parent_record_offset":44040,"ancestor_record_offset":39564,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1339,"record_offset":46820,"parent_record_offset":46796,"ancestor_record_offset":39564,"inlinee":5475,"source_line":373,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1339,"record_offset":46796,"parent_record_offset":46760,"ancestor_record_offset":39564,"inlinee":5473,"source_line":489,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1339,"record_offset":46760,"parent_record_offset":46732,"ancestor_record_offset":39564,"inlinee":5472,"source_line":501,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1339,"record_offset":43132,"parent_record_offset":43108,"ancestor_record_offset":39564,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1339,"record_offset":43260,"parent_record_offset":43196,"ancestor_record_offset":39564,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1339,"record_offset":42608,"parent_record_offset":42584,"ancestor_record_offset":39564,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1339,"record_offset":42836,"parent_record_offset":42772,"ancestor_record_offset":39564,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1339,"record_offset":43456,"parent_record_offset":43432,"ancestor_record_offset":39564,"inlinee":8600,"source_line":313,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1339,"record_offset":43488,"parent_record_offset":43456,"ancestor_record_offset":39564,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1339,"record_offset":42772,"parent_record_offset":42740,"ancestor_record_offset":39564,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1339,"record_offset":44720,"parent_record_offset":44692,"ancestor_record_offset":39564,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":1339,"record_offset":40744,"parent_record_offset":40720,"ancestor_record_offset":39564,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1339,"record_offset":44652,"parent_record_offset":44624,"ancestor_record_offset":39564,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1339,"record_offset":42972,"parent_record_offset":42948,"ancestor_record_offset":39564,"inlinee":62327,"source_line":132,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1339,"record_offset":44592,"parent_record_offset":44568,"ancestor_record_offset":39564,"inlinee":59460,"source_line":165,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1339,"record_offset":40944,"parent_record_offset":40072,"ancestor_record_offset":39564,"inlinee":6552,"source_line":1105,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1339,"record_offset":40072,"parent_record_offset":40036,"ancestor_record_offset":39564,"inlinee":6541,"source_line":34,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1339,"record_offset":40116,"parent_record_offset":40072,"ancestor_record_offset":39564,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":40608,"parent_record_offset":40584,"ancestor_record_offset":39564,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":42132,"parent_record_offset":41176,"ancestor_record_offset":39564,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":40904,"parent_record_offset":40168,"ancestor_record_offset":39564,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":40584,"parent_record_offset":40560,"ancestor_record_offset":39564,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":40144,"parent_record_offset":40072,"ancestor_record_offset":39564,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":41176,"parent_record_offset":41144,"ancestor_record_offset":39564,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":40972,"parent_record_offset":40944,"ancestor_record_offset":39564,"inlinee":6553,"source_line":1175,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":42020,"parent_record_offset":41176,"ancestor_record_offset":39564,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":40168,"parent_record_offset":40144,"ancestor_record_offset":39564,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":41260,"parent_record_offset":41224,"ancestor_record_offset":39564,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":41412,"parent_record_offset":41260,"ancestor_record_offset":39564,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":41576,"parent_record_offset":41260,"ancestor_record_offset":39564,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":41608,"parent_record_offset":41576,"ancestor_record_offset":39564,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":41224,"parent_record_offset":41176,"ancestor_record_offset":39564,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":40244,"parent_record_offset":40168,"ancestor_record_offset":39564,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":41920,"parent_record_offset":41896,"ancestor_record_offset":39564,"inlinee":6566,"source_line":2743,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":41896,"parent_record_offset":41224,"ancestor_record_offset":39564,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1339,"record_offset":40440,"parent_record_offset":40412,"ancestor_record_offset":39564,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1339,"record_offset":40468,"parent_record_offset":40440,"ancestor_record_offset":39564,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1339,"record_offset":40528,"parent_record_offset":40412,"ancestor_record_offset":39564,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1339,"record_offset":40412,"parent_record_offset":40168,"ancestor_record_offset":39564,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":40216,"parent_record_offset":40168,"ancestor_record_offset":39564,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":40560,"parent_record_offset":40168,"ancestor_record_offset":39564,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":41632,"parent_record_offset":41608,"ancestor_record_offset":39564,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":41992,"parent_record_offset":41176,"ancestor_record_offset":39564,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1339,"record_offset":40272,"parent_record_offset":40168,"ancestor_record_offset":39564,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1339,"record_offset":41664,"parent_record_offset":41576,"ancestor_record_offset":39564,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1339,"record_offset":40356,"parent_record_offset":40168,"ancestor_record_offset":39564,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1339,"record_offset":40812,"parent_record_offset":40168,"ancestor_record_offset":39564,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1339,"record_offset":41452,"parent_record_offset":41412,"ancestor_record_offset":39564,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcOccupiedEntry::into_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1339,"record_offset":41080,"parent_record_offset":41056,"ancestor_record_offset":39564,"inlinee":6550,"source_line":412,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1339,"record_offset":41144,"parent_record_offset":41112,"ancestor_record_offset":39564,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1339,"record_offset":40696,"parent_record_offset":40560,"ancestor_record_offset":39564,"inlinee":6544,"source_line":36,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::OccupiedEntry::into_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1339,"record_offset":41056,"parent_record_offset":41020,"ancestor_record_offset":39564,"inlinee":6548,"source_line":2927,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1339,"record_offset":41112,"parent_record_offset":41020,"ancestor_record_offset":39564,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::new_from_points
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1339,"record_offset":45640,"parent_record_offset":44988,"ancestor_record_offset":39564,"inlinee":21342,"source_line":216,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::try_slice
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1339,"record_offset":45924,"parent_record_offset":44988,"ancestor_record_offset":39564,"inlinee":61974,"source_line":677,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1339,"record_offset":45712,"parent_record_offset":44988,"ancestor_record_offset":39564,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1339,"record_offset":5840,"rva":15496832,"va":5384205952}
// 编译实例 RVAs: 0xec7680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da5f0
//     owner=unresolved_namespace; count=2
//     target_aliases=15 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12514366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e8c20b41ff","size":5,"soft_fail":false,"target_va":5371700720,"va":5384215081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11458639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e8b12751ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5384208330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=24
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11459303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e8192551ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384209010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405a0920
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9597369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e8478e6dff","size":5,"soft_fail":false,"target_va":5374609696,"va":5384207060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140667370
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8784524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e874f579ff","size":5,"soft_fail":false,"target_va":5375423344,"va":5384207863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e876c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-265056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e8a0f4fbff","size":5,"soft_fail":false,"target_va":5383943872,"va":5384208923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e89460
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-257313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e8df12fcff","size":5,"soft_fail":false,"target_va":5383951456,"va":5384208764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e92b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-216078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e8f2b3fcff","size":5,"soft_fail":false,"target_va":5383990064,"va":5384206137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410315f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1481975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e8f79c1600","size":5,"soft_fail":false,"target_va":5385688560,"va":5384206580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141031ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1478084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e8c48d1600","size":5,"soft_fail":false,"target_va":5385690336,"va":5384212247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141032030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1478450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e8328f1600","size":5,"soft_fail":false,"target_va":5385691184,"va":5384212729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141032380
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1485268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e8d4a91600","size":5,"soft_fail":false,"target_va":5385692032,"va":5384206759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141032bb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1486129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e831ad1600","size":5,"soft_fail":false,"target_va":5385694128,"va":5384207994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9ac0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18745101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e80d071e01","size":5,"soft_fail":false,"target_va":5402958528,"va":5384213422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420d45a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18920047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e86fb22001","size":5,"soft_fail":false,"target_va":5403133344,"va":5384213292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19385406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e83ecc2701","size":5,"soft_fail":false,"target_va":5403593344,"va":5384207933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19384633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e839c92701","size":5,"soft_fail":false,"target_va":5403593360,"va":5384208722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19380214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e8f6b72701","size":5,"soft_fail":false,"target_va":5403593376,"va":5384213157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19386841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e8d9d12701","size":5,"soft_fail":false,"target_va":5403593392,"va":5384206546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18764371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e853521e01","size":5,"soft_fail":false,"target_va":5402978144,"va":5384213768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-12901720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e8a8223bff","size":5,"soft_fail":false,"target_va":5371305232,"va":5384206947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6686628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e85cf899ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5384213743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18765076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e814551e01","size":5,"soft_fail":false,"target_va":5402978432,"va":5384213351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18765436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e87c561e01","size":5,"soft_fail":false,"target_va":5402978880,"va":5384213439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t18765672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e868571e01","size":5,"soft_fail":false,"target_va":5402979152,"va":5384213475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18766008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e8b8581e01","size":5,"soft_fail":false,"target_va":5402979344,"va":5384213331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18765242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e8ba551e01","size":5,"soft_fail":false,"target_va":5402978944,"va":5384213697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e817deffff","size":5,"soft_fail":false,"target_va":5384204496,"va":5384213172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18766508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e8ac5a1e01","size":5,"soft_fail":false,"target_va":5402980128,"va":5384213615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18766320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e8f0591e01","size":5,"soft_fail":false,"target_va":5402980288,"va":5384213963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::cubic_in_out
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7269300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384205952,"raw_hex":"e84c1491ff","size":5,"soft_fail":false,"target_va":5376938816,"va":5384208111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*%r12","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5384205952,"raw_hex":"41ffd4","size":3,"soft_fail":false,"va":5384206486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::ptr_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1339,"record_offset":9944,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":4640,"source_line":1833,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":9296,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":8824,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":20308,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":20432,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":8653,"source_line":1598,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":10280,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1339,"record_offset":19932,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1339,"record_offset":8500,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1339,"record_offset":11196,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":1339,"record_offset":8936,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":8593,"source_line":63,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1339,"record_offset":17564,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":8892,"source_line":728,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1339,"record_offset":18972,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1339,"record_offset":11132,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":19404,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1339,"record_offset":17228,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1339,"record_offset":8852,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1339,"record_offset":18940,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1339,"record_offset":19640,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":9336,"source_line":976,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1339,"record_offset":9464,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1339,"record_offset":10892,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data_mut
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1339,"record_offset":11168,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":23616,"source_line":296,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::MultiEdit::iter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1339,"record_offset":9588,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":25554,"source_line":248,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::history_chain::HistoryEditChain::iter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1339,"record_offset":9352,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":24519,"source_line":194,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":1339,"record_offset":8564,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":23761,"source_line":165,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::iter_points_xz
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":1339,"record_offset":10000,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":26028,"source_line":50,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::radius
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":1339,"record_offset":10112,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":26027,"source_line":46,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$1::extend_dirty
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1339,"record_offset":10836,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":62330,"source_line":354,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_path::replay_path::closure$2
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_path.rs
//     evidence={"module":1339,"record_offset":10376,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":62329,"source_line":34,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::get_stroke_from_0_to_t
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1339,"record_offset":11256,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":62331,"source_line":360,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1339,"record_offset":9324,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":25759,"source_line":132,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::Entry::or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1339,"record_offset":7288,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":6546,"source_line":2798,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1339,"record_offset":6304,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1339,"record_offset":11700,"parent_record_offset":11676,"ancestor_record_offset":5840,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1339,"record_offset":11724,"parent_record_offset":11700,"ancestor_record_offset":5840,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1339,"record_offset":12432,"parent_record_offset":12408,"ancestor_record_offset":5840,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1339,"record_offset":11676,"parent_record_offset":11584,"ancestor_record_offset":5840,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1339,"record_offset":12408,"parent_record_offset":12308,"ancestor_record_offset":5840,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1339,"record_offset":8592,"parent_record_offset":8564,"ancestor_record_offset":5840,"inlinee":23762,"source_line":1544,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1339,"record_offset":8620,"parent_record_offset":8592,"ancestor_record_offset":5840,"inlinee":21857,"source_line":697,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1339,"record_offset":8660,"parent_record_offset":8620,"ancestor_record_offset":5840,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1339,"record_offset":8728,"parent_record_offset":8620,"ancestor_record_offset":5840,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1339,"record_offset":8752,"parent_record_offset":8728,"ancestor_record_offset":5840,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::wrap_index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1339,"record_offset":8776,"parent_record_offset":8752,"ancestor_record_offset":5840,"inlinee":11291,"source_line":2679,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1339,"record_offset":12340,"parent_record_offset":12308,"ancestor_record_offset":5840,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1339,"record_offset":8696,"parent_record_offset":8660,"ancestor_record_offset":5840,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1339,"record_offset":11584,"parent_record_offset":11556,"ancestor_record_offset":5840,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1339,"record_offset":11556,"parent_record_offset":11528,"ancestor_record_offset":5840,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1339,"record_offset":11796,"parent_record_offset":11556,"ancestor_record_offset":5840,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1339,"record_offset":12308,"parent_record_offset":12284,"ancestor_record_offset":5840,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1339,"record_offset":11484,"parent_record_offset":11448,"ancestor_record_offset":5840,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1339,"record_offset":11448,"parent_record_offset":11412,"ancestor_record_offset":5840,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1339,"record_offset":11412,"parent_record_offset":11376,"ancestor_record_offset":5840,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":10560,"parent_record_offset":10520,"ancestor_record_offset":5840,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":20480,"parent_record_offset":20432,"ancestor_record_offset":5840,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":12232,"parent_record_offset":12192,"ancestor_record_offset":5840,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":10520,"parent_record_offset":10484,"ancestor_record_offset":5840,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":20640,"parent_record_offset":20432,"ancestor_record_offset":5840,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":11528,"parent_record_offset":11484,"ancestor_record_offset":5840,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":11376,"parent_record_offset":11256,"ancestor_record_offset":5840,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":12972,"parent_record_offset":11256,"ancestor_record_offset":5840,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1339,"record_offset":20696,"parent_record_offset":20672,"ancestor_record_offset":5840,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1339,"record_offset":20744,"parent_record_offset":20720,"ancestor_record_offset":5840,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1339,"record_offset":8528,"parent_record_offset":8500,"ancestor_record_offset":5840,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1339,"record_offset":10424,"parent_record_offset":10400,"ancestor_record_offset":5840,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1339,"record_offset":10400,"parent_record_offset":10376,"ancestor_record_offset":5840,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1339,"record_offset":10484,"parent_record_offset":10400,"ancestor_record_offset":5840,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1339,"record_offset":11620,"parent_record_offset":11584,"ancestor_record_offset":5840,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1339,"record_offset":11644,"parent_record_offset":11620,"ancestor_record_offset":5840,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":1339,"record_offset":6988,"parent_record_offset":6964,"ancestor_record_offset":5840,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1339,"record_offset":12068,"parent_record_offset":12044,"ancestor_record_offset":5840,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1339,"record_offset":7036,"parent_record_offset":7012,"ancestor_record_offset":5840,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1339,"record_offset":12092,"parent_record_offset":12068,"ancestor_record_offset":5840,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1339,"record_offset":10960,"parent_record_offset":10920,"ancestor_record_offset":5840,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1339,"record_offset":7956,"parent_record_offset":7932,"ancestor_record_offset":5840,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1339,"record_offset":6564,"parent_record_offset":6540,"ancestor_record_offset":5840,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1339,"record_offset":6648,"parent_record_offset":6624,"ancestor_record_offset":5840,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1339,"record_offset":12132,"parent_record_offset":11980,"ancestor_record_offset":5840,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1339,"record_offset":12160,"parent_record_offset":11980,"ancestor_record_offset":5840,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1339,"record_offset":6588,"parent_record_offset":6564,"ancestor_record_offset":5840,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1339,"record_offset":8984,"parent_record_offset":8960,"ancestor_record_offset":5840,"inlinee":8626,"source_line":73,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1339,"record_offset":9164,"parent_record_offset":9008,"ancestor_record_offset":5840,"inlinee":8628,"source_line":85,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1339,"record_offset":8960,"parent_record_offset":8936,"ancestor_record_offset":5840,"inlinee":8594,"source_line":2898,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1339,"record_offset":9192,"parent_record_offset":9164,"ancestor_record_offset":5840,"inlinee":8630,"source_line":2905,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1339,"record_offset":9008,"parent_record_offset":8984,"ancestor_record_offset":5840,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1339,"record_offset":19716,"parent_record_offset":19672,"ancestor_record_offset":5840,"inlinee":8892,"source_line":728,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap_simple
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1339,"record_offset":17636,"parent_record_offset":17612,"ancestor_record_offset":5840,"inlinee":8891,"source_line":761,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1339,"record_offset":12944,"parent_record_offset":11256,"ancestor_record_offset":5840,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1339,"record_offset":12372,"parent_record_offset":12340,"ancestor_record_offset":5840,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1339,"record_offset":8216,"parent_record_offset":8188,"ancestor_record_offset":5840,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1339,"record_offset":6760,"parent_record_offset":6736,"ancestor_record_offset":5840,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1339,"record_offset":9904,"parent_record_offset":9880,"ancestor_record_offset":5840,"inlinee":4401,"source_line":166,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1339,"record_offset":9216,"parent_record_offset":9192,"ancestor_record_offset":5840,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1339,"record_offset":12044,"parent_record_offset":12020,"ancestor_record_offset":5840,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1339,"record_offset":12020,"parent_record_offset":11980,"ancestor_record_offset":5840,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::addr_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":9968,"parent_record_offset":9944,"ancestor_record_offset":5840,"inlinee":4641,"source_line":2014,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1339,"record_offset":13112,"parent_record_offset":13088,"ancestor_record_offset":5840,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1339,"record_offset":11836,"parent_record_offset":11484,"ancestor_record_offset":5840,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=53
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":12260,"parent_record_offset":12192,"ancestor_record_offset":5840,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1339,"record_offset":8900,"parent_record_offset":8876,"ancestor_record_offset":5840,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1339,"record_offset":6924,"parent_record_offset":6900,"ancestor_record_offset":5840,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1339,"record_offset":6900,"parent_record_offset":6876,"ancestor_record_offset":5840,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1339,"record_offset":8424,"parent_record_offset":8400,"ancestor_record_offset":5840,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1339,"record_offset":9128,"parent_record_offset":9104,"ancestor_record_offset":5840,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1339,"record_offset":20012,"parent_record_offset":19932,"ancestor_record_offset":5840,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1339,"record_offset":9076,"parent_record_offset":9040,"ancestor_record_offset":5840,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":17912,"parent_record_offset":17636,"ancestor_record_offset":5840,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":11752,"parent_record_offset":11724,"ancestor_record_offset":5840,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":17588,"parent_record_offset":17564,"ancestor_record_offset":5840,"inlinee":8889,"source_line":1002,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_simple_untyped
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":17612,"parent_record_offset":17588,"ancestor_record_offset":5840,"inlinee":8890,"source_line":1059,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":8448,"parent_record_offset":8424,"ancestor_record_offset":5840,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1339,"record_offset":11936,"parent_record_offset":11908,"ancestor_record_offset":5840,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1339,"record_offset":11044,"parent_record_offset":11016,"ancestor_record_offset":5840,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1339,"record_offset":11072,"parent_record_offset":11044,"ancestor_record_offset":5840,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1339,"record_offset":9376,"parent_record_offset":9352,"ancestor_record_offset":5840,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1339,"record_offset":19672,"parent_record_offset":19640,"ancestor_record_offset":5840,"inlinee":9338,"source_line":999,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1339,"record_offset":10308,"parent_record_offset":10280,"ancestor_record_offset":5840,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1339,"record_offset":10336,"parent_record_offset":10308,"ancestor_record_offset":5840,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1339,"record_offset":13088,"parent_record_offset":13064,"ancestor_record_offset":5840,"inlinee":5475,"source_line":373,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1339,"record_offset":13064,"parent_record_offset":13028,"ancestor_record_offset":5840,"inlinee":5473,"source_line":489,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1339,"record_offset":13028,"parent_record_offset":13000,"ancestor_record_offset":5840,"inlinee":5472,"source_line":501,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1339,"record_offset":9400,"parent_record_offset":9376,"ancestor_record_offset":5840,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1339,"record_offset":9528,"parent_record_offset":9464,"ancestor_record_offset":5840,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1339,"record_offset":8876,"parent_record_offset":8852,"ancestor_record_offset":5840,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1339,"record_offset":9104,"parent_record_offset":9040,"ancestor_record_offset":5840,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1339,"record_offset":9724,"parent_record_offset":9700,"ancestor_record_offset":5840,"inlinee":8600,"source_line":313,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1339,"record_offset":9756,"parent_record_offset":9724,"ancestor_record_offset":5840,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1339,"record_offset":9040,"parent_record_offset":9008,"ancestor_record_offset":5840,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1339,"record_offset":10988,"parent_record_offset":10960,"ancestor_record_offset":5840,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":1339,"record_offset":7012,"parent_record_offset":6988,"ancestor_record_offset":5840,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1339,"record_offset":10920,"parent_record_offset":10892,"ancestor_record_offset":5840,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1339,"record_offset":9240,"parent_record_offset":9216,"ancestor_record_offset":5840,"inlinee":62327,"source_line":132,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1339,"record_offset":10860,"parent_record_offset":10836,"ancestor_record_offset":5840,"inlinee":59460,"source_line":165,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1339,"record_offset":7212,"parent_record_offset":6340,"ancestor_record_offset":5840,"inlinee":6552,"source_line":1105,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1339,"record_offset":6340,"parent_record_offset":6304,"ancestor_record_offset":5840,"inlinee":6541,"source_line":34,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1339,"record_offset":6384,"parent_record_offset":6340,"ancestor_record_offset":5840,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":6876,"parent_record_offset":6852,"ancestor_record_offset":5840,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":8400,"parent_record_offset":7444,"ancestor_record_offset":5840,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":7172,"parent_record_offset":6436,"ancestor_record_offset":5840,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":6852,"parent_record_offset":6828,"ancestor_record_offset":5840,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":6412,"parent_record_offset":6340,"ancestor_record_offset":5840,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":7444,"parent_record_offset":7412,"ancestor_record_offset":5840,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":7240,"parent_record_offset":7212,"ancestor_record_offset":5840,"inlinee":6553,"source_line":1175,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":8288,"parent_record_offset":7444,"ancestor_record_offset":5840,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":6436,"parent_record_offset":6412,"ancestor_record_offset":5840,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":7528,"parent_record_offset":7492,"ancestor_record_offset":5840,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":7680,"parent_record_offset":7528,"ancestor_record_offset":5840,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":7844,"parent_record_offset":7528,"ancestor_record_offset":5840,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":7876,"parent_record_offset":7844,"ancestor_record_offset":5840,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":7492,"parent_record_offset":7444,"ancestor_record_offset":5840,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":6512,"parent_record_offset":6436,"ancestor_record_offset":5840,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":8188,"parent_record_offset":8164,"ancestor_record_offset":5840,"inlinee":6566,"source_line":2743,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":8164,"parent_record_offset":7492,"ancestor_record_offset":5840,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1339,"record_offset":6708,"parent_record_offset":6680,"ancestor_record_offset":5840,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1339,"record_offset":6736,"parent_record_offset":6708,"ancestor_record_offset":5840,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1339,"record_offset":6796,"parent_record_offset":6680,"ancestor_record_offset":5840,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1339,"record_offset":6680,"parent_record_offset":6436,"ancestor_record_offset":5840,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":6484,"parent_record_offset":6436,"ancestor_record_offset":5840,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":6828,"parent_record_offset":6436,"ancestor_record_offset":5840,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":7900,"parent_record_offset":7876,"ancestor_record_offset":5840,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":8260,"parent_record_offset":7444,"ancestor_record_offset":5840,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1339,"record_offset":6540,"parent_record_offset":6436,"ancestor_record_offset":5840,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1339,"record_offset":7932,"parent_record_offset":7844,"ancestor_record_offset":5840,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1339,"record_offset":6624,"parent_record_offset":6436,"ancestor_record_offset":5840,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1339,"record_offset":7080,"parent_record_offset":6436,"ancestor_record_offset":5840,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1339,"record_offset":7720,"parent_record_offset":7680,"ancestor_record_offset":5840,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcOccupiedEntry::into_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1339,"record_offset":7348,"parent_record_offset":7324,"ancestor_record_offset":5840,"inlinee":6550,"source_line":412,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1339,"record_offset":7412,"parent_record_offset":7380,"ancestor_record_offset":5840,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1339,"record_offset":6964,"parent_record_offset":6828,"ancestor_record_offset":5840,"inlinee":6544,"source_line":36,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::OccupiedEntry::into_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1339,"record_offset":7324,"parent_record_offset":7288,"ancestor_record_offset":5840,"inlinee":6548,"source_line":2927,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1339,"record_offset":7380,"parent_record_offset":7288,"ancestor_record_offset":5840,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::new_from_points
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1339,"record_offset":11908,"parent_record_offset":11256,"ancestor_record_offset":5840,"inlinee":21342,"source_line":216,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::try_slice
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1339,"record_offset":12192,"parent_record_offset":11256,"ancestor_record_offset":5840,"inlinee":61974,"source_line":677,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1339,"record_offset":11980,"parent_record_offset":11256,"ancestor_record_offset":5840,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1339,"record_offset":22188,"rva":15506560,"va":5384215680}
// 编译实例 RVAs: 0xec9c80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da5f0
//     owner=unresolved_namespace; count=2
//     target_aliases=15 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12523438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e852e840ff","size":5,"soft_fail":false,"target_va":5371700720,"va":5384224153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11468520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e8180151ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5384218211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=24
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11469174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e88afe50ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384218881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405a0920
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9607246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e8b2676dff","size":5,"soft_fail":false,"target_va":5374609696,"va":5384216937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140667370
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8794401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e8dfce79ff","size":5,"soft_fail":false,"target_va":5375423344,"va":5384217740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e876c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-274924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e814cefbff","size":5,"soft_fail":false,"target_va":5383943872,"va":5384218791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e89460
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-267175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e859ecfbff","size":5,"soft_fail":false,"target_va":5383951456,"va":5384218626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e92b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-225800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e8f88dfcff","size":5,"soft_fail":false,"target_va":5383990064,"va":5384215859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410315f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1472247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e8f7761600","size":5,"soft_fail":false,"target_va":5385688560,"va":5384216308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141031ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1468204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e82c671600","size":5,"soft_fail":false,"target_va":5385690336,"va":5384222127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141032030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1468416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e800681600","size":5,"soft_fail":false,"target_va":5385691184,"va":5384222763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141032380
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1475540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e8d4831600","size":5,"soft_fail":false,"target_va":5385692032,"va":5384216487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141032bb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1476252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e89c861600","size":5,"soft_fail":false,"target_va":5385694128,"va":5384217871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9ac0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18735069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e8dddf1d01","size":5,"soft_fail":false,"target_va":5402958528,"va":5384223454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420d45a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18910016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e8408b2001","size":5,"soft_fail":false,"target_va":5403133344,"va":5384223323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19375529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e8a9a52701","size":5,"soft_fail":false,"target_va":5403593344,"va":5384217810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19374768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e8b0a22701","size":5,"soft_fail":false,"target_va":5403593360,"va":5384218587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19370176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e8c0902701","size":5,"soft_fail":false,"target_va":5403593376,"va":5384223195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19377113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e8d9ab2701","size":5,"soft_fail":false,"target_va":5403593392,"va":5384216274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18754330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e81a2b1e01","size":5,"soft_fail":false,"target_va":5402978144,"va":5384223809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-12911451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e8a5fc3aff","size":5,"soft_fail":false,"target_va":5371305232,"va":5384216678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6696669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e823d199ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5384223784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18755044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e8e42d1e01","size":5,"soft_fail":false,"target_va":5402978432,"va":5384223383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18755404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e84c2f1e01","size":5,"soft_fail":false,"target_va":5402978880,"va":5384223471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t18755640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e838301e01","size":5,"soft_fail":false,"target_va":5402979152,"va":5384223507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18755976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e888311e01","size":5,"soft_fail":false,"target_va":5402979344,"va":5384223363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18755201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e8812e1e01","size":5,"soft_fail":false,"target_va":5402978944,"va":5384223738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e8e1b6ffff","size":5,"soft_fail":false,"target_va":5384204496,"va":5384223210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18756476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e87c331e01","size":5,"soft_fail":false,"target_va":5402980128,"va":5384223647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18756270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e8ae321e01","size":5,"soft_fail":false,"target_va":5402980288,"va":5384224013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::cubic_in_out
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7279177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384215680,"raw_hex":"e8b7ed90ff","size":5,"soft_fail":false,"target_va":5376938816,"va":5384217988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*%r12","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5384215680,"raw_hex":"41ffd4","size":3,"soft_fail":false,"va":5384216214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::ptr_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1339,"record_offset":26296,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":4640,"source_line":1833,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":25648,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":25176,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":37684,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":37808,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":8653,"source_line":1598,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":26632,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1339,"record_offset":37308,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1339,"record_offset":24852,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1339,"record_offset":28068,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":1339,"record_offset":25288,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":8593,"source_line":63,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1339,"record_offset":34420,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":8892,"source_line":728,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1339,"record_offset":35828,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1339,"record_offset":28004,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":36780,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1339,"record_offset":34084,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1339,"record_offset":25204,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1339,"record_offset":35796,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1339,"record_offset":37016,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":9336,"source_line":976,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1339,"record_offset":25816,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1339,"record_offset":27764,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data_mut
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1339,"record_offset":28040,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":23616,"source_line":296,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::MultiEdit::iter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1339,"record_offset":25940,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":25554,"source_line":248,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::history_chain::HistoryEditChain::iter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1339,"record_offset":25704,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":24519,"source_line":194,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":1339,"record_offset":24916,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":23761,"source_line":165,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::iter_points_xz
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":1339,"record_offset":26352,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":26028,"source_line":50,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::radius
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":1339,"record_offset":26464,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":26027,"source_line":46,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$1::extend_dirty
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1339,"record_offset":27708,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":62330,"source_line":354,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::get_stroke_from_0_to_t
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1339,"record_offset":28128,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":62331,"source_line":360,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_water::replay_water::closure$2
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_water.rs
//     evidence={"module":1339,"record_offset":26728,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":62332,"source_line":41,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1339,"record_offset":25676,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":25759,"source_line":132,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::Entry::or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1339,"record_offset":23640,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":6546,"source_line":2798,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1339,"record_offset":22656,"parent_record_offset":22188,"ancestor_record_offset":22188,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1339,"record_offset":28564,"parent_record_offset":28540,"ancestor_record_offset":22188,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1339,"record_offset":28588,"parent_record_offset":28564,"ancestor_record_offset":22188,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1339,"record_offset":29296,"parent_record_offset":29272,"ancestor_record_offset":22188,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1339,"record_offset":28540,"parent_record_offset":28448,"ancestor_record_offset":22188,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1339,"record_offset":29272,"parent_record_offset":29172,"ancestor_record_offset":22188,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1339,"record_offset":24944,"parent_record_offset":24916,"ancestor_record_offset":22188,"inlinee":23762,"source_line":1544,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1339,"record_offset":24972,"parent_record_offset":24944,"ancestor_record_offset":22188,"inlinee":21857,"source_line":697,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1339,"record_offset":25012,"parent_record_offset":24972,"ancestor_record_offset":22188,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1339,"record_offset":25080,"parent_record_offset":24972,"ancestor_record_offset":22188,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1339,"record_offset":25104,"parent_record_offset":25080,"ancestor_record_offset":22188,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::wrap_index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1339,"record_offset":25128,"parent_record_offset":25104,"ancestor_record_offset":22188,"inlinee":11291,"source_line":2679,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1339,"record_offset":29204,"parent_record_offset":29172,"ancestor_record_offset":22188,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1339,"record_offset":25048,"parent_record_offset":25012,"ancestor_record_offset":22188,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1339,"record_offset":28448,"parent_record_offset":28420,"ancestor_record_offset":22188,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1339,"record_offset":28420,"parent_record_offset":28392,"ancestor_record_offset":22188,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1339,"record_offset":28660,"parent_record_offset":28420,"ancestor_record_offset":22188,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1339,"record_offset":29172,"parent_record_offset":29148,"ancestor_record_offset":22188,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1339,"record_offset":28352,"parent_record_offset":28316,"ancestor_record_offset":22188,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1339,"record_offset":28316,"parent_record_offset":28280,"ancestor_record_offset":22188,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1339,"record_offset":28280,"parent_record_offset":28244,"ancestor_record_offset":22188,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":26920,"parent_record_offset":26880,"ancestor_record_offset":22188,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":37856,"parent_record_offset":37808,"ancestor_record_offset":22188,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":29096,"parent_record_offset":29056,"ancestor_record_offset":22188,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":26880,"parent_record_offset":26844,"ancestor_record_offset":22188,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":38016,"parent_record_offset":37808,"ancestor_record_offset":22188,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":28392,"parent_record_offset":28352,"ancestor_record_offset":22188,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":28244,"parent_record_offset":28128,"ancestor_record_offset":22188,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":29836,"parent_record_offset":28128,"ancestor_record_offset":22188,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1339,"record_offset":38072,"parent_record_offset":38048,"ancestor_record_offset":22188,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1339,"record_offset":38120,"parent_record_offset":38096,"ancestor_record_offset":22188,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1339,"record_offset":24880,"parent_record_offset":24852,"ancestor_record_offset":22188,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1339,"record_offset":26784,"parent_record_offset":26760,"ancestor_record_offset":22188,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1339,"record_offset":26760,"parent_record_offset":26728,"ancestor_record_offset":22188,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1339,"record_offset":26844,"parent_record_offset":26760,"ancestor_record_offset":22188,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1339,"record_offset":28484,"parent_record_offset":28448,"ancestor_record_offset":22188,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1339,"record_offset":28508,"parent_record_offset":28484,"ancestor_record_offset":22188,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":1339,"record_offset":23340,"parent_record_offset":23316,"ancestor_record_offset":22188,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1339,"record_offset":28932,"parent_record_offset":28908,"ancestor_record_offset":22188,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1339,"record_offset":23388,"parent_record_offset":23364,"ancestor_record_offset":22188,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1339,"record_offset":28956,"parent_record_offset":28932,"ancestor_record_offset":22188,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1339,"record_offset":27832,"parent_record_offset":27792,"ancestor_record_offset":22188,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1339,"record_offset":24308,"parent_record_offset":24284,"ancestor_record_offset":22188,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1339,"record_offset":22916,"parent_record_offset":22892,"ancestor_record_offset":22188,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1339,"record_offset":23000,"parent_record_offset":22976,"ancestor_record_offset":22188,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1339,"record_offset":28996,"parent_record_offset":28844,"ancestor_record_offset":22188,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1339,"record_offset":29024,"parent_record_offset":28844,"ancestor_record_offset":22188,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1339,"record_offset":22940,"parent_record_offset":22916,"ancestor_record_offset":22188,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1339,"record_offset":25336,"parent_record_offset":25312,"ancestor_record_offset":22188,"inlinee":8626,"source_line":73,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1339,"record_offset":25516,"parent_record_offset":25360,"ancestor_record_offset":22188,"inlinee":8628,"source_line":85,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1339,"record_offset":25312,"parent_record_offset":25288,"ancestor_record_offset":22188,"inlinee":8594,"source_line":2898,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1339,"record_offset":25544,"parent_record_offset":25516,"ancestor_record_offset":22188,"inlinee":8630,"source_line":2905,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1339,"record_offset":25360,"parent_record_offset":25336,"ancestor_record_offset":22188,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1339,"record_offset":37092,"parent_record_offset":37048,"ancestor_record_offset":22188,"inlinee":8892,"source_line":728,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap_simple
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1339,"record_offset":34492,"parent_record_offset":34468,"ancestor_record_offset":22188,"inlinee":8891,"source_line":761,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1339,"record_offset":29808,"parent_record_offset":28128,"ancestor_record_offset":22188,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1339,"record_offset":29236,"parent_record_offset":29204,"ancestor_record_offset":22188,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1339,"record_offset":24568,"parent_record_offset":24540,"ancestor_record_offset":22188,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1339,"record_offset":23112,"parent_record_offset":23088,"ancestor_record_offset":22188,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1339,"record_offset":26256,"parent_record_offset":26232,"ancestor_record_offset":22188,"inlinee":4401,"source_line":166,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1339,"record_offset":25568,"parent_record_offset":25544,"ancestor_record_offset":22188,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1339,"record_offset":28908,"parent_record_offset":28884,"ancestor_record_offset":22188,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1339,"record_offset":28884,"parent_record_offset":28844,"ancestor_record_offset":22188,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::addr_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":26320,"parent_record_offset":26296,"ancestor_record_offset":22188,"inlinee":4641,"source_line":2014,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1339,"record_offset":29976,"parent_record_offset":29952,"ancestor_record_offset":22188,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1339,"record_offset":28700,"parent_record_offset":28352,"ancestor_record_offset":22188,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=53
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":29124,"parent_record_offset":29056,"ancestor_record_offset":22188,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1339,"record_offset":25252,"parent_record_offset":25228,"ancestor_record_offset":22188,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1339,"record_offset":23276,"parent_record_offset":23252,"ancestor_record_offset":22188,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1339,"record_offset":23252,"parent_record_offset":23228,"ancestor_record_offset":22188,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1339,"record_offset":24776,"parent_record_offset":24752,"ancestor_record_offset":22188,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1339,"record_offset":25480,"parent_record_offset":25456,"ancestor_record_offset":22188,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1339,"record_offset":37388,"parent_record_offset":37308,"ancestor_record_offset":22188,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1339,"record_offset":25428,"parent_record_offset":25392,"ancestor_record_offset":22188,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":34768,"parent_record_offset":34492,"ancestor_record_offset":22188,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":28616,"parent_record_offset":28588,"ancestor_record_offset":22188,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":34444,"parent_record_offset":34420,"ancestor_record_offset":22188,"inlinee":8889,"source_line":1002,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_simple_untyped
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":34468,"parent_record_offset":34444,"ancestor_record_offset":22188,"inlinee":8890,"source_line":1059,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":24800,"parent_record_offset":24776,"ancestor_record_offset":22188,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1339,"record_offset":28800,"parent_record_offset":28772,"ancestor_record_offset":22188,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1339,"record_offset":27916,"parent_record_offset":27888,"ancestor_record_offset":22188,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1339,"record_offset":27944,"parent_record_offset":27916,"ancestor_record_offset":22188,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1339,"record_offset":25728,"parent_record_offset":25704,"ancestor_record_offset":22188,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1339,"record_offset":37048,"parent_record_offset":37016,"ancestor_record_offset":22188,"inlinee":9338,"source_line":999,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1339,"record_offset":26660,"parent_record_offset":26632,"ancestor_record_offset":22188,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1339,"record_offset":26688,"parent_record_offset":26660,"ancestor_record_offset":22188,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1339,"record_offset":29952,"parent_record_offset":29928,"ancestor_record_offset":22188,"inlinee":5475,"source_line":373,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1339,"record_offset":29928,"parent_record_offset":29892,"ancestor_record_offset":22188,"inlinee":5473,"source_line":489,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1339,"record_offset":29892,"parent_record_offset":29864,"ancestor_record_offset":22188,"inlinee":5472,"source_line":501,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1339,"record_offset":25752,"parent_record_offset":25728,"ancestor_record_offset":22188,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1339,"record_offset":25880,"parent_record_offset":25816,"ancestor_record_offset":22188,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1339,"record_offset":25228,"parent_record_offset":25204,"ancestor_record_offset":22188,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1339,"record_offset":25456,"parent_record_offset":25392,"ancestor_record_offset":22188,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1339,"record_offset":26076,"parent_record_offset":26052,"ancestor_record_offset":22188,"inlinee":8600,"source_line":313,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1339,"record_offset":26108,"parent_record_offset":26076,"ancestor_record_offset":22188,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1339,"record_offset":25392,"parent_record_offset":25360,"ancestor_record_offset":22188,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1339,"record_offset":27860,"parent_record_offset":27832,"ancestor_record_offset":22188,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":1339,"record_offset":23364,"parent_record_offset":23340,"ancestor_record_offset":22188,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1339,"record_offset":27792,"parent_record_offset":27764,"ancestor_record_offset":22188,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1339,"record_offset":25592,"parent_record_offset":25568,"ancestor_record_offset":22188,"inlinee":62327,"source_line":132,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1339,"record_offset":27732,"parent_record_offset":27708,"ancestor_record_offset":22188,"inlinee":59460,"source_line":165,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1339,"record_offset":23564,"parent_record_offset":22692,"ancestor_record_offset":22188,"inlinee":6552,"source_line":1105,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1339,"record_offset":22692,"parent_record_offset":22656,"ancestor_record_offset":22188,"inlinee":6541,"source_line":34,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1339,"record_offset":22736,"parent_record_offset":22692,"ancestor_record_offset":22188,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":23228,"parent_record_offset":23204,"ancestor_record_offset":22188,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":24752,"parent_record_offset":23796,"ancestor_record_offset":22188,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":23524,"parent_record_offset":22788,"ancestor_record_offset":22188,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":23204,"parent_record_offset":23180,"ancestor_record_offset":22188,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":22764,"parent_record_offset":22692,"ancestor_record_offset":22188,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":23796,"parent_record_offset":23764,"ancestor_record_offset":22188,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":23592,"parent_record_offset":23564,"ancestor_record_offset":22188,"inlinee":6553,"source_line":1175,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":24640,"parent_record_offset":23796,"ancestor_record_offset":22188,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":22788,"parent_record_offset":22764,"ancestor_record_offset":22188,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":23880,"parent_record_offset":23844,"ancestor_record_offset":22188,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":24032,"parent_record_offset":23880,"ancestor_record_offset":22188,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":24196,"parent_record_offset":23880,"ancestor_record_offset":22188,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":24228,"parent_record_offset":24196,"ancestor_record_offset":22188,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":23844,"parent_record_offset":23796,"ancestor_record_offset":22188,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":22864,"parent_record_offset":22788,"ancestor_record_offset":22188,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":24540,"parent_record_offset":24516,"ancestor_record_offset":22188,"inlinee":6566,"source_line":2743,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":24516,"parent_record_offset":23844,"ancestor_record_offset":22188,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1339,"record_offset":23060,"parent_record_offset":23032,"ancestor_record_offset":22188,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1339,"record_offset":23088,"parent_record_offset":23060,"ancestor_record_offset":22188,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1339,"record_offset":23148,"parent_record_offset":23032,"ancestor_record_offset":22188,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1339,"record_offset":23032,"parent_record_offset":22788,"ancestor_record_offset":22188,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":22836,"parent_record_offset":22788,"ancestor_record_offset":22188,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":23180,"parent_record_offset":22788,"ancestor_record_offset":22188,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":24252,"parent_record_offset":24228,"ancestor_record_offset":22188,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1339,"record_offset":24612,"parent_record_offset":23796,"ancestor_record_offset":22188,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1339,"record_offset":22892,"parent_record_offset":22788,"ancestor_record_offset":22188,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1339,"record_offset":24284,"parent_record_offset":24196,"ancestor_record_offset":22188,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1339,"record_offset":22976,"parent_record_offset":22788,"ancestor_record_offset":22188,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1339,"record_offset":23432,"parent_record_offset":22788,"ancestor_record_offset":22188,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1339,"record_offset":24072,"parent_record_offset":24032,"ancestor_record_offset":22188,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcOccupiedEntry::into_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1339,"record_offset":23700,"parent_record_offset":23676,"ancestor_record_offset":22188,"inlinee":6550,"source_line":412,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1339,"record_offset":23764,"parent_record_offset":23732,"ancestor_record_offset":22188,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1339,"record_offset":23316,"parent_record_offset":23180,"ancestor_record_offset":22188,"inlinee":6544,"source_line":36,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::OccupiedEntry::into_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1339,"record_offset":23676,"parent_record_offset":23640,"ancestor_record_offset":22188,"inlinee":6548,"source_line":2927,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1339,"record_offset":23732,"parent_record_offset":23640,"ancestor_record_offset":22188,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::new_from_points
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1339,"record_offset":28772,"parent_record_offset":28128,"ancestor_record_offset":22188,"inlinee":21342,"source_line":216,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::try_slice
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1339,"record_offset":29056,"parent_record_offset":28128,"ancestor_record_offset":22188,"inlinee":61974,"source_line":677,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1339,"record_offset":28844,"parent_record_offset":28128,"ancestor_record_offset":22188,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::replay_raster::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF377
// 首条位置: {"module":1339,"inlinee":62327}
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
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 269
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F41
// 首条位置: {"module":421,"inlinee":24385}
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
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 360
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF37B
// 首条位置: {"module":1339,"inlinee":62331}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":11376,"parent_record_offset":11256,"ancestor_record_offset":11256,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":12972,"parent_record_offset":11256,"ancestor_record_offset":11256,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1339,"record_offset":12944,"parent_record_offset":11256,"ancestor_record_offset":11256,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":12480,"parent_record_offset":11256,"ancestor_record_offset":11256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::new_from_points
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1339,"record_offset":11908,"parent_record_offset":11256,"ancestor_record_offset":11256,"inlinee":21342,"source_line":216,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::try_slice
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1339,"record_offset":12192,"parent_record_offset":11256,"ancestor_record_offset":11256,"inlinee":61974,"source_line":677,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1339,"record_offset":11980,"parent_record_offset":11256,"ancestor_record_offset":11256,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1339,"record_offset":11700,"parent_record_offset":11676,"ancestor_record_offset":11256,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1339,"record_offset":11724,"parent_record_offset":11700,"ancestor_record_offset":11256,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1339,"record_offset":12432,"parent_record_offset":12408,"ancestor_record_offset":11256,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1339,"record_offset":11676,"parent_record_offset":11584,"ancestor_record_offset":11256,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1339,"record_offset":12408,"parent_record_offset":12308,"ancestor_record_offset":11256,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1339,"record_offset":12340,"parent_record_offset":12308,"ancestor_record_offset":11256,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1339,"record_offset":11584,"parent_record_offset":11556,"ancestor_record_offset":11256,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1339,"record_offset":11556,"parent_record_offset":11528,"ancestor_record_offset":11256,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1339,"record_offset":11796,"parent_record_offset":11556,"ancestor_record_offset":11256,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1339,"record_offset":12308,"parent_record_offset":12284,"ancestor_record_offset":11256,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1339,"record_offset":11484,"parent_record_offset":11448,"ancestor_record_offset":11256,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1339,"record_offset":11448,"parent_record_offset":11412,"ancestor_record_offset":11256,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1339,"record_offset":11412,"parent_record_offset":11376,"ancestor_record_offset":11256,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":12232,"parent_record_offset":12192,"ancestor_record_offset":11256,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1339,"record_offset":11528,"parent_record_offset":11484,"ancestor_record_offset":11256,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1339,"record_offset":11620,"parent_record_offset":11584,"ancestor_record_offset":11256,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1339,"record_offset":11644,"parent_record_offset":11620,"ancestor_record_offset":11256,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1339,"record_offset":12068,"parent_record_offset":12044,"ancestor_record_offset":11256,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1339,"record_offset":12092,"parent_record_offset":12068,"ancestor_record_offset":11256,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1339,"record_offset":12132,"parent_record_offset":11980,"ancestor_record_offset":11256,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1339,"record_offset":12160,"parent_record_offset":11980,"ancestor_record_offset":11256,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1339,"record_offset":11860,"parent_record_offset":11836,"ancestor_record_offset":11256,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1339,"record_offset":12372,"parent_record_offset":12340,"ancestor_record_offset":11256,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1339,"record_offset":12044,"parent_record_offset":12020,"ancestor_record_offset":11256,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1339,"record_offset":12020,"parent_record_offset":11980,"ancestor_record_offset":11256,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1339,"record_offset":13112,"parent_record_offset":13088,"ancestor_record_offset":11256,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1339,"record_offset":11836,"parent_record_offset":11484,"ancestor_record_offset":11256,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=84
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":12260,"parent_record_offset":12192,"ancestor_record_offset":11256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1339,"record_offset":11752,"parent_record_offset":11724,"ancestor_record_offset":11256,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1339,"record_offset":11936,"parent_record_offset":11908,"ancestor_record_offset":11256,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1339,"record_offset":13000,"parent_record_offset":12972,"ancestor_record_offset":11256,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1339,"record_offset":13088,"parent_record_offset":13064,"ancestor_record_offset":11256,"inlinee":5475,"source_line":373,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1339,"record_offset":13064,"parent_record_offset":13028,"ancestor_record_offset":11256,"inlinee":5473,"source_line":489,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1339,"record_offset":13028,"parent_record_offset":13000,"ancestor_record_offset":11256,"inlinee":5472,"source_line":501,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::duration_based_on_stroke_length
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 397
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6AD2
// 首条位置: {"module":485,"inlinee":27346}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":485,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":1028,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00485.jsonl.gz"}
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
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::history::replay_raster::RasterReplayDirtyLayerState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "tuple$<core::any::TypeId,country_core::systems::history::replay_raster::RasterReplayDirtyLayerState>", "constructor": "tuple$", "arguments": ["core::any::TypeId", "country_core::systems::history::replay_raster::RasterReplayDirtyLayerState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<core::any::TypeId,country_core::systems::history::replay_raster::RasterReplayDirtyLayerState,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["core::any::TypeId", "country_core::systems::history::replay_raster::RasterReplayDirtyLayerState", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<core::any::TypeId,country_core::systems::history::replay_raster::RasterReplayDirtyLayerState>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<core::any::TypeId,country_core::systems::history::replay_raster::RasterReplayDirtyLayerState,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<core::any::TypeId,country_core::systems::history::replay_raster::RasterReplayDirtyLayerState>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<core::any::TypeId,country_core::systems::history::replay_raster::RasterReplayDirtyLayerState,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::history::replay_raster::RasterReplayResources
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 84
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::initialize", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::run_unsafe", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::get_last_run<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::get_last_run", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::has_deferred<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::has_deferred", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::set_last_run<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::set_last_run", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::apply_deferred", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::queue_deferred", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::component_access<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::component_access", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::check_change_tick<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::check_change_tick", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::archetype_component_access<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::archetype_component_access", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::name<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::name", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_send<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_send", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::default_system_sets<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::default_system_sets", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::default_system_sets<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::default_system_sets", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::default_system_sets<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$7::default_system_sets", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::SystemMeta::new<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>", "constructor": "bevy_ecs::system::function_system::SystemMeta::new", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::SystemMeta::new<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::SystemMeta::new", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::SystemMeta::new<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::SystemMeta::new", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::initialize", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::run_unsafe", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::get_last_run<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::get_last_run", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::has_deferred<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::has_deferred", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_exclusive<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_exclusive", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::set_last_run<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::set_last_run", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::apply_deferred", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::queue_deferred", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::component_access<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::component_access", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::check_change_tick<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::check_change_tick", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::archetype_component_access<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::archetype_component_access", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::name<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::name", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_send<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_send", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)>", "constructor": "tuple$", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::SystemConfigTupleMarker", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoSystemConfigs::in_set<tuple$<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)>,tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)> >,country_core::systems::GameSystemSet>", "constructor": "bevy_ecs::schedule::config::IntoSystemConfigs::in_set", "arguments": ["tuple$<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)>", "tuple$<bevy_ecs::schedule::config::SystemConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)> >", "country_core::systems::GameSystemSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$7::initialize", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$7::run_unsafe", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::get_last_run<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$7::get_last_run", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::has_deferred<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$7::has_deferred", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::set_last_run<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$7::set_last_run", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$7::apply_deferred", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$7::queue_deferred", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::component_access<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$7::component_access", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::check_change_tick<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$7::check_change_tick", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::archetype_component_access<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$7::archetype_component_access", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::name<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$7::name", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_send<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_send", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::history::replay_raster::_::FetchState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::history::replay_raster::_::FetchState>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::history::replay_raster::_::FetchState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: replay_raster.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: replay_raster.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 25; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)
// 候选参数类型: ["country_core::systems::history::replay_raster::RasterReplayResources", "bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>"]
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::DebugHistoryMsg>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>", "bevy_ecs::event::EventWriter<country_core::systems::history::history_process::DebugHistoryMsg>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::history::history_process::DebugHistoryMsg>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathEraseEvent>,bevy_ecs::event::EventWriter<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "country_core::systems::world_raster::RasterDrawResources", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>", "bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>", "bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathEraseEvent>", "bevy_ecs::event::EventWriter<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>", "bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >", "bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathEraseEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)
// 候选参数类型: ["country_core::systems::history::replay_raster::RasterReplayResources", "bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>"]
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::event::EventReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::EventReader<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::change_detection::ResMut<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::event::EventReader<country_core::resources::session::NewSessionStartedCmd>", "bevy_ecs::event::EventReader<country_core::resources::session::SessionLoadedCmd>", "bevy_ecs::change_detection::ResMut<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::session::NewSessionStartedCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::session::SessionLoadedCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >
// [文件级接口候选，具体函数归属 unknown] serde::__private::de::content::ContentRefDeserializer<serde_json::error::Error> (*)(ref$<enum2$<serde::__private::de::content::Content> >)
// 候选参数类型: ["ref$<enum2$<serde::__private::de::content::Content> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::RasterReplayCmd::backward
// 信息 ID: e5068a62bf5801879ec10685
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::RasterReplayCmd::backward_instant
// 信息 ID: bfaba985e27fc2d60ddae199
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=484, inline_record=17480; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=484, inline_record=18076; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=484, inline_record=18676; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::RasterReplayCmd::forward
// 信息 ID: 2c4a868ed59064b57f1997a1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::RasterReplayCmd::forward_instant
// 信息 ID: 6ef45cfc4f06ca146ed688e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=484, inline_record=17564; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=484, inline_record=18160; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=484, inline_record=18760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::RasterReplayCmd::new
// 信息 ID: 0d595f39bfc8a829c6006e4d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=484, inline_record=17504; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=484, inline_record=17588; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=484, inline_record=18100; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=484, inline_record=18184; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=484, inline_record=18700; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=484, inline_record=18784; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::RasterReplayCmd::forward; module=485, inline_record=1084; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::RasterReplayCmd::backward; module=485, inline_record=2104; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::_::impl$0::get_param
// 信息 ID: 52f24a2e7c4c8a2a67ee9255
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 073ddd4fb6d7e2d26d6baea8: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)
// [泛型宿主/闭包/直接引用方参数候选] b2aee2b30023b94b63d3728c: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)
// [泛型宿主/闭包/直接引用方参数候选] fd27d2a8ea15d7e0f917f88e: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> @ 0x6afece -> 0x696830 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)> > @ 0xe3d0d9 -> 0x696830 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> > @ 0xe55e09 -> 0x696830 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> > @ 0xe585d9 -> 0x696830 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)> @ 0x107c33e -> 0x696830 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> @ 0x10892be -> 0x696830 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::_::impl$0::init_state
// 信息 ID: 062bf639abdfd9cdab2af990
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 073ddd4fb6d7e2d26d6baea8: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)
// [泛型宿主/闭包/直接引用方参数候选] b2aee2b30023b94b63d3728c: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)
// [泛型宿主/闭包/直接引用方参数候选] fd27d2a8ea15d7e0f917f88e: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>; module=491, inline_record=1012; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>; module=1426, inline_record=4536; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>; module=1431, inline_record=7456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::duration_based_on_stroke_length
// 信息 ID: 01b920bc5c4466b5c5fd158b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_raster::RasterReplayCmd::forward; module=485, inline_record=1028; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::RasterReplayCmd::backward; module=485, inline_record=2048; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::get_stroke_from_0_to_t
// 信息 ID: 799c98f9db1ddb9cd97ffe23
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>; module=1339, inline_record=11256; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>; module=1339, inline_record=14280; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=1339, inline_record=28128; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=1339, inline_record=31144; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>; module=1339, inline_record=44988; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>; module=1339, inline_record=48012; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::impl$2::default
// 信息 ID: 1d3b8abb6443971a8edd2aa8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 073ddd4fb6d7e2d26d6baea8: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)
// [泛型宿主/闭包/直接引用方参数候选] b2aee2b30023b94b63d3728c: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)
// [泛型宿主/闭包/直接引用方参数候选] fd27d2a8ea15d7e0f917f88e: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>; module=491, inline_record=1120; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)>; module=1426, inline_record=4644; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>; module=1431, inline_record=7564; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::replay_raster::closure$0
// 信息 ID: fbb7309afd2e507c5c590eed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::history::replay_raster::replay_raster", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>; module=1339, inline_record=9240; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=1339, inline_record=25592; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>; module=1339, inline_record=42972; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::replay_raster::closure$1
// 信息 ID: 7366051c3deea27a3bc0faf1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::history::replay_raster::replay_raster", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::history::replay_raster::replay_raster::closure_env$1 <- country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_raster::replay_raster::closure_env$1 <- country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_raster::replay_raster::closure_env$1 <- country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_raster::replay_raster::closure_env$1 <- conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_raster::replay_raster::closure_env$1 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec2::Vec2,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec2::Vec2>,core::iter::sources::once::Once<glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > >; module=421, inline_record=8432; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > >; module=421, inline_record=8720; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > >; module=421, inline_record=9948; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > >; module=421, inline_record=10236; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> > >; module=421, inline_record=11468; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> > >; module=421, inline_record=11756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>
// 信息 ID: 96c8bb186d52f665a62a961f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] fd27d2a8ea15d7e0f917f88e: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster -> ["country_core::resources::world_raster_defs::GardenRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_garden::replay_garden::closure_env$2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> @ 0x6b0040 -> 0xecc3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> > @ 0xe5874b -> 0xecc3c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>
// 信息 ID: 353ac7c50f823d174b37aeef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b2aee2b30023b94b63d3728c: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster -> ["country_core::resources::world_raster_defs::PathRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)> > @ 0xe3d24b -> 0xec7680 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)> @ 0x107c4b0 -> 0xec7680 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>
// 信息 ID: c6f4e1f925ceca86d0979fa2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 073ddd4fb6d7e2d26d6baea8: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster -> ["country_core::resources::world_raster_defs::WaterRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_water::replay_water::closure_env$2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> > @ 0xe5611c -> 0xec9c80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> @ 0x10895dd -> 0xec9c80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

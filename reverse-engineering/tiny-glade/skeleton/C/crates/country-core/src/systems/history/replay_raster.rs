// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/history/replay_raster.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 909deae1642da94fc633d2108e23ed7319a440ec181afe7b307816a920c8cb66
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: replay_raster.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 571, 593, 594, 1674, 1698, 1720, 1745, 1823
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::impl$7::eq
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8D6A
// 首条位置: {"module":571,"inlinee":36202}
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
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x142AC
// 首条位置: {"module":1674,"inlinee":82604}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::Arc::as_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1674,"record_offset":95328,"parent_record_offset":95304,"ancestor_record_offset":95304,"inlinee":7029,"source_line":1810,"archive":"_pdb/C/modules/01674.jsonl.gz"}
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
// 首条位置: {"module":1674,"record_offset":89164,"rva":25123280,"va":5393832400}
// 编译实例 RVAs: 0x17f59d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c92250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11942120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393832400,"raw_hex":"e818c749ff","size":5,"soft_fail":false,"target_va":5381890640,"va":5393832755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t22789070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393832400,"raw_hex":"e8cebb5b01","size":5,"soft_fail":false,"target_va":5416621762,"va":5393832687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1674,"record_offset":89792,"parent_record_offset":89164,"ancestor_record_offset":89164,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":1674,"record_offset":89332,"parent_record_offset":89164,"ancestor_record_offset":89164,"inlinee":37374,"source_line":70,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::RasterReplayCmd::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1674,"record_offset":89764,"parent_record_offset":89164,"ancestor_record_offset":89164,"inlinee":37844,"source_line":115,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::duration_based_on_stroke_length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1674,"record_offset":89708,"parent_record_offset":89164,"ancestor_record_offset":89164,"inlinee":82600,"source_line":472,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1674,"record_offset":89816,"parent_record_offset":89792,"ancestor_record_offset":89164,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1674,"record_offset":89428,"parent_record_offset":89400,"ancestor_record_offset":89164,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1674,"record_offset":89732,"parent_record_offset":89708,"ancestor_record_offset":89164,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1674,"record_offset":89660,"parent_record_offset":89636,"ancestor_record_offset":89164,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1674,"record_offset":89400,"parent_record_offset":89372,"ancestor_record_offset":89164,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1674,"record_offset":89372,"parent_record_offset":89332,"ancestor_record_offset":89164,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1674,"record_offset":89540,"parent_record_offset":89496,"ancestor_record_offset":89164,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1674,"record_offset":89852,"parent_record_offset":89816,"ancestor_record_offset":89164,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1674,"record_offset":89876,"parent_record_offset":89852,"ancestor_record_offset":89164,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1674,"record_offset":89908,"parent_record_offset":89816,"ancestor_record_offset":89164,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1674,"record_offset":89468,"parent_record_offset":89332,"ancestor_record_offset":89164,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1674,"record_offset":89600,"parent_record_offset":89496,"ancestor_record_offset":89164,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1674,"record_offset":89496,"parent_record_offset":89468,"ancestor_record_offset":89164,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1674,"record_offset":89636,"parent_record_offset":89600,"ancestor_record_offset":89164,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::RasterReplayCmd::forward_instant
// 记录形式: inline_instance, inlinee_declaration; instances=20
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x93D5
// 首条位置: {"module":594,"inlinee":37845}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::history::replay_raster::RasterReplayCmd::new
//     owner=private_source_ownership_candidate; count=18
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1674,"record_offset":95684,"parent_record_offset":95660,"ancestor_record_offset":95660,"inlinee":37844,"source_line":115,"archive":"_pdb/C/modules/01674.jsonl.gz"}
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
// 首条位置: {"module":1674,"record_offset":89948,"rva":25123664,"va":5393832784}
// 编译实例 RVAs: 0x17f5b50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c92250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11942504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393832784,"raw_hex":"e898c549ff","size":5,"soft_fail":false,"target_va":5381890640,"va":5393833139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t22788686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393832784,"raw_hex":"e84eba5b01","size":5,"soft_fail":false,"target_va":5416621762,"va":5393833071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1674,"record_offset":90576,"parent_record_offset":89948,"ancestor_record_offset":89948,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":1674,"record_offset":90116,"parent_record_offset":89948,"ancestor_record_offset":89948,"inlinee":37374,"source_line":70,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::RasterReplayCmd::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1674,"record_offset":90548,"parent_record_offset":89948,"ancestor_record_offset":89948,"inlinee":37844,"source_line":115,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::duration_based_on_stroke_length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1674,"record_offset":90492,"parent_record_offset":89948,"ancestor_record_offset":89948,"inlinee":82600,"source_line":472,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1674,"record_offset":90600,"parent_record_offset":90576,"ancestor_record_offset":89948,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1674,"record_offset":90212,"parent_record_offset":90184,"ancestor_record_offset":89948,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1674,"record_offset":90516,"parent_record_offset":90492,"ancestor_record_offset":89948,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1674,"record_offset":90444,"parent_record_offset":90420,"ancestor_record_offset":89948,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1674,"record_offset":90184,"parent_record_offset":90156,"ancestor_record_offset":89948,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1674,"record_offset":90156,"parent_record_offset":90116,"ancestor_record_offset":89948,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1674,"record_offset":90324,"parent_record_offset":90280,"ancestor_record_offset":89948,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1674,"record_offset":90636,"parent_record_offset":90600,"ancestor_record_offset":89948,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1674,"record_offset":90660,"parent_record_offset":90636,"ancestor_record_offset":89948,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1674,"record_offset":90692,"parent_record_offset":90600,"ancestor_record_offset":89948,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1674,"record_offset":90252,"parent_record_offset":90116,"ancestor_record_offset":89948,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1674,"record_offset":90384,"parent_record_offset":90280,"ancestor_record_offset":89948,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1674,"record_offset":90280,"parent_record_offset":90252,"ancestor_record_offset":89948,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1674,"record_offset":90420,"parent_record_offset":90384,"ancestor_record_offset":89948,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::RasterReplayCmd::backward_instant
// 记录形式: inline_instance, inlinee_declaration; instances=20
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x93D3
// 首条位置: {"module":594,"inlinee":37843}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::history::replay_raster::RasterReplayCmd::new
//     owner=private_source_ownership_candidate; count=18
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1674,"record_offset":95600,"parent_record_offset":95576,"ancestor_record_offset":95576,"inlinee":37844,"source_line":115,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::RasterReplayCmd::new
// 记录形式: inline_instance, inlinee_declaration; instances=40
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x93D4
// 首条位置: {"module":594,"inlinee":37844}
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
// IPI IDs: 0x93AE
// 首条位置: {"module":593,"record_offset":100660,"rva":12705776,"va":5381414896}
// 编译实例 RVAs: 0xc1dff0, 0x1904000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t35206898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381414896,"raw_hex":"e8f2361902","size":5,"soft_fail":false,"target_va":5416622240,"va":5381415337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$112::get_param
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":593,"record_offset":100812,"parent_record_offset":100660,"ancestor_record_offset":100660,"inlinee":4631,"source_line":2160,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":593,"record_offset":101252,"parent_record_offset":101228,"ancestor_record_offset":100660,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":593,"record_offset":101300,"parent_record_offset":101276,"ancestor_record_offset":100660,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":593,"record_offset":101276,"parent_record_offset":101252,"ancestor_record_offset":100660,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":593,"record_offset":101228,"parent_record_offset":101204,"ancestor_record_offset":100660,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":593,"record_offset":101204,"parent_record_offset":101164,"ancestor_record_offset":100660,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":593,"record_offset":101164,"parent_record_offset":101140,"ancestor_record_offset":100660,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":593,"record_offset":103160,"parent_record_offset":103136,"ancestor_record_offset":100660,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":593,"record_offset":103136,"parent_record_offset":103100,"ancestor_record_offset":100660,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":593,"record_offset":101456,"parent_record_offset":101420,"ancestor_record_offset":100660,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":593,"record_offset":101392,"parent_record_offset":100896,"ancestor_record_offset":100660,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":593,"record_offset":101420,"parent_record_offset":101392,"ancestor_record_offset":100660,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":593,"record_offset":100928,"parent_record_offset":100896,"ancestor_record_offset":100660,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":593,"record_offset":101484,"parent_record_offset":101420,"ancestor_record_offset":100660,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":593,"record_offset":100968,"parent_record_offset":100948,"ancestor_record_offset":100660,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":593,"record_offset":100948,"parent_record_offset":100928,"ancestor_record_offset":100660,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":593,"record_offset":101140,"parent_record_offset":101116,"ancestor_record_offset":100660,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$7::get_param
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":593,"record_offset":100868,"parent_record_offset":100812,"ancestor_record_offset":100660,"inlinee":4156,"source_line":803,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$8::get_param
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":593,"record_offset":102576,"parent_record_offset":100812,"ancestor_record_offset":100660,"inlinee":4194,"source_line":882,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$8::get_param::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":593,"record_offset":103344,"parent_record_offset":103320,"ancestor_record_offset":100660,"inlinee":4430,"source_line":890,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":593,"record_offset":103228,"parent_record_offset":103204,"ancestor_record_offset":100660,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":593,"record_offset":103204,"parent_record_offset":102620,"ancestor_record_offset":100660,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":593,"record_offset":102620,"parent_record_offset":102576,"ancestor_record_offset":100660,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_with_ticks
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":593,"record_offset":100896,"parent_record_offset":100868,"ancestor_record_offset":100660,"inlinee":4157,"source_line":641,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::rt::Argument::new_display
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\rt.rs
//     evidence={"module":593,"record_offset":103368,"parent_record_offset":103344,"ancestor_record_offset":100660,"inlinee":4432,"source_line":89,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":593,"record_offset":101056,"parent_record_offset":100996,"ancestor_record_offset":100660,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":593,"record_offset":101076,"parent_record_offset":101056,"ancestor_record_offset":100660,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":593,"record_offset":101116,"parent_record_offset":100948,"ancestor_record_offset":100660,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":593,"record_offset":103320,"parent_record_offset":102576,"ancestor_record_offset":100660,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::from_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":593,"record_offset":103392,"parent_record_offset":103368,"ancestor_record_offset":100660,"inlinee":4434,"source_line":282,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":593,"record_offset":100996,"parent_record_offset":100968,"ancestor_record_offset":100660,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":593,"record_offset":101028,"parent_record_offset":100996,"ancestor_record_offset":100660,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":593,"record_offset":101344,"parent_record_offset":101164,"ancestor_record_offset":100660,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":593,"record_offset":103252,"parent_record_offset":103228,"ancestor_record_offset":100660,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":593,"record_offset":103276,"parent_record_offset":103252,"ancestor_record_offset":100660,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":593,"record_offset":101832,"parent_record_offset":101808,"ancestor_record_offset":100660,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::_::impl$0::init_access
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8D82
// 首条位置: {"module":571,"inlinee":36226}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::_::impl$0::init_state
// 记录形式: compiled_procedure; instances=1
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1674,"record_offset":90732,"rva":25124048,"va":5393833168}
// 编译实例 RVAs: 0x17f5cd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t-18154596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393833168,"raw_hex":"e89cfbeafe","size":5,"soft_fail":false,"target_va":5375678704,"va":5393833295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416ce6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1209956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393833168,"raw_hex":"e89c89edff","size":5,"soft_fail":false,"target_va":5392623312,"va":5393833263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416d4c50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1184236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393833168,"raw_hex":"e814eeedff","size":5,"soft_fail":false,"target_va":5392649296,"va":5393833527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416da350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1161876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393833168,"raw_hex":"e86c45eeff","size":5,"soft_fail":false,"target_va":5392671568,"va":5393833439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416db570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1157148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393833168,"raw_hex":"e8e457eeff","size":5,"soft_fail":false,"target_va":5392676208,"va":5393833351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23223507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393833168,"raw_hex":"e8d35c6201","size":5,"soft_fail":false,"target_va":5417057088,"va":5393833576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$112::init_state
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":1674,"record_offset":90884,"parent_record_offset":90732,"ancestor_record_offset":90732,"inlinee":4629,"source_line":2123,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1674,"record_offset":91208,"parent_record_offset":91184,"ancestor_record_offset":90732,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1674,"record_offset":91232,"parent_record_offset":91208,"ancestor_record_offset":90732,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1674,"record_offset":91256,"parent_record_offset":91232,"ancestor_record_offset":90732,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1674,"record_offset":91184,"parent_record_offset":91096,"ancestor_record_offset":90732,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1674,"record_offset":91124,"parent_record_offset":91096,"ancestor_record_offset":90732,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1674,"record_offset":91096,"parent_record_offset":91072,"ancestor_record_offset":90732,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1674,"record_offset":91072,"parent_record_offset":91052,"ancestor_record_offset":90732,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::register::ComponentsRegistrator::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/register.rs
//     evidence={"module":1674,"record_offset":90980,"parent_record_offset":90948,"ancestor_record_offset":90732,"inlinee":4111,"source_line":85,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$17::init_state
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":1674,"record_offset":93860,"parent_record_offset":90884,"ancestor_record_offset":90732,"inlinee":5986,"source_line":1098,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$7::init_state
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":1674,"record_offset":90916,"parent_record_offset":90884,"ancestor_record_offset":90732,"inlinee":4108,"source_line":761,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$8::init_state
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":1674,"record_offset":93120,"parent_record_offset":90884,"ancestor_record_offset":90732,"inlinee":4129,"source_line":838,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::components_registrator
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":1674,"record_offset":90948,"parent_record_offset":90916,"ancestor_record_offset":90732,"inlinee":4109,"source_line":238,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$8::from_world
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":1674,"record_offset":93884,"parent_record_offset":93860,"ancestor_record_offset":90732,"inlinee":4618,"source_line":3695,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1674,"record_offset":91152,"parent_record_offset":91124,"ancestor_record_offset":90732,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1674,"record_offset":91012,"parent_record_offset":90916,"ancestor_record_offset":90732,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::impl$9::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1674,"record_offset":93908,"parent_record_offset":93884,"ancestor_record_offset":90732,"inlinee":36224,"source_line":145,"archive":"_pdb/C/modules/01674.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::_::impl$0::validate_param
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9399
// 首条位置: {"module":593,"inlinee":37785}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$7::validate_param
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":593,"record_offset":69492,"parent_record_offset":69468,"ancestor_record_offset":69468,"inlinee":4222,"source_line":783,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$8::validate_param
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":593,"record_offset":71048,"parent_record_offset":69468,"ancestor_record_offset":69468,"inlinee":4225,"source_line":862,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":593,"record_offset":69852,"parent_record_offset":69828,"ancestor_record_offset":69468,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":593,"record_offset":69900,"parent_record_offset":69876,"ancestor_record_offset":69468,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":593,"record_offset":69876,"parent_record_offset":69852,"ancestor_record_offset":69468,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":593,"record_offset":69828,"parent_record_offset":69804,"ancestor_record_offset":69468,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":593,"record_offset":69804,"parent_record_offset":69764,"ancestor_record_offset":69468,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":593,"record_offset":69764,"parent_record_offset":69740,"ancestor_record_offset":69468,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":593,"record_offset":70060,"parent_record_offset":70036,"ancestor_record_offset":69468,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":593,"record_offset":70036,"parent_record_offset":70016,"ancestor_record_offset":69468,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":593,"record_offset":69520,"parent_record_offset":69492,"ancestor_record_offset":69468,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":593,"record_offset":69568,"parent_record_offset":69544,"ancestor_record_offset":69468,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":593,"record_offset":69544,"parent_record_offset":69520,"ancestor_record_offset":69468,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":593,"record_offset":69740,"parent_record_offset":69716,"ancestor_record_offset":69468,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":593,"record_offset":69656,"parent_record_offset":69596,"ancestor_record_offset":69468,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":593,"record_offset":69676,"parent_record_offset":69656,"ancestor_record_offset":69468,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":593,"record_offset":69992,"parent_record_offset":69492,"ancestor_record_offset":69468,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":593,"record_offset":69716,"parent_record_offset":69544,"ancestor_record_offset":69468,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":593,"record_offset":69596,"parent_record_offset":69568,"ancestor_record_offset":69468,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":593,"record_offset":69628,"parent_record_offset":69596,"ancestor_record_offset":69468,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":593,"record_offset":69944,"parent_record_offset":69764,"ancestor_record_offset":69468,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":593,"record_offset":70380,"parent_record_offset":70356,"ancestor_record_offset":69468,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00593.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::impl$9::default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 145
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8D80
// 首条位置: {"module":571,"inlinee":36224}
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
// 首条位置: {"module":571,"record_offset":84716,"rva":11996992,"va":5380706112}
// 编译实例 RVAs: 0xb70f40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::vec::Vec<_, _>>::remove::assert_failed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t35907143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e847e62302","size":5,"soft_fail":false,"target_va":5416621000,"va":5380713852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5029668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e8dc40b3ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380708351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=24
//     evidence={"assembly":"\tcallq\t-5030174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e8e23eb3ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380708873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11986144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e8201b49ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380708331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081c20
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11473729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e8bfec50ff","size":5,"soft_fail":false,"target_va":5369240608,"va":5380714332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096e970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2114782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e822bbdfff","size":5,"soft_fail":false,"target_va":5378599280,"va":5380714057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a18d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1905363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e82dede2ff","size":5,"soft_fail":false,"target_va":5378808016,"va":5380713374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t869800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e8a8450d00","size":5,"soft_fail":false,"target_va":5381577744,"va":5380707939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c87180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1138885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e8c5601100","size":5,"soft_fail":false,"target_va":5381845376,"va":5380706486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c874c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1138199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e8175e1100","size":5,"soft_fail":false,"target_va":5381846208,"va":5380708004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c87a10
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1140722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e8f2671100","size":5,"soft_fail":false,"target_va":5381847568,"va":5380706841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c88320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1137619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e8d35b1100","size":5,"soft_fail":false,"target_va":5381849888,"va":5380712264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c886b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1138122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e8ca5d1100","size":5,"soft_fail":false,"target_va":5381850800,"va":5380712673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416296d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11240404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e8d483ab00","size":5,"soft_fail":false,"target_va":5391947472,"va":5380707063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196d050
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14661273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e899b6df00","size":5,"soft_fail":false,"target_va":5395370064,"va":5380708786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419709b0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14676107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e88bf0df00","size":5,"soft_fail":false,"target_va":5395384752,"va":5380708640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a70fc0
//     owner=unresolved_namespace; count=3
//     target_aliases=9 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15727740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e87cfcef00","size":5,"soft_fail":false,"target_va":5396434880,"va":5380707135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35639501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e8cdd01f02","size":5,"soft_fail":false,"target_va":5416353040,"va":5380713534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36343894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e856902a02","size":5,"soft_fail":false,"target_va":5417057072,"va":5380713173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36350629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e8a5aa2a02","size":5,"soft_fail":false,"target_va":5417057088,"va":5380706454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35653220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e864062002","size":5,"soft_fail":false,"target_va":5416365968,"va":5380712743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35906851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e823e52302","size":5,"soft_fail":false,"target_va":5416620794,"va":5380713938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35907725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e88de82302","size":5,"soft_fail":false,"target_va":5416621600,"va":5380713870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35908707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e863ec2302","size":5,"soft_fail":false,"target_va":5416622208,"va":5380713496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35907868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e81ce92302","size":5,"soft_fail":false,"target_va":5416621762,"va":5380713889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35908520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e8a8eb2302","size":5,"soft_fail":false,"target_va":5416622240,"va":5380713715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t14253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e8ad370000","size":5,"soft_fail":false,"target_va":5380720928,"va":5380706670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35908026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e8bae92302","size":5,"soft_fail":false,"target_va":5416621632,"va":5380713601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35908522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e8aaeb2302","size":5,"soft_fail":false,"target_va":5416622320,"va":5380713793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::cubic_in_out
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2588960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380706112,"raw_hex":"e820812700","size":5,"soft_fail":false,"target_va":5383297104,"va":5380708139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*%r12","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5380706112,"raw_hex":"41ffd4","size":3,"soft_fail":false,"va":5380706390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":86664,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":85256,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":97508,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":97968,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":6391,"source_line":2341,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":571,"record_offset":97828,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":571,"record_offset":85284,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":571,"record_offset":87944,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":571,"record_offset":85840,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":571,"record_offset":95912,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":5745,"source_line":756,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":571,"record_offset":96512,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":571,"record_offset":87880,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":571,"record_offset":96544,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":96872,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":86196,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":12073,"source_line":136,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":95576,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":85756,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":96484,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":97716,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":5742,"source_line":978,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":86364,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":571,"record_offset":87820,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":571,"record_offset":87916,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":28513,"source_line":306,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::MultiEdit::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":571,"record_offset":86464,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":35902,"source_line":477,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::history_chain::HistoryEditChain::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":571,"record_offset":86252,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":34437,"source_line":414,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":571,"record_offset":85340,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":35487,"source_line":202,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::iter_points_xz
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":571,"record_offset":87204,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":36204,"source_line":59,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::radius
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":571,"record_offset":87344,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":36205,"source_line":30,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$3::extend_dirty
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":571,"record_offset":87764,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":36208,"source_line":610,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_path::replay_path::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_path.rs
//     evidence={"module":571,"record_offset":87540,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":36207,"source_line":35,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::get_stroke_from_0_to_t
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":88004,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":36210,"source_line":435,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":87176,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":36202,"source_line":39,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":571,"record_offset":86224,"parent_record_offset":84716,"ancestor_record_offset":84716,"inlinee":30219,"source_line":156,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":88496,"parent_record_offset":88472,"ancestor_record_offset":84716,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":88520,"parent_record_offset":88496,"ancestor_record_offset":84716,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":89456,"parent_record_offset":89432,"ancestor_record_offset":84716,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":89480,"parent_record_offset":89456,"ancestor_record_offset":84716,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":88544,"parent_record_offset":88520,"ancestor_record_offset":84716,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":89504,"parent_record_offset":89480,"ancestor_record_offset":84716,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":88472,"parent_record_offset":88432,"ancestor_record_offset":84716,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":89432,"parent_record_offset":89340,"ancestor_record_offset":84716,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":85492,"parent_record_offset":85464,"ancestor_record_offset":84716,"inlinee":4786,"source_line":991,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":85368,"parent_record_offset":85340,"ancestor_record_offset":84716,"inlinee":30608,"source_line":1998,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":85396,"parent_record_offset":85368,"ancestor_record_offset":84716,"inlinee":30609,"source_line":937,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":85592,"parent_record_offset":85396,"ancestor_record_offset":84716,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":85436,"parent_record_offset":85396,"ancestor_record_offset":84716,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":85464,"parent_record_offset":85436,"ancestor_record_offset":84716,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":85524,"parent_record_offset":85492,"ancestor_record_offset":84716,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":85628,"parent_record_offset":85592,"ancestor_record_offset":84716,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":96720,"parent_record_offset":96692,"ancestor_record_offset":84716,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":88364,"parent_record_offset":88332,"ancestor_record_offset":84716,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":85548,"parent_record_offset":85524,"ancestor_record_offset":84716,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":89368,"parent_record_offset":89340,"ancestor_record_offset":84716,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":89340,"parent_record_offset":89316,"ancestor_record_offset":84716,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":96780,"parent_record_offset":96748,"ancestor_record_offset":84716,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":85676,"parent_record_offset":85652,"ancestor_record_offset":84716,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":85652,"parent_record_offset":85628,"ancestor_record_offset":84716,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":96748,"parent_record_offset":96720,"ancestor_record_offset":84716,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":88432,"parent_record_offset":88396,"ancestor_record_offset":84716,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":88396,"parent_record_offset":88364,"ancestor_record_offset":84716,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":89316,"parent_record_offset":89292,"ancestor_record_offset":84716,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":571,"record_offset":87312,"parent_record_offset":87204,"ancestor_record_offset":84716,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":86728,"parent_record_offset":86704,"ancestor_record_offset":84716,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":86704,"parent_record_offset":86664,"ancestor_record_offset":84716,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":88632,"parent_record_offset":88604,"ancestor_record_offset":84716,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":98048,"parent_record_offset":98004,"ancestor_record_offset":84716,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":89240,"parent_record_offset":89200,"ancestor_record_offset":84716,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":97532,"parent_record_offset":97508,"ancestor_record_offset":84716,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":96692,"parent_record_offset":96656,"ancestor_record_offset":84716,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":98264,"parent_record_offset":98004,"ancestor_record_offset":84716,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::try_remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":98004,"parent_record_offset":97968,"ancestor_record_offset":84716,"inlinee":6392,"source_line":2377,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":88300,"parent_record_offset":88252,"ancestor_record_offset":84716,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":88332,"parent_record_offset":88300,"ancestor_record_offset":84716,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":88180,"parent_record_offset":88144,"ancestor_record_offset":84716,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":88812,"parent_record_offset":88784,"ancestor_record_offset":84716,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":98700,"parent_record_offset":98672,"ancestor_record_offset":84716,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":571,"record_offset":98324,"parent_record_offset":98300,"ancestor_record_offset":84716,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":571,"record_offset":98372,"parent_record_offset":98348,"ancestor_record_offset":84716,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":571,"record_offset":88880,"parent_record_offset":88812,"ancestor_record_offset":84716,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":571,"record_offset":88604,"parent_record_offset":88252,"ancestor_record_offset":84716,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":571,"record_offset":88216,"parent_record_offset":88180,"ancestor_record_offset":84716,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":571,"record_offset":88252,"parent_record_offset":88216,"ancestor_record_offset":84716,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":571,"record_offset":85308,"parent_record_offset":85284,"ancestor_record_offset":84716,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":571,"record_offset":96628,"parent_record_offset":96600,"ancestor_record_offset":84716,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":571,"record_offset":96656,"parent_record_offset":96628,"ancestor_record_offset":84716,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::write_message
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":571,"record_offset":96600,"parent_record_offset":96572,"ancestor_record_offset":84716,"inlinee":35761,"source_line":1154,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":571,"record_offset":89132,"parent_record_offset":89108,"ancestor_record_offset":84716,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":571,"record_offset":89156,"parent_record_offset":89132,"ancestor_record_offset":84716,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::typed_swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":571,"record_offset":95936,"parent_record_offset":95912,"ancestor_record_offset":84716,"inlinee":5746,"source_line":2574,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":571,"record_offset":88756,"parent_record_offset":88716,"ancestor_record_offset":84716,"inlinee":10907,"source_line":36,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":571,"record_offset":88688,"parent_record_offset":88660,"ancestor_record_offset":84716,"inlinee":10905,"source_line":77,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":571,"record_offset":85888,"parent_record_offset":85864,"ancestor_record_offset":84716,"inlinee":6384,"source_line":106,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":571,"record_offset":86036,"parent_record_offset":85912,"ancestor_record_offset":84716,"inlinee":6386,"source_line":118,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":88144,"parent_record_offset":88004,"ancestor_record_offset":84716,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":85864,"parent_record_offset":85840,"ancestor_record_offset":84716,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":86064,"parent_record_offset":86036,"ancestor_record_offset":84716,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":88660,"parent_record_offset":88632,"ancestor_record_offset":84716,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":88784,"parent_record_offset":88756,"ancestor_record_offset":84716,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":85912,"parent_record_offset":85888,"ancestor_record_offset":84716,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":571,"record_offset":97788,"parent_record_offset":97748,"ancestor_record_offset":84716,"inlinee":5745,"source_line":756,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":571,"record_offset":90024,"parent_record_offset":88004,"ancestor_record_offset":84716,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":571,"record_offset":89400,"parent_record_offset":89368,"ancestor_record_offset":84716,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::impl$0::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":571,"record_offset":86152,"parent_record_offset":85912,"ancestor_record_offset":84716,"inlinee":5504,"source_line":115,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":571,"record_offset":86088,"parent_record_offset":86064,"ancestor_record_offset":84716,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":571,"record_offset":89108,"parent_record_offset":89084,"ancestor_record_offset":84716,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":571,"record_offset":89084,"parent_record_offset":89052,"ancestor_record_offset":84716,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=63
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":86844,"parent_record_offset":86664,"ancestor_record_offset":84716,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":571,"record_offset":85804,"parent_record_offset":85780,"ancestor_record_offset":84716,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":571,"record_offset":86004,"parent_record_offset":85944,"ancestor_record_offset":84716,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":571,"record_offset":97908,"parent_record_offset":97828,"ancestor_record_offset":84716,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":571,"record_offset":85976,"parent_record_offset":85944,"ancestor_record_offset":84716,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":97932,"parent_record_offset":97908,"ancestor_record_offset":84716,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_chunk
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":96064,"parent_record_offset":96040,"ancestor_record_offset":84716,"inlinee":5753,"source_line":1432,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":95960,"parent_record_offset":95936,"ancestor_record_offset":84716,"inlinee":5747,"source_line":1363,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":571,"record_offset":95988,"parent_record_offset":95960,"ancestor_record_offset":84716,"inlinee":5749,"source_line":2434,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":96016,"parent_record_offset":95988,"ancestor_record_offset":84716,"inlinee":5750,"source_line":1440,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_bytes::swap_nonoverlapping_chunks
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":96040,"parent_record_offset":96016,"ancestor_record_offset":84716,"inlinee":5752,"source_line":1442,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":88848,"parent_record_offset":88812,"ancestor_record_offset":84716,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":571,"record_offset":89012,"parent_record_offset":88984,"ancestor_record_offset":84716,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":86276,"parent_record_offset":86252,"ancestor_record_offset":84716,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":97748,"parent_record_offset":97716,"ancestor_record_offset":84716,"inlinee":5744,"source_line":1001,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":571,"record_offset":90052,"parent_record_offset":88004,"ancestor_record_offset":84716,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":571,"record_offset":90080,"parent_record_offset":90052,"ancestor_record_offset":84716,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":571,"record_offset":86300,"parent_record_offset":86276,"ancestor_record_offset":84716,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":571,"record_offset":85780,"parent_record_offset":85756,"ancestor_record_offset":84716,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":88716,"parent_record_offset":88688,"ancestor_record_offset":84716,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":86988,"parent_record_offset":86964,"ancestor_record_offset":84716,"inlinee":32137,"source_line":360,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":85944,"parent_record_offset":85912,"ancestor_record_offset":84716,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":571,"record_offset":87844,"parent_record_offset":87820,"ancestor_record_offset":84716,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":86112,"parent_record_offset":86088,"ancestor_record_offset":84716,"inlinee":36199,"source_line":200,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":87136,"parent_record_offset":87112,"ancestor_record_offset":84716,"inlinee":36200,"source_line":249,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":571,"record_offset":87788,"parent_record_offset":87764,"ancestor_record_offset":84716,"inlinee":33040,"source_line":189,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":571,"record_offset":88984,"parent_record_offset":88004,"ancestor_record_offset":84716,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::try_slice
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":571,"record_offset":89200,"parent_record_offset":88004,"ancestor_record_offset":84716,"inlinee":14742,"source_line":859,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":571,"record_offset":89052,"parent_record_offset":88004,"ancestor_record_offset":84716,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00571.jsonl.gz"}
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
// 首条位置: {"module":571,"record_offset":52168,"rva":11976432,"va":5380685552}
// 编译实例 RVAs: 0xb6bef0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::vec::Vec<_, _>>::remove::assert_failed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t35927703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e897362402","size":5,"soft_fail":false,"target_va":5416621000,"va":5380693292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5009108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e82c91b3ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380687791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=24
//     evidence={"assembly":"\tcallq\t-5009614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e8328fb3ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380688313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11965584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e8706b49ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380687771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081c20
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11453169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e80f3d51ff","size":5,"soft_fail":false,"target_va":5369240608,"va":5380693772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096e970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2094222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e8720be0ff","size":5,"soft_fail":false,"target_va":5378599280,"va":5380693497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a18d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1884803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e87d3de3ff","size":5,"soft_fail":false,"target_va":5378808016,"va":5380692814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t890360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e8f8950d00","size":5,"soft_fail":false,"target_va":5381577744,"va":5380687379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c87180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1159445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e815b11100","size":5,"soft_fail":false,"target_va":5381845376,"va":5380685926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c874c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1158759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e867ae1100","size":5,"soft_fail":false,"target_va":5381846208,"va":5380687444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c87a10
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1161282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e842b81100","size":5,"soft_fail":false,"target_va":5381847568,"va":5380686281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c88320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1158179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e823ac1100","size":5,"soft_fail":false,"target_va":5381849888,"va":5380691704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c886b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1158682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e81aae1100","size":5,"soft_fail":false,"target_va":5381850800,"va":5380692113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416296d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11260964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e824d4ab00","size":5,"soft_fail":false,"target_va":5391947472,"va":5380686503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196d050
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14681833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e8e906e000","size":5,"soft_fail":false,"target_va":5395370064,"va":5380688226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419709b0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14696667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e8db40e000","size":5,"soft_fail":false,"target_va":5395384752,"va":5380688080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a70fc0
//     owner=unresolved_namespace; count=3
//     target_aliases=9 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15748300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e8cc4cf000","size":5,"soft_fail":false,"target_va":5396434880,"va":5380686575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35660061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e81d212002","size":5,"soft_fail":false,"target_va":5416353040,"va":5380692974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36364454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e8a6e02a02","size":5,"soft_fail":false,"target_va":5417057072,"va":5380692613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36371189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e8f5fa2a02","size":5,"soft_fail":false,"target_va":5417057088,"va":5380685894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35673780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e8b4562002","size":5,"soft_fail":false,"target_va":5416365968,"va":5380692183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35927411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e873352402","size":5,"soft_fail":false,"target_va":5416620794,"va":5380693378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35928285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e8dd382402","size":5,"soft_fail":false,"target_va":5416621600,"va":5380693310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35929267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e8b33c2402","size":5,"soft_fail":false,"target_va":5416622208,"va":5380692936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35928428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e86c392402","size":5,"soft_fail":false,"target_va":5416621762,"va":5380693329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35929080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e8f83b2402","size":5,"soft_fail":false,"target_va":5416622240,"va":5380693155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e8fd870000","size":5,"soft_fail":false,"target_va":5380720928,"va":5380686110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35928586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e80a3a2402","size":5,"soft_fail":false,"target_va":5416621632,"va":5380693041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35929082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e8fa3b2402","size":5,"soft_fail":false,"target_va":5416622320,"va":5380693233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::cubic_in_out
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2609520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380685552,"raw_hex":"e870d12700","size":5,"soft_fail":false,"target_va":5383297104,"va":5380687579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*%r12","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5380685552,"raw_hex":"41ffd4","size":3,"soft_fail":false,"va":5380685830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":54116,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":52708,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":64960,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":65420,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":6391,"source_line":2341,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":571,"record_offset":65280,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":571,"record_offset":52736,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":571,"record_offset":55396,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":571,"record_offset":53292,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":571,"record_offset":63364,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":5745,"source_line":756,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":571,"record_offset":63964,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":571,"record_offset":55332,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":571,"record_offset":63996,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":64324,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":53648,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":12073,"source_line":136,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":63028,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":53208,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":63936,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":65168,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":5742,"source_line":978,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":53816,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":571,"record_offset":55272,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":571,"record_offset":55368,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":28513,"source_line":306,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::MultiEdit::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":571,"record_offset":53916,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":35902,"source_line":477,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::history_chain::HistoryEditChain::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":571,"record_offset":53704,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":34437,"source_line":414,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":571,"record_offset":52792,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":35487,"source_line":202,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::iter_points_xz
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":571,"record_offset":54656,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":36204,"source_line":59,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::radius
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":571,"record_offset":54796,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":36205,"source_line":30,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$3::extend_dirty
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":571,"record_offset":55216,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":36208,"source_line":610,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_path::replay_path::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_path.rs
//     evidence={"module":571,"record_offset":54992,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":36207,"source_line":35,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::get_stroke_from_0_to_t
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":55456,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":36210,"source_line":435,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":54628,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":36202,"source_line":39,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":571,"record_offset":53676,"parent_record_offset":52168,"ancestor_record_offset":52168,"inlinee":30219,"source_line":156,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":55948,"parent_record_offset":55924,"ancestor_record_offset":52168,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":55972,"parent_record_offset":55948,"ancestor_record_offset":52168,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":56908,"parent_record_offset":56884,"ancestor_record_offset":52168,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":56932,"parent_record_offset":56908,"ancestor_record_offset":52168,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":55996,"parent_record_offset":55972,"ancestor_record_offset":52168,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":56956,"parent_record_offset":56932,"ancestor_record_offset":52168,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":55924,"parent_record_offset":55884,"ancestor_record_offset":52168,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":56884,"parent_record_offset":56792,"ancestor_record_offset":52168,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":52944,"parent_record_offset":52916,"ancestor_record_offset":52168,"inlinee":4786,"source_line":991,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":52820,"parent_record_offset":52792,"ancestor_record_offset":52168,"inlinee":30608,"source_line":1998,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":52848,"parent_record_offset":52820,"ancestor_record_offset":52168,"inlinee":30609,"source_line":937,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":53044,"parent_record_offset":52848,"ancestor_record_offset":52168,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":52888,"parent_record_offset":52848,"ancestor_record_offset":52168,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":52916,"parent_record_offset":52888,"ancestor_record_offset":52168,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":52976,"parent_record_offset":52944,"ancestor_record_offset":52168,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":53080,"parent_record_offset":53044,"ancestor_record_offset":52168,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":64172,"parent_record_offset":64144,"ancestor_record_offset":52168,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":55816,"parent_record_offset":55784,"ancestor_record_offset":52168,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":53000,"parent_record_offset":52976,"ancestor_record_offset":52168,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":56820,"parent_record_offset":56792,"ancestor_record_offset":52168,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":56792,"parent_record_offset":56768,"ancestor_record_offset":52168,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":64232,"parent_record_offset":64200,"ancestor_record_offset":52168,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":53128,"parent_record_offset":53104,"ancestor_record_offset":52168,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":53104,"parent_record_offset":53080,"ancestor_record_offset":52168,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":64200,"parent_record_offset":64172,"ancestor_record_offset":52168,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":55884,"parent_record_offset":55848,"ancestor_record_offset":52168,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":55848,"parent_record_offset":55816,"ancestor_record_offset":52168,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":56768,"parent_record_offset":56744,"ancestor_record_offset":52168,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":571,"record_offset":54764,"parent_record_offset":54656,"ancestor_record_offset":52168,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":54180,"parent_record_offset":54156,"ancestor_record_offset":52168,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":54156,"parent_record_offset":54116,"ancestor_record_offset":52168,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":56084,"parent_record_offset":56056,"ancestor_record_offset":52168,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":65500,"parent_record_offset":65456,"ancestor_record_offset":52168,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":56692,"parent_record_offset":56652,"ancestor_record_offset":52168,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":64984,"parent_record_offset":64960,"ancestor_record_offset":52168,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":64144,"parent_record_offset":64108,"ancestor_record_offset":52168,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":65716,"parent_record_offset":65456,"ancestor_record_offset":52168,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::try_remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":65456,"parent_record_offset":65420,"ancestor_record_offset":52168,"inlinee":6392,"source_line":2377,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":55752,"parent_record_offset":55704,"ancestor_record_offset":52168,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":55784,"parent_record_offset":55752,"ancestor_record_offset":52168,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":55632,"parent_record_offset":55596,"ancestor_record_offset":52168,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":56264,"parent_record_offset":56236,"ancestor_record_offset":52168,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":66152,"parent_record_offset":66124,"ancestor_record_offset":52168,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":571,"record_offset":65776,"parent_record_offset":65752,"ancestor_record_offset":52168,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":571,"record_offset":65824,"parent_record_offset":65800,"ancestor_record_offset":52168,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":571,"record_offset":56332,"parent_record_offset":56264,"ancestor_record_offset":52168,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":571,"record_offset":56056,"parent_record_offset":55704,"ancestor_record_offset":52168,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":571,"record_offset":55668,"parent_record_offset":55632,"ancestor_record_offset":52168,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":571,"record_offset":55704,"parent_record_offset":55668,"ancestor_record_offset":52168,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":571,"record_offset":52760,"parent_record_offset":52736,"ancestor_record_offset":52168,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":571,"record_offset":64080,"parent_record_offset":64052,"ancestor_record_offset":52168,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":571,"record_offset":64108,"parent_record_offset":64080,"ancestor_record_offset":52168,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::write_message
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":571,"record_offset":64052,"parent_record_offset":64024,"ancestor_record_offset":52168,"inlinee":35761,"source_line":1154,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":571,"record_offset":56584,"parent_record_offset":56560,"ancestor_record_offset":52168,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":571,"record_offset":56608,"parent_record_offset":56584,"ancestor_record_offset":52168,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::typed_swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":571,"record_offset":63388,"parent_record_offset":63364,"ancestor_record_offset":52168,"inlinee":5746,"source_line":2574,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":571,"record_offset":56208,"parent_record_offset":56168,"ancestor_record_offset":52168,"inlinee":10907,"source_line":36,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":571,"record_offset":56140,"parent_record_offset":56112,"ancestor_record_offset":52168,"inlinee":10905,"source_line":77,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":571,"record_offset":53340,"parent_record_offset":53316,"ancestor_record_offset":52168,"inlinee":6384,"source_line":106,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":571,"record_offset":53488,"parent_record_offset":53364,"ancestor_record_offset":52168,"inlinee":6386,"source_line":118,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":55596,"parent_record_offset":55456,"ancestor_record_offset":52168,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":53316,"parent_record_offset":53292,"ancestor_record_offset":52168,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":53516,"parent_record_offset":53488,"ancestor_record_offset":52168,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":56112,"parent_record_offset":56084,"ancestor_record_offset":52168,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":56236,"parent_record_offset":56208,"ancestor_record_offset":52168,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":53364,"parent_record_offset":53340,"ancestor_record_offset":52168,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":571,"record_offset":65240,"parent_record_offset":65200,"ancestor_record_offset":52168,"inlinee":5745,"source_line":756,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":571,"record_offset":57476,"parent_record_offset":55456,"ancestor_record_offset":52168,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":571,"record_offset":56852,"parent_record_offset":56820,"ancestor_record_offset":52168,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::impl$0::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":571,"record_offset":53604,"parent_record_offset":53364,"ancestor_record_offset":52168,"inlinee":5504,"source_line":115,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":571,"record_offset":53540,"parent_record_offset":53516,"ancestor_record_offset":52168,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":571,"record_offset":56560,"parent_record_offset":56536,"ancestor_record_offset":52168,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":571,"record_offset":56536,"parent_record_offset":56504,"ancestor_record_offset":52168,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=63
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":54296,"parent_record_offset":54116,"ancestor_record_offset":52168,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":571,"record_offset":53256,"parent_record_offset":53232,"ancestor_record_offset":52168,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":571,"record_offset":53456,"parent_record_offset":53396,"ancestor_record_offset":52168,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":571,"record_offset":65360,"parent_record_offset":65280,"ancestor_record_offset":52168,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":571,"record_offset":53428,"parent_record_offset":53396,"ancestor_record_offset":52168,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":65384,"parent_record_offset":65360,"ancestor_record_offset":52168,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_chunk
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":63516,"parent_record_offset":63492,"ancestor_record_offset":52168,"inlinee":5753,"source_line":1432,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":63412,"parent_record_offset":63388,"ancestor_record_offset":52168,"inlinee":5747,"source_line":1363,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":571,"record_offset":63440,"parent_record_offset":63412,"ancestor_record_offset":52168,"inlinee":5749,"source_line":2434,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":63468,"parent_record_offset":63440,"ancestor_record_offset":52168,"inlinee":5750,"source_line":1440,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_bytes::swap_nonoverlapping_chunks
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":63492,"parent_record_offset":63468,"ancestor_record_offset":52168,"inlinee":5752,"source_line":1442,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":56300,"parent_record_offset":56264,"ancestor_record_offset":52168,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":571,"record_offset":56464,"parent_record_offset":56436,"ancestor_record_offset":52168,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":53728,"parent_record_offset":53704,"ancestor_record_offset":52168,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":65200,"parent_record_offset":65168,"ancestor_record_offset":52168,"inlinee":5744,"source_line":1001,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":571,"record_offset":57504,"parent_record_offset":55456,"ancestor_record_offset":52168,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":571,"record_offset":57532,"parent_record_offset":57504,"ancestor_record_offset":52168,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":571,"record_offset":53752,"parent_record_offset":53728,"ancestor_record_offset":52168,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":571,"record_offset":53232,"parent_record_offset":53208,"ancestor_record_offset":52168,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":56168,"parent_record_offset":56140,"ancestor_record_offset":52168,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":54440,"parent_record_offset":54416,"ancestor_record_offset":52168,"inlinee":32137,"source_line":360,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":53396,"parent_record_offset":53364,"ancestor_record_offset":52168,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":571,"record_offset":55296,"parent_record_offset":55272,"ancestor_record_offset":52168,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":53564,"parent_record_offset":53540,"ancestor_record_offset":52168,"inlinee":36199,"source_line":200,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":54588,"parent_record_offset":54564,"ancestor_record_offset":52168,"inlinee":36200,"source_line":249,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":571,"record_offset":55240,"parent_record_offset":55216,"ancestor_record_offset":52168,"inlinee":33040,"source_line":189,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":571,"record_offset":56436,"parent_record_offset":55456,"ancestor_record_offset":52168,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::try_slice
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":571,"record_offset":56652,"parent_record_offset":55456,"ancestor_record_offset":52168,"inlinee":14742,"source_line":859,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":571,"record_offset":56504,"parent_record_offset":55456,"ancestor_record_offset":52168,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00571.jsonl.gz"}
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
// 首条位置: {"module":571,"record_offset":3004,"rva":11945632,"va":5380654752}
// 编译实例 RVAs: 0xb646a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::vec::Vec<_, _>>::remove::assert_failed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t35958503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e8e7ae2402","size":5,"soft_fail":false,"target_va":5416621000,"va":5380662492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4978308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e87c09b4ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380656991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=24
//     evidence={"assembly":"\tcallq\t-4978814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e88207b4ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380657513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11934784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e8c0e349ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380656971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081c20
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11422369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e85fb551ff","size":5,"soft_fail":false,"target_va":5369240608,"va":5380662972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096e970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2063422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e8c283e0ff","size":5,"soft_fail":false,"target_va":5378599280,"va":5380662697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a18d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1854003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e8cdb5e3ff","size":5,"soft_fail":false,"target_va":5378808016,"va":5380662014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t921160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e8480e0e00","size":5,"soft_fail":false,"target_va":5381577744,"va":5380656579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c87180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1190245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e865291200","size":5,"soft_fail":false,"target_va":5381845376,"va":5380655126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c874c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1189559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e8b7261200","size":5,"soft_fail":false,"target_va":5381846208,"va":5380656644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c87a10
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1192082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e892301200","size":5,"soft_fail":false,"target_va":5381847568,"va":5380655481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c88320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1188979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e873241200","size":5,"soft_fail":false,"target_va":5381849888,"va":5380660904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c886b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1189482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e86a261200","size":5,"soft_fail":false,"target_va":5381850800,"va":5380661313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416296d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11291764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e8744cac00","size":5,"soft_fail":false,"target_va":5391947472,"va":5380655703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196d050
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14712633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e8397fe000","size":5,"soft_fail":false,"target_va":5395370064,"va":5380657426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419709b0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14727467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e82bb9e000","size":5,"soft_fail":false,"target_va":5395384752,"va":5380657280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a70fc0
//     owner=unresolved_namespace; count=3
//     target_aliases=9 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15779100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e81cc5f000","size":5,"soft_fail":false,"target_va":5396434880,"va":5380655775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35690861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e86d992002","size":5,"soft_fail":false,"target_va":5416353040,"va":5380662174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36395254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e8f6582b02","size":5,"soft_fail":false,"target_va":5417057072,"va":5380661813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36401989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e845732b02","size":5,"soft_fail":false,"target_va":5417057088,"va":5380655094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35704580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e804cf2002","size":5,"soft_fail":false,"target_va":5416365968,"va":5380661383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35958211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e8c3ad2402","size":5,"soft_fail":false,"target_va":5416620794,"va":5380662578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35959085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e82db12402","size":5,"soft_fail":false,"target_va":5416621600,"va":5380662510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35960067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e803b52402","size":5,"soft_fail":false,"target_va":5416622208,"va":5380662136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35959228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e8bcb12402","size":5,"soft_fail":false,"target_va":5416621762,"va":5380662529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35959880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e848b42402","size":5,"soft_fail":false,"target_va":5416622240,"va":5380662355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t65613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e84d000100","size":5,"soft_fail":false,"target_va":5380720928,"va":5380655310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35959386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e85ab22402","size":5,"soft_fail":false,"target_va":5416621632,"va":5380662241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35959882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e84ab42402","size":5,"soft_fail":false,"target_va":5416622320,"va":5380662433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::cubic_in_out
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2640320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380654752,"raw_hex":"e8c0492800","size":5,"soft_fail":false,"target_va":5383297104,"va":5380656779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*%r12","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5380654752,"raw_hex":"41ffd4","size":3,"soft_fail":false,"va":5380655030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":4952,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":3544,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":15796,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":16256,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":6391,"source_line":2341,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":571,"record_offset":16116,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":571,"record_offset":3572,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":571,"record_offset":6232,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":571,"record_offset":4128,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":571,"record_offset":14200,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":5745,"source_line":756,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":571,"record_offset":14800,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":571,"record_offset":6168,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":571,"record_offset":14832,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":15160,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":4484,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":12073,"source_line":136,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":13864,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":4044,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":14772,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":16004,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":5742,"source_line":978,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":4652,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":571,"record_offset":6108,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":571,"record_offset":6204,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":28513,"source_line":306,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::MultiEdit::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":571,"record_offset":4752,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":35902,"source_line":477,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::history_chain::HistoryEditChain::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":571,"record_offset":4540,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":34437,"source_line":414,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":571,"record_offset":3628,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":35487,"source_line":202,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::iter_points_xz
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":571,"record_offset":5492,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":36204,"source_line":59,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::radius
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":571,"record_offset":5632,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":36205,"source_line":30,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$3::extend_dirty
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":571,"record_offset":6052,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":36208,"source_line":610,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_path::replay_path::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_path.rs
//     evidence={"module":571,"record_offset":5828,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":36207,"source_line":35,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::get_stroke_from_0_to_t
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":6292,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":36210,"source_line":435,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":5464,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":36202,"source_line":39,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":571,"record_offset":4512,"parent_record_offset":3004,"ancestor_record_offset":3004,"inlinee":30219,"source_line":156,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":6784,"parent_record_offset":6760,"ancestor_record_offset":3004,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":6808,"parent_record_offset":6784,"ancestor_record_offset":3004,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":7744,"parent_record_offset":7720,"ancestor_record_offset":3004,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":7768,"parent_record_offset":7744,"ancestor_record_offset":3004,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":6832,"parent_record_offset":6808,"ancestor_record_offset":3004,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":7792,"parent_record_offset":7768,"ancestor_record_offset":3004,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":6760,"parent_record_offset":6720,"ancestor_record_offset":3004,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":7720,"parent_record_offset":7628,"ancestor_record_offset":3004,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":3780,"parent_record_offset":3752,"ancestor_record_offset":3004,"inlinee":4786,"source_line":991,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":3656,"parent_record_offset":3628,"ancestor_record_offset":3004,"inlinee":30608,"source_line":1998,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":3684,"parent_record_offset":3656,"ancestor_record_offset":3004,"inlinee":30609,"source_line":937,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":3880,"parent_record_offset":3684,"ancestor_record_offset":3004,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":3724,"parent_record_offset":3684,"ancestor_record_offset":3004,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":3752,"parent_record_offset":3724,"ancestor_record_offset":3004,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":3812,"parent_record_offset":3780,"ancestor_record_offset":3004,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":3916,"parent_record_offset":3880,"ancestor_record_offset":3004,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":15008,"parent_record_offset":14980,"ancestor_record_offset":3004,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":6652,"parent_record_offset":6620,"ancestor_record_offset":3004,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":3836,"parent_record_offset":3812,"ancestor_record_offset":3004,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":7656,"parent_record_offset":7628,"ancestor_record_offset":3004,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":7628,"parent_record_offset":7604,"ancestor_record_offset":3004,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":15068,"parent_record_offset":15036,"ancestor_record_offset":3004,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":3964,"parent_record_offset":3940,"ancestor_record_offset":3004,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":3940,"parent_record_offset":3916,"ancestor_record_offset":3004,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":15036,"parent_record_offset":15008,"ancestor_record_offset":3004,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":6720,"parent_record_offset":6684,"ancestor_record_offset":3004,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":6684,"parent_record_offset":6652,"ancestor_record_offset":3004,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":7604,"parent_record_offset":7580,"ancestor_record_offset":3004,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":571,"record_offset":5600,"parent_record_offset":5492,"ancestor_record_offset":3004,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":5016,"parent_record_offset":4992,"ancestor_record_offset":3004,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":4992,"parent_record_offset":4952,"ancestor_record_offset":3004,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":6920,"parent_record_offset":6892,"ancestor_record_offset":3004,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":16336,"parent_record_offset":16292,"ancestor_record_offset":3004,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":7528,"parent_record_offset":7488,"ancestor_record_offset":3004,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":15820,"parent_record_offset":15796,"ancestor_record_offset":3004,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":14980,"parent_record_offset":14944,"ancestor_record_offset":3004,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":16552,"parent_record_offset":16292,"ancestor_record_offset":3004,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::try_remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":16292,"parent_record_offset":16256,"ancestor_record_offset":3004,"inlinee":6392,"source_line":2377,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":6588,"parent_record_offset":6540,"ancestor_record_offset":3004,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":6620,"parent_record_offset":6588,"ancestor_record_offset":3004,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":6468,"parent_record_offset":6432,"ancestor_record_offset":3004,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":7100,"parent_record_offset":7072,"ancestor_record_offset":3004,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":16988,"parent_record_offset":16960,"ancestor_record_offset":3004,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":571,"record_offset":16612,"parent_record_offset":16588,"ancestor_record_offset":3004,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":571,"record_offset":16660,"parent_record_offset":16636,"ancestor_record_offset":3004,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":571,"record_offset":7168,"parent_record_offset":7100,"ancestor_record_offset":3004,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":571,"record_offset":6892,"parent_record_offset":6540,"ancestor_record_offset":3004,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":571,"record_offset":6504,"parent_record_offset":6468,"ancestor_record_offset":3004,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":571,"record_offset":6540,"parent_record_offset":6504,"ancestor_record_offset":3004,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":571,"record_offset":3596,"parent_record_offset":3572,"ancestor_record_offset":3004,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":571,"record_offset":14916,"parent_record_offset":14888,"ancestor_record_offset":3004,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":571,"record_offset":14944,"parent_record_offset":14916,"ancestor_record_offset":3004,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::write_message
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":571,"record_offset":14888,"parent_record_offset":14860,"ancestor_record_offset":3004,"inlinee":35761,"source_line":1154,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":571,"record_offset":7420,"parent_record_offset":7396,"ancestor_record_offset":3004,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":571,"record_offset":7444,"parent_record_offset":7420,"ancestor_record_offset":3004,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::typed_swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":571,"record_offset":14224,"parent_record_offset":14200,"ancestor_record_offset":3004,"inlinee":5746,"source_line":2574,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":571,"record_offset":7044,"parent_record_offset":7004,"ancestor_record_offset":3004,"inlinee":10907,"source_line":36,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":571,"record_offset":6976,"parent_record_offset":6948,"ancestor_record_offset":3004,"inlinee":10905,"source_line":77,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":571,"record_offset":4176,"parent_record_offset":4152,"ancestor_record_offset":3004,"inlinee":6384,"source_line":106,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":571,"record_offset":4324,"parent_record_offset":4200,"ancestor_record_offset":3004,"inlinee":6386,"source_line":118,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":6432,"parent_record_offset":6292,"ancestor_record_offset":3004,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":4152,"parent_record_offset":4128,"ancestor_record_offset":3004,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":4352,"parent_record_offset":4324,"ancestor_record_offset":3004,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":6948,"parent_record_offset":6920,"ancestor_record_offset":3004,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":7072,"parent_record_offset":7044,"ancestor_record_offset":3004,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":4200,"parent_record_offset":4176,"ancestor_record_offset":3004,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":571,"record_offset":16076,"parent_record_offset":16036,"ancestor_record_offset":3004,"inlinee":5745,"source_line":756,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":571,"record_offset":8312,"parent_record_offset":6292,"ancestor_record_offset":3004,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":571,"record_offset":7688,"parent_record_offset":7656,"ancestor_record_offset":3004,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::impl$0::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":571,"record_offset":4440,"parent_record_offset":4200,"ancestor_record_offset":3004,"inlinee":5504,"source_line":115,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":571,"record_offset":4376,"parent_record_offset":4352,"ancestor_record_offset":3004,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":571,"record_offset":7396,"parent_record_offset":7372,"ancestor_record_offset":3004,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":571,"record_offset":7372,"parent_record_offset":7340,"ancestor_record_offset":3004,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=63
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":5132,"parent_record_offset":4952,"ancestor_record_offset":3004,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":571,"record_offset":4092,"parent_record_offset":4068,"ancestor_record_offset":3004,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":571,"record_offset":4292,"parent_record_offset":4232,"ancestor_record_offset":3004,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":571,"record_offset":16196,"parent_record_offset":16116,"ancestor_record_offset":3004,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":571,"record_offset":4264,"parent_record_offset":4232,"ancestor_record_offset":3004,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":16220,"parent_record_offset":16196,"ancestor_record_offset":3004,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_chunk
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":14352,"parent_record_offset":14328,"ancestor_record_offset":3004,"inlinee":5753,"source_line":1432,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":14248,"parent_record_offset":14224,"ancestor_record_offset":3004,"inlinee":5747,"source_line":1363,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":571,"record_offset":14276,"parent_record_offset":14248,"ancestor_record_offset":3004,"inlinee":5749,"source_line":2434,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":14304,"parent_record_offset":14276,"ancestor_record_offset":3004,"inlinee":5750,"source_line":1440,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_bytes::swap_nonoverlapping_chunks
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":14328,"parent_record_offset":14304,"ancestor_record_offset":3004,"inlinee":5752,"source_line":1442,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":7136,"parent_record_offset":7100,"ancestor_record_offset":3004,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":571,"record_offset":7300,"parent_record_offset":7272,"ancestor_record_offset":3004,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":4564,"parent_record_offset":4540,"ancestor_record_offset":3004,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":16036,"parent_record_offset":16004,"ancestor_record_offset":3004,"inlinee":5744,"source_line":1001,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":571,"record_offset":8340,"parent_record_offset":6292,"ancestor_record_offset":3004,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":571,"record_offset":8368,"parent_record_offset":8340,"ancestor_record_offset":3004,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":571,"record_offset":4588,"parent_record_offset":4564,"ancestor_record_offset":3004,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":571,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":3004,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":7004,"parent_record_offset":6976,"ancestor_record_offset":3004,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":5276,"parent_record_offset":5252,"ancestor_record_offset":3004,"inlinee":32137,"source_line":360,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":4232,"parent_record_offset":4200,"ancestor_record_offset":3004,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":571,"record_offset":6132,"parent_record_offset":6108,"ancestor_record_offset":3004,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":4400,"parent_record_offset":4376,"ancestor_record_offset":3004,"inlinee":36199,"source_line":200,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":5424,"parent_record_offset":5400,"ancestor_record_offset":3004,"inlinee":36200,"source_line":249,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":571,"record_offset":6076,"parent_record_offset":6052,"ancestor_record_offset":3004,"inlinee":33040,"source_line":189,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":571,"record_offset":7272,"parent_record_offset":6292,"ancestor_record_offset":3004,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::try_slice
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":571,"record_offset":7488,"parent_record_offset":6292,"ancestor_record_offset":3004,"inlinee":14742,"source_line":859,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":571,"record_offset":7340,"parent_record_offset":6292,"ancestor_record_offset":3004,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00571.jsonl.gz"}
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
// 首条位置: {"module":571,"record_offset":68360,"rva":11986608,"va":5380695728}
// 编译实例 RVAs: 0xb6e6b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::vec::Vec<_, _>>::remove::assert_failed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t35917329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e8110e2402","size":5,"soft_fail":false,"target_va":5416621000,"va":5380703666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5019292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e86469b3ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380697975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=24
//     evidence={"assembly":"\tcallq\t-5019822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e85267b3ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380698521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11975768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e8a84349ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380697955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081c20
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11463553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e87f1451ff","size":5,"soft_fail":false,"target_va":5369240608,"va":5380704156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096e970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2104606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e8e2e2dfff","size":5,"soft_fail":false,"target_va":5378599280,"va":5380703881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a18d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1895203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e8dd14e3ff","size":5,"soft_fail":false,"target_va":5378808016,"va":5380703214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t880149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e8156e0d00","size":5,"soft_fail":false,"target_va":5381577744,"va":5380697590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c87180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1149269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e855891100","size":5,"soft_fail":false,"target_va":5381845376,"va":5380696102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c874c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1148548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e884861100","size":5,"soft_fail":false,"target_va":5381846208,"va":5380697655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c87a10
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1151106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e882901100","size":5,"soft_fail":false,"target_va":5381847568,"va":5380696457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c88320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1147771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e87b831100","size":5,"soft_fail":false,"target_va":5381849888,"va":5380702112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c886b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1148281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e879851100","size":5,"soft_fail":false,"target_va":5381850800,"va":5380702514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414bbb50
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9752801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e8e1d09400","size":5,"soft_fail":false,"target_va":5390449488,"va":5380696682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196d050
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14671629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e80ddfdf00","size":5,"soft_fail":false,"target_va":5395370064,"va":5380698430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419709b0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14686466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e80219e000","size":5,"soft_fail":false,"target_va":5395384752,"va":5380698281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a70fc0
//     owner=unresolved_namespace; count=3
//     target_aliases=9 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15738121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e80925f000","size":5,"soft_fail":false,"target_va":5396434880,"va":5380696754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35647983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e8eff11f02","size":5,"soft_fail":false,"target_va":5416350384,"va":5380702396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36354054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e806b82a02","size":5,"soft_fail":false,"target_va":5417057072,"va":5380703013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36361013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e835d32a02","size":5,"soft_fail":false,"target_va":5417057088,"va":5380696070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35663379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e8132e2002","size":5,"soft_fail":false,"target_va":5416365968,"va":5380702584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35917037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e8ed0c2402","size":5,"soft_fail":false,"target_va":5416620794,"va":5380703752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35917911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e857102402","size":5,"soft_fail":false,"target_va":5416621600,"va":5380703684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35918867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e813142402","size":5,"soft_fail":false,"target_va":5416622208,"va":5380703336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35918054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e8e6102402","size":5,"soft_fail":false,"target_va":5416621762,"va":5380703703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35918709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e875132402","size":5,"soft_fail":false,"target_va":5416622240,"va":5380703526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t24637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e83d600000","size":5,"soft_fail":false,"target_va":5380720928,"va":5380696286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35918221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e88d112402","size":5,"soft_fail":false,"target_va":5416621632,"va":5380703406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35918708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e874132402","size":5,"soft_fail":false,"target_va":5416622320,"va":5380703607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::cubic_in_out
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2599319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380695728,"raw_hex":"e897a92700","size":5,"soft_fail":false,"target_va":5383297104,"va":5380697780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*%r12","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5380695728,"raw_hex":"41ffd4","size":3,"soft_fail":false,"va":5380696006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":70236,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":68828,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":81316,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":81776,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":6391,"source_line":2341,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":571,"record_offset":81636,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":571,"record_offset":68856,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":571,"record_offset":71516,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":571,"record_offset":69412,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":571,"record_offset":79508,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":5745,"source_line":756,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":571,"record_offset":80224,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":571,"record_offset":71452,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":571,"record_offset":80256,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":80680,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":69768,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":12073,"source_line":136,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":79172,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":69328,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":80196,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":81524,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":5742,"source_line":978,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":69936,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":571,"record_offset":71392,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":571,"record_offset":71488,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":28513,"source_line":306,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::MultiEdit::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":571,"record_offset":70036,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":35902,"source_line":477,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::history_chain::HistoryEditChain::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":571,"record_offset":69824,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":34437,"source_line":414,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":571,"record_offset":68912,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":35487,"source_line":202,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::iter_points_xz
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":571,"record_offset":70776,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":36204,"source_line":59,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::radius
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":571,"record_offset":70916,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":36205,"source_line":30,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$3::extend_dirty
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":571,"record_offset":71336,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":36208,"source_line":610,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_garden::replay_garden::closure$2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_garden.rs
//     evidence={"module":571,"record_offset":71112,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":36213,"source_line":39,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::get_stroke_from_0_to_t
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":71576,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":36210,"source_line":435,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":70748,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":36202,"source_line":39,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":571,"record_offset":69796,"parent_record_offset":68360,"ancestor_record_offset":68360,"inlinee":30219,"source_line":156,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":72072,"parent_record_offset":72048,"ancestor_record_offset":68360,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":72096,"parent_record_offset":72072,"ancestor_record_offset":68360,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":73032,"parent_record_offset":73008,"ancestor_record_offset":68360,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":73056,"parent_record_offset":73032,"ancestor_record_offset":68360,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":72120,"parent_record_offset":72096,"ancestor_record_offset":68360,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":73080,"parent_record_offset":73056,"ancestor_record_offset":68360,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":72048,"parent_record_offset":72008,"ancestor_record_offset":68360,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":73008,"parent_record_offset":72916,"ancestor_record_offset":68360,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":69064,"parent_record_offset":69036,"ancestor_record_offset":68360,"inlinee":4786,"source_line":991,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":68940,"parent_record_offset":68912,"ancestor_record_offset":68360,"inlinee":30608,"source_line":1998,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":68968,"parent_record_offset":68940,"ancestor_record_offset":68360,"inlinee":30609,"source_line":937,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":69164,"parent_record_offset":68968,"ancestor_record_offset":68360,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":69008,"parent_record_offset":68968,"ancestor_record_offset":68360,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":69036,"parent_record_offset":69008,"ancestor_record_offset":68360,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":69096,"parent_record_offset":69064,"ancestor_record_offset":68360,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":69200,"parent_record_offset":69164,"ancestor_record_offset":68360,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":71940,"parent_record_offset":71908,"ancestor_record_offset":68360,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":69120,"parent_record_offset":69096,"ancestor_record_offset":68360,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":72944,"parent_record_offset":72916,"ancestor_record_offset":68360,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":72916,"parent_record_offset":72892,"ancestor_record_offset":68360,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":69248,"parent_record_offset":69224,"ancestor_record_offset":68360,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":69224,"parent_record_offset":69200,"ancestor_record_offset":68360,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":72008,"parent_record_offset":71972,"ancestor_record_offset":68360,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":71972,"parent_record_offset":71940,"ancestor_record_offset":68360,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":72892,"parent_record_offset":72868,"ancestor_record_offset":68360,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":571,"record_offset":70884,"parent_record_offset":70776,"ancestor_record_offset":68360,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":70300,"parent_record_offset":70276,"ancestor_record_offset":68360,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":70276,"parent_record_offset":70236,"ancestor_record_offset":68360,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":72208,"parent_record_offset":72180,"ancestor_record_offset":68360,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":81856,"parent_record_offset":81812,"ancestor_record_offset":68360,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":72816,"parent_record_offset":72776,"ancestor_record_offset":68360,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":80432,"parent_record_offset":80396,"ancestor_record_offset":68360,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":80464,"parent_record_offset":80432,"ancestor_record_offset":68360,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":82072,"parent_record_offset":81812,"ancestor_record_offset":68360,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::try_remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":81812,"parent_record_offset":81776,"ancestor_record_offset":68360,"inlinee":6392,"source_line":2377,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":71876,"parent_record_offset":71828,"ancestor_record_offset":68360,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":71908,"parent_record_offset":71876,"ancestor_record_offset":68360,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":71756,"parent_record_offset":71720,"ancestor_record_offset":68360,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":72388,"parent_record_offset":72360,"ancestor_record_offset":68360,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":82508,"parent_record_offset":82480,"ancestor_record_offset":68360,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":571,"record_offset":82132,"parent_record_offset":82108,"ancestor_record_offset":68360,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":571,"record_offset":82180,"parent_record_offset":82156,"ancestor_record_offset":68360,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":571,"record_offset":72456,"parent_record_offset":72388,"ancestor_record_offset":68360,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":571,"record_offset":72180,"parent_record_offset":71828,"ancestor_record_offset":68360,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":571,"record_offset":71792,"parent_record_offset":71756,"ancestor_record_offset":68360,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":571,"record_offset":71828,"parent_record_offset":71792,"ancestor_record_offset":68360,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":571,"record_offset":68880,"parent_record_offset":68856,"ancestor_record_offset":68360,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":571,"record_offset":80332,"parent_record_offset":80308,"ancestor_record_offset":68360,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":571,"record_offset":80308,"parent_record_offset":80284,"ancestor_record_offset":68360,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":571,"record_offset":80396,"parent_record_offset":80332,"ancestor_record_offset":68360,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":571,"record_offset":72708,"parent_record_offset":72684,"ancestor_record_offset":68360,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":571,"record_offset":72732,"parent_record_offset":72708,"ancestor_record_offset":68360,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::typed_swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":571,"record_offset":79532,"parent_record_offset":79508,"ancestor_record_offset":68360,"inlinee":5746,"source_line":2574,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":571,"record_offset":72332,"parent_record_offset":72292,"ancestor_record_offset":68360,"inlinee":10907,"source_line":36,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":571,"record_offset":72264,"parent_record_offset":72236,"ancestor_record_offset":68360,"inlinee":10905,"source_line":77,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":571,"record_offset":69460,"parent_record_offset":69436,"ancestor_record_offset":68360,"inlinee":6384,"source_line":106,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":571,"record_offset":69608,"parent_record_offset":69484,"ancestor_record_offset":68360,"inlinee":6386,"source_line":118,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":71720,"parent_record_offset":71576,"ancestor_record_offset":68360,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":69436,"parent_record_offset":69412,"ancestor_record_offset":68360,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":69636,"parent_record_offset":69608,"ancestor_record_offset":68360,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":72236,"parent_record_offset":72208,"ancestor_record_offset":68360,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":72360,"parent_record_offset":72332,"ancestor_record_offset":68360,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":69484,"parent_record_offset":69460,"ancestor_record_offset":68360,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":571,"record_offset":81596,"parent_record_offset":81556,"ancestor_record_offset":68360,"inlinee":5745,"source_line":756,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":571,"record_offset":73912,"parent_record_offset":71576,"ancestor_record_offset":68360,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":571,"record_offset":72976,"parent_record_offset":72944,"ancestor_record_offset":68360,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::impl$0::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":571,"record_offset":69724,"parent_record_offset":69484,"ancestor_record_offset":68360,"inlinee":5504,"source_line":115,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":571,"record_offset":69660,"parent_record_offset":69636,"ancestor_record_offset":68360,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":571,"record_offset":72684,"parent_record_offset":72660,"ancestor_record_offset":68360,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":571,"record_offset":72660,"parent_record_offset":72628,"ancestor_record_offset":68360,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=63
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":70416,"parent_record_offset":70236,"ancestor_record_offset":68360,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":571,"record_offset":69376,"parent_record_offset":69352,"ancestor_record_offset":68360,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":571,"record_offset":69576,"parent_record_offset":69516,"ancestor_record_offset":68360,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":571,"record_offset":81716,"parent_record_offset":81636,"ancestor_record_offset":68360,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":571,"record_offset":69548,"parent_record_offset":69516,"ancestor_record_offset":68360,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":81740,"parent_record_offset":81716,"ancestor_record_offset":68360,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_chunk
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":79660,"parent_record_offset":79636,"ancestor_record_offset":68360,"inlinee":5753,"source_line":1432,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":79556,"parent_record_offset":79532,"ancestor_record_offset":68360,"inlinee":5747,"source_line":1363,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":571,"record_offset":79584,"parent_record_offset":79556,"ancestor_record_offset":68360,"inlinee":5749,"source_line":2434,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":79612,"parent_record_offset":79584,"ancestor_record_offset":68360,"inlinee":5750,"source_line":1440,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_bytes::swap_nonoverlapping_chunks
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":79636,"parent_record_offset":79612,"ancestor_record_offset":68360,"inlinee":5752,"source_line":1442,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":72424,"parent_record_offset":72388,"ancestor_record_offset":68360,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":571,"record_offset":72588,"parent_record_offset":72560,"ancestor_record_offset":68360,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":69848,"parent_record_offset":69824,"ancestor_record_offset":68360,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":81556,"parent_record_offset":81524,"ancestor_record_offset":68360,"inlinee":5744,"source_line":1001,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":571,"record_offset":73940,"parent_record_offset":71576,"ancestor_record_offset":68360,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":571,"record_offset":73968,"parent_record_offset":73940,"ancestor_record_offset":68360,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":571,"record_offset":69872,"parent_record_offset":69848,"ancestor_record_offset":68360,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":571,"record_offset":69352,"parent_record_offset":69328,"ancestor_record_offset":68360,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":72292,"parent_record_offset":72264,"ancestor_record_offset":68360,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":70560,"parent_record_offset":70536,"ancestor_record_offset":68360,"inlinee":32137,"source_line":360,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":69516,"parent_record_offset":69484,"ancestor_record_offset":68360,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":571,"record_offset":71416,"parent_record_offset":71392,"ancestor_record_offset":68360,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":69684,"parent_record_offset":69660,"ancestor_record_offset":68360,"inlinee":36199,"source_line":200,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":70708,"parent_record_offset":70684,"ancestor_record_offset":68360,"inlinee":36200,"source_line":249,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":571,"record_offset":71360,"parent_record_offset":71336,"ancestor_record_offset":68360,"inlinee":33040,"source_line":189,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":571,"record_offset":72560,"parent_record_offset":71576,"ancestor_record_offset":68360,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::try_slice
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":571,"record_offset":72776,"parent_record_offset":71576,"ancestor_record_offset":68360,"inlinee":14742,"source_line":859,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":571,"record_offset":72628,"parent_record_offset":71576,"ancestor_record_offset":68360,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00571.jsonl.gz"}
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
// 首条位置: {"module":571,"record_offset":19196,"rva":11955808,"va":5380664928}
// 编译实例 RVAs: 0xb66e60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::vec::Vec<_, _>>::remove::assert_failed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t35948347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e83b872402","size":5,"soft_fail":false,"target_va":5416621000,"va":5380672648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4988472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e8c8e1b3ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380667155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=24
//     evidence={"assembly":"\tcallq\t-4988992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e8c0dfb3ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380667691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11944948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e80cbc49ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380667135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081c20
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11432529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e8af8d51ff","size":5,"soft_fail":false,"target_va":5369240608,"va":5380673132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096e970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2073582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e8125ce0ff","size":5,"soft_fail":false,"target_va":5378599280,"va":5380672857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a18d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1864147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e82d8ee3ff","size":5,"soft_fail":false,"target_va":5378808016,"va":5380672158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t910972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e87ce60d00","size":5,"soft_fail":false,"target_va":5381577744,"va":5380666767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c87180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1180068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e8a4011200","size":5,"soft_fail":false,"target_va":5381845376,"va":5380665303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c874c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1179371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e8ebfe1100","size":5,"soft_fail":false,"target_va":5381846208,"va":5380666832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c87a10
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1181906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e8d2081200","size":5,"soft_fail":false,"target_va":5381847568,"va":5380665657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c88320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1178855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e8e7fc1100","size":5,"soft_fail":false,"target_va":5381849888,"va":5380671028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c886b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1179358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e8defe1100","size":5,"soft_fail":false,"target_va":5381850800,"va":5380671437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416296d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11281576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e8a824ac00","size":5,"soft_fail":false,"target_va":5391947472,"va":5380665891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196d050
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14702459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e87b57e000","size":5,"soft_fail":false,"target_va":5395370064,"va":5380667600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419709b0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14717296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e87091e000","size":5,"soft_fail":false,"target_va":5395384752,"va":5380667451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a70fc0
//     owner=unresolved_namespace; count=3
//     target_aliases=9 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15768912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e8509df000","size":5,"soft_fail":false,"target_va":5396434880,"va":5380665963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35680717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e8cd712002","size":5,"soft_fail":false,"target_va":5416353040,"va":5380672318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36385110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e856312b02","size":5,"soft_fail":false,"target_va":5417057072,"va":5380671957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36391814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e8864b2b02","size":5,"soft_fail":false,"target_va":5417057088,"va":5380665269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35694456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e878a72002","size":5,"soft_fail":false,"target_va":5416365968,"va":5380671507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35948055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e817862402","size":5,"soft_fail":false,"target_va":5416620794,"va":5380672734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35948929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e881892402","size":5,"soft_fail":false,"target_va":5416621600,"va":5380672666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35949923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e8638d2402","size":5,"soft_fail":false,"target_va":5416622208,"va":5380672280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35949072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e8108a2402","size":5,"soft_fail":false,"target_va":5416621762,"va":5380672685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35949727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e89f8c2402","size":5,"soft_fail":false,"target_va":5416622240,"va":5380672508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t55437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e88dd80000","size":5,"soft_fail":false,"target_va":5380720928,"va":5380665486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35949239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e8b78a2402","size":5,"soft_fail":false,"target_va":5416621632,"va":5380672388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35949726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e89e8c2402","size":5,"soft_fail":false,"target_va":5416622320,"va":5380672589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::cubic_in_out
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2630138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380664928,"raw_hex":"e8fa212800","size":5,"soft_fail":false,"target_va":5383297104,"va":5380666961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*%r13","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5380664928,"raw_hex":"41ffd5","size":3,"soft_fail":false,"va":5380665206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":21128,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":19720,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":31996,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":32456,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":6391,"source_line":2341,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":571,"record_offset":32316,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":571,"record_offset":19748,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":571,"record_offset":22408,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":571,"record_offset":20304,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":571,"record_offset":30400,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":5745,"source_line":756,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":571,"record_offset":31000,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":571,"record_offset":22344,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":571,"record_offset":31032,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":31360,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":20660,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":12073,"source_line":136,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":30064,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":20220,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":30972,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":32204,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":5742,"source_line":978,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":20828,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":571,"record_offset":22284,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":571,"record_offset":22380,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":28513,"source_line":306,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::MultiEdit::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":571,"record_offset":20928,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":35902,"source_line":477,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::history_chain::HistoryEditChain::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":571,"record_offset":20716,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":34437,"source_line":414,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":571,"record_offset":19804,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":35487,"source_line":202,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::iter_points_xz
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":571,"record_offset":21668,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":36204,"source_line":59,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::radius
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":571,"record_offset":21808,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":36205,"source_line":30,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$3::extend_dirty
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":571,"record_offset":22228,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":36208,"source_line":610,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_path::replay_path::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_path.rs
//     evidence={"module":571,"record_offset":22004,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":36207,"source_line":35,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::get_stroke_from_0_to_t
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":22468,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":36210,"source_line":435,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":21640,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":36202,"source_line":39,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":571,"record_offset":20688,"parent_record_offset":19196,"ancestor_record_offset":19196,"inlinee":30219,"source_line":156,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":22964,"parent_record_offset":22940,"ancestor_record_offset":19196,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":22988,"parent_record_offset":22964,"ancestor_record_offset":19196,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":23924,"parent_record_offset":23900,"ancestor_record_offset":19196,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":23948,"parent_record_offset":23924,"ancestor_record_offset":19196,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":23012,"parent_record_offset":22988,"ancestor_record_offset":19196,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":23972,"parent_record_offset":23948,"ancestor_record_offset":19196,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":22940,"parent_record_offset":22900,"ancestor_record_offset":19196,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":23900,"parent_record_offset":23808,"ancestor_record_offset":19196,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":19956,"parent_record_offset":19928,"ancestor_record_offset":19196,"inlinee":4786,"source_line":991,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":19832,"parent_record_offset":19804,"ancestor_record_offset":19196,"inlinee":30608,"source_line":1998,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":19860,"parent_record_offset":19832,"ancestor_record_offset":19196,"inlinee":30609,"source_line":937,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":20056,"parent_record_offset":19860,"ancestor_record_offset":19196,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":19900,"parent_record_offset":19860,"ancestor_record_offset":19196,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":19928,"parent_record_offset":19900,"ancestor_record_offset":19196,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":19988,"parent_record_offset":19956,"ancestor_record_offset":19196,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":20092,"parent_record_offset":20056,"ancestor_record_offset":19196,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":31208,"parent_record_offset":31180,"ancestor_record_offset":19196,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":22832,"parent_record_offset":22800,"ancestor_record_offset":19196,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":20012,"parent_record_offset":19988,"ancestor_record_offset":19196,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":23836,"parent_record_offset":23808,"ancestor_record_offset":19196,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":23808,"parent_record_offset":23784,"ancestor_record_offset":19196,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":31268,"parent_record_offset":31236,"ancestor_record_offset":19196,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":20140,"parent_record_offset":20116,"ancestor_record_offset":19196,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":20116,"parent_record_offset":20092,"ancestor_record_offset":19196,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":31236,"parent_record_offset":31208,"ancestor_record_offset":19196,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":22900,"parent_record_offset":22864,"ancestor_record_offset":19196,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":22864,"parent_record_offset":22832,"ancestor_record_offset":19196,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":23784,"parent_record_offset":23760,"ancestor_record_offset":19196,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":571,"record_offset":21776,"parent_record_offset":21668,"ancestor_record_offset":19196,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":21192,"parent_record_offset":21168,"ancestor_record_offset":19196,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":21168,"parent_record_offset":21128,"ancestor_record_offset":19196,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":23100,"parent_record_offset":23072,"ancestor_record_offset":19196,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":32536,"parent_record_offset":32492,"ancestor_record_offset":19196,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":23708,"parent_record_offset":23668,"ancestor_record_offset":19196,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":32020,"parent_record_offset":31996,"ancestor_record_offset":19196,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":31180,"parent_record_offset":31144,"ancestor_record_offset":19196,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":32752,"parent_record_offset":32492,"ancestor_record_offset":19196,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::try_remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":32492,"parent_record_offset":32456,"ancestor_record_offset":19196,"inlinee":6392,"source_line":2377,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":22768,"parent_record_offset":22720,"ancestor_record_offset":19196,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":22800,"parent_record_offset":22768,"ancestor_record_offset":19196,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":22648,"parent_record_offset":22612,"ancestor_record_offset":19196,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":23280,"parent_record_offset":23252,"ancestor_record_offset":19196,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":33188,"parent_record_offset":33160,"ancestor_record_offset":19196,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":571,"record_offset":32812,"parent_record_offset":32788,"ancestor_record_offset":19196,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":571,"record_offset":32860,"parent_record_offset":32836,"ancestor_record_offset":19196,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":571,"record_offset":23348,"parent_record_offset":23280,"ancestor_record_offset":19196,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":571,"record_offset":23072,"parent_record_offset":22720,"ancestor_record_offset":19196,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":571,"record_offset":22684,"parent_record_offset":22648,"ancestor_record_offset":19196,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":571,"record_offset":22720,"parent_record_offset":22684,"ancestor_record_offset":19196,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":571,"record_offset":19772,"parent_record_offset":19748,"ancestor_record_offset":19196,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":571,"record_offset":31116,"parent_record_offset":31088,"ancestor_record_offset":19196,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":571,"record_offset":31144,"parent_record_offset":31116,"ancestor_record_offset":19196,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::write_message
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":571,"record_offset":31088,"parent_record_offset":31060,"ancestor_record_offset":19196,"inlinee":35761,"source_line":1154,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":571,"record_offset":23600,"parent_record_offset":23576,"ancestor_record_offset":19196,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":571,"record_offset":23624,"parent_record_offset":23600,"ancestor_record_offset":19196,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::typed_swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":571,"record_offset":30424,"parent_record_offset":30400,"ancestor_record_offset":19196,"inlinee":5746,"source_line":2574,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":571,"record_offset":23224,"parent_record_offset":23184,"ancestor_record_offset":19196,"inlinee":10907,"source_line":36,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":571,"record_offset":23156,"parent_record_offset":23128,"ancestor_record_offset":19196,"inlinee":10905,"source_line":77,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":571,"record_offset":20352,"parent_record_offset":20328,"ancestor_record_offset":19196,"inlinee":6384,"source_line":106,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":571,"record_offset":20500,"parent_record_offset":20376,"ancestor_record_offset":19196,"inlinee":6386,"source_line":118,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":22612,"parent_record_offset":22468,"ancestor_record_offset":19196,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":20328,"parent_record_offset":20304,"ancestor_record_offset":19196,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":20528,"parent_record_offset":20500,"ancestor_record_offset":19196,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":23128,"parent_record_offset":23100,"ancestor_record_offset":19196,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":23252,"parent_record_offset":23224,"ancestor_record_offset":19196,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":20376,"parent_record_offset":20352,"ancestor_record_offset":19196,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":571,"record_offset":32276,"parent_record_offset":32236,"ancestor_record_offset":19196,"inlinee":5745,"source_line":756,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":571,"record_offset":24804,"parent_record_offset":22468,"ancestor_record_offset":19196,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":571,"record_offset":23868,"parent_record_offset":23836,"ancestor_record_offset":19196,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::impl$0::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":571,"record_offset":20616,"parent_record_offset":20376,"ancestor_record_offset":19196,"inlinee":5504,"source_line":115,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":571,"record_offset":20552,"parent_record_offset":20528,"ancestor_record_offset":19196,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":571,"record_offset":23576,"parent_record_offset":23552,"ancestor_record_offset":19196,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":571,"record_offset":23552,"parent_record_offset":23520,"ancestor_record_offset":19196,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=63
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":21308,"parent_record_offset":21128,"ancestor_record_offset":19196,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":571,"record_offset":20268,"parent_record_offset":20244,"ancestor_record_offset":19196,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":571,"record_offset":20468,"parent_record_offset":20408,"ancestor_record_offset":19196,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":571,"record_offset":32396,"parent_record_offset":32316,"ancestor_record_offset":19196,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":571,"record_offset":20440,"parent_record_offset":20408,"ancestor_record_offset":19196,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":32420,"parent_record_offset":32396,"ancestor_record_offset":19196,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_chunk
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":30552,"parent_record_offset":30528,"ancestor_record_offset":19196,"inlinee":5753,"source_line":1432,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":30448,"parent_record_offset":30424,"ancestor_record_offset":19196,"inlinee":5747,"source_line":1363,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":571,"record_offset":30476,"parent_record_offset":30448,"ancestor_record_offset":19196,"inlinee":5749,"source_line":2434,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":30504,"parent_record_offset":30476,"ancestor_record_offset":19196,"inlinee":5750,"source_line":1440,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_bytes::swap_nonoverlapping_chunks
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":30528,"parent_record_offset":30504,"ancestor_record_offset":19196,"inlinee":5752,"source_line":1442,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":23316,"parent_record_offset":23280,"ancestor_record_offset":19196,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":571,"record_offset":23480,"parent_record_offset":23452,"ancestor_record_offset":19196,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":20740,"parent_record_offset":20716,"ancestor_record_offset":19196,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":32236,"parent_record_offset":32204,"ancestor_record_offset":19196,"inlinee":5744,"source_line":1001,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":571,"record_offset":24832,"parent_record_offset":22468,"ancestor_record_offset":19196,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":571,"record_offset":24860,"parent_record_offset":24832,"ancestor_record_offset":19196,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":571,"record_offset":20764,"parent_record_offset":20740,"ancestor_record_offset":19196,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":571,"record_offset":20244,"parent_record_offset":20220,"ancestor_record_offset":19196,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":23184,"parent_record_offset":23156,"ancestor_record_offset":19196,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":21452,"parent_record_offset":21428,"ancestor_record_offset":19196,"inlinee":32137,"source_line":360,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":20408,"parent_record_offset":20376,"ancestor_record_offset":19196,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":571,"record_offset":22308,"parent_record_offset":22284,"ancestor_record_offset":19196,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":20576,"parent_record_offset":20552,"ancestor_record_offset":19196,"inlinee":36199,"source_line":200,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":21600,"parent_record_offset":21576,"ancestor_record_offset":19196,"inlinee":36200,"source_line":249,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":571,"record_offset":22252,"parent_record_offset":22228,"ancestor_record_offset":19196,"inlinee":33040,"source_line":189,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":571,"record_offset":23452,"parent_record_offset":22468,"ancestor_record_offset":19196,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::try_slice
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":571,"record_offset":23668,"parent_record_offset":22468,"ancestor_record_offset":19196,"inlinee":14742,"source_line":859,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":571,"record_offset":23520,"parent_record_offset":22468,"ancestor_record_offset":19196,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00571.jsonl.gz"}
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
// 首条位置: {"module":571,"record_offset":35396,"rva":11965968,"va":5380675088}
// 编译实例 RVAs: 0xb69610
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::vec::Vec<_, _>>::remove::assert_failed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t35937889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e8615e2402","size":5,"soft_fail":false,"target_va":5416621000,"va":5380683106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4998649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e807bab3ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380677332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=24
//     evidence={"assembly":"\tcallq\t-4999166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e802b8b3ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380677865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11955125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e84b9449ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380677312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081c20
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11442993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e8cf6451ff","size":5,"soft_fail":false,"target_va":5369240608,"va":5380683596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096e970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2084046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e83233e0ff","size":5,"soft_fail":false,"target_va":5378599280,"va":5380683321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a18d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1874643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e82d65e3ff","size":5,"soft_fail":false,"target_va":5378808016,"va":5380682654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t900794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e8babe0d00","size":5,"soft_fail":false,"target_va":5381577744,"va":5380676945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c87180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1169905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e8f1d91100","size":5,"soft_fail":false,"target_va":5381845376,"va":5380675466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c874c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1169193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e829d71100","size":5,"soft_fail":false,"target_va":5381846208,"va":5380677010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c87a10
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1171746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e822e11100","size":5,"soft_fail":false,"target_va":5381847568,"va":5380675817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c88320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1168429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e82dd41100","size":5,"soft_fail":false,"target_va":5381849888,"va":5380681454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c886b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1168846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e8ced51100","size":5,"soft_fail":false,"target_va":5381850800,"va":5380681949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196d050
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14692285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e8bd2fe000","size":5,"soft_fail":false,"target_va":5395370064,"va":5380677774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419709b0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14707122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e8b269e000","size":5,"soft_fail":false,"target_va":5395384752,"va":5380677625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a70fc0
//     owner=unresolved_namespace; count=3
//     target_aliases=9 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15758766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e8ae75f000","size":5,"soft_fail":false,"target_va":5396434880,"va":5380676109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35668633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e899422002","size":5,"soft_fail":false,"target_va":5416350384,"va":5380681746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35668868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e884432002","size":5,"soft_fail":false,"target_va":5416350720,"va":5380681847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36374614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e856082b02","size":5,"soft_fail":false,"target_va":5417057072,"va":5380682453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: replay_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36381651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e8d3232b02","size":5,"soft_fail":false,"target_va":5417057088,"va":5380675432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35683944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e8687e2002","size":5,"soft_fail":false,"target_va":5416365968,"va":5380682019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35937597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e83d5d2402","size":5,"soft_fail":false,"target_va":5416620794,"va":5380683192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35938471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e8a7602402","size":5,"soft_fail":false,"target_va":5416621600,"va":5380683124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35939427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e863642402","size":5,"soft_fail":false,"target_va":5416622208,"va":5380682776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35938614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e836612402","size":5,"soft_fail":false,"target_va":5416621762,"va":5380683143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35939269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e8c5632402","size":5,"soft_fail":false,"target_va":5416622240,"va":5380682966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t45277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e8ddb00000","size":5,"soft_fail":false,"target_va":5380720928,"va":5380675646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35938781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e8dd612402","size":5,"soft_fail":false,"target_va":5416621632,"va":5380682846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35939268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e8c4632402","size":5,"soft_fail":false,"target_va":5416622320,"va":5380683047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::history::replay_water::replay_water::closure$2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e816acffff","size":5,"soft_fail":false,"target_va":5380654560,"va":5380676037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::cubic_in_out
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2619960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380675088,"raw_hex":"e838fa2700","size":5,"soft_fail":false,"target_va":5383297104,"va":5380677139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*%r12","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5380675088,"raw_hex":"41ffd4","size":3,"soft_fail":false,"va":5380675366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":37268,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":35860,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":48768,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":49228,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":6391,"source_line":2341,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":571,"record_offset":49088,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":571,"record_offset":35888,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":571,"record_offset":38520,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":571,"record_offset":36444,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":571,"record_offset":46512,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":5745,"source_line":756,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":571,"record_offset":47228,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":571,"record_offset":38456,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":571,"record_offset":47260,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":48132,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":36800,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":12073,"source_line":136,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":46176,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":36360,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":47200,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":48976,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":5742,"source_line":978,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":36968,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":571,"record_offset":38396,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":571,"record_offset":38492,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":28513,"source_line":306,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::MultiEdit::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":571,"record_offset":37068,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":35902,"source_line":477,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::history_chain::HistoryEditChain::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":571,"record_offset":36856,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":34437,"source_line":414,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":571,"record_offset":35944,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":35487,"source_line":202,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::iter_points_xz
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":571,"record_offset":37808,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":36204,"source_line":59,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::raster_stroke::RasterStroke::radius
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":571,"record_offset":37948,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":36205,"source_line":30,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$3::extend_dirty
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":571,"record_offset":38340,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":36208,"source_line":610,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::get_stroke_from_0_to_t
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":38580,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":36210,"source_line":435,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_raster::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":37780,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":36202,"source_line":39,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_water::replay_water::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_water.rs
//     evidence={"module":571,"record_offset":47288,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":36211,"source_line":43,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":571,"record_offset":36828,"parent_record_offset":35396,"ancestor_record_offset":35396,"inlinee":30219,"source_line":156,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":39076,"parent_record_offset":39052,"ancestor_record_offset":35396,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":39100,"parent_record_offset":39076,"ancestor_record_offset":35396,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":40036,"parent_record_offset":40012,"ancestor_record_offset":35396,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":40060,"parent_record_offset":40036,"ancestor_record_offset":35396,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":39124,"parent_record_offset":39100,"ancestor_record_offset":35396,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":40084,"parent_record_offset":40060,"ancestor_record_offset":35396,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":39052,"parent_record_offset":39012,"ancestor_record_offset":35396,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":40012,"parent_record_offset":39920,"ancestor_record_offset":35396,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":36096,"parent_record_offset":36068,"ancestor_record_offset":35396,"inlinee":4786,"source_line":991,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":35972,"parent_record_offset":35944,"ancestor_record_offset":35396,"inlinee":30608,"source_line":1998,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":36000,"parent_record_offset":35972,"ancestor_record_offset":35396,"inlinee":30609,"source_line":937,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":36196,"parent_record_offset":36000,"ancestor_record_offset":35396,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":36040,"parent_record_offset":36000,"ancestor_record_offset":35396,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":571,"record_offset":36068,"parent_record_offset":36040,"ancestor_record_offset":35396,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":36128,"parent_record_offset":36096,"ancestor_record_offset":35396,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":36232,"parent_record_offset":36196,"ancestor_record_offset":35396,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":38944,"parent_record_offset":38912,"ancestor_record_offset":35396,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":36152,"parent_record_offset":36128,"ancestor_record_offset":35396,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":39948,"parent_record_offset":39920,"ancestor_record_offset":35396,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":39920,"parent_record_offset":39896,"ancestor_record_offset":35396,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":36280,"parent_record_offset":36256,"ancestor_record_offset":35396,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":36256,"parent_record_offset":36232,"ancestor_record_offset":35396,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":39012,"parent_record_offset":38976,"ancestor_record_offset":35396,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":38976,"parent_record_offset":38944,"ancestor_record_offset":35396,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":39896,"parent_record_offset":39872,"ancestor_record_offset":35396,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":571,"record_offset":37916,"parent_record_offset":37808,"ancestor_record_offset":35396,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":37332,"parent_record_offset":37308,"ancestor_record_offset":35396,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":37308,"parent_record_offset":37268,"ancestor_record_offset":35396,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":39212,"parent_record_offset":39184,"ancestor_record_offset":35396,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":49308,"parent_record_offset":49264,"ancestor_record_offset":35396,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":39820,"parent_record_offset":39780,"ancestor_record_offset":35396,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":47444,"parent_record_offset":47408,"ancestor_record_offset":35396,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":47476,"parent_record_offset":47444,"ancestor_record_offset":35396,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":49524,"parent_record_offset":49264,"ancestor_record_offset":35396,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::try_remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":49264,"parent_record_offset":49228,"ancestor_record_offset":35396,"inlinee":6392,"source_line":2377,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":38880,"parent_record_offset":38832,"ancestor_record_offset":35396,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":38912,"parent_record_offset":38880,"ancestor_record_offset":35396,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":38760,"parent_record_offset":38724,"ancestor_record_offset":35396,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":39392,"parent_record_offset":39364,"ancestor_record_offset":35396,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":49960,"parent_record_offset":49932,"ancestor_record_offset":35396,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":571,"record_offset":49584,"parent_record_offset":49560,"ancestor_record_offset":35396,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":571,"record_offset":49632,"parent_record_offset":49608,"ancestor_record_offset":35396,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":571,"record_offset":39460,"parent_record_offset":39392,"ancestor_record_offset":35396,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":571,"record_offset":39184,"parent_record_offset":38832,"ancestor_record_offset":35396,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":571,"record_offset":38796,"parent_record_offset":38760,"ancestor_record_offset":35396,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":571,"record_offset":38832,"parent_record_offset":38796,"ancestor_record_offset":35396,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":571,"record_offset":35912,"parent_record_offset":35888,"ancestor_record_offset":35396,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":571,"record_offset":47344,"parent_record_offset":47320,"ancestor_record_offset":35396,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":571,"record_offset":47320,"parent_record_offset":47288,"ancestor_record_offset":35396,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":571,"record_offset":47408,"parent_record_offset":47344,"ancestor_record_offset":35396,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":571,"record_offset":39712,"parent_record_offset":39688,"ancestor_record_offset":35396,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":571,"record_offset":39736,"parent_record_offset":39712,"ancestor_record_offset":35396,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::typed_swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":571,"record_offset":46536,"parent_record_offset":46512,"ancestor_record_offset":35396,"inlinee":5746,"source_line":2574,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":571,"record_offset":39336,"parent_record_offset":39296,"ancestor_record_offset":35396,"inlinee":10907,"source_line":36,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":571,"record_offset":39268,"parent_record_offset":39240,"ancestor_record_offset":35396,"inlinee":10905,"source_line":77,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":571,"record_offset":36492,"parent_record_offset":36468,"ancestor_record_offset":35396,"inlinee":6384,"source_line":106,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":571,"record_offset":36640,"parent_record_offset":36516,"ancestor_record_offset":35396,"inlinee":6386,"source_line":118,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":38724,"parent_record_offset":38580,"ancestor_record_offset":35396,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":36468,"parent_record_offset":36444,"ancestor_record_offset":35396,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":36668,"parent_record_offset":36640,"ancestor_record_offset":35396,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":39240,"parent_record_offset":39212,"ancestor_record_offset":35396,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":39364,"parent_record_offset":39336,"ancestor_record_offset":35396,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":36516,"parent_record_offset":36492,"ancestor_record_offset":35396,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":571,"record_offset":49048,"parent_record_offset":49008,"ancestor_record_offset":35396,"inlinee":5745,"source_line":756,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":571,"record_offset":40916,"parent_record_offset":38580,"ancestor_record_offset":35396,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":571,"record_offset":39980,"parent_record_offset":39948,"ancestor_record_offset":35396,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::impl$0::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":571,"record_offset":36756,"parent_record_offset":36516,"ancestor_record_offset":35396,"inlinee":5504,"source_line":115,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":571,"record_offset":36692,"parent_record_offset":36668,"ancestor_record_offset":35396,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":571,"record_offset":39688,"parent_record_offset":39664,"ancestor_record_offset":35396,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":571,"record_offset":39664,"parent_record_offset":39632,"ancestor_record_offset":35396,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=63
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":37448,"parent_record_offset":37268,"ancestor_record_offset":35396,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":571,"record_offset":36408,"parent_record_offset":36384,"ancestor_record_offset":35396,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":571,"record_offset":36608,"parent_record_offset":36548,"ancestor_record_offset":35396,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":571,"record_offset":49168,"parent_record_offset":49088,"ancestor_record_offset":35396,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":571,"record_offset":36580,"parent_record_offset":36548,"ancestor_record_offset":35396,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":49192,"parent_record_offset":49168,"ancestor_record_offset":35396,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_chunk
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":46664,"parent_record_offset":46640,"ancestor_record_offset":35396,"inlinee":5753,"source_line":1432,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":46560,"parent_record_offset":46536,"ancestor_record_offset":35396,"inlinee":5747,"source_line":1363,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":571,"record_offset":46588,"parent_record_offset":46560,"ancestor_record_offset":35396,"inlinee":5749,"source_line":2434,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":46616,"parent_record_offset":46588,"ancestor_record_offset":35396,"inlinee":5750,"source_line":1440,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_bytes::swap_nonoverlapping_chunks
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":46640,"parent_record_offset":46616,"ancestor_record_offset":35396,"inlinee":5752,"source_line":1442,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":39428,"parent_record_offset":39392,"ancestor_record_offset":35396,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":571,"record_offset":39592,"parent_record_offset":39564,"ancestor_record_offset":35396,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":36880,"parent_record_offset":36856,"ancestor_record_offset":35396,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":571,"record_offset":49008,"parent_record_offset":48976,"ancestor_record_offset":35396,"inlinee":5744,"source_line":1001,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":571,"record_offset":40944,"parent_record_offset":38580,"ancestor_record_offset":35396,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":571,"record_offset":40972,"parent_record_offset":40944,"ancestor_record_offset":35396,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":571,"record_offset":36904,"parent_record_offset":36880,"ancestor_record_offset":35396,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":571,"record_offset":36384,"parent_record_offset":36360,"ancestor_record_offset":35396,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":39296,"parent_record_offset":39268,"ancestor_record_offset":35396,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":37592,"parent_record_offset":37568,"ancestor_record_offset":35396,"inlinee":32137,"source_line":360,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":36548,"parent_record_offset":36516,"ancestor_record_offset":35396,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":571,"record_offset":48068,"parent_record_offset":48044,"ancestor_record_offset":35396,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":571,"record_offset":48092,"parent_record_offset":48068,"ancestor_record_offset":35396,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":571,"record_offset":38420,"parent_record_offset":38396,"ancestor_record_offset":35396,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":36716,"parent_record_offset":36692,"ancestor_record_offset":35396,"inlinee":36199,"source_line":200,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":571,"record_offset":37740,"parent_record_offset":37716,"ancestor_record_offset":35396,"inlinee":36200,"source_line":249,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback::on_terrain_modified
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/terrain/delayed_terrain_changed_msg.rs
//     evidence={"module":571,"record_offset":48044,"parent_record_offset":47288,"ancestor_record_offset":35396,"inlinee":32048,"source_line":39,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":571,"record_offset":38364,"parent_record_offset":38340,"ancestor_record_offset":35396,"inlinee":33040,"source_line":189,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":571,"record_offset":39564,"parent_record_offset":38580,"ancestor_record_offset":35396,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::try_slice
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":571,"record_offset":39780,"parent_record_offset":38580,"ancestor_record_offset":35396,"inlinee":14742,"source_line":859,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":571,"record_offset":39632,"parent_record_offset":38580,"ancestor_record_offset":35396,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::replay_raster::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 200
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8D67
// 首条位置: {"module":571,"inlinee":36199}
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
// IPI IDs: 0x8D68
// 首条位置: {"module":571,"inlinee":36200}
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
// 记录形式: inline_instance, inlinee_declaration; instances=24
// 源码行: 342
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14784
// 首条位置: {"module":1745,"inlinee":83844}
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
// IPI IDs: 0x8D72
// 首条位置: {"module":571,"inlinee":36210}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":571,"record_offset":6404,"parent_record_offset":6292,"ancestor_record_offset":6292,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":6432,"parent_record_offset":6292,"ancestor_record_offset":6292,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":571,"record_offset":8312,"parent_record_offset":6292,"ancestor_record_offset":6292,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=72
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":7852,"parent_record_offset":6292,"ancestor_record_offset":6292,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":571,"record_offset":8340,"parent_record_offset":6292,"ancestor_record_offset":6292,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":571,"record_offset":7272,"parent_record_offset":6292,"ancestor_record_offset":6292,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::try_slice
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":571,"record_offset":7488,"parent_record_offset":6292,"ancestor_record_offset":6292,"inlinee":14742,"source_line":859,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":571,"record_offset":7340,"parent_record_offset":6292,"ancestor_record_offset":6292,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":6784,"parent_record_offset":6760,"ancestor_record_offset":6292,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":6808,"parent_record_offset":6784,"ancestor_record_offset":6292,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":7744,"parent_record_offset":7720,"ancestor_record_offset":6292,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":7768,"parent_record_offset":7744,"ancestor_record_offset":6292,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":6832,"parent_record_offset":6808,"ancestor_record_offset":6292,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":7792,"parent_record_offset":7768,"ancestor_record_offset":6292,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":6760,"parent_record_offset":6720,"ancestor_record_offset":6292,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":7720,"parent_record_offset":7628,"ancestor_record_offset":6292,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":6652,"parent_record_offset":6620,"ancestor_record_offset":6292,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=99
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":7656,"parent_record_offset":7628,"ancestor_record_offset":6292,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=108
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":7628,"parent_record_offset":7604,"ancestor_record_offset":6292,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":6720,"parent_record_offset":6684,"ancestor_record_offset":6292,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":6684,"parent_record_offset":6652,"ancestor_record_offset":6292,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=108
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":571,"record_offset":7604,"parent_record_offset":7580,"ancestor_record_offset":6292,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":6920,"parent_record_offset":6892,"ancestor_record_offset":6292,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":7528,"parent_record_offset":7488,"ancestor_record_offset":6292,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":6588,"parent_record_offset":6540,"ancestor_record_offset":6292,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":6620,"parent_record_offset":6588,"ancestor_record_offset":6292,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":6468,"parent_record_offset":6432,"ancestor_record_offset":6292,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":7100,"parent_record_offset":7072,"ancestor_record_offset":6292,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":571,"record_offset":7168,"parent_record_offset":7100,"ancestor_record_offset":6292,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":571,"record_offset":6892,"parent_record_offset":6540,"ancestor_record_offset":6292,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":571,"record_offset":6504,"parent_record_offset":6468,"ancestor_record_offset":6292,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":571,"record_offset":6540,"parent_record_offset":6504,"ancestor_record_offset":6292,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":571,"record_offset":7420,"parent_record_offset":7396,"ancestor_record_offset":6292,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":571,"record_offset":7444,"parent_record_offset":7420,"ancestor_record_offset":6292,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":571,"record_offset":7044,"parent_record_offset":7004,"ancestor_record_offset":6292,"inlinee":10907,"source_line":36,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":571,"record_offset":6976,"parent_record_offset":6948,"ancestor_record_offset":6292,"inlinee":10905,"source_line":77,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":6948,"parent_record_offset":6920,"ancestor_record_offset":6292,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":7072,"parent_record_offset":7044,"ancestor_record_offset":6292,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":571,"record_offset":7688,"parent_record_offset":7656,"ancestor_record_offset":6292,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":571,"record_offset":7396,"parent_record_offset":7372,"ancestor_record_offset":6292,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":571,"record_offset":7372,"parent_record_offset":7340,"ancestor_record_offset":6292,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=192
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":7556,"parent_record_offset":7488,"ancestor_record_offset":6292,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":571,"record_offset":7208,"parent_record_offset":7004,"ancestor_record_offset":6292,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":7136,"parent_record_offset":7100,"ancestor_record_offset":6292,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":571,"record_offset":7300,"parent_record_offset":7272,"ancestor_record_offset":6292,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":571,"record_offset":8368,"parent_record_offset":8340,"ancestor_record_offset":6292,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":571,"record_offset":7004,"parent_record_offset":6976,"ancestor_record_offset":6292,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_raster::duration_based_on_stroke_length
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 472
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x142A8
// 首条位置: {"module":1674,"inlinee":82600}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1674,"record_offset":89732,"parent_record_offset":89708,"ancestor_record_offset":89708,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01674.jsonl.gz"}
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
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::history::replay_raster::RasterReplayDirtyState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::Local<country_core::systems::history::replay_raster::RasterReplayDirtyState>", "constructor": "bevy_ecs::system::system_param::Local", "arguments": ["country_core::systems::history::replay_raster::RasterReplayDirtyState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$112::init_access<bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::system::system_param::Local<country_core::systems::history::replay_raster::RasterReplayDirtyState> >", "constructor": "bevy_ecs::system::system_param::impl$112::init_access", "arguments": ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::system::system_param::Local<country_core::systems::history::replay_raster::RasterReplayDirtyState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::history::replay_raster::RasterReplayResources
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 118
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$112::init_access<country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback> >", "constructor": "bevy_ecs::system::system_param::impl$112::init_access", "arguments": ["country_core::systems::history::replay_raster::RasterReplayResources", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>", "bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::set_last_run<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$12::set_last_run", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::set_last_run<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::set_last_run", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::system::System::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::system::System::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)>", "constructor": "tuple$", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>)", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::ScheduleConfigTupleMarker", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<tuple$<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)> >,country_core::systems::GameSystemSet>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set", "arguments": ["tuple$<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)>", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)> >", "country_core::systems::GameSystemSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::set_last_run<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$12::set_last_run", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::name<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$12::name", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)", "tuple$<>", "void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::system::System::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 282; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >", "bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound> >", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >", "bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>", "bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayGroupBuildingMoveAudio>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayBuildingAudio>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayGroupBuildingMoveAudio>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayGroupBuildingMoveAudio>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::startup::install_mod_clutter::ModClutterInstalls>,bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::InstallModClutterCmd>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::UninstallModClutterCmd>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::startup::install_mod_clutter::ModClutterInstalls>", "bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::InstallModClutterCmd>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::UninstallModClutterCmd>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::install_mod_clutter::ModClutterInstalls>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::InstallModClutterCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::UninstallModClutterCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::RasterReplayCmd::backward
// 信息 ID: 1e51800fc39b22364eb61ee0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::RasterReplayCmd::backward_instant
// 信息 ID: bda7cf00dbdb89786016c8a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 18 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=95576; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=96392; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=96860; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=97652; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=98120; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=98936; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=101852; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=103064; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::RasterReplayCmd::forward
// 信息 ID: 4218df487a21dc08b970e53d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::RasterReplayCmd::forward_instant
// 信息 ID: 78c90e5e1bd4a8c274bd55e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 18 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=95660; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=96476; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=96944; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=97736; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=98204; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=99020; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=101936; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=103148; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::RasterReplayCmd::new
// 信息 ID: 8c373892fed2ac6067206c7c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 38 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_raster::RasterReplayCmd::forward; module=1674, inline_record=89764; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::RasterReplayCmd::backward; module=1674, inline_record=90548; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=95600; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=95684; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=96416; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=96500; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=96884; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=96968; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::_::impl$0::get_param
// 信息 ID: bef4c210601d25ad4fc8adba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1240f3baf2b37ffd4baa4cde: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)
// [泛型宿主/闭包/直接引用方参数候选] 724590ef75324dbbc996662f: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] beeb1749b20c1ef79c4b27a8: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] bfbf3f13a58ea0ca42a281f1: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] d55efa96b8c611f52326f04f: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] f67642ecef8632b777dba5ac: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)
// 引用方: 原程序直接转移 11 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> @ 0xc1ae85 -> 0xc1dff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> @ 0xc1b275 -> 0xc1dff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> @ 0xc1b965 -> 0xc1dff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> @ 0xc1bc89 -> 0xc1dff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> @ 0xc1c0a5 -> 0xc1dff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> > @ 0x19331c3 -> 0x1904000 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> > @ 0x1939b13 -> 0x1904000 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > @ 0x194a753 -> 0x1904000 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> > @ 0x19563e3 -> 0x1904000 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> > @ 0x1959628 -> 0x1904000 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> > @ 0x195f10e -> 0x1904000 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>; module=1720, inline_record=36572; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::_::impl$0::init_access
// 信息 ID: 40f057e46d45ed94234f0e88
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1240f3baf2b37ffd4baa4cde: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)
// [泛型宿主/闭包/直接引用方参数候选] 724590ef75324dbbc996662f: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] beeb1749b20c1ef79c4b27a8: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] bfbf3f13a58ea0ca42a281f1: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] d55efa96b8c611f52326f04f: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::system_param::impl$112::init_access<country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback> >; module=571, inline_record=120292; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)>; module=593, inline_record=1912; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)>; module=593, inline_record=7696; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)>; module=593, inline_record=18024; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>; module=593, inline_record=19896; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>; module=1720, inline_record=19000; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::_::impl$0::init_state
// 信息 ID: 55e7dc6a3003ad5e548de4f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1240f3baf2b37ffd4baa4cde: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)
// [泛型宿主/闭包/直接引用方参数候选] 724590ef75324dbbc996662f: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] beeb1749b20c1ef79c4b27a8: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] bfbf3f13a58ea0ca42a281f1: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] d55efa96b8c611f52326f04f: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] f67642ecef8632b777dba5ac: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> @ 0xc195a5 -> 0x17f5cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> @ 0xc19855 -> 0x17f5cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> @ 0xc19c85 -> 0x17f5cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> @ 0xc1a735 -> 0x17f5cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> @ 0xc1a9c5 -> 0x17f5cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::initialize<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> @ 0x19dd043 -> 0x17f5cd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::_::impl$0::validate_param
// 信息 ID: aa1ed783d024e98d91ec4cd2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1240f3baf2b37ffd4baa4cde: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)
// [泛型宿主/闭包/直接引用方参数候选] beeb1749b20c1ef79c4b27a8: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] f67642ecef8632b777dba5ac: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>; module=593, inline_record=69468; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>; module=593, inline_record=73476; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>; module=1698, inline_record=1384388; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)>; module=1698, inline_record=1498744; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>; module=1698, inline_record=1585412; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)>; module=1720, inline_record=115364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::duration_based_on_stroke_length
// 信息 ID: e02fe5688699d662a7a3b0c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_raster::RasterReplayCmd::forward; module=1674, inline_record=89708; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::RasterReplayCmd::backward; module=1674, inline_record=90492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::get_stroke_from_0_to_t
// 信息 ID: 144aa4e31dcf6495097928b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >; module=571, inline_record=6292; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >; module=571, inline_record=10056; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >; module=571, inline_record=22468; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >; module=571, inline_record=26256; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=571, inline_record=38580; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=571, inline_record=42368; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >; module=571, inline_record=55456; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >; module=571, inline_record=59220; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::impl$0::from
// 信息 ID: a0f67a4c697f671cddf4f644
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=95304; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=96120; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=96588; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=97376; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=97848; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1674, inline_record=98664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::impl$7::eq
// 信息 ID: f4bb1855e4b37d822065a09e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >; module=571, inline_record=5464; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >; module=571, inline_record=21640; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=571, inline_record=37780; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >; module=571, inline_record=54628; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>; module=571, inline_record=70748; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> >; module=571, inline_record=87176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::impl$9::default
// 信息 ID: 646f6921ef79e26c72f5ae5a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_raster::_::impl$0::init_state; module=1674, inline_record=93908; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::replay_raster::closure$0
// 信息 ID: 27ef0fb527d4a4996e451636
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::history::replay_raster::replay_raster", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >; module=571, inline_record=4400; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >; module=571, inline_record=20576; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=571, inline_record=36716; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >; module=571, inline_record=53564; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>; module=571, inline_record=69684; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> >; module=571, inline_record=86112; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::replay_raster::closure$1
// 信息 ID: 6fa7ecca58488782b7bae248
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::history::replay_raster::replay_raster", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >; module=571, inline_record=5424; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >; module=571, inline_record=21600; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=571, inline_record=37740; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >; module=571, inline_record=54588; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>; module=571, inline_record=70708; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> >; module=571, inline_record=87136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::replay_raster::closure$2
// 信息 ID: 3b78703e2a22433df991a506
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::history::replay_raster::replay_raster", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::history::replay_raster::replay_raster::closure_env$2 <- country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_raster::replay_raster::closure_env$2 <- country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_raster::replay_raster::closure_env$2 <- country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_raster::replay_raster::closure_env$2 <- country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_raster::replay_raster::closure_env$2 <- country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_raster::replay_raster::closure_env$2 <- country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > >; module=1745, inline_record=12980; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > >; module=1745, inline_record=13356; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > >; module=1745, inline_record=14900; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > >; module=1745, inline_record=15276; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > >; module=1745, inline_record=18428; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > >; module=1745, inline_record=18804; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > > >; module=1745, inline_record=20364; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > > >; module=1745, inline_record=20740; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> >
// 信息 ID: 2eef49cccc81e8af76fd70e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 724590ef75324dbbc996662f: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_path::replay_path::closure_env$1 -> ["country_core::systems::history::replay_path::Cobblestone1PathFilter"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster -> ["country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> @ 0xc1c192 -> 0xb70f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::system::commands::Commands)> > @ 0x19564d0 -> 0xb70f40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >
// 信息 ID: ee776aef17a7b36f44e39171
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] bfbf3f13a58ea0ca42a281f1: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_path::replay_path::closure_env$1 -> ["country_core::systems::history::replay_path::Cobblestone2PathFilter"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster -> ["country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> @ 0xc1af72 -> 0xb6bef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::system::commands::Commands)> > @ 0x1939c00 -> 0xb6bef0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >
// 信息 ID: 6c5fb99e835c0cabf056c7f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d55efa96b8c611f52326f04f: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_path::replay_path::closure_env$1 -> ["country_core::systems::history::replay_path::Cobblestone3PathFilter"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster -> ["country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> @ 0xc1b362 -> 0xb646a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::system::commands::Commands)> > @ 0x19332b0 -> 0xb646a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>
// 信息 ID: 739aabbf59af2dc7ed8c4f72
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1240f3baf2b37ffd4baa4cde: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster -> ["country_core::resources::world_raster_defs::GardenRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_garden::replay_garden::closure_env$2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> > @ 0x195977e -> 0xb6e6b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)> @ 0x19de42e -> 0xb6e6b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >
// 信息 ID: 2861d512c14c67814c28570c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] beeb1749b20c1ef79c4b27a8: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_path::replay_path::closure_env$1 -> ["country_core::systems::history::replay_path::DirtPathFilter"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster -> ["country_core::resources::world_raster_defs::PathRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> @ 0xc1ba52 -> 0xb66e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > @ 0x194a840 -> 0xb66e60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>
// 信息 ID: 91372219ea95d312bff6dc69
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f67642ecef8632b777dba5ac: void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster -> ["country_core::resources::world_raster_defs::WaterRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_water::replay_water::closure_env$2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> @ 0xc1bf2f -> 0xb69610 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),tuple$<>,void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>,bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> > @ 0x195f3bf -> 0xb69610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

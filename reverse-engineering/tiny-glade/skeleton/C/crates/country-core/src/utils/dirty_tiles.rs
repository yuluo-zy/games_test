// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/utils/dirty_tiles.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 634609128761cbee08cc7bf86da12b69b67f8e9f524391c6a192f23c14e1ce72
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: dirty_tiles.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 160, 177, 517, 519, 527, 538, 550, 560, 571, 604, 605, 607, 609, 1602, 1643, 1664, 1745, 2754, 2755, 3882, 4056, 4729, 4815, 4997
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use ahash; // namespace candidate
//   package identities: [{"name":"ahash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.11"}]
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.14"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use std; // namespace candidate
//   package identities: []
// use system_clutter; // namespace candidate
//   package identities: [{"name":"system-clutter","source":"local_or_path","version":"0.1.0"}]
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use tabloid; // namespace candidate
//   package identities: [{"name":"tabloid","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::impl$4::clone
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x97A5
// 首条位置: {"module":609,"inlinee":38821}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::new_clear
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":538,"record_offset":9796,"rva":10854176,"va":5379563296}
// 编译实例 RVAs: 0xa59f20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37493771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563296,"raw_hex":"e80b1c3c02","size":5,"soft_fail":false,"target_va":5417057088,"va":5379563312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1745,"record_offset":15948,"rva":27725760,"va":5396434880}
// 编译实例 RVAs: 0x1a70fc0, 0x2afa290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140cabef0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14438845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e843ae23ff","size":5,"soft_fail":false,"target_va":5381996272,"va":5396435112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417c0200
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2821929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e8d7f0d4ff","size":5,"soft_fail":false,"target_va":5393613312,"va":5396435236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142cfdc80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2111623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413773968,"raw_hex":"e887382000","size":5,"soft_fail":false,"target_va":5415885952,"va":5413774324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be10
//     owner=unresolved_namespace; count=16
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20622737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e891ad3a01","size":5,"soft_fail":false,"target_va":5417057808,"va":5396435066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1745,"record_offset":16268,"parent_record_offset":15948,"ancestor_record_offset":15948,"inlinee":15334,"source_line":43,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1745,"record_offset":17272,"parent_record_offset":16268,"ancestor_record_offset":15948,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::mul_add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1745,"record_offset":17100,"parent_record_offset":17072,"ancestor_record_offset":15948,"inlinee":15323,"source_line":1919,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1745,"record_offset":16640,"parent_record_offset":16608,"ancestor_record_offset":15948,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1745,"record_offset":16704,"parent_record_offset":16672,"ancestor_record_offset":15948,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1745,"record_offset":16376,"parent_record_offset":16332,"ancestor_record_offset":15948,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1745,"record_offset":16736,"parent_record_offset":16704,"ancestor_record_offset":15948,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1745,"record_offset":16536,"parent_record_offset":16508,"ancestor_record_offset":15948,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1745,"record_offset":16508,"parent_record_offset":16376,"ancestor_record_offset":15948,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1745,"record_offset":16672,"parent_record_offset":16640,"ancestor_record_offset":15948,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1745,"record_offset":16476,"parent_record_offset":16408,"ancestor_record_offset":15948,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1745,"record_offset":16444,"parent_record_offset":16408,"ancestor_record_offset":15948,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1745,"record_offset":16408,"parent_record_offset":16376,"ancestor_record_offset":15948,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::raster_stroke::impl$2::iter_points_xz::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":1745,"record_offset":16564,"parent_record_offset":16536,"ancestor_record_offset":15948,"inlinee":82267,"source_line":60,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":1745,"record_offset":17044,"parent_record_offset":17004,"ancestor_record_offset":15948,"inlinee":15321,"source_line":252,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":17004,"parent_record_offset":16268,"ancestor_record_offset":15948,"inlinee":15320,"source_line":903,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$95::from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":17240,"parent_record_offset":16268,"ancestor_record_offset":15948,"inlinee":15325,"source_line":1920,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1745,"record_offset":16608,"parent_record_offset":16332,"ancestor_record_offset":15948,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1745,"record_offset":16332,"parent_record_offset":16268,"ancestor_record_offset":15948,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1745,"record_offset":17072,"parent_record_offset":17044,"ancestor_record_offset":15948,"inlinee":15322,"source_line":221,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1745,"record_offset":10428,"rva":27725760,"va":5396434880}
// 编译实例 RVAs: 0x1a70fc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140cabef0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14438845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e843ae23ff","size":5,"soft_fail":false,"target_va":5381996272,"va":5396435112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417c0200
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2821929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e8d7f0d4ff","size":5,"soft_fail":false,"target_va":5393613312,"va":5396435236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be10
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20622737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e891ad3a01","size":5,"soft_fail":false,"target_va":5417057808,"va":5396435066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1745,"record_offset":10748,"parent_record_offset":10428,"ancestor_record_offset":10428,"inlinee":15334,"source_line":43,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1745,"record_offset":11752,"parent_record_offset":10748,"ancestor_record_offset":10428,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1745,"record_offset":11580,"parent_record_offset":11552,"ancestor_record_offset":10428,"inlinee":15323,"source_line":1919,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1745,"record_offset":11120,"parent_record_offset":11088,"ancestor_record_offset":10428,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1745,"record_offset":11184,"parent_record_offset":11152,"ancestor_record_offset":10428,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1745,"record_offset":10856,"parent_record_offset":10812,"ancestor_record_offset":10428,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1745,"record_offset":11216,"parent_record_offset":11184,"ancestor_record_offset":10428,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1745,"record_offset":11016,"parent_record_offset":10988,"ancestor_record_offset":10428,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1745,"record_offset":10988,"parent_record_offset":10856,"ancestor_record_offset":10428,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1745,"record_offset":11152,"parent_record_offset":11120,"ancestor_record_offset":10428,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1745,"record_offset":10956,"parent_record_offset":10888,"ancestor_record_offset":10428,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1745,"record_offset":10924,"parent_record_offset":10888,"ancestor_record_offset":10428,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1745,"record_offset":10888,"parent_record_offset":10856,"ancestor_record_offset":10428,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::raster_stroke::impl$3::iter_points_xz::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":1745,"record_offset":11044,"parent_record_offset":11016,"ancestor_record_offset":10428,"inlinee":83843,"source_line":80,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":1745,"record_offset":11524,"parent_record_offset":11484,"ancestor_record_offset":10428,"inlinee":15321,"source_line":252,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":11484,"parent_record_offset":10748,"ancestor_record_offset":10428,"inlinee":15320,"source_line":903,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$95::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":11720,"parent_record_offset":10748,"ancestor_record_offset":10428,"inlinee":15325,"source_line":1920,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1745,"record_offset":11088,"parent_record_offset":10812,"ancestor_record_offset":10428,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1745,"record_offset":10812,"parent_record_offset":10748,"ancestor_record_offset":10428,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1745,"record_offset":11552,"parent_record_offset":11524,"ancestor_record_offset":10428,"inlinee":15322,"source_line":221,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1745,"record_offset":23272,"rva":27725760,"va":5396434880}
// 编译实例 RVAs: 0x1a70fc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140cabef0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14438845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e843ae23ff","size":5,"soft_fail":false,"target_va":5381996272,"va":5396435112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417c0200
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2821929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e8d7f0d4ff","size":5,"soft_fail":false,"target_va":5393613312,"va":5396435236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be10
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20622737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e891ad3a01","size":5,"soft_fail":false,"target_va":5417057808,"va":5396435066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1745,"record_offset":23864,"parent_record_offset":23272,"ancestor_record_offset":23272,"inlinee":15334,"source_line":43,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1745,"record_offset":24868,"parent_record_offset":23864,"ancestor_record_offset":23272,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1745,"record_offset":24696,"parent_record_offset":24668,"ancestor_record_offset":23272,"inlinee":15323,"source_line":1919,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1745,"record_offset":24236,"parent_record_offset":24204,"ancestor_record_offset":23272,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1745,"record_offset":24300,"parent_record_offset":24268,"ancestor_record_offset":23272,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1745,"record_offset":23972,"parent_record_offset":23928,"ancestor_record_offset":23272,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1745,"record_offset":24332,"parent_record_offset":24300,"ancestor_record_offset":23272,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1745,"record_offset":24132,"parent_record_offset":24104,"ancestor_record_offset":23272,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1745,"record_offset":24104,"parent_record_offset":23972,"ancestor_record_offset":23272,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1745,"record_offset":24268,"parent_record_offset":24236,"ancestor_record_offset":23272,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1745,"record_offset":24072,"parent_record_offset":24004,"ancestor_record_offset":23272,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1745,"record_offset":24040,"parent_record_offset":24004,"ancestor_record_offset":23272,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1745,"record_offset":24004,"parent_record_offset":23972,"ancestor_record_offset":23272,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1745,"record_offset":24160,"parent_record_offset":24132,"ancestor_record_offset":23272,"inlinee":83844,"source_line":342,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":1745,"record_offset":24640,"parent_record_offset":24600,"ancestor_record_offset":23272,"inlinee":15321,"source_line":252,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":24600,"parent_record_offset":23864,"ancestor_record_offset":23272,"inlinee":15320,"source_line":903,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$95::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":24836,"parent_record_offset":23864,"ancestor_record_offset":23272,"inlinee":15325,"source_line":1920,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1745,"record_offset":24204,"parent_record_offset":23928,"ancestor_record_offset":23272,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1745,"record_offset":23928,"parent_record_offset":23864,"ancestor_record_offset":23272,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1745,"record_offset":24668,"parent_record_offset":24640,"ancestor_record_offset":23272,"inlinee":15322,"source_line":221,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1745,"record_offset":12092,"rva":27725760,"va":5396434880}
// 编译实例 RVAs: 0x1a70fc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140cabef0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14438845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e843ae23ff","size":5,"soft_fail":false,"target_va":5381996272,"va":5396435112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417c0200
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2821929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e8d7f0d4ff","size":5,"soft_fail":false,"target_va":5393613312,"va":5396435236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be10
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20622737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e891ad3a01","size":5,"soft_fail":false,"target_va":5417057808,"va":5396435066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1745,"record_offset":12684,"parent_record_offset":12092,"ancestor_record_offset":12092,"inlinee":15334,"source_line":43,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1745,"record_offset":13688,"parent_record_offset":12684,"ancestor_record_offset":12092,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1745,"record_offset":13516,"parent_record_offset":13488,"ancestor_record_offset":12092,"inlinee":15323,"source_line":1919,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1745,"record_offset":13056,"parent_record_offset":13024,"ancestor_record_offset":12092,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1745,"record_offset":13120,"parent_record_offset":13088,"ancestor_record_offset":12092,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1745,"record_offset":12792,"parent_record_offset":12748,"ancestor_record_offset":12092,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1745,"record_offset":13152,"parent_record_offset":13120,"ancestor_record_offset":12092,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1745,"record_offset":12952,"parent_record_offset":12924,"ancestor_record_offset":12092,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1745,"record_offset":12924,"parent_record_offset":12792,"ancestor_record_offset":12092,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1745,"record_offset":13088,"parent_record_offset":13056,"ancestor_record_offset":12092,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1745,"record_offset":12892,"parent_record_offset":12824,"ancestor_record_offset":12092,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1745,"record_offset":12860,"parent_record_offset":12824,"ancestor_record_offset":12092,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1745,"record_offset":12824,"parent_record_offset":12792,"ancestor_record_offset":12092,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1745,"record_offset":12980,"parent_record_offset":12952,"ancestor_record_offset":12092,"inlinee":83844,"source_line":342,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":1745,"record_offset":13460,"parent_record_offset":13420,"ancestor_record_offset":12092,"inlinee":15321,"source_line":252,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":13420,"parent_record_offset":12684,"ancestor_record_offset":12092,"inlinee":15320,"source_line":903,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$95::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":13656,"parent_record_offset":12684,"ancestor_record_offset":12092,"inlinee":15325,"source_line":1920,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1745,"record_offset":13024,"parent_record_offset":12748,"ancestor_record_offset":12092,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1745,"record_offset":12748,"parent_record_offset":12684,"ancestor_record_offset":12092,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1745,"record_offset":13488,"parent_record_offset":13460,"ancestor_record_offset":12092,"inlinee":15322,"source_line":221,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1745,"record_offset":19476,"rva":27725760,"va":5396434880}
// 编译实例 RVAs: 0x1a70fc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140cabef0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14438845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e843ae23ff","size":5,"soft_fail":false,"target_va":5381996272,"va":5396435112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417c0200
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2821929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e8d7f0d4ff","size":5,"soft_fail":false,"target_va":5393613312,"va":5396435236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be10
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20622737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e891ad3a01","size":5,"soft_fail":false,"target_va":5417057808,"va":5396435066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1745,"record_offset":20068,"parent_record_offset":19476,"ancestor_record_offset":19476,"inlinee":15334,"source_line":43,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1745,"record_offset":21072,"parent_record_offset":20068,"ancestor_record_offset":19476,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1745,"record_offset":20900,"parent_record_offset":20872,"ancestor_record_offset":19476,"inlinee":15323,"source_line":1919,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1745,"record_offset":20440,"parent_record_offset":20408,"ancestor_record_offset":19476,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1745,"record_offset":20504,"parent_record_offset":20472,"ancestor_record_offset":19476,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1745,"record_offset":20176,"parent_record_offset":20132,"ancestor_record_offset":19476,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1745,"record_offset":20536,"parent_record_offset":20504,"ancestor_record_offset":19476,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1745,"record_offset":20336,"parent_record_offset":20308,"ancestor_record_offset":19476,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1745,"record_offset":20308,"parent_record_offset":20176,"ancestor_record_offset":19476,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1745,"record_offset":20472,"parent_record_offset":20440,"ancestor_record_offset":19476,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1745,"record_offset":20276,"parent_record_offset":20208,"ancestor_record_offset":19476,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1745,"record_offset":20244,"parent_record_offset":20208,"ancestor_record_offset":19476,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1745,"record_offset":20208,"parent_record_offset":20176,"ancestor_record_offset":19476,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1745,"record_offset":20364,"parent_record_offset":20336,"ancestor_record_offset":19476,"inlinee":83844,"source_line":342,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":1745,"record_offset":20844,"parent_record_offset":20804,"ancestor_record_offset":19476,"inlinee":15321,"source_line":252,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":20804,"parent_record_offset":20068,"ancestor_record_offset":19476,"inlinee":15320,"source_line":903,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$95::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":21040,"parent_record_offset":20068,"ancestor_record_offset":19476,"inlinee":15325,"source_line":1920,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1745,"record_offset":20408,"parent_record_offset":20132,"ancestor_record_offset":19476,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1745,"record_offset":20132,"parent_record_offset":20068,"ancestor_record_offset":19476,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1745,"record_offset":20872,"parent_record_offset":20844,"ancestor_record_offset":19476,"inlinee":15322,"source_line":221,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1745,"record_offset":17612,"rva":27725760,"va":5396434880}
// 编译实例 RVAs: 0x1a70fc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140cabef0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14438845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e843ae23ff","size":5,"soft_fail":false,"target_va":5381996272,"va":5396435112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417c0200
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2821929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e8d7f0d4ff","size":5,"soft_fail":false,"target_va":5393613312,"va":5396435236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be10
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20622737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e891ad3a01","size":5,"soft_fail":false,"target_va":5417057808,"va":5396435066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1745,"record_offset":18132,"parent_record_offset":17612,"ancestor_record_offset":17612,"inlinee":15334,"source_line":43,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1745,"record_offset":19136,"parent_record_offset":18132,"ancestor_record_offset":17612,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1745,"record_offset":18964,"parent_record_offset":18936,"ancestor_record_offset":17612,"inlinee":15323,"source_line":1919,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1745,"record_offset":18504,"parent_record_offset":18472,"ancestor_record_offset":17612,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1745,"record_offset":18568,"parent_record_offset":18536,"ancestor_record_offset":17612,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1745,"record_offset":18240,"parent_record_offset":18196,"ancestor_record_offset":17612,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1745,"record_offset":18600,"parent_record_offset":18568,"ancestor_record_offset":17612,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1745,"record_offset":18400,"parent_record_offset":18372,"ancestor_record_offset":17612,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1745,"record_offset":18372,"parent_record_offset":18240,"ancestor_record_offset":17612,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1745,"record_offset":18536,"parent_record_offset":18504,"ancestor_record_offset":17612,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1745,"record_offset":18340,"parent_record_offset":18272,"ancestor_record_offset":17612,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1745,"record_offset":18308,"parent_record_offset":18272,"ancestor_record_offset":17612,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1745,"record_offset":18272,"parent_record_offset":18240,"ancestor_record_offset":17612,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1745,"record_offset":18428,"parent_record_offset":18400,"ancestor_record_offset":17612,"inlinee":83844,"source_line":342,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":1745,"record_offset":18908,"parent_record_offset":18868,"ancestor_record_offset":17612,"inlinee":15321,"source_line":252,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":18868,"parent_record_offset":18132,"ancestor_record_offset":17612,"inlinee":15320,"source_line":903,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$95::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":19104,"parent_record_offset":18132,"ancestor_record_offset":17612,"inlinee":15325,"source_line":1920,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1745,"record_offset":18472,"parent_record_offset":18196,"ancestor_record_offset":17612,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1745,"record_offset":18196,"parent_record_offset":18132,"ancestor_record_offset":17612,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1745,"record_offset":18936,"parent_record_offset":18908,"ancestor_record_offset":17612,"inlinee":15322,"source_line":221,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1745,"record_offset":14028,"rva":27725760,"va":5396434880}
// 编译实例 RVAs: 0x1a70fc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140cabef0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14438845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e843ae23ff","size":5,"soft_fail":false,"target_va":5381996272,"va":5396435112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417c0200
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2821929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e8d7f0d4ff","size":5,"soft_fail":false,"target_va":5393613312,"va":5396435236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be10
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20622737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e891ad3a01","size":5,"soft_fail":false,"target_va":5417057808,"va":5396435066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1745,"record_offset":14604,"parent_record_offset":14028,"ancestor_record_offset":14028,"inlinee":15334,"source_line":43,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1745,"record_offset":15608,"parent_record_offset":14604,"ancestor_record_offset":14028,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1745,"record_offset":15436,"parent_record_offset":15408,"ancestor_record_offset":14028,"inlinee":15323,"source_line":1919,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1745,"record_offset":14976,"parent_record_offset":14944,"ancestor_record_offset":14028,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1745,"record_offset":15040,"parent_record_offset":15008,"ancestor_record_offset":14028,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1745,"record_offset":14712,"parent_record_offset":14668,"ancestor_record_offset":14028,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1745,"record_offset":15072,"parent_record_offset":15040,"ancestor_record_offset":14028,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1745,"record_offset":14872,"parent_record_offset":14844,"ancestor_record_offset":14028,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1745,"record_offset":14844,"parent_record_offset":14712,"ancestor_record_offset":14028,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1745,"record_offset":15008,"parent_record_offset":14976,"ancestor_record_offset":14028,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1745,"record_offset":14812,"parent_record_offset":14744,"ancestor_record_offset":14028,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1745,"record_offset":14780,"parent_record_offset":14744,"ancestor_record_offset":14028,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1745,"record_offset":14744,"parent_record_offset":14712,"ancestor_record_offset":14028,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1745,"record_offset":14900,"parent_record_offset":14872,"ancestor_record_offset":14028,"inlinee":83844,"source_line":342,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":1745,"record_offset":15380,"parent_record_offset":15340,"ancestor_record_offset":14028,"inlinee":15321,"source_line":252,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":15340,"parent_record_offset":14604,"ancestor_record_offset":14028,"inlinee":15320,"source_line":903,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$95::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":15576,"parent_record_offset":14604,"ancestor_record_offset":14028,"inlinee":15325,"source_line":1920,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1745,"record_offset":14944,"parent_record_offset":14668,"ancestor_record_offset":14028,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1745,"record_offset":14668,"parent_record_offset":14604,"ancestor_record_offset":14028,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1745,"record_offset":15408,"parent_record_offset":15380,"ancestor_record_offset":14028,"inlinee":15322,"source_line":221,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1745,"record_offset":21412,"rva":27725760,"va":5396434880}
// 编译实例 RVAs: 0x1a70fc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140cabef0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14438845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e843ae23ff","size":5,"soft_fail":false,"target_va":5381996272,"va":5396435112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417c0200
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2821929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e8d7f0d4ff","size":5,"soft_fail":false,"target_va":5393613312,"va":5396435236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be10
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20622737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434880,"raw_hex":"e891ad3a01","size":5,"soft_fail":false,"target_va":5417057808,"va":5396435066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1745,"record_offset":21928,"parent_record_offset":21412,"ancestor_record_offset":21412,"inlinee":15334,"source_line":43,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1745,"record_offset":22932,"parent_record_offset":21928,"ancestor_record_offset":21412,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1745,"record_offset":22760,"parent_record_offset":22732,"ancestor_record_offset":21412,"inlinee":15323,"source_line":1919,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1745,"record_offset":22300,"parent_record_offset":22268,"ancestor_record_offset":21412,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1745,"record_offset":22364,"parent_record_offset":22332,"ancestor_record_offset":21412,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1745,"record_offset":22036,"parent_record_offset":21992,"ancestor_record_offset":21412,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1745,"record_offset":22396,"parent_record_offset":22364,"ancestor_record_offset":21412,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1745,"record_offset":22196,"parent_record_offset":22168,"ancestor_record_offset":21412,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1745,"record_offset":22168,"parent_record_offset":22036,"ancestor_record_offset":21412,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1745,"record_offset":22332,"parent_record_offset":22300,"ancestor_record_offset":21412,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1745,"record_offset":22136,"parent_record_offset":22068,"ancestor_record_offset":21412,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1745,"record_offset":22104,"parent_record_offset":22068,"ancestor_record_offset":21412,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1745,"record_offset":22068,"parent_record_offset":22036,"ancestor_record_offset":21412,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1745,"record_offset":22224,"parent_record_offset":22196,"ancestor_record_offset":21412,"inlinee":83844,"source_line":342,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":1745,"record_offset":22704,"parent_record_offset":22664,"ancestor_record_offset":21412,"inlinee":15321,"source_line":252,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":22664,"parent_record_offset":21928,"ancestor_record_offset":21412,"inlinee":15320,"source_line":903,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$95::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":22900,"parent_record_offset":21928,"ancestor_record_offset":21412,"inlinee":15325,"source_line":1920,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1745,"record_offset":22268,"parent_record_offset":21992,"ancestor_record_offset":21412,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1745,"record_offset":21992,"parent_record_offset":21928,"ancestor_record_offset":21412,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1745,"record_offset":22732,"parent_record_offset":22704,"ancestor_record_offset":21412,"inlinee":15322,"source_line":221,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=26
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x141AC
// 首条位置: {"module":1664,"inlinee":82348}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=24
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1664,"record_offset":5464,"parent_record_offset":5432,"ancestor_record_offset":5432,"inlinee":34245,"source_line":221,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells
// 记录形式: inline_instance, inlinee_declaration; instances=18
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3BE6
// 首条位置: {"module":1745,"inlinee":15334}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1745,"record_offset":11752,"parent_record_offset":10748,"ancestor_record_offset":10748,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":11484,"parent_record_offset":10748,"ancestor_record_offset":10748,"inlinee":15320,"source_line":903,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$95::from
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":11720,"parent_record_offset":10748,"ancestor_record_offset":10748,"inlinee":15325,"source_line":1920,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1745,"record_offset":10812,"parent_record_offset":10748,"ancestor_record_offset":10748,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::mul_add
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1745,"record_offset":11580,"parent_record_offset":11552,"ancestor_record_offset":10748,"inlinee":15323,"source_line":1919,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1745,"record_offset":11120,"parent_record_offset":11088,"ancestor_record_offset":10748,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1745,"record_offset":11184,"parent_record_offset":11152,"ancestor_record_offset":10748,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1745,"record_offset":10856,"parent_record_offset":10812,"ancestor_record_offset":10748,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1745,"record_offset":11216,"parent_record_offset":11184,"ancestor_record_offset":10748,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1745,"record_offset":11016,"parent_record_offset":10988,"ancestor_record_offset":10748,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1745,"record_offset":10988,"parent_record_offset":10856,"ancestor_record_offset":10748,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1745,"record_offset":11152,"parent_record_offset":11120,"ancestor_record_offset":10748,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1745,"record_offset":10956,"parent_record_offset":10888,"ancestor_record_offset":10748,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1745,"record_offset":10924,"parent_record_offset":10888,"ancestor_record_offset":10748,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1745,"record_offset":10888,"parent_record_offset":10856,"ancestor_record_offset":10748,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::raster_stroke::impl$2::iter_points_xz::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":1745,"record_offset":16564,"parent_record_offset":16536,"ancestor_record_offset":16268,"inlinee":82267,"source_line":60,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::raster_stroke::impl$3::iter_points_xz::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":1745,"record_offset":11044,"parent_record_offset":11016,"ancestor_record_offset":10748,"inlinee":83843,"source_line":80,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$2
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":1745,"record_offset":12980,"parent_record_offset":12952,"ancestor_record_offset":12684,"inlinee":83844,"source_line":342,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=36
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":1745,"record_offset":11524,"parent_record_offset":11484,"ancestor_record_offset":10748,"inlinee":15321,"source_line":252,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1745,"record_offset":11088,"parent_record_offset":10812,"ancestor_record_offset":10748,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1745,"record_offset":11552,"parent_record_offset":11524,"ancestor_record_offset":10748,"inlinee":15322,"source_line":221,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<false> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":177,"record_offset":40336,"rva":5339472,"va":5374048592}
// 编译实例 RVAs: 0x517950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1404ab6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-443270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374048592,"raw_hex":"e87a3cf9ff","size":5,"soft_fail":false,"target_va":5373605616,"va":5374048881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cabef0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7947502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374048592,"raw_hex":"e8ee447900","size":5,"soft_fail":false,"target_va":5381996272,"va":5374048765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43009081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374048592,"raw_hex":"e839449002","size":5,"soft_fail":false,"target_va":5417057808,"va":5374048722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::array::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":177,"record_offset":40684,"parent_record_offset":40336,"ancestor_record_offset":40336,"inlinee":15318,"source_line":54,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":177,"record_offset":40968,"parent_record_offset":40336,"ancestor_record_offset":40336,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":177,"record_offset":40740,"parent_record_offset":40336,"ancestor_record_offset":40336,"inlinee":15320,"source_line":903,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$95::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":177,"record_offset":40940,"parent_record_offset":40336,"ancestor_record_offset":40336,"inlinee":15325,"source_line":1920,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::new_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":177,"record_offset":40708,"parent_record_offset":40684,"ancestor_record_offset":40336,"inlinee":15319,"source_line":94,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":177,"record_offset":40816,"parent_record_offset":40792,"ancestor_record_offset":40336,"inlinee":15323,"source_line":1919,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":177,"record_offset":40768,"parent_record_offset":40740,"ancestor_record_offset":40336,"inlinee":15321,"source_line":252,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":177,"record_offset":40792,"parent_record_offset":40768,"ancestor_record_offset":40336,"inlinee":15322,"source_line":221,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":177,"record_offset":39416,"rva":5339472,"va":5374048592}
// 编译实例 RVAs: 0x517950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1404ab6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-443270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374048592,"raw_hex":"e87a3cf9ff","size":5,"soft_fail":false,"target_va":5373605616,"va":5374048881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cabef0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7947502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374048592,"raw_hex":"e8ee447900","size":5,"soft_fail":false,"target_va":5381996272,"va":5374048765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43009081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374048592,"raw_hex":"e839449002","size":5,"soft_fail":false,"target_va":5417057808,"va":5374048722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::array::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":177,"record_offset":39764,"parent_record_offset":39416,"ancestor_record_offset":39416,"inlinee":15318,"source_line":54,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":177,"record_offset":40048,"parent_record_offset":39416,"ancestor_record_offset":39416,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":177,"record_offset":39820,"parent_record_offset":39416,"ancestor_record_offset":39416,"inlinee":15320,"source_line":903,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$95::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":177,"record_offset":40020,"parent_record_offset":39416,"ancestor_record_offset":39416,"inlinee":15325,"source_line":1920,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::new_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":177,"record_offset":39788,"parent_record_offset":39764,"ancestor_record_offset":39416,"inlinee":15319,"source_line":94,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":177,"record_offset":39896,"parent_record_offset":39872,"ancestor_record_offset":39416,"inlinee":15323,"source_line":1919,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":177,"record_offset":39848,"parent_record_offset":39820,"ancestor_record_offset":39416,"inlinee":15321,"source_line":252,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":177,"record_offset":39872,"parent_record_offset":39848,"ancestor_record_offset":39416,"inlinee":15322,"source_line":221,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=39
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x39C3
// 首条位置: {"module":160,"inlinee":14787}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure$0
//     owner=private_source_ownership_candidate; count=24
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1664,"record_offset":5432,"parent_record_offset":5392,"ancestor_record_offset":5392,"inlinee":82348,"source_line":37,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure$15
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/playermade_stairs_assemble_visuals.rs
//     evidence={"module":160,"record_offset":5736,"parent_record_offset":5708,"ancestor_record_offset":5708,"inlinee":14788,"source_line":298,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=24
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1664,"record_offset":5464,"parent_record_offset":5432,"ancestor_record_offset":5392,"inlinee":34245,"source_line":221,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":160,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":5708,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":160,"record_offset":5760,"parent_record_offset":5736,"ancestor_record_offset":5708,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1745,"record_offset":7060,"rva":27724480,"va":5396433600}
// 编译实例 RVAs: 0x1a70ac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20623925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396433600,"raw_hex":"e835b23a01","size":5,"soft_fail":false,"target_va":5417057664,"va":5396433734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20624077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396433600,"raw_hex":"e8cdb23a01","size":5,"soft_fail":false,"target_va":5417057792,"va":5396433710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20624098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396433600,"raw_hex":"e8e2b23a01","size":5,"soft_fail":false,"target_va":5417057808,"va":5396433705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20187470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396433600,"raw_hex":"e84e093401","size":5,"soft_fail":false,"target_va":5416621762,"va":5396434287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1745,"record_offset":8416,"parent_record_offset":7060,"ancestor_record_offset":7060,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1745,"record_offset":8536,"parent_record_offset":7060,"ancestor_record_offset":7060,"inlinee":83838,"source_line":129,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":8344,"parent_record_offset":7060,"ancestor_record_offset":7060,"inlinee":8904,"source_line":1088,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::ceil
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":7940,"parent_record_offset":7060,"ancestor_record_offset":7060,"inlinee":38564,"source_line":698,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":8144,"parent_record_offset":7060,"ancestor_record_offset":7060,"inlinee":8903,"source_line":238,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::floor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":7532,"parent_record_offset":7060,"ancestor_record_offset":7060,"inlinee":26740,"source_line":687,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":7328,"parent_record_offset":7060,"ancestor_record_offset":7060,"inlinee":15320,"source_line":903,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1745,"record_offset":8488,"parent_record_offset":8452,"ancestor_record_offset":7060,"inlinee":5308,"source_line":1916,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1745,"record_offset":8020,"parent_record_offset":7996,"ancestor_record_offset":7060,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1745,"record_offset":7612,"parent_record_offset":7588,"ancestor_record_offset":7060,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1745,"record_offset":7408,"parent_record_offset":7384,"ancestor_record_offset":7060,"inlinee":15323,"source_line":1919,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1745,"record_offset":8740,"parent_record_offset":8664,"ancestor_record_offset":7060,"inlinee":5303,"source_line":211,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1745,"record_offset":8452,"parent_record_offset":8416,"ancestor_record_offset":7060,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1745,"record_offset":8768,"parent_record_offset":8740,"ancestor_record_offset":7060,"inlinee":5304,"source_line":832,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::bitmap_intersects
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1745,"record_offset":8588,"parent_record_offset":8536,"ancestor_record_offset":7060,"inlinee":83839,"source_line":238,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::ceil
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":1745,"record_offset":7972,"parent_record_offset":7940,"ancestor_record_offset":7060,"inlinee":38565,"source_line":217,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::floor
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":1745,"record_offset":7564,"parent_record_offset":7532,"ancestor_record_offset":7060,"inlinee":26741,"source_line":222,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":1745,"record_offset":7360,"parent_record_offset":7328,"ancestor_record_offset":7060,"inlinee":15321,"source_line":252,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":8184,"parent_record_offset":8144,"ancestor_record_offset":7060,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":8284,"parent_record_offset":8240,"ancestor_record_offset":7060,"inlinee":8899,"source_line":204,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1745,"record_offset":7996,"parent_record_offset":7972,"ancestor_record_offset":7060,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1745,"record_offset":7588,"parent_record_offset":7564,"ancestor_record_offset":7060,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1745,"record_offset":7384,"parent_record_offset":7360,"ancestor_record_offset":7060,"inlinee":15322,"source_line":221,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1745,"record_offset":8816,"rva":27725200,"va":5396434320}
// 编译实例 RVAs: 0x1a70d90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20623156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434320,"raw_hex":"e834af3a01","size":5,"soft_fail":false,"target_va":5417057664,"va":5396434503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20623354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434320,"raw_hex":"e8faaf3a01","size":5,"soft_fail":false,"target_va":5417057792,"va":5396434433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20623402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434320,"raw_hex":"e82ab03a01","size":5,"soft_fail":false,"target_va":5417057808,"va":5396434401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20186908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396434320,"raw_hex":"e81c073401","size":5,"soft_fail":false,"target_va":5416621762,"va":5396434849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1745,"record_offset":10116,"parent_record_offset":8816,"ancestor_record_offset":8816,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1745,"record_offset":10216,"parent_record_offset":8816,"ancestor_record_offset":8816,"inlinee":83841,"source_line":120,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":10048,"parent_record_offset":8816,"ancestor_record_offset":8816,"inlinee":8904,"source_line":1088,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::ceil
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":9676,"parent_record_offset":8816,"ancestor_record_offset":8816,"inlinee":38564,"source_line":698,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":9876,"parent_record_offset":8816,"ancestor_record_offset":8816,"inlinee":8903,"source_line":238,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::floor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":9280,"parent_record_offset":8816,"ancestor_record_offset":8816,"inlinee":26740,"source_line":687,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":9084,"parent_record_offset":8816,"ancestor_record_offset":8816,"inlinee":15320,"source_line":903,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1745,"record_offset":10172,"parent_record_offset":10144,"ancestor_record_offset":8816,"inlinee":5308,"source_line":1916,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1745,"record_offset":9752,"parent_record_offset":9728,"ancestor_record_offset":8816,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1745,"record_offset":9352,"parent_record_offset":9328,"ancestor_record_offset":8816,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1745,"record_offset":9156,"parent_record_offset":9132,"ancestor_record_offset":8816,"inlinee":15323,"source_line":1919,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1745,"record_offset":10332,"parent_record_offset":10304,"ancestor_record_offset":8816,"inlinee":5303,"source_line":211,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1745,"record_offset":10144,"parent_record_offset":10116,"ancestor_record_offset":8816,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1745,"record_offset":10356,"parent_record_offset":10332,"ancestor_record_offset":8816,"inlinee":5304,"source_line":832,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1745,"record_offset":10244,"parent_record_offset":10216,"ancestor_record_offset":8816,"inlinee":34245,"source_line":221,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::ceil
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":1745,"record_offset":9704,"parent_record_offset":9676,"ancestor_record_offset":8816,"inlinee":38565,"source_line":217,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::floor
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":1745,"record_offset":9304,"parent_record_offset":9280,"ancestor_record_offset":8816,"inlinee":26741,"source_line":222,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":1745,"record_offset":9108,"parent_record_offset":9084,"ancestor_record_offset":8816,"inlinee":15321,"source_line":252,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":9912,"parent_record_offset":9876,"ancestor_record_offset":8816,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1745,"record_offset":10000,"parent_record_offset":9964,"ancestor_record_offset":8816,"inlinee":8899,"source_line":204,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1745,"record_offset":9728,"parent_record_offset":9704,"ancestor_record_offset":8816,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1745,"record_offset":9328,"parent_record_offset":9304,"ancestor_record_offset":8816,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1745,"record_offset":9132,"parent_record_offset":9108,"ancestor_record_offset":8816,"inlinee":15322,"source_line":221,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::autoclutter::impl$1::add_region_to_lazy_grid::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2755,"record_offset":6436,"rva":35425056,"va":5404134176}
// 编译实例 RVAs: 0x21c8b20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12215947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404134176,"raw_hex":"e88b66ba00","size":5,"soft_fail":false,"target_va":5416350720,"va":5404134768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12923284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404134176,"raw_hex":"e89431c500","size":5,"soft_fail":false,"target_va":5417057664,"va":5404134375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12923482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404134176,"raw_hex":"e85a32c500","size":5,"soft_fail":false,"target_va":5417057792,"va":5404134305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12923530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404134176,"raw_hex":"e88a32c500","size":5,"soft_fail":false,"target_va":5417057808,"va":5404134273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12486909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404134176,"raw_hex":"e8fd88be00","size":5,"soft_fail":false,"target_va":5416621762,"va":5404134848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2755,"record_offset":7728,"parent_record_offset":6436,"ancestor_record_offset":6436,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":7660,"parent_record_offset":6436,"ancestor_record_offset":6436,"inlinee":8904,"source_line":1088,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::ceil
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":7288,"parent_record_offset":6436,"ancestor_record_offset":6436,"inlinee":38564,"source_line":698,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":7488,"parent_record_offset":6436,"ancestor_record_offset":6436,"inlinee":8903,"source_line":238,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::floor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":6892,"parent_record_offset":6436,"ancestor_record_offset":6436,"inlinee":26740,"source_line":687,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":6696,"parent_record_offset":6436,"ancestor_record_offset":6436,"inlinee":15320,"source_line":903,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::impl$1::add_region_to_lazy_grid::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":2755,"record_offset":7976,"parent_record_offset":6436,"ancestor_record_offset":6436,"inlinee":108537,"source_line":259,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2755,"record_offset":8100,"parent_record_offset":8076,"ancestor_record_offset":6436,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2755,"record_offset":8148,"parent_record_offset":8124,"ancestor_record_offset":6436,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2755,"record_offset":8124,"parent_record_offset":8100,"ancestor_record_offset":6436,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2755,"record_offset":8076,"parent_record_offset":8036,"ancestor_record_offset":6436,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2755,"record_offset":8008,"parent_record_offset":7976,"ancestor_record_offset":6436,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2755,"record_offset":8036,"parent_record_offset":8008,"ancestor_record_offset":6436,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2755,"record_offset":7788,"parent_record_offset":7756,"ancestor_record_offset":6436,"inlinee":5308,"source_line":1916,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2755,"record_offset":7364,"parent_record_offset":7340,"ancestor_record_offset":6436,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2755,"record_offset":6964,"parent_record_offset":6940,"ancestor_record_offset":6436,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2755,"record_offset":6768,"parent_record_offset":6744,"ancestor_record_offset":6436,"inlinee":15323,"source_line":1919,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2755,"record_offset":7820,"parent_record_offset":7756,"ancestor_record_offset":6436,"inlinee":5303,"source_line":211,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2755,"record_offset":7756,"parent_record_offset":7728,"ancestor_record_offset":6436,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2755,"record_offset":7844,"parent_record_offset":7820,"ancestor_record_offset":6436,"inlinee":5304,"source_line":832,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2755,"record_offset":8188,"parent_record_offset":8036,"ancestor_record_offset":6436,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::ceil
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":2755,"record_offset":7316,"parent_record_offset":7288,"ancestor_record_offset":6436,"inlinee":38565,"source_line":217,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::floor
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":2755,"record_offset":6916,"parent_record_offset":6892,"ancestor_record_offset":6436,"inlinee":26741,"source_line":222,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":2755,"record_offset":6720,"parent_record_offset":6696,"ancestor_record_offset":6436,"inlinee":15321,"source_line":252,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":7524,"parent_record_offset":7488,"ancestor_record_offset":6436,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":7612,"parent_record_offset":7576,"ancestor_record_offset":6436,"inlinee":8899,"source_line":204,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2755,"record_offset":7340,"parent_record_offset":7316,"ancestor_record_offset":6436,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2755,"record_offset":6940,"parent_record_offset":6916,"ancestor_record_offset":6436,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2755,"record_offset":6744,"parent_record_offset":6720,"ancestor_record_offset":6436,"inlinee":15322,"source_line":221,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::autoclutter::impl$1::remove_region_from_lazy_grid::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2755,"record_offset":4320,"rva":35424288,"va":5404133408}
// 编译实例 RVAs: 0x21c8820
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12924055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404133408,"raw_hex":"e89734c500","size":5,"soft_fail":false,"target_va":5417057664,"va":5404133604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12924253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404133408,"raw_hex":"e85d35c500","size":5,"soft_fail":false,"target_va":5417057792,"va":5404133534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12924301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404133408,"raw_hex":"e88d35c500","size":5,"soft_fail":false,"target_va":5417057808,"va":5404133502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12487594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404133408,"raw_hex":"e8aa8bbe00","size":5,"soft_fail":false,"target_va":5416621762,"va":5404134163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2755,"record_offset":5616,"parent_record_offset":4320,"ancestor_record_offset":4320,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":5548,"parent_record_offset":4320,"ancestor_record_offset":4320,"inlinee":8904,"source_line":1088,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::ceil
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":5176,"parent_record_offset":4320,"ancestor_record_offset":4320,"inlinee":38564,"source_line":698,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":5376,"parent_record_offset":4320,"ancestor_record_offset":4320,"inlinee":8903,"source_line":238,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::floor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":4780,"parent_record_offset":4320,"ancestor_record_offset":4320,"inlinee":26740,"source_line":687,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":4584,"parent_record_offset":4320,"ancestor_record_offset":4320,"inlinee":15320,"source_line":903,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::impl$1::remove_region_from_lazy_grid::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":2755,"record_offset":5864,"parent_record_offset":4320,"ancestor_record_offset":4320,"inlinee":108533,"source_line":274,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2755,"record_offset":6044,"parent_record_offset":5932,"ancestor_record_offset":4320,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2755,"record_offset":5904,"parent_record_offset":5864,"ancestor_record_offset":4320,"inlinee":11071,"source_line":2425,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2755,"record_offset":5932,"parent_record_offset":5904,"ancestor_record_offset":4320,"inlinee":10770,"source_line":2451,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2755,"record_offset":6016,"parent_record_offset":5932,"ancestor_record_offset":4320,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2755,"record_offset":6072,"parent_record_offset":5932,"ancestor_record_offset":4320,"inlinee":10772,"source_line":2429,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2755,"record_offset":6120,"parent_record_offset":6096,"ancestor_record_offset":4320,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2755,"record_offset":5676,"parent_record_offset":5644,"ancestor_record_offset":4320,"inlinee":5308,"source_line":1916,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2755,"record_offset":5252,"parent_record_offset":5228,"ancestor_record_offset":4320,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2755,"record_offset":4852,"parent_record_offset":4828,"ancestor_record_offset":4320,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2755,"record_offset":4656,"parent_record_offset":4632,"ancestor_record_offset":4320,"inlinee":15323,"source_line":1919,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":2755,"record_offset":6184,"parent_record_offset":5932,"ancestor_record_offset":4320,"inlinee":10761,"source_line":713,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2755,"record_offset":6208,"parent_record_offset":6184,"ancestor_record_offset":4320,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2755,"record_offset":5708,"parent_record_offset":5644,"ancestor_record_offset":4320,"inlinee":5303,"source_line":211,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2755,"record_offset":5644,"parent_record_offset":5616,"ancestor_record_offset":4320,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2755,"record_offset":5732,"parent_record_offset":5708,"ancestor_record_offset":4320,"inlinee":5304,"source_line":832,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2755,"record_offset":6384,"parent_record_offset":5932,"ancestor_record_offset":4320,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::ceil
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":2755,"record_offset":5204,"parent_record_offset":5176,"ancestor_record_offset":4320,"inlinee":38565,"source_line":217,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::floor
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":2755,"record_offset":4804,"parent_record_offset":4780,"ancestor_record_offset":4320,"inlinee":26741,"source_line":222,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":2755,"record_offset":4608,"parent_record_offset":4584,"ancestor_record_offset":4320,"inlinee":15321,"source_line":252,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":5412,"parent_record_offset":5376,"ancestor_record_offset":4320,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":5500,"parent_record_offset":5464,"ancestor_record_offset":4320,"inlinee":8899,"source_line":204,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2755,"record_offset":5228,"parent_record_offset":5204,"ancestor_record_offset":4320,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2755,"record_offset":4828,"parent_record_offset":4804,"ancestor_record_offset":4320,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2755,"record_offset":4632,"parent_record_offset":4608,"ancestor_record_offset":4320,"inlinee":15322,"source_line":221,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::autoclutter::impl$1::remove_region_from_lazy_grid::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":2755,"record_offset":6096,"parent_record_offset":6072,"ancestor_record_offset":4320,"inlinee":108535,"source_line":278,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::autoclutter::impl$24::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":2755,"record_offset":6144,"parent_record_offset":6120,"ancestor_record_offset":4320,"inlinee":67214,"source_line":322,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::autoclutter::read_bush_positions_from_gpu::impl$0::bushes_near_point::closure_env$0<system_clutter::autoclutter::populate_autoclutter_regions::filter_clutter_object::closure_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2755,"record_offset":2208,"rva":35423552,"va":5404132672}
// 编译实例 RVAs: 0x21c8540
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140965cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25569848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404132672,"raw_hex":"e8c8d579fe","size":5,"soft_fail":false,"target_va":5378563312,"va":5404133155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12924788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404132672,"raw_hex":"e87437c500","size":5,"soft_fail":false,"target_va":5417057664,"va":5404132871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12924986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404132672,"raw_hex":"e83a38c500","size":5,"soft_fail":false,"target_va":5417057792,"va":5404132801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12925034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404132672,"raw_hex":"e86a38c500","size":5,"soft_fail":false,"target_va":5417057808,"va":5404132769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12488361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404132672,"raw_hex":"e8a98ebe00","size":5,"soft_fail":false,"target_va":5416621762,"va":5404133396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2755,"record_offset":3620,"parent_record_offset":2208,"ancestor_record_offset":2208,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":3552,"parent_record_offset":2208,"ancestor_record_offset":2208,"inlinee":8904,"source_line":1088,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::ceil
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":3180,"parent_record_offset":2208,"ancestor_record_offset":2208,"inlinee":38564,"source_line":698,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":3380,"parent_record_offset":2208,"ancestor_record_offset":2208,"inlinee":8903,"source_line":238,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::floor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":2784,"parent_record_offset":2208,"ancestor_record_offset":2208,"inlinee":26740,"source_line":687,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":2588,"parent_record_offset":2208,"ancestor_record_offset":2208,"inlinee":15320,"source_line":903,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::read_bush_positions_from_gpu::impl$0::bushes_near_point::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/read_bush_positions_from_gpu.rs
//     evidence={"module":2755,"record_offset":3896,"parent_record_offset":2208,"ancestor_record_offset":2208,"inlinee":108529,"source_line":50,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2755,"record_offset":3940,"parent_record_offset":3896,"ancestor_record_offset":2208,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2755,"record_offset":3700,"parent_record_offset":3656,"ancestor_record_offset":2208,"inlinee":5308,"source_line":1916,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2755,"record_offset":3256,"parent_record_offset":3232,"ancestor_record_offset":2208,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2755,"record_offset":2856,"parent_record_offset":2832,"ancestor_record_offset":2208,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2755,"record_offset":2660,"parent_record_offset":2636,"ancestor_record_offset":2208,"inlinee":15323,"source_line":1919,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2755,"record_offset":4264,"parent_record_offset":4240,"ancestor_record_offset":2208,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2755,"record_offset":3740,"parent_record_offset":3656,"ancestor_record_offset":2208,"inlinee":5303,"source_line":211,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2755,"record_offset":3656,"parent_record_offset":3620,"ancestor_record_offset":2208,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2755,"record_offset":3764,"parent_record_offset":3740,"ancestor_record_offset":2208,"inlinee":5304,"source_line":832,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2755,"record_offset":4180,"parent_record_offset":4152,"ancestor_record_offset":2208,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2755,"record_offset":4060,"parent_record_offset":4036,"ancestor_record_offset":2208,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2755,"record_offset":3968,"parent_record_offset":3940,"ancestor_record_offset":2208,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2755,"record_offset":3996,"parent_record_offset":3968,"ancestor_record_offset":2208,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2755,"record_offset":4036,"parent_record_offset":3896,"ancestor_record_offset":2208,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::ceil
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":2755,"record_offset":3208,"parent_record_offset":3180,"ancestor_record_offset":2208,"inlinee":38565,"source_line":217,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::floor
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":2755,"record_offset":2808,"parent_record_offset":2784,"ancestor_record_offset":2208,"inlinee":26741,"source_line":222,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":2755,"record_offset":2612,"parent_record_offset":2588,"ancestor_record_offset":2208,"inlinee":15321,"source_line":252,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":4128,"parent_record_offset":4092,"ancestor_record_offset":2208,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":4208,"parent_record_offset":4152,"ancestor_record_offset":2208,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":3416,"parent_record_offset":3380,"ancestor_record_offset":2208,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":3504,"parent_record_offset":3468,"ancestor_record_offset":2208,"inlinee":8899,"source_line":204,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":4152,"parent_record_offset":4128,"ancestor_record_offset":2208,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2755,"record_offset":3232,"parent_record_offset":3208,"ancestor_record_offset":2208,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2755,"record_offset":2832,"parent_record_offset":2808,"ancestor_record_offset":2208,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2755,"record_offset":2636,"parent_record_offset":2612,"ancestor_record_offset":2208,"inlinee":15322,"source_line":221,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2755,"record_offset":4240,"parent_record_offset":4208,"ancestor_record_offset":2208,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::autoclutter::populate_autoclutter_regions::filter_clutter_object::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/populate_autoclutter_regions.rs
//     evidence={"module":2755,"record_offset":4092,"parent_record_offset":3896,"ancestor_record_offset":2208,"inlinee":108531,"source_line":748,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2755,"record_offset":8232,"rva":35425744,"va":5404134864}
// 编译实例 RVAs: 0x21c8dd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140965cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25572096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404134864,"raw_hex":"e800cd79fe","size":5,"soft_fail":false,"target_va":5378563312,"va":5404135403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12922596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404134864,"raw_hex":"e8e42ec500","size":5,"soft_fail":false,"target_va":5417057664,"va":5404135063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12922794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404134864,"raw_hex":"e8aa2fc500","size":5,"soft_fail":false,"target_va":5417057792,"va":5404134993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12922842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404134864,"raw_hex":"e8da2fc500","size":5,"soft_fail":false,"target_va":5417057808,"va":5404134961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12244782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404134864,"raw_hex":"e82ed7ba00","size":5,"soft_fail":false,"target_va":5416380224,"va":5404135437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12486246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404134864,"raw_hex":"e86686be00","size":5,"soft_fail":false,"target_va":5416621762,"va":5404135511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2755,"record_offset":9552,"parent_record_offset":8232,"ancestor_record_offset":8232,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":9484,"parent_record_offset":8232,"ancestor_record_offset":8232,"inlinee":8904,"source_line":1088,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::ceil
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":9112,"parent_record_offset":8232,"ancestor_record_offset":8232,"inlinee":38564,"source_line":698,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":9312,"parent_record_offset":8232,"ancestor_record_offset":8232,"inlinee":8903,"source_line":238,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::floor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":8716,"parent_record_offset":8232,"ancestor_record_offset":8232,"inlinee":26740,"source_line":687,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":8520,"parent_record_offset":8232,"ancestor_record_offset":8232,"inlinee":15320,"source_line":903,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/read_bush_positions_from_gpu.rs
//     evidence={"module":2755,"record_offset":9820,"parent_record_offset":8232,"ancestor_record_offset":8232,"inlinee":108538,"source_line":147,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2755,"record_offset":9952,"parent_record_offset":9928,"ancestor_record_offset":8232,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2755,"record_offset":10000,"parent_record_offset":9976,"ancestor_record_offset":8232,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2755,"record_offset":9976,"parent_record_offset":9952,"ancestor_record_offset":8232,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2755,"record_offset":9928,"parent_record_offset":9888,"ancestor_record_offset":8232,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2755,"record_offset":9860,"parent_record_offset":9820,"ancestor_record_offset":8232,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2755,"record_offset":9888,"parent_record_offset":9860,"ancestor_record_offset":8232,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2755,"record_offset":10076,"parent_record_offset":9820,"ancestor_record_offset":8232,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2755,"record_offset":9620,"parent_record_offset":9584,"ancestor_record_offset":8232,"inlinee":5308,"source_line":1916,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2755,"record_offset":9188,"parent_record_offset":9164,"ancestor_record_offset":8232,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2755,"record_offset":8788,"parent_record_offset":8764,"ancestor_record_offset":8232,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2755,"record_offset":8592,"parent_record_offset":8568,"ancestor_record_offset":8232,"inlinee":15323,"source_line":1919,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2755,"record_offset":9656,"parent_record_offset":9584,"ancestor_record_offset":8232,"inlinee":5303,"source_line":211,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2755,"record_offset":9584,"parent_record_offset":9552,"ancestor_record_offset":8232,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2755,"record_offset":9680,"parent_record_offset":9656,"ancestor_record_offset":8232,"inlinee":5304,"source_line":832,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2755,"record_offset":10040,"parent_record_offset":9888,"ancestor_record_offset":8232,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2755,"record_offset":10104,"parent_record_offset":10076,"ancestor_record_offset":8232,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2755,"record_offset":10132,"parent_record_offset":10104,"ancestor_record_offset":8232,"inlinee":8559,"source_line":276,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::ceil
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":2755,"record_offset":9140,"parent_record_offset":9112,"ancestor_record_offset":8232,"inlinee":38565,"source_line":217,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::floor
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":2755,"record_offset":8740,"parent_record_offset":8716,"ancestor_record_offset":8232,"inlinee":26741,"source_line":222,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":2755,"record_offset":8544,"parent_record_offset":8520,"ancestor_record_offset":8232,"inlinee":15321,"source_line":252,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":9348,"parent_record_offset":9312,"ancestor_record_offset":8232,"inlinee":8902,"source_line":219,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":9436,"parent_record_offset":9400,"ancestor_record_offset":8232,"inlinee":8899,"source_line":204,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2755,"record_offset":9164,"parent_record_offset":9140,"ancestor_record_offset":8232,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2755,"record_offset":8764,"parent_record_offset":8740,"ancestor_record_offset":8232,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2755,"record_offset":8568,"parent_record_offset":8544,"ancestor_record_offset":8232,"inlinee":15322,"source_line":221,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_rect
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 119
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7A1B
// 首条位置: {"module":527,"inlinee":31259}
// 编译实例 RVAs: 0xa59c10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x141a70d90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t16871794","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379562512,"raw_hex":"e972710101","size":5,"soft_fail":false,"target_va":5396434320,"va":5379562521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14781
// 首条位置: {"module":1745,"inlinee":83841}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1745,"record_offset":10244,"parent_record_offset":10216,"ancestor_record_offset":10216,"inlinee":34245,"source_line":221,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::intersects_world_space_rect
// 记录形式: compiled_procedure; instances=1
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":538,"record_offset":9388,"rva":10853408,"va":5379562528}
// 编译实例 RVAs: 0xa59c20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141a70ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16871044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379562528,"raw_hex":"e8846e0101","size":5,"soft_fail":false,"target_va":5396433600,"va":5379562551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1477E
// 首条位置: {"module":1745,"inlinee":83838}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::bitmap_intersects
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1745,"record_offset":8588,"parent_record_offset":8536,"ancestor_record_offset":8536,"inlinee":83839,"source_line":238,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::cell_from_world_space
// 记录形式: compiled_procedure; instances=1
// 源码行: 141
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":538,"record_offset":8852,"rva":10853184,"va":5379562304}
// 编译实例 RVAs: 0xa59b40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37495439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379562304,"raw_hex":"e88f223c02","size":5,"soft_fail":false,"target_va":5417057808,"va":5379562364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_ivec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":538,"record_offset":9208,"parent_record_offset":8852,"ancestor_record_offset":8852,"inlinee":26742,"source_line":1081,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":538,"record_offset":9008,"parent_record_offset":8852,"ancestor_record_offset":8852,"inlinee":15320,"source_line":903,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::mul_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":538,"record_offset":9084,"parent_record_offset":9060,"ancestor_record_offset":8852,"inlinee":15323,"source_line":1919,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":538,"record_offset":9036,"parent_record_offset":9008,"ancestor_record_offset":8852,"inlinee":15321,"source_line":252,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":538,"record_offset":9060,"parent_record_offset":9036,"ancestor_record_offset":8852,"inlinee":15322,"source_line":221,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::clear
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=50
// 源码行: 156, 157
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x760B
// 首条位置: {"module":517,"inlinee":30219}
// 编译实例 RVAs: 0xa59c50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t37494499","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379562576,"raw_hex":"e9e31e3c02","size":5,"soft_fail":false,"target_va":5417057088,"va":5379562584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::iter_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 160
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1745,"record_offset":4136,"rva":27723728,"va":5396432848}
// 编译实例 RVAs: 0x1a707d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14040ebd0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23469693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396432848,"raw_hex":"e883e199fe","size":5,"soft_fail":false,"target_va":5372963792,"va":5396433480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22954091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396432848,"raw_hex":"e895bfa1fe","size":5,"soft_fail":false,"target_va":5373479040,"va":5396433126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20188624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396432848,"raw_hex":"e8d00d3401","size":5,"soft_fail":false,"target_va":5416622208,"va":5396433579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20188166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396432848,"raw_hex":"e8060c3401","size":5,"soft_fail":false,"target_va":5416621762,"va":5396433591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1745,"record_offset":4396,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1745,"record_offset":4536,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":8995,"source_line":177,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/publish_public_wall_state.rs
//     evidence={"module":1745,"record_offset":4564,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":83834,"source_line":249,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1745,"record_offset":4660,"parent_record_offset":4632,"ancestor_record_offset":4136,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1745,"record_offset":5636,"parent_record_offset":5612,"ancestor_record_offset":4136,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1745,"record_offset":5204,"parent_record_offset":5180,"ancestor_record_offset":4136,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1745,"record_offset":5288,"parent_record_offset":5264,"ancestor_record_offset":4136,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1745,"record_offset":5704,"parent_record_offset":5068,"ancestor_record_offset":4136,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1745,"record_offset":4420,"parent_record_offset":4396,"ancestor_record_offset":4136,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1745,"record_offset":5436,"parent_record_offset":5408,"ancestor_record_offset":4136,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1745,"record_offset":5408,"parent_record_offset":5380,"ancestor_record_offset":4136,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1745,"record_offset":4500,"parent_record_offset":4420,"ancestor_record_offset":4136,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1745,"record_offset":6940,"parent_record_offset":4564,"ancestor_record_offset":4136,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1745,"record_offset":5228,"parent_record_offset":5204,"ancestor_record_offset":4136,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1745,"record_offset":5552,"parent_record_offset":5528,"ancestor_record_offset":4136,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1745,"record_offset":4468,"parent_record_offset":4444,"ancestor_record_offset":4136,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1745,"record_offset":4692,"parent_record_offset":4660,"ancestor_record_offset":4136,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1745,"record_offset":4724,"parent_record_offset":4692,"ancestor_record_offset":4136,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1745,"record_offset":4444,"parent_record_offset":4420,"ancestor_record_offset":4136,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1745,"record_offset":5660,"parent_record_offset":5636,"ancestor_record_offset":4136,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":1745,"record_offset":4864,"parent_record_offset":4564,"ancestor_record_offset":4136,"inlinee":29283,"source_line":128,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":1745,"record_offset":5936,"parent_record_offset":4564,"ancestor_record_offset":4136,"inlinee":28986,"source_line":38,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/publish_public_wall_state.rs
//     evidence={"module":1745,"record_offset":6964,"parent_record_offset":6940,"ancestor_record_offset":4136,"inlinee":83836,"source_line":257,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::GridCell::walls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":1745,"record_offset":4772,"parent_record_offset":4564,"ancestor_record_offset":4136,"inlinee":34310,"source_line":84,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cell_at_index
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":1745,"record_offset":4632,"parent_record_offset":4564,"ancestor_record_offset":4136,"inlinee":34305,"source_line":534,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1745,"record_offset":5348,"parent_record_offset":5320,"ancestor_record_offset":4136,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1745,"record_offset":5380,"parent_record_offset":5348,"ancestor_record_offset":4136,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1745,"record_offset":5320,"parent_record_offset":5068,"ancestor_record_offset":4136,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1745,"record_offset":5180,"parent_record_offset":5068,"ancestor_record_offset":4136,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1745,"record_offset":5732,"parent_record_offset":5068,"ancestor_record_offset":4136,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1745,"record_offset":5264,"parent_record_offset":5068,"ancestor_record_offset":4136,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1745,"record_offset":5124,"parent_record_offset":5068,"ancestor_record_offset":4136,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1745,"record_offset":5612,"parent_record_offset":5588,"ancestor_record_offset":4136,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1745,"record_offset":4944,"parent_record_offset":4904,"ancestor_record_offset":4136,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1745,"record_offset":5588,"parent_record_offset":5480,"ancestor_record_offset":4136,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1745,"record_offset":4992,"parent_record_offset":4944,"ancestor_record_offset":4136,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1745,"record_offset":5528,"parent_record_offset":5504,"ancestor_record_offset":4136,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1745,"record_offset":5824,"parent_record_offset":5068,"ancestor_record_offset":4136,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1745,"record_offset":5504,"parent_record_offset":5480,"ancestor_record_offset":4136,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1745,"record_offset":5044,"parent_record_offset":5020,"ancestor_record_offset":4136,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1745,"record_offset":5020,"parent_record_offset":4944,"ancestor_record_offset":4136,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1745,"record_offset":5868,"parent_record_offset":4944,"ancestor_record_offset":4136,"inlinee":9370,"source_line":1370,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1745,"record_offset":5892,"parent_record_offset":5868,"ancestor_record_offset":4136,"inlinee":9371,"source_line":1364,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1745,"record_offset":5068,"parent_record_offset":5044,"ancestor_record_offset":4136,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1745,"record_offset":5152,"parent_record_offset":5068,"ancestor_record_offset":4136,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1745,"record_offset":5480,"parent_record_offset":5068,"ancestor_record_offset":4136,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1745,"record_offset":7020,"parent_record_offset":6996,"ancestor_record_offset":4136,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1745,"record_offset":4904,"parent_record_offset":4864,"ancestor_record_offset":4136,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1745,"record_offset":6996,"parent_record_offset":4564,"ancestor_record_offset":4136,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1745,"record_offset":6908,"parent_record_offset":5936,"ancestor_record_offset":4136,"inlinee":28987,"source_line":165,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::iter_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 160
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1745,"record_offset":268,"rva":27722608,"va":5396431728}
// 编译实例 RVAs: 0x1a70370
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14040ebd0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23468744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396431728,"raw_hex":"e838e599fe","size":5,"soft_fail":false,"target_va":5372963792,"va":5396432531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=3
//     target_aliases=6 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22953100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396431728,"raw_hex":"e874c3a1fe","size":5,"soft_fail":false,"target_va":5373479040,"va":5396432135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20189387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396431728,"raw_hex":"e8cb103401","size":5,"soft_fail":false,"target_va":5416622208,"va":5396432816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20188929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396431728,"raw_hex":"e8010f3401","size":5,"soft_fail":false,"target_va":5416621762,"va":5396432828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1745,"record_offset":528,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1745,"record_offset":668,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8995,"source_line":177,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/publish_public_wall_state.rs
//     evidence={"module":1745,"record_offset":696,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":83833,"source_line":274,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1745,"record_offset":812,"parent_record_offset":784,"ancestor_record_offset":268,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1745,"record_offset":1740,"parent_record_offset":1716,"ancestor_record_offset":268,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1745,"record_offset":1308,"parent_record_offset":1284,"ancestor_record_offset":268,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1745,"record_offset":1392,"parent_record_offset":1368,"ancestor_record_offset":268,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1745,"record_offset":1808,"parent_record_offset":1228,"ancestor_record_offset":268,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1745,"record_offset":552,"parent_record_offset":528,"ancestor_record_offset":268,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1745,"record_offset":1540,"parent_record_offset":1512,"ancestor_record_offset":268,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1745,"record_offset":1512,"parent_record_offset":1484,"ancestor_record_offset":268,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1745,"record_offset":632,"parent_record_offset":552,"ancestor_record_offset":268,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1745,"record_offset":3076,"parent_record_offset":696,"ancestor_record_offset":268,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1745,"record_offset":1332,"parent_record_offset":1308,"ancestor_record_offset":268,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1745,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":268,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1745,"record_offset":1036,"parent_record_offset":952,"ancestor_record_offset":268,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1745,"record_offset":600,"parent_record_offset":576,"ancestor_record_offset":268,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1745,"record_offset":844,"parent_record_offset":812,"ancestor_record_offset":268,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1745,"record_offset":876,"parent_record_offset":844,"ancestor_record_offset":268,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1745,"record_offset":576,"parent_record_offset":552,"ancestor_record_offset":268,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1745,"record_offset":1764,"parent_record_offset":1740,"ancestor_record_offset":268,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":1745,"record_offset":1068,"parent_record_offset":696,"ancestor_record_offset":268,"inlinee":29283,"source_line":128,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":1745,"record_offset":2068,"parent_record_offset":696,"ancestor_record_offset":268,"inlinee":28986,"source_line":38,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::GridCell::walls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":1745,"record_offset":924,"parent_record_offset":696,"ancestor_record_offset":268,"inlinee":34310,"source_line":84,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cell_at_index
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":1745,"record_offset":784,"parent_record_offset":696,"ancestor_record_offset":268,"inlinee":34305,"source_line":534,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1745,"record_offset":1452,"parent_record_offset":1424,"ancestor_record_offset":268,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1745,"record_offset":1484,"parent_record_offset":1452,"ancestor_record_offset":268,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1745,"record_offset":1424,"parent_record_offset":1228,"ancestor_record_offset":268,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1745,"record_offset":1284,"parent_record_offset":1228,"ancestor_record_offset":268,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1745,"record_offset":1836,"parent_record_offset":1228,"ancestor_record_offset":268,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1745,"record_offset":1368,"parent_record_offset":1228,"ancestor_record_offset":268,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1745,"record_offset":1960,"parent_record_offset":1228,"ancestor_record_offset":268,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1745,"record_offset":1716,"parent_record_offset":1692,"ancestor_record_offset":268,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1745,"record_offset":3152,"parent_record_offset":3128,"ancestor_record_offset":268,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1745,"record_offset":1132,"parent_record_offset":1100,"ancestor_record_offset":268,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1745,"record_offset":1692,"parent_record_offset":1584,"ancestor_record_offset":268,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1745,"record_offset":2028,"parent_record_offset":1132,"ancestor_record_offset":268,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1745,"record_offset":1632,"parent_record_offset":1608,"ancestor_record_offset":268,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1745,"record_offset":1928,"parent_record_offset":1228,"ancestor_record_offset":268,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1745,"record_offset":1608,"parent_record_offset":1584,"ancestor_record_offset":268,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1745,"record_offset":1200,"parent_record_offset":1172,"ancestor_record_offset":268,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1745,"record_offset":1172,"parent_record_offset":1132,"ancestor_record_offset":268,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1745,"record_offset":1228,"parent_record_offset":1200,"ancestor_record_offset":268,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1745,"record_offset":1988,"parent_record_offset":1228,"ancestor_record_offset":268,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1745,"record_offset":1584,"parent_record_offset":1228,"ancestor_record_offset":268,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1745,"record_offset":3128,"parent_record_offset":3104,"ancestor_record_offset":268,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1745,"record_offset":4096,"parent_record_offset":4072,"ancestor_record_offset":268,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1745,"record_offset":1100,"parent_record_offset":1068,"ancestor_record_offset":268,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1745,"record_offset":3104,"parent_record_offset":696,"ancestor_record_offset":268,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1745,"record_offset":4072,"parent_record_offset":696,"ancestor_record_offset":268,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1745,"record_offset":3044,"parent_record_offset":2068,"ancestor_record_offset":268,"inlinee":28987,"source_line":165,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::iter_tiles<system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 160
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2755,"record_offset":268,"rva":35423008,"va":5404132128}
// 编译实例 RVAs: 0x21c8320
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::raw_vec::RawVec<u16>>::grow_one
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t12485835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404132128,"raw_hex":"e8cb84be00","size":5,"soft_fail":false,"target_va":5416618384,"va":5404132544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140965cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25568903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404132128,"raw_hex":"e879d979fe","size":5,"soft_fail":false,"target_va":5378563312,"va":5404132210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14218ff60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-230553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404132128,"raw_hex":"e8677bfcff","size":5,"soft_fail":false,"target_va":5403901792,"va":5404132340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12488950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404132128,"raw_hex":"e8f690be00","size":5,"soft_fail":false,"target_va":5416621600,"va":5404132645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12489100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404132128,"raw_hex":"e88c91be00","size":5,"soft_fail":false,"target_va":5416621762,"va":5404132657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":2755,"record_offset":700,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2755,"record_offset":524,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2755,"record_offset":672,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8995,"source_line":177,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/populate_autoclutter_regions.rs
//     evidence={"module":2755,"record_offset":728,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":108527,"source_line":156,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2755,"record_offset":1080,"parent_record_offset":1056,"ancestor_record_offset":268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2755,"record_offset":1128,"parent_record_offset":1104,"ancestor_record_offset":268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2755,"record_offset":1104,"parent_record_offset":1080,"ancestor_record_offset":268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2755,"record_offset":1056,"parent_record_offset":1016,"ancestor_record_offset":268,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2755,"record_offset":988,"parent_record_offset":756,"ancestor_record_offset":268,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2755,"record_offset":1016,"parent_record_offset":988,"ancestor_record_offset":268,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2755,"record_offset":816,"parent_record_offset":756,"ancestor_record_offset":268,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2755,"record_offset":1900,"parent_record_offset":1876,"ancestor_record_offset":268,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2755,"record_offset":1468,"parent_record_offset":1444,"ancestor_record_offset":268,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2755,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":268,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2755,"record_offset":1968,"parent_record_offset":1332,"ancestor_record_offset":268,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":2755,"record_offset":552,"parent_record_offset":524,"ancestor_record_offset":268,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2755,"record_offset":1700,"parent_record_offset":1672,"ancestor_record_offset":268,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2755,"record_offset":1672,"parent_record_offset":1644,"ancestor_record_offset":268,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2755,"record_offset":580,"parent_record_offset":552,"ancestor_record_offset":268,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2755,"record_offset":2168,"parent_record_offset":756,"ancestor_record_offset":268,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2755,"record_offset":1492,"parent_record_offset":1468,"ancestor_record_offset":268,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2755,"record_offset":1816,"parent_record_offset":1792,"ancestor_record_offset":268,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2755,"record_offset":632,"parent_record_offset":608,"ancestor_record_offset":268,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2755,"record_offset":1168,"parent_record_offset":1016,"ancestor_record_offset":268,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2755,"record_offset":848,"parent_record_offset":816,"ancestor_record_offset":268,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2755,"record_offset":880,"parent_record_offset":848,"ancestor_record_offset":268,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2755,"record_offset":608,"parent_record_offset":552,"ancestor_record_offset":268,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2755,"record_offset":1612,"parent_record_offset":1584,"ancestor_record_offset":268,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2755,"record_offset":1644,"parent_record_offset":1612,"ancestor_record_offset":268,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2755,"record_offset":1584,"parent_record_offset":1332,"ancestor_record_offset":268,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2755,"record_offset":1444,"parent_record_offset":1332,"ancestor_record_offset":268,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2755,"record_offset":1996,"parent_record_offset":1332,"ancestor_record_offset":268,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2755,"record_offset":1528,"parent_record_offset":1332,"ancestor_record_offset":268,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":2755,"record_offset":1388,"parent_record_offset":1332,"ancestor_record_offset":268,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":2755,"record_offset":1876,"parent_record_offset":1852,"ancestor_record_offset":268,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2755,"record_offset":1228,"parent_record_offset":1204,"ancestor_record_offset":268,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2755,"record_offset":1852,"parent_record_offset":1744,"ancestor_record_offset":268,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2755,"record_offset":1256,"parent_record_offset":1228,"ancestor_record_offset":268,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2755,"record_offset":1792,"parent_record_offset":1768,"ancestor_record_offset":268,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2755,"record_offset":2088,"parent_record_offset":1332,"ancestor_record_offset":268,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2755,"record_offset":1768,"parent_record_offset":1744,"ancestor_record_offset":268,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2755,"record_offset":1308,"parent_record_offset":1284,"ancestor_record_offset":268,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2755,"record_offset":1284,"parent_record_offset":1228,"ancestor_record_offset":268,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2755,"record_offset":1332,"parent_record_offset":1308,"ancestor_record_offset":268,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2755,"record_offset":1416,"parent_record_offset":1332,"ancestor_record_offset":268,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2755,"record_offset":1744,"parent_record_offset":1332,"ancestor_record_offset":268,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2755,"record_offset":1204,"parent_record_offset":756,"ancestor_record_offset":268,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::autoclutter::AutoClutterState::mark_dirty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":2755,"record_offset":756,"parent_record_offset":728,"ancestor_record_offset":268,"inlinee":67243,"source_line":214,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::autoclutter::ClutterCandidateIndex::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":2755,"record_offset":2140,"parent_record_offset":756,"ancestor_record_offset":268,"inlinee":67227,"source_line":347,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::autoclutter::impl$24::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":2755,"record_offset":1924,"parent_record_offset":1900,"ancestor_record_offset":268,"inlinee":67214,"source_line":322,"archive":"_pdb/C/modules/02755.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::iter_tiles<system_decorator::decorator_visual::playermade_stairs::stairs_lazy_update::playermade_stairs_assembly_propagate_dirty_flags::closure_env$2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 160
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1602,"record_offset":272,"rva":21957632,"va":5390666752}
// 编译实例 RVAs: 0x14f0c00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c9e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17187797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390666752,"raw_hex":"e82bbcf9fe","size":5,"soft_fail":false,"target_va":5373479392,"va":5390667184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140965cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12103593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390666752,"raw_hex":"e8575047ff","size":5,"soft_fail":false,"target_va":5378563312,"va":5390666900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1602,"record_offset":804,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1602,"record_offset":568,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1602,"record_offset":776,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8995,"source_line":177,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1602,"record_offset":748,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::playermade_stairs::stairs_lazy_update::playermade_stairs_assembly_propagate_dirty_flags::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/stairs_lazy_update.rs
//     evidence={"module":1602,"record_offset":832,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":80733,"source_line":365,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1602,"record_offset":932,"parent_record_offset":904,"ancestor_record_offset":272,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1602,"record_offset":904,"parent_record_offset":832,"ancestor_record_offset":272,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":1602,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":272,"inlinee":10467,"source_line":10,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":1602,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":272,"inlinee":10469,"source_line":153,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1602,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":272,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1602,"record_offset":1208,"parent_record_offset":1184,"ancestor_record_offset":272,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1602,"record_offset":2044,"parent_record_offset":2020,"ancestor_record_offset":272,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1602,"record_offset":2128,"parent_record_offset":2104,"ancestor_record_offset":272,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1602,"record_offset":2572,"parent_record_offset":1964,"ancestor_record_offset":272,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1602,"record_offset":596,"parent_record_offset":568,"ancestor_record_offset":272,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1602,"record_offset":1556,"parent_record_offset":1532,"ancestor_record_offset":272,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1602,"record_offset":1532,"parent_record_offset":1508,"ancestor_record_offset":272,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1602,"record_offset":712,"parent_record_offset":596,"ancestor_record_offset":272,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1602,"record_offset":1240,"parent_record_offset":1156,"ancestor_record_offset":272,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1602,"record_offset":2068,"parent_record_offset":2044,"ancestor_record_offset":272,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1602,"record_offset":1680,"parent_record_offset":1656,"ancestor_record_offset":272,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1602,"record_offset":652,"parent_record_offset":624,"ancestor_record_offset":272,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1602,"record_offset":680,"parent_record_offset":624,"ancestor_record_offset":272,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1602,"record_offset":624,"parent_record_offset":596,"ancestor_record_offset":272,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] country_core::LazyTile::y
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/lib.rs
//     evidence={"module":1602,"record_offset":876,"parent_record_offset":832,"ancestor_record_offset":272,"inlinee":80734,"source_line":80,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1602,"record_offset":1476,"parent_record_offset":1448,"ancestor_record_offset":272,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1602,"record_offset":1508,"parent_record_offset":1476,"ancestor_record_offset":272,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1602,"record_offset":1448,"parent_record_offset":1408,"ancestor_record_offset":272,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1602,"record_offset":2020,"parent_record_offset":1964,"ancestor_record_offset":272,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1602,"record_offset":1184,"parent_record_offset":1156,"ancestor_record_offset":272,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1602,"record_offset":2600,"parent_record_offset":1964,"ancestor_record_offset":272,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1602,"record_offset":2104,"parent_record_offset":1964,"ancestor_record_offset":272,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1602,"record_offset":2724,"parent_record_offset":1964,"ancestor_record_offset":272,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1602,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":272,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1602,"record_offset":1872,"parent_record_offset":1844,"ancestor_record_offset":272,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1602,"record_offset":1028,"parent_record_offset":996,"ancestor_record_offset":272,"inlinee":20113,"source_line":799,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1602,"record_offset":2428,"parent_record_offset":2320,"ancestor_record_offset":272,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$48::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1602,"record_offset":1348,"parent_record_offset":1320,"ancestor_record_offset":272,"inlinee":13411,"source_line":3267,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1602,"record_offset":2792,"parent_record_offset":1872,"ancestor_record_offset":272,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1602,"record_offset":2368,"parent_record_offset":2344,"ancestor_record_offset":272,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1602,"record_offset":1656,"parent_record_offset":1408,"ancestor_record_offset":272,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1602,"record_offset":2692,"parent_record_offset":1964,"ancestor_record_offset":272,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1602,"record_offset":1156,"parent_record_offset":1092,"ancestor_record_offset":272,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1602,"record_offset":1408,"parent_record_offset":1376,"ancestor_record_offset":272,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1602,"record_offset":2344,"parent_record_offset":2320,"ancestor_record_offset":272,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1602,"record_offset":1936,"parent_record_offset":1908,"ancestor_record_offset":272,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1602,"record_offset":1908,"parent_record_offset":1872,"ancestor_record_offset":272,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1602,"record_offset":1060,"parent_record_offset":1028,"ancestor_record_offset":272,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1602,"record_offset":1128,"parent_record_offset":1092,"ancestor_record_offset":272,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1602,"record_offset":1964,"parent_record_offset":1936,"ancestor_record_offset":272,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1602,"record_offset":1092,"parent_record_offset":1060,"ancestor_record_offset":272,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1602,"record_offset":2752,"parent_record_offset":1964,"ancestor_record_offset":272,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1602,"record_offset":1376,"parent_record_offset":1348,"ancestor_record_offset":272,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1602,"record_offset":2320,"parent_record_offset":1964,"ancestor_record_offset":272,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1602,"record_offset":1844,"parent_record_offset":1816,"ancestor_record_offset":272,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1602,"record_offset":996,"parent_record_offset":964,"ancestor_record_offset":272,"inlinee":20112,"source_line":673,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1602,"record_offset":964,"parent_record_offset":832,"ancestor_record_offset":272,"inlinee":20111,"source_line":576,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$41::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1602,"record_offset":1320,"parent_record_offset":1292,"ancestor_record_offset":272,"inlinee":21911,"source_line":2113,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$55::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1602,"record_offset":1292,"parent_record_offset":832,"ancestor_record_offset":272,"inlinee":21910,"source_line":2271,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1602,"record_offset":1816,"parent_record_offset":832,"ancestor_record_offset":272,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/01602.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$3::copy_to_staging::closure_env$0<512> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 172
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1745,"record_offset":25208,"rva":27726464,"va":5396435584}
// 编译实例 RVAs: 0x1a71280, 0x2afa550
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140965cf0
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35211606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413774672,"raw_hex":"e8aab6e6fd","size":5,"soft_fail":false,"target_va":5378563312,"va":5413774913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20184627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396435584,"raw_hex":"e833fe3301","size":5,"soft_fail":false,"target_va":5416621762,"va":5396437130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::copy_from_slice_impl::len_mismatch_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20186277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396435584,"raw_hex":"e8a5043401","size":5,"soft_fail":false,"target_va":5416622768,"va":5396436486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=22
//     evidence={"assembly":"\tcallq\t20185849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396435584,"raw_hex":"e8f9023401","size":5,"soft_fail":false,"target_va":5416622320,"va":5396436466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::world_raster::build_tile_hasher
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32124402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413774672,"raw_hex":"e80ed215fe","size":5,"soft_fail":false,"target_va":5381650752,"va":5413775149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":4729,"record_offset":2448,"parent_record_offset":1932,"ancestor_record_offset":1932,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/04729.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1745,"record_offset":25440,"parent_record_offset":25208,"ancestor_record_offset":25208,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1745,"record_offset":25656,"parent_record_offset":25208,"ancestor_record_offset":25208,"inlinee":8995,"source_line":177,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$3::copy_to_staging::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1745,"record_offset":25720,"parent_record_offset":25208,"ancestor_record_offset":25208,"inlinee":83846,"source_line":639,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] ahash::convert::impl$0::read_last_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/convert.rs
//     evidence={"module":1745,"record_offset":27940,"parent_record_offset":27900,"ancestor_record_offset":25208,"inlinee":15801,"source_line":134,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] ahash::convert::impl$0::read_u128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/convert.rs
//     evidence={"module":1745,"record_offset":27600,"parent_record_offset":27392,"ancestor_record_offset":25208,"inlinee":15797,"source_line":110,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] ahash::convert::impl$0::read_u16
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/convert.rs
//     evidence={"module":4729,"record_offset":5384,"parent_record_offset":5284,"ancestor_record_offset":1932,"inlinee":15802,"source_line":92,"archive":"_pdb/C/modules/04729.jsonl.gz"}
//   [pdb_inline_transitive] ahash::convert::impl$0::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/convert.rs
//     evidence={"module":4729,"record_offset":5328,"parent_record_offset":5284,"ancestor_record_offset":1932,"inlinee":15800,"source_line":98,"archive":"_pdb/C/modules/04729.jsonl.gz"}
//   [pdb_inline_transitive] ahash::fallback_hash::AHasher::large_update
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/fallback_hash.rs
//     evidence={"module":1745,"record_offset":27452,"parent_record_offset":27392,"ancestor_record_offset":25208,"inlinee":15795,"source_line":111,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] ahash::fallback_hash::impl$1::finish
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/fallback_hash.rs
//     evidence={"module":1745,"record_offset":28292,"parent_record_offset":25720,"ancestor_record_offset":25208,"inlinee":6068,"source_line":196,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] ahash::fallback_hash::impl$1::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/fallback_hash.rs
//     evidence={"module":1745,"record_offset":27392,"parent_record_offset":25720,"ancestor_record_offset":25208,"inlinee":15794,"source_line":171,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] ahash::operations::folded_multiply
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/operations.rs
//     evidence={"module":1745,"record_offset":27480,"parent_record_offset":27452,"ancestor_record_offset":25208,"inlinee":6062,"source_line":17,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] ahash::operations::read_small
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/operations.rs
//     evidence={"module":1745,"record_offset":27900,"parent_record_offset":27392,"ancestor_record_offset":25208,"inlinee":15799,"source_line":33,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1745,"record_offset":28416,"parent_record_offset":25720,"ancestor_record_offset":25208,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1745,"record_offset":27564,"parent_record_offset":27540,"ancestor_record_offset":25208,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1745,"record_offset":27300,"parent_record_offset":27276,"ancestor_record_offset":25208,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1745,"record_offset":27276,"parent_record_offset":25820,"ancestor_record_offset":25208,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::wrapping_mul
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1745,"record_offset":27508,"parent_record_offset":27480,"ancestor_record_offset":25208,"inlinee":6064,"source_line":2532,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1745,"record_offset":26368,"parent_record_offset":26272,"ancestor_record_offset":25208,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1745,"record_offset":27540,"parent_record_offset":27452,"ancestor_record_offset":25208,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1745,"record_offset":27832,"parent_record_offset":27740,"ancestor_record_offset":25208,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1745,"record_offset":27696,"parent_record_offset":27672,"ancestor_record_offset":25208,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1745,"record_offset":26608,"parent_record_offset":26552,"ancestor_record_offset":25208,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1745,"record_offset":25620,"parent_record_offset":25496,"ancestor_record_offset":25208,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1745,"record_offset":25560,"parent_record_offset":25496,"ancestor_record_offset":25208,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1745,"record_offset":26552,"parent_record_offset":26468,"ancestor_record_offset":25208,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1745,"record_offset":26468,"parent_record_offset":25820,"ancestor_record_offset":25208,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1745,"record_offset":27624,"parent_record_offset":27600,"ancestor_record_offset":25208,"inlinee":15798,"source_line":1952,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1745,"record_offset":27648,"parent_record_offset":27624,"ancestor_record_offset":25208,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1745,"record_offset":27672,"parent_record_offset":27648,"ancestor_record_offset":25208,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1745,"record_offset":26036,"parent_record_offset":25820,"ancestor_record_offset":25208,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1745,"record_offset":27332,"parent_record_offset":25820,"ancestor_record_offset":25208,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1745,"record_offset":28472,"parent_record_offset":28444,"ancestor_record_offset":25208,"inlinee":8559,"source_line":276,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1745,"record_offset":26272,"parent_record_offset":26176,"ancestor_record_offset":25208,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1745,"record_offset":27356,"parent_record_offset":27332,"ancestor_record_offset":25208,"inlinee":8427,"source_line":448,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1745,"record_offset":26100,"parent_record_offset":26036,"ancestor_record_offset":25208,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1745,"record_offset":25496,"parent_record_offset":25440,"ancestor_record_offset":25208,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::WorldRaster::prepare_tile
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1745,"record_offset":25820,"parent_record_offset":25720,"ancestor_record_offset":25208,"inlinee":38841,"source_line":255,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::extend
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=57
// 源码行: 189, 192
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8110
// 首条位置: {"module":538,"inlinee":33040}
// 编译实例 RVAs: 0xa59c60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::set
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 204
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x85C4
// 首条位置: {"module":550,"inlinee":34244}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":550,"record_offset":34440,"parent_record_offset":34408,"ancestor_record_offset":34408,"inlinee":34245,"source_line":221,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::is_set
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 215
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20FA5
// 首条位置: {"module":4056,"inlinee":135077}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::bitmap_intersects
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":4056,"record_offset":5060,"parent_record_offset":5028,"ancestor_record_offset":5028,"inlinee":83839,"source_line":238,"archive":"_pdb/C/modules/04056.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::bitmap_set
// 记录形式: inline_instance, inlinee_declaration; instances=35
// 源码行: 221
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x85C5
// 首条位置: {"module":550,"inlinee":34245}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::bitmap_intersects
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1477F
// 首条位置: {"module":1745,"inlinee":83839}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::dilate
// 记录形式: inlinee_declaration; instances=1
// 源码行: 243
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x80C9
// 首条位置: {"module":538,"inlinee":32969}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::impl$2::apply_to
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 310, 311
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x815B
// 首条位置: {"module":538,"record_offset":112120,"rva":10892528,"va":5379601648}
// 编译实例 RVAs: 0xa634f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":538,"record_offset":112256,"parent_record_offset":112120,"ancestor_record_offset":112120,"inlinee":33040,"source_line":189,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::impl$3::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 320
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":519,"record_offset":19168,"rva":10248752,"va":5378957872}
// 编译实例 RVAs: 0x9c6230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1409aa660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-113816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378957872,"raw_hex":"e86843feff","size":5,"soft_fail":false,"target_va":5378844256,"va":5378958067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409c8230
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378957872,"raw_hex":"e8de1f0000","size":5,"soft_fail":false,"target_va":5378966064,"va":5378957901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c90c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2927048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378957872,"raw_hex":"e8c8a92c00","size":5,"soft_fail":false,"target_va":5381885072,"va":5378958019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a707d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17474907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378957872,"raw_hex":"e85ba50a01","size":5,"soft_fail":false,"target_va":5396432848,"va":5378957936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37614104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378957872,"raw_hex":"e818f23d02","size":5,"soft_fail":false,"target_va":5416572144,"va":5378958035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5027406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378957872,"raw_hex":"e84eb64c00","size":5,"soft_fail":false,"target_va":5383985360,"va":5378957949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":519,"record_offset":19424,"parent_record_offset":19168,"ancestor_record_offset":19168,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":19784,"parent_record_offset":19448,"ancestor_record_offset":19168,"inlinee":4851,"source_line":1644,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":19808,"parent_record_offset":19784,"ancestor_record_offset":19168,"inlinee":4852,"source_line":1888,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":19920,"parent_record_offset":19896,"ancestor_record_offset":19168,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":519,"record_offset":19484,"parent_record_offset":19448,"ancestor_record_offset":19168,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":519,"record_offset":19756,"parent_record_offset":19448,"ancestor_record_offset":19168,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":519,"record_offset":19832,"parent_record_offset":19808,"ancestor_record_offset":19168,"inlinee":4853,"source_line":993,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":519,"record_offset":19856,"parent_record_offset":19832,"ancestor_record_offset":19168,"inlinee":4854,"source_line":935,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":519,"record_offset":19512,"parent_record_offset":19484,"ancestor_record_offset":19168,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":19684,"parent_record_offset":19564,"ancestor_record_offset":19168,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":19708,"parent_record_offset":19684,"ancestor_record_offset":19168,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":20012,"parent_record_offset":19920,"ancestor_record_offset":19168,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$46::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":519,"record_offset":19536,"parent_record_offset":19512,"ancestor_record_offset":19168,"inlinee":14416,"source_line":2924,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":519,"record_offset":19600,"parent_record_offset":19564,"ancestor_record_offset":19168,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":519,"record_offset":19564,"parent_record_offset":19536,"ancestor_record_offset":19168,"inlinee":12295,"source_line":129,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":519,"record_offset":19624,"parent_record_offset":19600,"ancestor_record_offset":19168,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] tabloid::impl$2::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tabloid","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tabloid/src/lib.rs
//     evidence={"module":519,"record_offset":19448,"parent_record_offset":19424,"ancestor_record_offset":19168,"inlinee":26559,"source_line":147,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::impl$3::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 320
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":519,"record_offset":20060,"rva":10248960,"va":5378958080}
// 编译实例 RVAs: 0x9c6300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1409aa660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-114024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378958080,"raw_hex":"e89842feff","size":5,"soft_fail":false,"target_va":5378844256,"va":5378958275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409c8230
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378958080,"raw_hex":"e80e1f0000","size":5,"soft_fail":false,"target_va":5378966064,"va":5378958109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c90c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2926840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378958080,"raw_hex":"e8f8a82c00","size":5,"soft_fail":false,"target_va":5381885072,"va":5378958227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a70370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17473579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378958080,"raw_hex":"e82ba00a01","size":5,"soft_fail":false,"target_va":5396431728,"va":5378958144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37613896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378958080,"raw_hex":"e848f13d02","size":5,"soft_fail":false,"target_va":5416572144,"va":5378958243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5027198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378958080,"raw_hex":"e87eb54c00","size":5,"soft_fail":false,"target_va":5383985360,"va":5378958157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":519,"record_offset":20316,"parent_record_offset":20060,"ancestor_record_offset":20060,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":20676,"parent_record_offset":20340,"ancestor_record_offset":20060,"inlinee":4851,"source_line":1644,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":20700,"parent_record_offset":20676,"ancestor_record_offset":20060,"inlinee":4852,"source_line":1888,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":519,"record_offset":20812,"parent_record_offset":20788,"ancestor_record_offset":20060,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":519,"record_offset":20376,"parent_record_offset":20340,"ancestor_record_offset":20060,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":519,"record_offset":20648,"parent_record_offset":20340,"ancestor_record_offset":20060,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":519,"record_offset":20724,"parent_record_offset":20700,"ancestor_record_offset":20060,"inlinee":4853,"source_line":993,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":519,"record_offset":20748,"parent_record_offset":20724,"ancestor_record_offset":20060,"inlinee":4854,"source_line":935,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":519,"record_offset":20404,"parent_record_offset":20376,"ancestor_record_offset":20060,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":20576,"parent_record_offset":20456,"ancestor_record_offset":20060,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":20600,"parent_record_offset":20576,"ancestor_record_offset":20060,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":519,"record_offset":20904,"parent_record_offset":20812,"ancestor_record_offset":20060,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$46::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":519,"record_offset":20428,"parent_record_offset":20404,"ancestor_record_offset":20060,"inlinee":14416,"source_line":2924,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":519,"record_offset":20492,"parent_record_offset":20456,"ancestor_record_offset":20060,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":519,"record_offset":20456,"parent_record_offset":20428,"ancestor_record_offset":20060,"inlinee":12295,"source_line":129,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":519,"record_offset":20516,"parent_record_offset":20492,"ancestor_record_offset":20060,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] tabloid::impl$2::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tabloid","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tabloid/src/lib.rs
//     evidence={"module":519,"record_offset":20340,"parent_record_offset":20316,"ancestor_record_offset":20060,"inlinee":26559,"source_line":147,"archive":"_pdb/C/modules/00519.jsonl.gz"}
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
// 完整结构及原始类型记录: dirty_tiles.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::utils::dirty_tiles::DirtyTiles
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 16; 过程引用数: 89
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Subscriber<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Subscriber", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tabloid::Subscriber<country_core::utils::dirty_tiles::DirtyTiles>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tabloid::Subscriber<country_core::utils::dirty_tiles::DirtyTiles>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tabloid::Subscriber<country_core::utils::dirty_tiles::DirtyTiles>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tabloid::Subscriber<country_core::utils::dirty_tiles::DirtyTiles>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Subscription", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Publisher", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells", "arguments": ["array$<glam::f32::vec2::Vec2,2>", "system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<false> >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells", "arguments": ["array$<glam::f32::vec2::Vec2,2>", "system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<false>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::ArcInner<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> > >", "constructor": "alloc::sync::ArcInner", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::ArcInner<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::ArcInner<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Publication", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "lock_api::rwlock::RwLock", "arguments": ["parking_lot::raw_rwlock::RawRwLock", "tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::PublicationReader<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::PublicationReader", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tabloid::PublicationReader<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tabloid::PublicationReader<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Publisher::publish", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles", "country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publisher::subscribe<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Publisher::subscribe", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Subscriber::take<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Subscriber::take", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::iter_tiles<system_decorator::decorator_visual::playermade_stairs::stairs_lazy_update::playermade_stairs_assembly_propagate_dirty_flags::closure_env$2>", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::iter_tiles", "arguments": ["system_decorator::decorator_visual::playermade_stairs::stairs_lazy_update::playermade_stairs_assembly_propagate_dirty_flags::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::rwlock::RwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "lock_api::rwlock::RwLockWriteGuard", "arguments": ["parking_lot::raw_rwlock::RawRwLock", "tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::rwlock::RwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::rwlock::RwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::impl$1::publish::closure_env$0", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles", "country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "alloc::vec::Vec::retain", "arguments": ["alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "alloc::alloc::Global", "tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::iter_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1>", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::iter_tiles", "arguments": ["country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::iter_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0>", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::iter_tiles", "arguments": ["country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure_env$0>", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells", "arguments": ["country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure_env$0>", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells", "arguments": ["country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$3::copy_to_staging::closure_env$0<512> >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::retain", "arguments": ["country_core::resources::world_raster::impl$3::copy_to_staging::closure_env$0<512>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$1::retain::closure$0<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "alloc::vec::impl$1::retain::closure$0", "arguments": ["alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "alloc::alloc::Global", "tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::iter_tiles<system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions::closure_env$0>", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::iter_tiles", "arguments": ["system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::autoclutter::read_bush_positions_from_gpu::impl$0::bushes_near_point::closure_env$0<system_clutter::autoclutter::populate_autoclutter_regions::filter_clutter_object::closure_env$0> >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells", "arguments": ["system_clutter::autoclutter::read_bush_positions_from_gpu::impl$0::bushes_near_point::closure_env$0<system_clutter::autoclutter::populate_autoclutter_regions::filter_clutter_object::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::autoclutter::impl$1::remove_region_from_lazy_grid::closure_env$0>", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells", "arguments": ["system_clutter::autoclutter::impl$1::remove_region_from_lazy_grid::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::autoclutter::impl$1::add_region_to_lazy_grid::closure_env$0>", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells", "arguments": ["system_clutter::autoclutter::impl$1::add_region_to_lazy_grid::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu::closure_env$1>", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells", "arguments": ["system_clutter::autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: dirty_tiles.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: dirty_tiles.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 32; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::icc::IccProfileChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::icc::IccProfileChanged>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::icc::IccProfileChanged>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>
// [文件级接口候选，具体函数归属 unknown] core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> > (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] core::cell::Cell<tuple$<u64,u64> > (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] crossbeam_epoch::collector::LocalHandle (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] alloc::sync::Arc<bevy_tasks::thread_executor::ThreadExecutor,alloc::alloc::Global> (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] arc_swap::debt::list::LocalNode (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] core::cell::RefCell<array$<alloc::vec::Vec<bvh::bvh::bvh_node::ShapeIndex,alloc::alloc::Global>,6> > (*)()
// 候选参数类型: []
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::cell_from_world_space
// 信息 ID: d56c83a42f8cefc6bc30e37f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::add_birdnests::add_birdnests @ 0x28c7997 -> 0xa59b40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::clear
// 信息 ID: fd9eca83bdaa9f79fcd15797
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 44 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3,system_water::draw_water::draw_water::closure_env$4> @ 0x2859369 -> 0xa59c50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3,system_water::draw_water::draw_water::closure_env$4> @ 0x285990d -> 0xa59c50 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu; module=517, inline_record=51404; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks; module=517, inline_record=80108; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks; module=517, inline_record=80224; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks; module=517, inline_record=80340; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks; module=517, inline_record=80456; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_from_cpu; module=560, inline_record=11084; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >; module=571, inline_record=4512; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >; module=571, inline_record=5856; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::extend
// 信息 ID: 34ee05baa7b6eef8ec4a160b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 51 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x8381cc -> 0xa59c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::stairs_lazy_update::playermade_stairs_assembly_propagate_dirty_flags @ 0x891da9 -> 0xa59c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3,system_water::draw_water::draw_water::closure_env$4> @ 0x2859396 -> 0xa59c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3,system_water::draw_water::draw_water::closure_env$4> @ 0x28593a5 -> 0xa59c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3,system_water::draw_water::draw_water::closure_env$4> @ 0x2859905 -> 0xa59c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bc53d -> 0xa59c60 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::utils::dirty_tiles::impl$2::apply_to; module=538, inline_record=112256; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >; module=571, inline_record=6076; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >; module=571, inline_record=14112; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >; module=571, inline_record=14168; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >; module=571, inline_record=22252; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >; module=571, inline_record=30312; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >; module=571, inline_record=30368; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=571, inline_record=38364; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::intersects_world_space_rect
// 信息 ID: e8e216f3b7840dade4370fd6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x8387b5 -> 0xa59c20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x8388b3 -> 0xa59c20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::is_set
// 信息 ID: 77fa21ca568ca57c6529971c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::autoclutter::add_birdnests::add_birdnests; module=4056, inline_record=5028; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::iter_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0>
// 信息 ID: be11f0ee81f9c9aaf858c872
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::iter_tiles -> ["country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::dirty_tiles::impl$3::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0> @ 0x9c6270 -> 0x1a707d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::iter_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1>
// 信息 ID: 806eb12a1f98da7114bce11b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::iter_tiles -> ["country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::dirty_tiles::impl$3::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1> @ 0x9c6340 -> 0x1a70370 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::iter_tiles<system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions::closure_env$0>
// 信息 ID: 83d48b43ef0af470fd50cc6f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::iter_tiles -> ["system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bc5c9 -> 0x21c8320 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::iter_tiles<system_decorator::decorator_visual::playermade_stairs::stairs_lazy_update::playermade_stairs_assembly_propagate_dirty_flags::closure_env$2>
// 信息 ID: 48b3ce5b39be49dfde3ba091
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::iter_tiles -> ["system_decorator::decorator_visual::playermade_stairs::stairs_lazy_update::playermade_stairs_assembly_propagate_dirty_flags::closure_env$2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::stairs_lazy_update::playermade_stairs_assembly_propagate_dirty_flags @ 0x891e3c -> 0x14f0c00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure_env$0>
// 信息 ID: 5067192643654d38953d276d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells -> ["country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::intersects_world_space_rect @ 0xa59c37 -> 0x1a70ac0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure_env$0>
// 信息 ID: 6f09fa838dfab30e25603511
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells -> ["country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::dispose_collider_mesh_instance @ 0xa01e93 -> 0x1a70d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_rect @ 0xa59c19 -> 0x1a70d90 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::upsert_collider_entry<bevy_ecs::entity::Entity,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0xc79961 -> 0x1a70d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::upsert_collider_entry<bevy_ecs::entity::Entity,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0xc79d94 -> 0x1a70d90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::autoclutter::impl$1::add_region_to_lazy_grid::closure_env$0>
// 信息 ID: e3bdb74a580cf602fd6786ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells -> ["system_clutter::autoclutter::impl$1::add_region_to_lazy_grid::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::AutoClutterState::add_region @ 0x10d479b -> 0x21c8b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::AutoClutterState::add_region @ 0x10d4600 -> 0x21c8b20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::autoclutter::impl$1::remove_region_from_lazy_grid::closure_env$0>
// 信息 ID: a8897d1bcd319de6ec2e6e4e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells -> ["system_clutter::autoclutter::impl$1::remove_region_from_lazy_grid::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::AutoClutterState::add_region @ 0x10d4527 -> 0x21c8820 (direct_call); 同地址别名未裁定。
// [静态直接引用] hashbrown::map::HashMap::retain<system_clutter::autoclutter::ClutterRegionId,system_clutter::autoclutter::AutoClutterRegion,std::hash::random::RandomState,alloc::alloc::Global,system_clutter::autoclutter::impl$1::remove_stale_regions::closure_env$0> @ 0x21becb9 -> 0x21c8820 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::autoclutter::read_bush_positions_from_gpu::impl$0::bushes_near_point::closure_env$0<system_clutter::autoclutter::populate_autoclutter_regions::filter_clutter_object::closure_env$0> >
// 信息 ID: 80edec2059d347486157ef56
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_clutter::autoclutter::read_bush_positions_from_gpu::impl$0::bushes_near_point::closure_env$0 -> ["system_clutter::autoclutter::populate_autoclutter_regions::filter_clutter_object::closure_env$0"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells -> ["system_clutter::autoclutter::read_bush_positions_from_gpu::impl$0::bushes_near_point::closure_env$0<system_clutter::autoclutter::populate_autoclutter_regions::filter_clutter_object::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::read_bush_positions_from_gpu::BushGrid::bushes_near_point<system_clutter::autoclutter::populate_autoclutter_regions::filter_clutter_object::closure_env$0> @ 0x10d29ce -> 0x21c8540 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu::closure_env$1>
// 信息 ID: c59b7fa8e2087863f61878c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells -> ["system_clutter::autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu @ 0x21c8185 -> 0x21c8dd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells
// 信息 ID: 91e6bc26b7daca26ee1ec015
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> >; module=1745, inline_record=10748; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > >; module=1745, inline_record=12684; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > >; module=1745, inline_record=14604; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >; module=1745, inline_record=16268; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > >; module=1745, inline_record=18132; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > > >; module=1745, inline_record=20068; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> > >; module=1745, inline_record=21928; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > > >; module=1745, inline_record=23864; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<false> >
// 信息 ID: 009a595ff3042276d511b570
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["glam::f32::vec2::Vec2", "2"]; 字段布局 = unknown。
// [类型关系] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15 -> ["false"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells -> ["array$<glam::f32::vec2::Vec2,2>", "system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<false>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x367ad2 -> 0x517950 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37a755 -> 0x517950 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> >
// 信息 ID: c75e4d89eb65875128a7da2d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["glam::f32::vec2::Vec2", "2"]; 字段布局 = unknown。
// [类型关系] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15 -> ["true"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells -> ["array$<glam::f32::vec2::Vec2,2>", "system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x367ad2 -> 0x517950 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37a755 -> 0x517950 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::new_clear
// 信息 ID: d920ab7ce446fbf544cf6778
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x838131 -> 0xa59f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::stairs_lazy_update::playermade_stairs_assembly_propagate_dirty_flags @ 0x891d26 -> 0xa59f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu @ 0x21c81ba -> 0xa59f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bc4af -> 0xa59f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_birdnests::add_birdnests @ 0x28c7770 -> 0xa59f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> | tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> @ 0x28d48c5 -> 0xa59f20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_rect
// 信息 ID: f5dd6e5e5563f2d7f03bc74a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 14 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0x30953b -> 0xa59c10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0x309a58 -> 0xa59c10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0x1417b9b -> 0xa59c10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0x14180b8 -> 0xa59c10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::WallId,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0x1418ddc -> 0xa59c10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::WallId,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0x14192fd -> 0xa59c10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::decorator::DecoratorId,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0x14f28bd -> 0xa59c10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::decorator::DecoratorId,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0x14f2ddb -> 0xa59c10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::stairs::stairs_state::StairGraphEdge,utils::IsoTransform> @ 0x14f3b2b -> 0xa59c10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::stairs::stairs_state::StairGraphEdge,utils::IsoTransform> @ 0x14f4048 -> 0xa59c10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::decorator::DecoratorId,country_core::systems::collision::world::OffsetColliderTransform> @ 0x14f4d5d -> 0xa59c10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::decorator::DecoratorId,country_core::systems::collision::world::OffsetColliderTransform> @ 0x14f527b -> 0xa59c10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,utils::IsoTransform> @ 0x14f5f05 -> 0xa59c10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,utils::IsoTransform> @ 0x14f640a -> 0xa59c10 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::collision::world::dispose_collider_mesh_instance; module=527, inline_record=125252; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::world::upsert_collider_entry<bevy_ecs::entity::Entity,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> >; module=607, inline_record=122412; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::world::upsert_collider_entry<bevy_ecs::entity::Entity,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> >; module=607, inline_record=126112; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >
// 信息 ID: 8a47b5ddd397d39e8a6e3ab8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 29 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb64a9f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb6528e -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb65517 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb6726b -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb67994 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb67cee -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb69a0d -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6a14b -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6a49d -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6c2ef -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6cade -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6cd67 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6eab2 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6f1fc -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6f54f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0xb7133f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> >
// 信息 ID: 05f2d90b79c6960e01b68762
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 28 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb64a9f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb6528e -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb65517 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb6726b -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb67994 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb67cee -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb69a0d -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6a14b -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6a49d -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6c2ef -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6cade -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6cd67 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6eab2 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6f1fc -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6f54f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0xb7133f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > > >
// 信息 ID: 5d78aff9fb7346f85b15aa9d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_path::replay_path::closure_env$1 -> ["country_core::systems::history::replay_path::Cobblestone1PathFilter"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster::closure_env$2 -> ["country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> >"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 28 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb64a9f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb6528e -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb65517 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb6726b -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb67994 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb67cee -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb69a0d -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6a14b -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6a49d -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6c2ef -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6cade -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6cd67 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6eab2 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6f1fc -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6f54f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0xb7133f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > >
// 信息 ID: ac75af91e93e2a598336e756
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_path::replay_path::closure_env$1 -> ["country_core::systems::history::replay_path::Cobblestone2PathFilter"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster::closure_env$2 -> ["country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 28 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb64a9f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb6528e -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb65517 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb6726b -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb67994 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb67cee -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb69a0d -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6a14b -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6a49d -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6c2ef -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6cade -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6cd67 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6eab2 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6f1fc -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6f54f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0xb7133f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > > >
// 信息 ID: e0a4d6b89b6e62970141984c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_path::replay_path::closure_env$1 -> ["country_core::systems::history::replay_path::Cobblestone3PathFilter"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster::closure_env$2 -> ["country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 28 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb64a9f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb6528e -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb65517 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb6726b -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb67994 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb67cee -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb69a0d -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6a14b -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6a49d -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6c2ef -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6cade -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6cd67 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6eab2 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6f1fc -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6f54f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0xb7133f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > >
// 信息 ID: e98950008015cf7b6af8dab7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster::closure_env$2 -> ["country_core::resources::world_raster_defs::GardenRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_garden::replay_garden::closure_env$2"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 28 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb64a9f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb6528e -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb65517 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb6726b -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb67994 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb67cee -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb69a0d -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6a14b -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6a49d -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6c2ef -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6cade -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6cd67 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6eab2 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6f1fc -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6f54f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0xb7133f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > >
// 信息 ID: 6899672d3cfbf3c0a4b27f04
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_path::replay_path::closure_env$1 -> ["country_core::systems::history::replay_path::DirtPathFilter"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster::closure_env$2 -> ["country_core::resources::world_raster_defs::PathRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 28 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb64a9f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb6528e -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb65517 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb6726b -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb67994 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb67cee -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb69a0d -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6a14b -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6a49d -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6c2ef -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6cade -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6cd67 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6eab2 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6f1fc -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6f54f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0xb7133f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> > >
// 信息 ID: 387f04298cfb0e1c1b440bc1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster::closure_env$2 -> ["country_core::resources::world_raster_defs::WaterRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_water::replay_water::closure_env$2"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 28 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb64a9f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb6528e -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb65517 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb6726b -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb67994 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb67cee -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb69a0d -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6a14b -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6a49d -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6c2ef -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6cade -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6cd67 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6eab2 -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6f1fc -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6f54f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0xb7133f -> 0x1a70fc0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$3::copy_to_staging::closure_env$0<512> >
// 信息 ID: 926ada50679c4a89f50ffd1a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::world_raster::impl$3::copy_to_staging::closure_env$0 -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::retain -> ["country_core::resources::world_raster::impl$3::copy_to_staging::closure_env$0<512>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::world_raster::impl$3::copy_to_staging<512> @ 0xc87714 -> 0x1a71280 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster::impl$3::copy_to_staging<512> @ 0x2859f94 -> 0x2afa550 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::set
// 信息 ID: 86b2541334600560125fe737
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::plant_sample_raster::PlantSampleRaster::begin_stroke; module=550, inline_record=34408; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::plant_sample_raster::PlantSampleRaster::accumulate_stroke; module=550, inline_record=45332; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::terrain_height::TerrainHeightsData::copy_from_gpu<country_core::resources::terrain_height::TerrainHeightSsbo>; module=605, inline_record=51016; 不是独立调用边。
// [PDB 内联宿主] system_clutter::autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu; module=2754, inline_record=1384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::bitmap_intersects
// 信息 ID: 0f7084032fd1aea40510a56b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure_env$0>; module=1745, inline_record=8588; 不是独立调用边。
// [PDB 内联宿主] system_clutter::autoclutter::add_birdnests::add_birdnests; module=4056, inline_record=5060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::bitmap_set
// 信息 ID: ec5fd051f050a2b41a3258c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 29 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::plant_sample_raster::PlantSampleRaster::begin_stroke; module=550, inline_record=34440; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::plant_sample_raster::PlantSampleRaster::accumulate_stroke; module=550, inline_record=45356; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::terrain_height::TerrainHeightsData::copy_from_gpu<country_core::resources::terrain_height::TerrainHeightSsbo>; module=605, inline_record=51068; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=5464; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=5908; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=6268; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=6808; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=7284; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::dilate
// 信息 ID: af8ddf9aeeae4d5b39f77171
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure$0
// 信息 ID: b336f98c3efe7c59c94ee041
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure_env$0 <- country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure_env$0>; module=1745, inline_record=8536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure$0
// 信息 ID: 2e4d0d07060bca66add427de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0 <- conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0 <- conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0 <- conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 36 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=5708; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=6024; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=6256; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=6636; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=6980; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=7240; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=7588; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=7904; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure$0
// 信息 ID: 8652c385af85e938598ff5ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure_env$0 <- country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure_env$0>; module=1745, inline_record=10216; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure$0
// 信息 ID: 16450e0e653906341179bfe5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0 <- conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0 <- conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 24 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=5432; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=5876; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=6236; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=6776; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=7252; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=7644; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=8148; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=8592; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::impl$2::apply_to
// 信息 ID: a78f1ee72ee1d48f5518579b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::impl$1::retain::closure$0<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> > | alloc::vec::impl$1::retain::closure$0<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> > @ 0x207bd68 -> 0xa634f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> | tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> @ 0x28d48e7 -> 0xa634f0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >; module=1643, inline_record=203664; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >; module=1643, inline_record=204700; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::impl$3::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0>
// 信息 ID: 676edded77f3cc2050f4db08
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::dirty_tiles::impl$3::iter_dirty_tiles -> ["country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state @ 0xbac5df -> 0x9c6230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::impl$3::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1>
// 信息 ID: 4314afb18072ae97bb5b4318
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::dirty_tiles::impl$3::iter_dirty_tiles -> ["country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state @ 0xbac671 -> 0x9c6300 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::impl$4::clone
// 信息 ID: aeeddfdff656787b674477c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::world_raster::impl$3::copy_to_staging<512>; module=609, inline_record=27048; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster::impl$3::copy_to_staging<512>; module=609, inline_record=27316; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster::impl$3::copy_to_staging<512>; module=3882, inline_record=2716; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster::impl$3::copy_to_staging<512>; module=3882, inline_record=2984; 不是独立调用边。
// [PDB 内联宿主] system_clutter::autoclutter::add_birdnests::add_birdnests; module=4056, inline_record=2960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

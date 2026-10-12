// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/conservative-line-raster/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: abe7152d1c06ece759ecf01ad71ba04c2e143b5f1f7a123780eaf08b9aaeb5f6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 160, 625, 1664, 4997
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use conservative_line_raster; // namespace candidate
//   package identities: [{"name":"conservative-line-raster","source":"local_or_path","version":"0.1.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::impl$0::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":625,"record_offset":280,"rva":13287152,"va":5381996272}
// 编译实例 RVAs: 0xcabef0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":160,"record_offset":4956,"rva":4896496,"va":5373605616}
// 编译实例 RVAs: 0x4ab6f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14040deb0
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-645733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373605616,"raw_hex":"e89b25f6ff","size":5,"soft_fail":false,"target_va":5372960432,"va":5373606160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cabf00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8389783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373605616,"raw_hex":"e897048000","size":5,"soft_fail":false,"target_va":5381996288,"va":5373606500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cabf50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8390352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373605616,"raw_hex":"e8d0068000","size":5,"soft_fail":false,"target_va":5381996368,"va":5373606011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bc10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43451264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373605616,"raw_hex":"e880039702","size":5,"soft_fail":false,"target_va":5417057296,"va":5373606027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43451931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373605616,"raw_hex":"e81b069702","size":5,"soft_fail":false,"target_va":5417057792,"va":5373605856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] conservative_line_raster::conservative_plot_line_low
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":160,"record_offset":5472,"parent_record_offset":4956,"ancestor_record_offset":4956,"inlinee":14782,"source_line":119,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":160,"record_offset":5360,"parent_record_offset":4956,"ancestor_record_offset":4956,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":160,"record_offset":5416,"parent_record_offset":4956,"ancestor_record_offset":4956,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::conservative_plot_line::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":160,"record_offset":6612,"parent_record_offset":6376,"ancestor_record_offset":4956,"inlinee":14790,"source_line":58,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::conservative_plot_line::closure$1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":160,"record_offset":8480,"parent_record_offset":8256,"ancestor_record_offset":4956,"inlinee":14795,"source_line":66,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::impl$2::quantize_range
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":160,"record_offset":5588,"parent_record_offset":5472,"ancestor_record_offset":4956,"inlinee":14785,"source_line":93,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::impl$3::quantize_range
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":160,"record_offset":7472,"parent_record_offset":7364,"ancestor_record_offset":4956,"inlinee":14792,"source_line":105,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":160,"record_offset":5668,"parent_record_offset":5644,"ancestor_record_offset":4956,"inlinee":7964,"source_line":1916,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$77::cmp
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":160,"record_offset":5880,"parent_record_offset":5472,"ancestor_record_offset":4956,"inlinee":14794,"source_line":2010,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":160,"record_offset":5848,"parent_record_offset":5824,"ancestor_record_offset":4956,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":160,"record_offset":5644,"parent_record_offset":5620,"ancestor_record_offset":4956,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":160,"record_offset":5620,"parent_record_offset":5472,"ancestor_record_offset":4956,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure$0
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":160,"record_offset":5708,"parent_record_offset":5472,"ancestor_record_offset":4956,"inlinee":14787,"source_line":69,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":160,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":4956,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":160,"record_offset":5760,"parent_record_offset":5736,"ancestor_record_offset":4956,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":160,"record_offset":5824,"parent_record_offset":5472,"ancestor_record_offset":4956,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::hypot
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":160,"record_offset":5560,"parent_record_offset":5472,"ancestor_record_offset":4956,"inlinee":14783,"source_line":675,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure$15
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/playermade_stairs_assemble_visuals.rs
//     evidence={"module":160,"record_offset":5736,"parent_record_offset":5708,"ancestor_record_offset":4956,"inlinee":14788,"source_line":298,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4997,"record_offset":268,"rva":47176832,"va":5415885952}
// 编译实例 RVAs: 0x2cfdc80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140cabf00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33891187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415885952,"raw_hex":"e88ddcfafd","size":5,"soft_fail":false,"target_va":5381996288,"va":5415887470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cabf50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33889964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415885952,"raw_hex":"e854e1fafd","size":5,"soft_fail":false,"target_va":5381996368,"va":5415886327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bc10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1170948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415885952,"raw_hex":"e804de1100","size":5,"soft_fail":false,"target_va":5417057296,"va":5415886343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1171612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5415885952,"raw_hex":"e89ce01100","size":5,"soft_fail":false,"target_va":5417057792,"va":5415886175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] conservative_line_raster::conservative_plot_line_low
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":4997,"record_offset":1016,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":14782,"source_line":119,"archive":"_pdb/C/modules/04997.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4997,"record_offset":904,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/04997.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4997,"record_offset":960,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/04997.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::conservative_plot_line::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":4997,"record_offset":2724,"parent_record_offset":2332,"ancestor_record_offset":268,"inlinee":14790,"source_line":58,"archive":"_pdb/C/modules/04997.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::conservative_plot_line::closure$1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":4997,"record_offset":5428,"parent_record_offset":5048,"ancestor_record_offset":268,"inlinee":14795,"source_line":66,"archive":"_pdb/C/modules/04997.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::impl$2::quantize_range
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":4997,"record_offset":1152,"parent_record_offset":1016,"ancestor_record_offset":268,"inlinee":14785,"source_line":93,"archive":"_pdb/C/modules/04997.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::impl$3::quantize_range
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":4997,"record_offset":3872,"parent_record_offset":3744,"ancestor_record_offset":268,"inlinee":14792,"source_line":105,"archive":"_pdb/C/modules/04997.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4997,"record_offset":1256,"parent_record_offset":1216,"ancestor_record_offset":268,"inlinee":7964,"source_line":1916,"archive":"_pdb/C/modules/04997.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$77::cmp
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4997,"record_offset":1580,"parent_record_offset":1016,"ancestor_record_offset":268,"inlinee":14794,"source_line":2010,"archive":"_pdb/C/modules/04997.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4997,"record_offset":1548,"parent_record_offset":1524,"ancestor_record_offset":268,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/04997.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$40::forward_unchecked
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":4997,"record_offset":1288,"parent_record_offset":1216,"ancestor_record_offset":268,"inlinee":7966,"source_line":195,"archive":"_pdb/C/modules/04997.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":4997,"record_offset":1216,"parent_record_offset":1184,"ancestor_record_offset":268,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/04997.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":4997,"record_offset":1184,"parent_record_offset":1016,"ancestor_record_offset":268,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/04997.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::checked_add_unsigned
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":4997,"record_offset":1312,"parent_record_offset":1288,"ancestor_record_offset":268,"inlinee":7967,"source_line":613,"archive":"_pdb/C/modules/04997.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":4997,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":268,"inlinee":7969,"source_line":2498,"archive":"_pdb/C/modules/04997.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add_unsigned
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":4997,"record_offset":1336,"parent_record_offset":1312,"ancestor_record_offset":268,"inlinee":7968,"source_line":2581,"archive":"_pdb/C/modules/04997.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":4997,"record_offset":1480,"parent_record_offset":1448,"ancestor_record_offset":268,"inlinee":34245,"source_line":221,"archive":"_pdb/C/modules/04997.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure$0
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":4997,"record_offset":1408,"parent_record_offset":1016,"ancestor_record_offset":268,"inlinee":14787,"source_line":69,"archive":"_pdb/C/modules/04997.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure$0
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":4997,"record_offset":1448,"parent_record_offset":1408,"ancestor_record_offset":268,"inlinee":82348,"source_line":37,"archive":"_pdb/C/modules/04997.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":4997,"record_offset":1524,"parent_record_offset":1016,"ancestor_record_offset":268,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/04997.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::hypot
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":4997,"record_offset":1124,"parent_record_offset":1016,"ancestor_record_offset":268,"inlinee":14783,"source_line":675,"archive":"_pdb/C/modules/04997.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1664,"record_offset":4252,"rva":24904192,"va":5393613312}
// 编译实例 RVAs: 0x17c0200
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140cabf00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11618547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393613312,"raw_hex":"e80db74eff","size":5,"soft_fail":false,"target_va":5381996288,"va":5393614830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cabf50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11617324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393613312,"raw_hex":"e8d4bb4eff","size":5,"soft_fail":false,"target_va":5381996368,"va":5393613687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bc10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23443588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393613312,"raw_hex":"e884b86501","size":5,"soft_fail":false,"target_va":5417057296,"va":5393613703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23444252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393613312,"raw_hex":"e81cbb6501","size":5,"soft_fail":false,"target_va":5417057792,"va":5393613535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] conservative_line_raster::conservative_plot_line_low
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":1664,"record_offset":5000,"parent_record_offset":4252,"ancestor_record_offset":4252,"inlinee":14782,"source_line":119,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1664,"record_offset":4888,"parent_record_offset":4252,"ancestor_record_offset":4252,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1664,"record_offset":4944,"parent_record_offset":4252,"ancestor_record_offset":4252,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::conservative_plot_line::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":1664,"record_offset":6708,"parent_record_offset":6316,"ancestor_record_offset":4252,"inlinee":14790,"source_line":58,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::conservative_plot_line::closure$1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":1664,"record_offset":9412,"parent_record_offset":9032,"ancestor_record_offset":4252,"inlinee":14795,"source_line":66,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::impl$2::quantize_range
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":1664,"record_offset":5136,"parent_record_offset":5000,"ancestor_record_offset":4252,"inlinee":14785,"source_line":93,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::impl$3::quantize_range
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":1664,"record_offset":7856,"parent_record_offset":7728,"ancestor_record_offset":4252,"inlinee":14792,"source_line":105,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1664,"record_offset":5240,"parent_record_offset":5200,"ancestor_record_offset":4252,"inlinee":7964,"source_line":1916,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$77::cmp
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1664,"record_offset":5564,"parent_record_offset":5000,"ancestor_record_offset":4252,"inlinee":14794,"source_line":2010,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1664,"record_offset":5532,"parent_record_offset":5508,"ancestor_record_offset":4252,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$40::forward_unchecked
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1664,"record_offset":5272,"parent_record_offset":5200,"ancestor_record_offset":4252,"inlinee":7966,"source_line":195,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1664,"record_offset":5200,"parent_record_offset":5168,"ancestor_record_offset":4252,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1664,"record_offset":5168,"parent_record_offset":5000,"ancestor_record_offset":4252,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::checked_add_unsigned
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":1664,"record_offset":5296,"parent_record_offset":5272,"ancestor_record_offset":4252,"inlinee":7967,"source_line":613,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":1664,"record_offset":5344,"parent_record_offset":5320,"ancestor_record_offset":4252,"inlinee":7969,"source_line":2498,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add_unsigned
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":1664,"record_offset":5320,"parent_record_offset":5296,"ancestor_record_offset":4252,"inlinee":7968,"source_line":2581,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1664,"record_offset":5464,"parent_record_offset":5432,"ancestor_record_offset":4252,"inlinee":34245,"source_line":221,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure$0
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1664,"record_offset":5392,"parent_record_offset":5000,"ancestor_record_offset":4252,"inlinee":14787,"source_line":69,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure$0
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1664,"record_offset":5432,"parent_record_offset":5392,"ancestor_record_offset":4252,"inlinee":82348,"source_line":37,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1664,"record_offset":5508,"parent_record_offset":5000,"ancestor_record_offset":4252,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::hypot
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1664,"record_offset":5108,"parent_record_offset":5000,"ancestor_record_offset":4252,"inlinee":14783,"source_line":675,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::conservative_plot_line::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x39C6
// 首条位置: {"module":160,"inlinee":14790}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure$0
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":160,"record_offset":6636,"parent_record_offset":6612,"ancestor_record_offset":6612,"inlinee":14787,"source_line":69,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1664,"record_offset":6808,"parent_record_offset":6776,"ancestor_record_offset":6708,"inlinee":34245,"source_line":221,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure$0
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1664,"record_offset":6776,"parent_record_offset":6736,"ancestor_record_offset":6708,"inlinee":82348,"source_line":37,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":160,"record_offset":6712,"parent_record_offset":6688,"ancestor_record_offset":6612,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":160,"record_offset":6688,"parent_record_offset":6664,"ancestor_record_offset":6612,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure$15
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/playermade_stairs_assemble_visuals.rs
//     evidence={"module":160,"record_offset":6664,"parent_record_offset":6636,"ancestor_record_offset":6612,"inlinee":14788,"source_line":298,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::conservative_plot_line::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x39CB
// 首条位置: {"module":160,"inlinee":14795}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure$0
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":160,"record_offset":8504,"parent_record_offset":8480,"ancestor_record_offset":8480,"inlinee":14787,"source_line":69,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1664,"record_offset":9512,"parent_record_offset":9480,"ancestor_record_offset":9412,"inlinee":34245,"source_line":221,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure$0
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1664,"record_offset":9480,"parent_record_offset":9440,"ancestor_record_offset":9412,"inlinee":82348,"source_line":37,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":160,"record_offset":8580,"parent_record_offset":8556,"ancestor_record_offset":8480,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":160,"record_offset":8556,"parent_record_offset":8532,"ancestor_record_offset":8480,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure$15
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/playermade_stairs_assemble_visuals.rs
//     evidence={"module":160,"record_offset":8532,"parent_record_offset":8504,"ancestor_record_offset":8480,"inlinee":14788,"source_line":298,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::impl$2::quantize_range
// 记录形式: inline_instance, inlinee_declaration; instances=21
// 源码行: 93
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x39C1
// 首条位置: {"module":160,"inlinee":14785}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::impl$2::assert_delta
// 记录形式: compiled_procedure; instances=1
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":625,"record_offset":520,"rva":13287248,"va":5381996368}
// 编译实例 RVAs: 0xcabf50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34625807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381996368,"raw_hex":"e80f591002","size":5,"soft_fail":false,"target_va":5416622240,"va":5381996428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::impl$3::quantize_range
// 记录形式: inline_instance, inlinee_declaration; instances=21
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x39C8
// 首条位置: {"module":160,"inlinee":14792}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::impl$3::assert_delta
// 记录形式: compiled_procedure; instances=1
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":625,"record_offset":396,"rva":13287168,"va":5381996288}
// 编译实例 RVAs: 0xcabf00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34625887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381996288,"raw_hex":"e85f591002","size":5,"soft_fail":false,"target_va":5416622240,"va":5381996348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::conservative_plot_line_low
// 记录形式: inline_instance, inlinee_declaration; instances=24
// 源码行: 119
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x39BE
// 首条位置: {"module":160,"inlinee":14782}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] conservative_line_raster::conservative_plot_line::closure$0
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":160,"record_offset":6612,"parent_record_offset":6376,"ancestor_record_offset":6376,"inlinee":14790,"source_line":58,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_direct] conservative_line_raster::conservative_plot_line::closure$1
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":160,"record_offset":8480,"parent_record_offset":8256,"ancestor_record_offset":8256,"inlinee":14795,"source_line":66,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_direct] conservative_line_raster::impl$2::quantize_range
//     owner=private_source_ownership_candidate; count=18
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":160,"record_offset":5588,"parent_record_offset":5472,"ancestor_record_offset":5472,"inlinee":14785,"source_line":93,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_direct] conservative_line_raster::impl$3::quantize_range
//     owner=private_source_ownership_candidate; count=18
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":160,"record_offset":7472,"parent_record_offset":7364,"ancestor_record_offset":7364,"inlinee":14792,"source_line":105,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$77::cmp
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":160,"record_offset":5880,"parent_record_offset":5472,"ancestor_record_offset":5472,"inlinee":14794,"source_line":2010,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":160,"record_offset":5620,"parent_record_offset":5472,"ancestor_record_offset":5472,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure$0
//     owner=private_source_ownership_candidate; count=18
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":160,"record_offset":5708,"parent_record_offset":5472,"ancestor_record_offset":5472,"inlinee":14787,"source_line":69,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":160,"record_offset":5824,"parent_record_offset":5472,"ancestor_record_offset":5472,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::hypot
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":160,"record_offset":5560,"parent_record_offset":5472,"ancestor_record_offset":5472,"inlinee":14783,"source_line":675,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":160,"record_offset":5668,"parent_record_offset":5644,"ancestor_record_offset":5472,"inlinee":7964,"source_line":1916,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":160,"record_offset":5848,"parent_record_offset":5824,"ancestor_record_offset":5472,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$40::forward_unchecked
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1664,"record_offset":5272,"parent_record_offset":5200,"ancestor_record_offset":5000,"inlinee":7966,"source_line":195,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":160,"record_offset":5644,"parent_record_offset":5620,"ancestor_record_offset":5472,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::checked_add_unsigned
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":1664,"record_offset":5296,"parent_record_offset":5272,"ancestor_record_offset":5000,"inlinee":7967,"source_line":613,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":1664,"record_offset":5344,"parent_record_offset":5320,"ancestor_record_offset":5000,"inlinee":7969,"source_line":2498,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add_unsigned
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":1664,"record_offset":5320,"parent_record_offset":5296,"ancestor_record_offset":5000,"inlinee":7968,"source_line":2581,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=24
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1664,"record_offset":5464,"parent_record_offset":5432,"ancestor_record_offset":5000,"inlinee":34245,"source_line":221,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure$0
//     owner=private_source_ownership_candidate; count=18
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":160,"record_offset":6636,"parent_record_offset":6612,"ancestor_record_offset":6376,"inlinee":14787,"source_line":69,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure$0
//     owner=private_source_ownership_candidate; count=24
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1664,"record_offset":5432,"parent_record_offset":5392,"ancestor_record_offset":5000,"inlinee":82348,"source_line":37,"archive":"_pdb/C/modules/01664.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":160,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":5472,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":160,"record_offset":5760,"parent_record_offset":5736,"ancestor_record_offset":5472,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure$15
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/playermade_stairs/playermade_stairs_assemble_visuals.rs
//     evidence={"module":160,"record_offset":5736,"parent_record_offset":5708,"ancestor_record_offset":5472,"inlinee":14788,"source_line":298,"archive":"_pdb/C/modules/00160.jsonl.gz"}
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
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: conservative_line_raster::conservative_plot_line::closure$0
// 信息 ID: 3649c097fd34e6d63d7d111a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "conservative_line_raster::conservative_plot_line", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=6612; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=6956; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=7216; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=6708; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=7184; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=7576; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=4997, inline_record=2724; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=4997, inline_record=3200; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::conservative_plot_line::closure$1
// 信息 ID: 69311211a75ee349f75c3088
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "conservative_line_raster::conservative_plot_line", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=8480; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=8824; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=9084; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=9412; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=9888; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=10288; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=4997, inline_record=5428; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=4997, inline_record=5904; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >
// 信息 ID: 4890f9de456e80e05388dc02
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["glam::f32::vec2::Vec2", "2"]; 字段布局 = unknown。
// [类型关系] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15 -> ["true"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0 -> ["array$<glam::f32::vec2::Vec2,2>", "system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true>"]; 字段布局 = unknown。
// [类型关系] conservative_line_raster::conservative_plot_line -> ["country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > | country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<false> > @ 0x517a71 -> 0x4ab6f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >
// 信息 ID: 03658062434f0e364e279b9b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0"]; 字段布局 = unknown。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0 -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0 -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>", "country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >"]; 字段布局 = unknown。
// [类型关系] conservative_line_raster::conservative_plot_line -> ["country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > @ 0x2afa3f4 -> 0x2cfdc80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > @ 0x2afa4e1 -> 0x2cfdc80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >
// 信息 ID: aa75fd224fc6f3298f77ea21
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0"]; 字段布局 = unknown。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0 -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0 -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>", "country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> >"]; 字段布局 = unknown。
// [类型关系] conservative_line_raster::conservative_plot_line -> ["country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > | 其余 4 个同地址别名见共享索引 @ 0x1a71124 -> 0x17c0200 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > | 其余 4 个同地址别名见共享索引 @ 0x1a71211 -> 0x17c0200 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::conservative_plot_line_low
// 信息 ID: ca16ed419d1996a1d82127c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=5472; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=6376; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=7364; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=8256; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=5000; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=6316; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=7728; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=9032; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::impl$0::from
// 信息 ID: bbecb2deac9d3732dba975d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > | country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<false> > @ 0x5179fd -> 0xcabef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > | country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<false> > @ 0x517a42 -> 0xcabef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > | 其余 4 个同地址别名见共享索引 @ 0x1a710a8 -> 0xcabef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > | 其余 4 个同地址别名见共享索引 @ 0x1a710f3 -> 0xcabef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > | 其余 4 个同地址别名见共享索引 @ 0x1a711a0 -> 0xcabef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > | 其余 4 个同地址别名见共享索引 @ 0x1a711eb -> 0xcabef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > @ 0x2afa378 -> 0xcabef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > @ 0x2afa3c3 -> 0xcabef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > @ 0x2afa470 -> 0xcabef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > @ 0x2afa4bb -> 0xcabef0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::impl$2::assert_delta
// 信息 ID: daa86feb2880ee34b0e49f56
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > > @ 0x4ab87b -> 0xcabf50 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > > @ 0x4ab9c8 -> 0xcabf50 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > > @ 0x17c0377 -> 0xcabf50 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > > @ 0x17c0617 -> 0xcabf50 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > > @ 0x2cfddf7 -> 0xcabf50 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > > @ 0x2cfe097 -> 0xcabf50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::impl$2::quantize_range
// 信息 ID: 24c16e361af7f9ee22d6fd76
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 18 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=5588; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=5908; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=6140; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=6492; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=6840; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=7100; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=5136; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=5592; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::impl$3::assert_delta
// 信息 ID: a51494abf4f7c924c8e62931
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > > @ 0x4aba64 -> 0xcabf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > > @ 0x4abbe9 -> 0xcabf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > > @ 0x17c07ee -> 0xcabf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > > @ 0x17c0aa5 -> 0xcabf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > > @ 0x2cfe26e -> 0xcabf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > > @ 0x2cfe525 -> 0xcabf00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::impl$3::quantize_range
// 信息 ID: 1bc9f8d79a6f7a617e9c2246
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 18 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=7472; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=7788; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=8020; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=8364; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=8708; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble::closure_env$15<true> > >; module=160, inline_record=8968; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=7856; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1664, inline_record=8308; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

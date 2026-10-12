// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/conservative-line-raster/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: abe7152d1c06ece759ecf01ad71ba04c2e143b5f1f7a123780eaf08b9aaeb5f6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 85, 580, 610, 4677
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
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
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
// 首条位置: {"module":610,"record_offset":272,"rva":12078656,"va":5380787776}
// 编译实例 RVAs: 0xb84e40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":85,"record_offset":240,"rva":2458320,"va":5371167440}
// 编译实例 RVAs: 0x2582d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ca470
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t466715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371167440,"raw_hex":"e81b1f0700","size":5,"soft_fail":false,"target_va":5371634800,"va":5371168080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b84e50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9619860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371167440,"raw_hex":"e894c99200","size":5,"soft_fail":false,"target_va":5380787792,"va":5371167927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b84ec0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9619479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371167440,"raw_hex":"e817c89200","size":5,"soft_fail":false,"target_va":5380787904,"va":5371168420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923cf0
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40679460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371167440,"raw_hex":"e824b86c02","size":5,"soft_fail":false,"target_va":5411847408,"va":5371167943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ee0
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40680099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371167440,"raw_hex":"e8a3ba6c02","size":5,"soft_fail":false,"target_va":5411847904,"va":5371167800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] conservative_line_raster::conservative_plot_line_low
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":85,"record_offset":736,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":10998,"source_line":119,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":85,"record_offset":608,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":85,"record_offset":664,"parent_record_offset":240,"ancestor_record_offset":240,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::conservative_plot_line::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":85,"record_offset":1864,"parent_record_offset":1620,"ancestor_record_offset":240,"inlinee":11006,"source_line":58,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::conservative_plot_line::closure$1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":85,"record_offset":3676,"parent_record_offset":3452,"ancestor_record_offset":240,"inlinee":11010,"source_line":66,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::impl$2::quantize_range
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":85,"record_offset":860,"parent_record_offset":736,"ancestor_record_offset":240,"inlinee":11001,"source_line":93,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::impl$3::quantize_range
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":85,"record_offset":2696,"parent_record_offset":2588,"ancestor_record_offset":240,"inlinee":11008,"source_line":105,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":85,"record_offset":940,"parent_record_offset":916,"ancestor_record_offset":240,"inlinee":10985,"source_line":1824,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$77::cmp
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":85,"record_offset":1124,"parent_record_offset":736,"ancestor_record_offset":240,"inlinee":8316,"source_line":1836,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":85,"record_offset":916,"parent_record_offset":892,"ancestor_record_offset":240,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":85,"record_offset":892,"parent_record_offset":736,"ancestor_record_offset":240,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure$0
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":85,"record_offset":980,"parent_record_offset":736,"ancestor_record_offset":240,"inlinee":11003,"source_line":69,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":85,"record_offset":1056,"parent_record_offset":1032,"ancestor_record_offset":240,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":85,"record_offset":1032,"parent_record_offset":1008,"ancestor_record_offset":240,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":85,"record_offset":1096,"parent_record_offset":736,"ancestor_record_offset":240,"inlinee":11009,"source_line":48,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::hypot
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":85,"record_offset":828,"parent_record_offset":736,"ancestor_record_offset":240,"inlinee":10999,"source_line":627,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure$6
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/stairs_assemble.rs
//     evidence={"module":85,"record_offset":1008,"parent_record_offset":980,"ancestor_record_offset":240,"inlinee":11004,"source_line":553,"archive":"_pdb/B/modules/00085.jsonl.gz"}
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
// 首条位置: {"module":4677,"record_offset":260,"rva":42078128,"va":5410787248}
// 编译实例 RVAs: 0x2820fb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140b84e50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29999939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410787248,"raw_hex":"e8bd3c36fe","size":5,"soft_fail":false,"target_va":5380787792,"va":5410787726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b84ec0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30000967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410787248,"raw_hex":"e8b93836fe","size":5,"soft_fail":false,"target_va":5380787904,"va":5410788866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923cf0
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1059661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410787248,"raw_hex":"e84d2b1000","size":5,"soft_fail":false,"target_va":5411847408,"va":5410787742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ee0
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1060297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5410787248,"raw_hex":"e8c92d1000","size":5,"soft_fail":false,"target_va":5411847904,"va":5410787602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] conservative_line_raster::conservative_plot_line_low
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":4677,"record_offset":1024,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":10998,"source_line":119,"archive":"_pdb/B/modules/04677.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":4677,"record_offset":896,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/04677.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":4677,"record_offset":952,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/04677.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::conservative_plot_line::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":4677,"record_offset":2720,"parent_record_offset":2320,"ancestor_record_offset":260,"inlinee":11006,"source_line":58,"archive":"_pdb/B/modules/04677.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::conservative_plot_line::closure$1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":4677,"record_offset":5368,"parent_record_offset":4988,"ancestor_record_offset":260,"inlinee":11010,"source_line":66,"archive":"_pdb/B/modules/04677.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::impl$2::quantize_range
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":4677,"record_offset":1168,"parent_record_offset":1024,"ancestor_record_offset":260,"inlinee":11001,"source_line":93,"archive":"_pdb/B/modules/04677.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::impl$3::quantize_range
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":4677,"record_offset":3840,"parent_record_offset":3712,"ancestor_record_offset":260,"inlinee":11008,"source_line":105,"archive":"_pdb/B/modules/04677.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4677,"record_offset":1272,"parent_record_offset":1232,"ancestor_record_offset":260,"inlinee":10985,"source_line":1824,"archive":"_pdb/B/modules/04677.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$77::cmp
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4677,"record_offset":1568,"parent_record_offset":1024,"ancestor_record_offset":260,"inlinee":8316,"source_line":1836,"archive":"_pdb/B/modules/04677.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$40::forward_unchecked
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4677,"record_offset":1304,"parent_record_offset":1232,"ancestor_record_offset":260,"inlinee":15400,"source_line":187,"archive":"_pdb/B/modules/04677.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4677,"record_offset":1232,"parent_record_offset":1200,"ancestor_record_offset":260,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/04677.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4677,"record_offset":1200,"parent_record_offset":1024,"ancestor_record_offset":260,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/04677.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::checked_add_unsigned
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":4677,"record_offset":1328,"parent_record_offset":1304,"ancestor_record_offset":260,"inlinee":15401,"source_line":545,"archive":"_pdb/B/modules/04677.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":4677,"record_offset":1376,"parent_record_offset":1352,"ancestor_record_offset":260,"inlinee":15403,"source_line":2297,"archive":"_pdb/B/modules/04677.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add_unsigned
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":4677,"record_offset":1352,"parent_record_offset":1328,"ancestor_record_offset":260,"inlinee":15402,"source_line":2379,"archive":"_pdb/B/modules/04677.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":4677,"record_offset":1496,"parent_record_offset":1464,"ancestor_record_offset":260,"inlinee":30662,"source_line":221,"archive":"_pdb/B/modules/04677.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure$0
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":4677,"record_offset":1424,"parent_record_offset":1024,"ancestor_record_offset":260,"inlinee":11003,"source_line":69,"archive":"_pdb/B/modules/04677.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure$0
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":4677,"record_offset":1464,"parent_record_offset":1424,"ancestor_record_offset":260,"inlinee":33165,"source_line":37,"archive":"_pdb/B/modules/04677.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":4677,"record_offset":1540,"parent_record_offset":1024,"ancestor_record_offset":260,"inlinee":11009,"source_line":48,"archive":"_pdb/B/modules/04677.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::hypot
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":4677,"record_offset":1136,"parent_record_offset":1024,"ancestor_record_offset":260,"inlinee":10999,"source_line":627,"archive":"_pdb/B/modules/04677.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":580,"record_offset":260,"rva":11658336,"va":5380367456}
// 编译实例 RVAs: 0xb1e460
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140b84e50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t419853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380367456,"raw_hex":"e80d680600","size":5,"soft_fail":false,"target_va":5380787792,"va":5380367934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b84ec0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t418825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380367456,"raw_hex":"e809640600","size":5,"soft_fail":false,"target_va":5380787904,"va":5380369074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923cf0
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31479453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380367456,"raw_hex":"e89d56e001","size":5,"soft_fail":false,"target_va":5411847408,"va":5380367950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ee0
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31480089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380367456,"raw_hex":"e81959e001","size":5,"soft_fail":false,"target_va":5411847904,"va":5380367810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] conservative_line_raster::conservative_plot_line_low
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":580,"record_offset":1568,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":10998,"source_line":119,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":1440,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":1496,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::conservative_plot_line::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":580,"record_offset":3264,"parent_record_offset":2864,"ancestor_record_offset":260,"inlinee":11006,"source_line":58,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::conservative_plot_line::closure$1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":580,"record_offset":5912,"parent_record_offset":5532,"ancestor_record_offset":260,"inlinee":11010,"source_line":66,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::impl$2::quantize_range
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":580,"record_offset":1712,"parent_record_offset":1568,"ancestor_record_offset":260,"inlinee":11001,"source_line":93,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::impl$3::quantize_range
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":580,"record_offset":4384,"parent_record_offset":4256,"ancestor_record_offset":260,"inlinee":11008,"source_line":105,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":580,"record_offset":1816,"parent_record_offset":1776,"ancestor_record_offset":260,"inlinee":10985,"source_line":1824,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$77::cmp
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":580,"record_offset":2112,"parent_record_offset":1568,"ancestor_record_offset":260,"inlinee":8316,"source_line":1836,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$40::forward_unchecked
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":580,"record_offset":1848,"parent_record_offset":1776,"ancestor_record_offset":260,"inlinee":15400,"source_line":187,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":580,"record_offset":1776,"parent_record_offset":1744,"ancestor_record_offset":260,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":580,"record_offset":1744,"parent_record_offset":1568,"ancestor_record_offset":260,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::checked_add_unsigned
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":580,"record_offset":1872,"parent_record_offset":1848,"ancestor_record_offset":260,"inlinee":15401,"source_line":545,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":580,"record_offset":1920,"parent_record_offset":1896,"ancestor_record_offset":260,"inlinee":15403,"source_line":2297,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add_unsigned
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":580,"record_offset":1896,"parent_record_offset":1872,"ancestor_record_offset":260,"inlinee":15402,"source_line":2379,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":580,"record_offset":2040,"parent_record_offset":2008,"ancestor_record_offset":260,"inlinee":30662,"source_line":221,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure$0
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":580,"record_offset":1968,"parent_record_offset":1568,"ancestor_record_offset":260,"inlinee":11003,"source_line":69,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure$0
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":580,"record_offset":2008,"parent_record_offset":1968,"ancestor_record_offset":260,"inlinee":33165,"source_line":37,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":580,"record_offset":2084,"parent_record_offset":1568,"ancestor_record_offset":260,"inlinee":11009,"source_line":48,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::hypot
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":580,"record_offset":1680,"parent_record_offset":1568,"ancestor_record_offset":260,"inlinee":10999,"source_line":627,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::conservative_plot_line::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2AFE
// 首条位置: {"module":85,"inlinee":11006}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure$0
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":85,"record_offset":1888,"parent_record_offset":1864,"ancestor_record_offset":1864,"inlinee":11003,"source_line":69,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":580,"record_offset":3364,"parent_record_offset":3332,"ancestor_record_offset":3264,"inlinee":30662,"source_line":221,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure$0
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":580,"record_offset":3332,"parent_record_offset":3292,"ancestor_record_offset":3264,"inlinee":33165,"source_line":37,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":85,"record_offset":1964,"parent_record_offset":1940,"ancestor_record_offset":1864,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":85,"record_offset":1940,"parent_record_offset":1916,"ancestor_record_offset":1864,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure$6
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/stairs_assemble.rs
//     evidence={"module":85,"record_offset":1916,"parent_record_offset":1888,"ancestor_record_offset":1864,"inlinee":11004,"source_line":553,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::conservative_plot_line::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2B02
// 首条位置: {"module":85,"inlinee":11010}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure$0
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":85,"record_offset":3700,"parent_record_offset":3676,"ancestor_record_offset":3676,"inlinee":11003,"source_line":69,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":580,"record_offset":6012,"parent_record_offset":5980,"ancestor_record_offset":5912,"inlinee":30662,"source_line":221,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure$0
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":580,"record_offset":5980,"parent_record_offset":5940,"ancestor_record_offset":5912,"inlinee":33165,"source_line":37,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":85,"record_offset":3776,"parent_record_offset":3752,"ancestor_record_offset":3676,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":85,"record_offset":3752,"parent_record_offset":3728,"ancestor_record_offset":3676,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure$6
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/stairs_assemble.rs
//     evidence={"module":85,"record_offset":3728,"parent_record_offset":3700,"ancestor_record_offset":3676,"inlinee":11004,"source_line":553,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::impl$2::quantize_range
// 记录形式: inline_instance, inlinee_declaration; instances=21
// 源码行: 93
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2AF9
// 首条位置: {"module":85,"inlinee":11001}
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
// 首条位置: {"module":610,"record_offset":388,"rva":12078672,"va":5380787792}
// 编译实例 RVAs: 0xb84e50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30754008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380787792,"raw_hex":"e8d844d501","size":5,"soft_fail":false,"target_va":5411541904,"va":5380787891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":610,"record_offset":520,"parent_record_offset":388,"ancestor_record_offset":388,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::impl$3::quantize_range
// 记录形式: inline_instance, inlinee_declaration; instances=21
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2B00
// 首条位置: {"module":85,"inlinee":11008}
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
// 首条位置: {"module":610,"record_offset":552,"rva":12078784,"va":5380787904}
// 编译实例 RVAs: 0xb84ec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30753896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380787904,"raw_hex":"e86844d501","size":5,"soft_fail":false,"target_va":5411541904,"va":5380788003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":610,"record_offset":684,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::conservative_plot_line_low
// 记录形式: inline_instance, inlinee_declaration; instances=24
// 源码行: 119
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2AF6
// 首条位置: {"module":85,"inlinee":10998}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] conservative_line_raster::conservative_plot_line::closure$0
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":85,"record_offset":1864,"parent_record_offset":1620,"ancestor_record_offset":1620,"inlinee":11006,"source_line":58,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_direct] conservative_line_raster::conservative_plot_line::closure$1
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":85,"record_offset":3676,"parent_record_offset":3452,"ancestor_record_offset":3452,"inlinee":11010,"source_line":66,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_direct] conservative_line_raster::impl$2::quantize_range
//     owner=private_source_ownership_candidate; count=18
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":85,"record_offset":860,"parent_record_offset":736,"ancestor_record_offset":736,"inlinee":11001,"source_line":93,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_direct] conservative_line_raster::impl$3::quantize_range
//     owner=private_source_ownership_candidate; count=18
//     packages=[{"name":"conservative-line-raster","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":85,"record_offset":2696,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":11008,"source_line":105,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$77::cmp
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":85,"record_offset":1124,"parent_record_offset":736,"ancestor_record_offset":736,"inlinee":8316,"source_line":1836,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":85,"record_offset":892,"parent_record_offset":736,"ancestor_record_offset":736,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure$0
//     owner=private_source_ownership_candidate; count=18
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":85,"record_offset":980,"parent_record_offset":736,"ancestor_record_offset":736,"inlinee":11003,"source_line":69,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":85,"record_offset":1096,"parent_record_offset":736,"ancestor_record_offset":736,"inlinee":11009,"source_line":48,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::hypot
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":85,"record_offset":828,"parent_record_offset":736,"ancestor_record_offset":736,"inlinee":10999,"source_line":627,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":85,"record_offset":940,"parent_record_offset":916,"ancestor_record_offset":736,"inlinee":10985,"source_line":1824,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$40::forward_unchecked
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":580,"record_offset":1848,"parent_record_offset":1776,"ancestor_record_offset":1568,"inlinee":15400,"source_line":187,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":85,"record_offset":916,"parent_record_offset":892,"ancestor_record_offset":736,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::checked_add_unsigned
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":580,"record_offset":1872,"parent_record_offset":1848,"ancestor_record_offset":1568,"inlinee":15401,"source_line":545,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":580,"record_offset":1920,"parent_record_offset":1896,"ancestor_record_offset":1568,"inlinee":15403,"source_line":2297,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add_unsigned
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":580,"record_offset":1896,"parent_record_offset":1872,"ancestor_record_offset":1568,"inlinee":15402,"source_line":2379,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=24
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":580,"record_offset":2040,"parent_record_offset":2008,"ancestor_record_offset":1568,"inlinee":30662,"source_line":221,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure$0
//     owner=private_source_ownership_candidate; count=18
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":85,"record_offset":1888,"parent_record_offset":1864,"ancestor_record_offset":1620,"inlinee":11003,"source_line":69,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure$0
//     owner=private_source_ownership_candidate; count=24
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":580,"record_offset":2008,"parent_record_offset":1968,"ancestor_record_offset":1568,"inlinee":33165,"source_line":37,"archive":"_pdb/B/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":85,"record_offset":1056,"parent_record_offset":1032,"ancestor_record_offset":736,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":85,"record_offset":1032,"parent_record_offset":1008,"ancestor_record_offset":736,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/00085.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure$6
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/stairs_assemble.rs
//     evidence={"module":85,"record_offset":1008,"parent_record_offset":980,"ancestor_record_offset":736,"inlinee":11004,"source_line":553,"archive":"_pdb/B/modules/00085.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: conservative_line_raster::conservative_plot_line::closure$0
// 信息 ID: c4da8f36555c11614472b7d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "conservative_line_raster::conservative_plot_line", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >; module=85, inline_record=1864; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >; module=85, inline_record=2180; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >; module=85, inline_record=2440; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > > >; module=580, inline_record=3264; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > > >; module=580, inline_record=3712; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > > >; module=580, inline_record=4104; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=4677, inline_record=2720; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=4677, inline_record=3168; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::conservative_plot_line::closure$1
// 信息 ID: 8661231af59d8342849cfe01
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "conservative_line_raster::conservative_plot_line", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >; module=85, inline_record=3676; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >; module=85, inline_record=3992; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >; module=85, inline_record=4252; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > > >; module=580, inline_record=5912; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > > >; module=580, inline_record=6360; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > > >; module=580, inline_record=6760; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=4677, inline_record=5368; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=4677, inline_record=5816; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >
// 信息 ID: 51584eef849359076ab3b24c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["glam::f32::vec2::Vec2", "2"]; 字段布局 = unknown。
// [类型关系] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6 -> ["false"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0 -> ["array$<glam::f32::vec2::Vec2,2>", "system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false>"]; 字段布局 = unknown。
// [类型关系] conservative_line_raster::conservative_plot_line -> ["country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0x99d5 -> 0x2582d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x18ce1 -> 0x2582d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >
// 信息 ID: d09f3c24d3f51630dc65115c
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
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > @ 0x25d9ff4 -> 0x2820fb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > @ 0x25da0e1 -> 0x2820fb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > > >
// 信息 ID: 487915f1c70688d8253a2927
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_path::replay_path::closure_env$1 -> ["country_core::systems::history::replay_path::Cobblestone2PathFilter"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster::closure_env$2 -> ["country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >"]; 字段布局 = unknown。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_path::replay_path::closure_env$1 -> ["country_core::systems::history::replay_path::Cobblestone2PathFilter"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster::closure_env$2 -> ["country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0 -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0 -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >", "country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > >"]; 字段布局 = unknown。
// [类型关系] conservative_line_raster::conservative_plot_line -> ["country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > > | 其余 4 个同地址别名见共享索引 @ 0xade784 -> 0xb1e460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > > | 其余 4 个同地址别名见共享索引 @ 0xade871 -> 0xb1e460 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::conservative_plot_line_low
// 信息 ID: 4da642e93faabd12accf6ded
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >; module=85, inline_record=736; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >; module=85, inline_record=1620; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >; module=85, inline_record=2588; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >; module=85, inline_record=3452; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > > >; module=580, inline_record=1568; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > > >; module=580, inline_record=2864; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > > >; module=580, inline_record=4256; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > > >; module=580, inline_record=5532; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::impl$0::from
// 信息 ID: 8ef5442d143992ce43691c5f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0x994f -> 0xb84e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0x99a9 -> 0xb84e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x18c5f -> 0xb84e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x18cb5 -> 0xb84e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > > | 其余 4 个同地址别名见共享索引 @ 0xade708 -> 0xb84e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > > | 其余 4 个同地址别名见共享索引 @ 0xade753 -> 0xb84e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > > | 其余 4 个同地址别名见共享索引 @ 0xade800 -> 0xb84e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > > | 其余 4 个同地址别名见共享索引 @ 0xade84b -> 0xb84e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > @ 0x25d9f78 -> 0xb84e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > @ 0x25d9fc3 -> 0xb84e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > @ 0x25da070 -> 0xb84e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > @ 0x25da0bb -> 0xb84e40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::impl$2::assert_delta
// 信息 ID: 52f2a7d1ea36251286af01f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > > @ 0x2584b7 -> 0xb84e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > > @ 0x258604 -> 0xb84e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > > > @ 0xb1e63e -> 0xb84e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > > > @ 0xb1e8cd -> 0xb84e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > > @ 0x282118e -> 0xb84e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > > @ 0x282141d -> 0xb84e50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::impl$2::quantize_range
// 信息 ID: cab96e902755a6ca1ab4d576
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 18 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >; module=85, inline_record=860; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >; module=85, inline_record=1152; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >; module=85, inline_record=1384; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >; module=85, inline_record=1744; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >; module=85, inline_record=2064; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >; module=85, inline_record=2324; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > > >; module=580, inline_record=1712; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > > >; module=580, inline_record=2140; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::impl$3::assert_delta
// 信息 ID: 920c335b63bf307109c9369e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > > @ 0x2586a4 -> 0xb84ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > > @ 0x258829 -> 0xb84ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > > > @ 0xb1eab2 -> 0xb84ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > > > @ 0xb1ed69 -> 0xb84ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > > @ 0x2821602 -> 0xb84ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > > @ 0x28218b9 -> 0xb84ec0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::impl$3::quantize_range
// 信息 ID: 2cc41fedcaba59480dbcadc5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 18 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >; module=85, inline_record=2696; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >; module=85, inline_record=2984; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >; module=85, inline_record=3216; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >; module=85, inline_record=3560; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >; module=85, inline_record=3876; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >; module=85, inline_record=4136; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > > >; module=580, inline_record=4384; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >,country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > > > >; module=580, inline_record=4808; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

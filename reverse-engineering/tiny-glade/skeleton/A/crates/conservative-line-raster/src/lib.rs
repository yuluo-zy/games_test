// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/conservative-line-raster/src/lib.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: abe7152d1c06ece759ecf01ad71ba04c2e143b5f1f7a123780eaf08b9aaeb5f6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 479, 1420, 4484
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use conservative_line_raster; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::impl$0::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1420,"record_offset":264,"rva":17189792,"va":5385898912}
// 编译实例 RVAs: 0x1064ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4484,"record_offset":252,"rva":32208336,"va":5400917456}
// 编译实例 RVAs: 0x1eb75d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141064bb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15018926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400917456,"raw_hex":"e852d41aff","size":5,"soft_fail":false,"target_va":5385898928,"va":5400917849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141064c20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15019975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400917456,"raw_hex":"e839d01aff","size":5,"soft_fail":false,"target_va":5385899040,"va":5400919010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144b80
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2675730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400917456,"raw_hex":"e812d42800","size":5,"soft_fail":false,"target_va":5403593600,"va":5400917865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d70
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2676394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400917456,"raw_hex":"e8aad62800","size":5,"soft_fail":false,"target_va":5403594096,"va":5400917697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] conservative_line_raster::conservative_plot_line_low
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":4484,"record_offset":808,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":27084,"source_line":119,"archive":"_pdb/A/modules/04484.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":4484,"record_offset":696,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/04484.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":4484,"record_offset":640,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/04484.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::conservative_plot_line::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":4484,"record_offset":2424,"parent_record_offset":2020,"ancestor_record_offset":252,"inlinee":27091,"source_line":58,"archive":"_pdb/A/modules/04484.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::conservative_plot_line::closure$1
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":4484,"record_offset":4892,"parent_record_offset":4508,"ancestor_record_offset":252,"inlinee":27094,"source_line":66,"archive":"_pdb/A/modules/04484.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::impl$2::quantize_range
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":4484,"record_offset":952,"parent_record_offset":808,"ancestor_record_offset":252,"inlinee":27087,"source_line":93,"archive":"_pdb/A/modules/04484.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::impl$3::quantize_range
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":4484,"record_offset":3444,"parent_record_offset":3316,"ancestor_record_offset":252,"inlinee":27093,"source_line":105,"archive":"_pdb/A/modules/04484.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$72::lt
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":4484,"record_offset":1056,"parent_record_offset":1016,"ancestor_record_offset":252,"inlinee":10908,"source_line":1553,"archive":"_pdb/A/modules/04484.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$73::cmp
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":4484,"record_offset":1324,"parent_record_offset":808,"ancestor_record_offset":252,"inlinee":12560,"source_line":1565,"archive":"_pdb/A/modules/04484.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":4484,"record_offset":720,"parent_record_offset":696,"ancestor_record_offset":252,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/04484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$40::forward_unchecked
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":4484,"record_offset":1088,"parent_record_offset":1016,"ancestor_record_offset":252,"inlinee":10910,"source_line":191,"archive":"_pdb/A/modules/04484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":4484,"record_offset":1016,"parent_record_offset":984,"ancestor_record_offset":252,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/04484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":4484,"record_offset":984,"parent_record_offset":808,"ancestor_record_offset":252,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/04484.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::checked_add_unsigned
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\int_macros.rs
//     evidence={"module":4484,"record_offset":1112,"parent_record_offset":1088,"ancestor_record_offset":252,"inlinee":10911,"source_line":534,"archive":"_pdb/A/modules/04484.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\int_macros.rs
//     evidence={"module":4484,"record_offset":1160,"parent_record_offset":1136,"ancestor_record_offset":252,"inlinee":10913,"source_line":2138,"archive":"_pdb/A/modules/04484.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add_unsigned
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\int_macros.rs
//     evidence={"module":4484,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":252,"inlinee":10912,"source_line":2221,"archive":"_pdb/A/modules/04484.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=12
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":4484,"record_offset":1256,"parent_record_offset":1212,"ancestor_record_offset":252,"inlinee":24398,"source_line":192,"archive":"_pdb/A/modules/04484.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure$0
//     owner=private_source_ownership_candidate; count=12
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":4484,"record_offset":1212,"parent_record_offset":808,"ancestor_record_offset":252,"inlinee":27089,"source_line":51,"archive":"_pdb/A/modules/04484.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":4484,"record_offset":1296,"parent_record_offset":808,"ancestor_record_offset":252,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/04484.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::hypot
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":4484,"record_offset":920,"parent_record_offset":808,"ancestor_record_offset":252,"inlinee":27085,"source_line":690,"archive":"_pdb/A/modules/04484.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":479,"record_offset":248,"rva":6781824,"va":5375490944}
// 编译实例 RVAs: 0x677b80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141064bb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10407586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375490944,"raw_hex":"e8a2ce9e00","size":5,"soft_fail":false,"target_va":5385898928,"va":5375491337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141064c20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10406537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375490944,"raw_hex":"e889ca9e00","size":5,"soft_fail":false,"target_va":5385899040,"va":5375492498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144b80
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28102242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375490944,"raw_hex":"e862ceac01","size":5,"soft_fail":false,"target_va":5403593600,"va":5375491353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d70
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28102906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375490944,"raw_hex":"e8fad0ac01","size":5,"soft_fail":false,"target_va":5403594096,"va":5375491185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] conservative_line_raster::conservative_plot_line_low
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":479,"record_offset":1000,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":27084,"source_line":119,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":479,"record_offset":888,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":479,"record_offset":832,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::conservative_plot_line::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":479,"record_offset":2616,"parent_record_offset":2212,"ancestor_record_offset":248,"inlinee":27091,"source_line":58,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::conservative_plot_line::closure$1
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":479,"record_offset":5084,"parent_record_offset":4700,"ancestor_record_offset":248,"inlinee":27094,"source_line":66,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::impl$2::quantize_range
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":479,"record_offset":1144,"parent_record_offset":1000,"ancestor_record_offset":248,"inlinee":27087,"source_line":93,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] conservative_line_raster::impl$3::quantize_range
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":479,"record_offset":3636,"parent_record_offset":3508,"ancestor_record_offset":248,"inlinee":27093,"source_line":105,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$72::lt
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":479,"record_offset":1248,"parent_record_offset":1208,"ancestor_record_offset":248,"inlinee":10908,"source_line":1553,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$73::cmp
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":479,"record_offset":1516,"parent_record_offset":1000,"ancestor_record_offset":248,"inlinee":12560,"source_line":1565,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":479,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":248,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$40::forward_unchecked
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":479,"record_offset":1280,"parent_record_offset":1208,"ancestor_record_offset":248,"inlinee":10910,"source_line":191,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":479,"record_offset":1208,"parent_record_offset":1176,"ancestor_record_offset":248,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":479,"record_offset":1176,"parent_record_offset":1000,"ancestor_record_offset":248,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::checked_add_unsigned
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\int_macros.rs
//     evidence={"module":479,"record_offset":1304,"parent_record_offset":1280,"ancestor_record_offset":248,"inlinee":10911,"source_line":534,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\int_macros.rs
//     evidence={"module":479,"record_offset":1352,"parent_record_offset":1328,"ancestor_record_offset":248,"inlinee":10913,"source_line":2138,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add_unsigned
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\int_macros.rs
//     evidence={"module":479,"record_offset":1328,"parent_record_offset":1304,"ancestor_record_offset":248,"inlinee":10912,"source_line":2221,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=12
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":479,"record_offset":1448,"parent_record_offset":1404,"ancestor_record_offset":248,"inlinee":24398,"source_line":192,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure$0
//     owner=private_source_ownership_candidate; count=12
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":479,"record_offset":1404,"parent_record_offset":1000,"ancestor_record_offset":248,"inlinee":27089,"source_line":51,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":479,"record_offset":1488,"parent_record_offset":1000,"ancestor_record_offset":248,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::hypot
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":479,"record_offset":1112,"parent_record_offset":1000,"ancestor_record_offset":248,"inlinee":27085,"source_line":690,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::conservative_plot_line::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69D3
// 首条位置: {"module":479,"inlinee":27091}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure$0
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":479,"record_offset":2644,"parent_record_offset":2616,"ancestor_record_offset":2616,"inlinee":27089,"source_line":51,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":479,"record_offset":2684,"parent_record_offset":2644,"ancestor_record_offset":2616,"inlinee":24398,"source_line":192,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::conservative_plot_line::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69D6
// 首条位置: {"module":479,"inlinee":27094}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure$0
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":479,"record_offset":5112,"parent_record_offset":5084,"ancestor_record_offset":5084,"inlinee":27089,"source_line":51,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":479,"record_offset":5152,"parent_record_offset":5112,"ancestor_record_offset":5084,"inlinee":24398,"source_line":192,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::impl$2::quantize_range
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 93
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69CF
// 首条位置: {"module":479,"inlinee":27087}
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
// 首条位置: {"module":1420,"record_offset":380,"rva":17189808,"va":5385898928}
// 编译实例 RVAs: 0x1064bb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17079912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385898928,"raw_hex":"e8689e0401","size":5,"soft_fail":false,"target_va":5402978944,"va":5385899027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1420,"record_offset":512,"parent_record_offset":380,"ancestor_record_offset":380,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01420.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::impl$3::quantize_range
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69D5
// 首条位置: {"module":479,"inlinee":27093}
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
// 首条位置: {"module":1420,"record_offset":544,"rva":17189920,"va":5385899040}
// 编译实例 RVAs: 0x1064c20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17079800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385899040,"raw_hex":"e8f89d0401","size":5,"soft_fail":false,"target_va":5402978944,"va":5385899139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1420,"record_offset":676,"parent_record_offset":544,"ancestor_record_offset":544,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01420.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: conservative_line_raster::conservative_plot_line_low
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 119
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69CC
// 首条位置: {"module":479,"inlinee":27084}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] conservative_line_raster::conservative_plot_line::closure$0
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":479,"record_offset":2616,"parent_record_offset":2212,"ancestor_record_offset":2212,"inlinee":27091,"source_line":58,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] conservative_line_raster::conservative_plot_line::closure$1
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":479,"record_offset":5084,"parent_record_offset":4700,"ancestor_record_offset":4700,"inlinee":27094,"source_line":66,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] conservative_line_raster::impl$2::quantize_range
//     owner=private_source_ownership_candidate; count=12
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":479,"record_offset":1144,"parent_record_offset":1000,"ancestor_record_offset":1000,"inlinee":27087,"source_line":93,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] conservative_line_raster::impl$3::quantize_range
//     owner=private_source_ownership_candidate; count=12
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/conservative-line-raster/src/lib.rs
//     evidence={"module":479,"record_offset":3636,"parent_record_offset":3508,"ancestor_record_offset":3508,"inlinee":27093,"source_line":105,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$73::cmp
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":479,"record_offset":1516,"parent_record_offset":1000,"ancestor_record_offset":1000,"inlinee":12560,"source_line":1565,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":479,"record_offset":1176,"parent_record_offset":1000,"ancestor_record_offset":1000,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure$0
//     owner=private_source_ownership_candidate; count=12
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":479,"record_offset":1404,"parent_record_offset":1000,"ancestor_record_offset":1000,"inlinee":27089,"source_line":51,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":479,"record_offset":1488,"parent_record_offset":1000,"ancestor_record_offset":1000,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::hypot
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":479,"record_offset":1112,"parent_record_offset":1000,"ancestor_record_offset":1000,"inlinee":27085,"source_line":690,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$72::lt
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":479,"record_offset":1248,"parent_record_offset":1208,"ancestor_record_offset":1000,"inlinee":10908,"source_line":1553,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$40::forward_unchecked
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":479,"record_offset":1280,"parent_record_offset":1208,"ancestor_record_offset":1000,"inlinee":10910,"source_line":191,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":479,"record_offset":1208,"parent_record_offset":1176,"ancestor_record_offset":1000,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::checked_add_unsigned
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\int_macros.rs
//     evidence={"module":479,"record_offset":1304,"parent_record_offset":1280,"ancestor_record_offset":1000,"inlinee":10911,"source_line":534,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\int_macros.rs
//     evidence={"module":479,"record_offset":1352,"parent_record_offset":1328,"ancestor_record_offset":1000,"inlinee":10913,"source_line":2138,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add_unsigned
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\int_macros.rs
//     evidence={"module":479,"record_offset":1328,"parent_record_offset":1304,"ancestor_record_offset":1000,"inlinee":10912,"source_line":2221,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=24
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":479,"record_offset":1448,"parent_record_offset":1404,"ancestor_record_offset":1000,"inlinee":24398,"source_line":192,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure$0
//     owner=private_source_ownership_candidate; count=12
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":479,"record_offset":2644,"parent_record_offset":2616,"ancestor_record_offset":2212,"inlinee":27089,"source_line":51,"archive":"_pdb/A/modules/00479.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: conservative_line_raster::conservative_plot_line::closure$0
// 信息 ID: d7e4642fc6c184809375f591
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "conservative_line_raster::conservative_plot_line", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=2616; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=3032; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=3392; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >; module=4484, inline_record=2424; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >; module=4484, inline_record=2840; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >; module=4484, inline_record=3200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::conservative_plot_line::closure$1
// 信息 ID: a64c0c021ecd3c66178b3f42
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "conservative_line_raster::conservative_plot_line", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=5084; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=5500; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=5868; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >; module=4484, inline_record=4892; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >; module=4484, inline_record=5308; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >; module=4484, inline_record=5676; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >
// 信息 ID: 939b6134c7e39c7938ffff7b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0 -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0>"]; 字段布局 = unknown。
// [类型关系] conservative_line_raster::conservative_plot_line -> ["country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > @ 0x1db7802 -> 0x1eb75d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >
// 信息 ID: 42793f935cfe48b110442c12
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster::closure_env$1 -> ["country_core::resources::world_raster_defs::PathRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$2"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0 -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> >"]; 字段布局 = unknown。
// [类型关系] conservative_line_raster::conservative_plot_line -> ["country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > @ 0x5a0ad2 -> 0x677b80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::conservative_plot_line_low
// 信息 ID: ef7d9edbbc3f2ec7c1e805e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=1000; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=2212; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=3508; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=4700; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >; module=4484, inline_record=808; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >; module=4484, inline_record=2020; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >; module=4484, inline_record=3316; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >; module=4484, inline_record=4508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::impl$0::from
// 信息 ID: eab12a48966145eb968c04e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > @ 0x5a0a6d -> 0x1064ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> > > | country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > @ 0x5a0ab2 -> 0x1064ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > @ 0x1db779d -> 0x1064ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > @ 0x1db77e2 -> 0x1064ba0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::impl$2::assert_delta
// 信息 ID: 656e7328b0dc694b121272fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > > @ 0x677d09 -> 0x1064bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > > @ 0x677fa1 -> 0x1064bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > @ 0x1eb7759 -> 0x1064bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > @ 0x1eb79f1 -> 0x1064bb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::impl$2::quantize_range
// 信息 ID: 2ee4122545235d0cd8e89d81
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=1144; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=1544; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=1876; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=2356; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=2784; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=3144; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >; module=4484, inline_record=952; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >; module=4484, inline_record=1352; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::impl$3::assert_delta
// 信息 ID: cd136ac5d13cf9cd7cecb4f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > > @ 0x678192 -> 0x1064c20 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > > @ 0x678449 -> 0x1064c20 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > @ 0x1eb7be2 -> 0x1064c20 (direct_call); 同地址别名未裁定。
// [静态直接引用] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > @ 0x1eb7e99 -> 0x1064c20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: conservative_line_raster::impl$3::quantize_range
// 信息 ID: 4f5c23fe021a73495f0e3de1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=3636; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=4032; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=4364; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=4828; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=5252; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=5612; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >; module=4484, inline_record=3444; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >; module=4484, inline_record=3840; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

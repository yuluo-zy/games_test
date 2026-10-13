// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/utils/dirty_tiles.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 62698b7817c6f1f8171478ac646cb2ca3abd59bbdd74dd964c19858329ea2113
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: dirty_tiles.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 417, 421, 448, 459, 479, 498, 1202, 1207, 1308, 1339, 1369, 1398, 2280, 2289, 4205, 4206, 4484
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use ahash; // namespace candidate
//   package identities: [{"name":"ahash","source":"PDB_registry_path_only","version":"0.8.11"}]
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_clutter; // namespace candidate
//   package identities: []
// use tabloid; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::impl$3::clone
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFDC3
// 首条位置: {"module":1398,"inlinee":64963}
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
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE7F6
// 首条位置: {"module":1202,"inlinee":59382}
// 编译实例 RVAs: 0xc96850
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21684811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381908560,"raw_hex":"e84be24a01","size":5,"soft_fail":false,"target_va":5403593392,"va":5381908576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":421,"record_offset":12204,"rva":5900576,"va":5374609696}
// 编译实例 RVAs: 0x5a0920, 0x1db7650
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140677b80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t880809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374609696,"raw_hex":"e8a9700d00","size":5,"soft_fail":false,"target_va":5375490944,"va":5374610130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140efcc70
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9814677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374609696,"raw_hex":"e895c29500","size":5,"soft_fail":false,"target_va":5384424560,"va":5374609878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141064ba0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11288878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374609696,"raw_hex":"e82e41ac00","size":5,"soft_fail":false,"target_va":5385898912,"va":5374610029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141eb75d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1048009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399869008,"raw_hex":"e8c9fd0f00","size":5,"soft_fail":false,"target_va":5400917456,"va":5399869442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d80
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28984121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374609696,"raw_hex":"e83943ba01","size":5,"soft_fail":false,"target_va":5403594112,"va":5374609986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":421,"record_offset":13296,"parent_record_offset":12204,"ancestor_record_offset":12204,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":421,"record_offset":13124,"parent_record_offset":12204,"ancestor_record_offset":12204,"inlinee":24386,"source_line":771,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$39::from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":421,"record_offset":13268,"parent_record_offset":12204,"ancestor_record_offset":12204,"inlinee":17202,"source_line":1266,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":421,"record_offset":12540,"parent_record_offset":12204,"ancestor_record_offset":12204,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":421,"record_offset":12892,"parent_record_offset":12204,"ancestor_record_offset":12204,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":421,"record_offset":12852,"parent_record_offset":12828,"ancestor_record_offset":12204,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":421,"record_offset":12596,"parent_record_offset":12564,"ancestor_record_offset":12204,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":421,"record_offset":12828,"parent_record_offset":12564,"ancestor_record_offset":12204,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":421,"record_offset":12768,"parent_record_offset":12744,"ancestor_record_offset":12204,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":421,"record_offset":12744,"parent_record_offset":12596,"ancestor_record_offset":12204,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":421,"record_offset":12708,"parent_record_offset":12684,"ancestor_record_offset":12204,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":421,"record_offset":12656,"parent_record_offset":12620,"ancestor_record_offset":12204,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":421,"record_offset":12684,"parent_record_offset":12620,"ancestor_record_offset":12204,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":421,"record_offset":12620,"parent_record_offset":12596,"ancestor_record_offset":12204,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::raster_stroke::impl$1::iter_points_xz::closure$0
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":421,"record_offset":12792,"parent_record_offset":12768,"ancestor_record_offset":12204,"inlinee":24390,"source_line":51,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":421,"record_offset":13152,"parent_record_offset":13124,"ancestor_record_offset":12204,"inlinee":24387,"source_line":232,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":421,"record_offset":12564,"parent_record_offset":12540,"ancestor_record_offset":12204,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":421,"record_offset":13176,"parent_record_offset":13152,"ancestor_record_offset":12204,"inlinee":24388,"source_line":292,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":421,"record_offset":7644,"rva":5900576,"va":5374609696}
// 编译实例 RVAs: 0x5a0920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140677b80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t880809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374609696,"raw_hex":"e8a9700d00","size":5,"soft_fail":false,"target_va":5375490944,"va":5374610130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140efcc70
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9814677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374609696,"raw_hex":"e895c29500","size":5,"soft_fail":false,"target_va":5384424560,"va":5374609878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141064ba0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11288878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374609696,"raw_hex":"e82e41ac00","size":5,"soft_fail":false,"target_va":5385898912,"va":5374610029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d80
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28984121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374609696,"raw_hex":"e83943ba01","size":5,"soft_fail":false,"target_va":5403594112,"va":5374609986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":421,"record_offset":8936,"parent_record_offset":7644,"ancestor_record_offset":7644,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":421,"record_offset":8764,"parent_record_offset":7644,"ancestor_record_offset":7644,"inlinee":24386,"source_line":771,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$39::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":421,"record_offset":8908,"parent_record_offset":7644,"ancestor_record_offset":7644,"inlinee":17202,"source_line":1266,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":421,"record_offset":8180,"parent_record_offset":7644,"ancestor_record_offset":7644,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":421,"record_offset":8532,"parent_record_offset":7644,"ancestor_record_offset":7644,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":421,"record_offset":8492,"parent_record_offset":8468,"ancestor_record_offset":7644,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":421,"record_offset":8236,"parent_record_offset":8204,"ancestor_record_offset":7644,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":421,"record_offset":8468,"parent_record_offset":8204,"ancestor_record_offset":7644,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":421,"record_offset":8408,"parent_record_offset":8384,"ancestor_record_offset":7644,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":421,"record_offset":8384,"parent_record_offset":8236,"ancestor_record_offset":7644,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":421,"record_offset":8348,"parent_record_offset":8324,"ancestor_record_offset":7644,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":421,"record_offset":8296,"parent_record_offset":8260,"ancestor_record_offset":7644,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":421,"record_offset":8324,"parent_record_offset":8260,"ancestor_record_offset":7644,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":421,"record_offset":8260,"parent_record_offset":8236,"ancestor_record_offset":7644,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":421,"record_offset":8432,"parent_record_offset":8408,"ancestor_record_offset":7644,"inlinee":24385,"source_line":269,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":421,"record_offset":8792,"parent_record_offset":8764,"ancestor_record_offset":7644,"inlinee":24387,"source_line":232,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":421,"record_offset":8204,"parent_record_offset":8180,"ancestor_record_offset":7644,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":421,"record_offset":8816,"parent_record_offset":8792,"ancestor_record_offset":7644,"inlinee":24388,"source_line":292,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":421,"record_offset":9168,"rva":5900576,"va":5374609696}
// 编译实例 RVAs: 0x5a0920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140677b80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t880809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374609696,"raw_hex":"e8a9700d00","size":5,"soft_fail":false,"target_va":5375490944,"va":5374610130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140efcc70
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9814677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374609696,"raw_hex":"e895c29500","size":5,"soft_fail":false,"target_va":5384424560,"va":5374609878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141064ba0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11288878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374609696,"raw_hex":"e82e41ac00","size":5,"soft_fail":false,"target_va":5385898912,"va":5374610029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d80
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28984121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374609696,"raw_hex":"e83943ba01","size":5,"soft_fail":false,"target_va":5403594112,"va":5374609986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":421,"record_offset":10452,"parent_record_offset":9168,"ancestor_record_offset":9168,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":421,"record_offset":10280,"parent_record_offset":9168,"ancestor_record_offset":9168,"inlinee":24386,"source_line":771,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$39::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":421,"record_offset":10424,"parent_record_offset":9168,"ancestor_record_offset":9168,"inlinee":17202,"source_line":1266,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":421,"record_offset":9696,"parent_record_offset":9168,"ancestor_record_offset":9168,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":421,"record_offset":10048,"parent_record_offset":9168,"ancestor_record_offset":9168,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":421,"record_offset":10008,"parent_record_offset":9984,"ancestor_record_offset":9168,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":421,"record_offset":9752,"parent_record_offset":9720,"ancestor_record_offset":9168,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":421,"record_offset":9984,"parent_record_offset":9720,"ancestor_record_offset":9168,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":421,"record_offset":9924,"parent_record_offset":9900,"ancestor_record_offset":9168,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":421,"record_offset":9900,"parent_record_offset":9752,"ancestor_record_offset":9168,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":421,"record_offset":9864,"parent_record_offset":9840,"ancestor_record_offset":9168,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":421,"record_offset":9812,"parent_record_offset":9776,"ancestor_record_offset":9168,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":421,"record_offset":9840,"parent_record_offset":9776,"ancestor_record_offset":9168,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":421,"record_offset":9776,"parent_record_offset":9752,"ancestor_record_offset":9168,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":421,"record_offset":9948,"parent_record_offset":9924,"ancestor_record_offset":9168,"inlinee":24385,"source_line":269,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":421,"record_offset":10308,"parent_record_offset":10280,"ancestor_record_offset":9168,"inlinee":24387,"source_line":232,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":421,"record_offset":9720,"parent_record_offset":9696,"ancestor_record_offset":9168,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":421,"record_offset":10332,"parent_record_offset":10308,"ancestor_record_offset":9168,"inlinee":24388,"source_line":292,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":421,"record_offset":10684,"rva":5900576,"va":5374609696}
// 编译实例 RVAs: 0x5a0920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140677b80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t880809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374609696,"raw_hex":"e8a9700d00","size":5,"soft_fail":false,"target_va":5375490944,"va":5374610130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140efcc70
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9814677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374609696,"raw_hex":"e895c29500","size":5,"soft_fail":false,"target_va":5384424560,"va":5374609878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141064ba0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11288878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374609696,"raw_hex":"e82e41ac00","size":5,"soft_fail":false,"target_va":5385898912,"va":5374610029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d80
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28984121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374609696,"raw_hex":"e83943ba01","size":5,"soft_fail":false,"target_va":5403594112,"va":5374609986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":421,"record_offset":11972,"parent_record_offset":10684,"ancestor_record_offset":10684,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":421,"record_offset":11800,"parent_record_offset":10684,"ancestor_record_offset":10684,"inlinee":24386,"source_line":771,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$39::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":421,"record_offset":11944,"parent_record_offset":10684,"ancestor_record_offset":10684,"inlinee":17202,"source_line":1266,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":421,"record_offset":11216,"parent_record_offset":10684,"ancestor_record_offset":10684,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":421,"record_offset":11568,"parent_record_offset":10684,"ancestor_record_offset":10684,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":421,"record_offset":11528,"parent_record_offset":11504,"ancestor_record_offset":10684,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":421,"record_offset":11272,"parent_record_offset":11240,"ancestor_record_offset":10684,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":421,"record_offset":11504,"parent_record_offset":11240,"ancestor_record_offset":10684,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":421,"record_offset":11444,"parent_record_offset":11420,"ancestor_record_offset":10684,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":421,"record_offset":11420,"parent_record_offset":11272,"ancestor_record_offset":10684,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":421,"record_offset":11384,"parent_record_offset":11360,"ancestor_record_offset":10684,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":421,"record_offset":11332,"parent_record_offset":11296,"ancestor_record_offset":10684,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":421,"record_offset":11360,"parent_record_offset":11296,"ancestor_record_offset":10684,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":421,"record_offset":11296,"parent_record_offset":11272,"ancestor_record_offset":10684,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_raster::replay_raster::closure$1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_raster.rs
//     evidence={"module":421,"record_offset":11468,"parent_record_offset":11444,"ancestor_record_offset":10684,"inlinee":24385,"source_line":269,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":421,"record_offset":11828,"parent_record_offset":11800,"ancestor_record_offset":10684,"inlinee":24387,"source_line":232,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":421,"record_offset":11240,"parent_record_offset":11216,"ancestor_record_offset":10684,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":421,"record_offset":11852,"parent_record_offset":11828,"ancestor_record_offset":10684,"inlinee":24388,"source_line":292,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=26
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69D1
// 首条位置: {"module":479,"inlinee":27089}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=24
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":479,"record_offset":1448,"parent_record_offset":1404,"ancestor_record_offset":1404,"inlinee":24398,"source_line":192,"archive":"_pdb/A/modules/00479.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":421,"record_offset":14756,"rva":5901968,"va":5374611088}
// 编译实例 RVAs: 0x5a0e90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28982656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374611088,"raw_hex":"e8803dba01","size":5,"soft_fail":false,"target_va":5403593968,"va":5374611307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d70
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28982877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374611088,"raw_hex":"e85d3eba01","size":5,"soft_fail":false,"target_va":5403594096,"va":5374611214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d80
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28982924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374611088,"raw_hex":"e88c3eba01","size":5,"soft_fail":false,"target_va":5403594112,"va":5374611183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28367696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374611088,"raw_hex":"e850dbb001","size":5,"soft_fail":false,"target_va":5402979344,"va":5374611643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":421,"record_offset":15836,"parent_record_offset":14756,"ancestor_record_offset":14756,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":421,"record_offset":16140,"parent_record_offset":14756,"ancestor_record_offset":14756,"inlinee":24400,"source_line":105,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":421,"record_offset":15508,"parent_record_offset":14756,"ancestor_record_offset":14756,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::ceil
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":421,"record_offset":15688,"parent_record_offset":14756,"ancestor_record_offset":14756,"inlinee":24393,"source_line":600,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":421,"record_offset":15320,"parent_record_offset":14756,"ancestor_record_offset":14756,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::floor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":421,"record_offset":15168,"parent_record_offset":14756,"ancestor_record_offset":14756,"inlinee":24391,"source_line":589,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":421,"record_offset":15024,"parent_record_offset":14756,"ancestor_record_offset":14756,"inlinee":24386,"source_line":771,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$60::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":421,"record_offset":15896,"parent_record_offset":15864,"ancestor_record_offset":14756,"inlinee":5261,"source_line":1553,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":421,"record_offset":15388,"parent_record_offset":15348,"ancestor_record_offset":14756,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":421,"record_offset":15416,"parent_record_offset":15348,"ancestor_record_offset":14756,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":421,"record_offset":15928,"parent_record_offset":15864,"ancestor_record_offset":14756,"inlinee":5988,"source_line":207,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":421,"record_offset":15864,"parent_record_offset":15836,"ancestor_record_offset":14756,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":421,"record_offset":15952,"parent_record_offset":15928,"ancestor_record_offset":14756,"inlinee":5989,"source_line":520,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::bitmap_intersects
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":421,"record_offset":16180,"parent_record_offset":16140,"ancestor_record_offset":14756,"inlinee":24401,"source_line":209,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::ceil
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":421,"record_offset":15720,"parent_record_offset":15688,"ancestor_record_offset":14756,"inlinee":24394,"source_line":212,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::floor
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":421,"record_offset":15196,"parent_record_offset":15168,"ancestor_record_offset":14756,"inlinee":24392,"source_line":217,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":421,"record_offset":15052,"parent_record_offset":15024,"ancestor_record_offset":14756,"inlinee":24387,"source_line":232,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":421,"record_offset":15348,"parent_record_offset":15320,"ancestor_record_offset":14756,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":421,"record_offset":15744,"parent_record_offset":15720,"ancestor_record_offset":14756,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":421,"record_offset":15220,"parent_record_offset":15196,"ancestor_record_offset":14756,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":421,"record_offset":15076,"parent_record_offset":15052,"ancestor_record_offset":14756,"inlinee":24388,"source_line":292,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":421,"record_offset":13528,"rva":5901120,"va":5374610240}
// 编译实例 RVAs: 0x5a0b40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28983326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374610240,"raw_hex":"e81e40ba01","size":5,"soft_fail":false,"target_va":5403593968,"va":5374610637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d70
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28983713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374610240,"raw_hex":"e8a141ba01","size":5,"soft_fail":false,"target_va":5403594096,"va":5374610378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d80
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28983754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374610240,"raw_hex":"e8ca41ba01","size":5,"soft_fail":false,"target_va":5403594112,"va":5374610353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28368288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374610240,"raw_hex":"e8a0ddb001","size":5,"soft_fail":false,"target_va":5402979344,"va":5374611051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":421,"record_offset":14440,"parent_record_offset":13528,"ancestor_record_offset":13528,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":421,"record_offset":14600,"parent_record_offset":13528,"ancestor_record_offset":13528,"inlinee":24396,"source_line":96,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":421,"record_offset":14096,"parent_record_offset":13528,"ancestor_record_offset":13528,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::ceil
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":421,"record_offset":14212,"parent_record_offset":13528,"ancestor_record_offset":13528,"inlinee":24393,"source_line":600,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":421,"record_offset":13964,"parent_record_offset":13528,"ancestor_record_offset":13528,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::floor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":421,"record_offset":13880,"parent_record_offset":13528,"ancestor_record_offset":13528,"inlinee":24391,"source_line":589,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":421,"record_offset":13796,"parent_record_offset":13528,"ancestor_record_offset":13528,"inlinee":24386,"source_line":771,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$60::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":421,"record_offset":14564,"parent_record_offset":14468,"ancestor_record_offset":13528,"inlinee":5261,"source_line":1553,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":421,"record_offset":14028,"parent_record_offset":13992,"ancestor_record_offset":13528,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":421,"record_offset":14060,"parent_record_offset":13992,"ancestor_record_offset":13528,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":421,"record_offset":14504,"parent_record_offset":14468,"ancestor_record_offset":13528,"inlinee":5988,"source_line":207,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":421,"record_offset":14468,"parent_record_offset":14440,"ancestor_record_offset":13528,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":421,"record_offset":14528,"parent_record_offset":14504,"ancestor_record_offset":13528,"inlinee":5989,"source_line":520,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":421,"record_offset":14628,"parent_record_offset":14600,"ancestor_record_offset":13528,"inlinee":24398,"source_line":192,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::ceil
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":421,"record_offset":14236,"parent_record_offset":14212,"ancestor_record_offset":13528,"inlinee":24394,"source_line":212,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::floor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":421,"record_offset":13904,"parent_record_offset":13880,"ancestor_record_offset":13528,"inlinee":24392,"source_line":217,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":421,"record_offset":13820,"parent_record_offset":13796,"ancestor_record_offset":13528,"inlinee":24387,"source_line":232,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":421,"record_offset":13992,"parent_record_offset":13964,"ancestor_record_offset":13528,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":421,"record_offset":14260,"parent_record_offset":14236,"ancestor_record_offset":13528,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":421,"record_offset":13928,"parent_record_offset":13904,"ancestor_record_offset":13528,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":421,"record_offset":13844,"parent_record_offset":13820,"ancestor_record_offset":13528,"inlinee":24388,"source_line":292,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::clutter::impl$1::add_region_to_lazy_grid::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2289,"record_offset":4416,"rva":21935520,"va":5390644640}
// 编译实例 RVAs: 0x14eb5a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12949044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390644640,"raw_hex":"e83496c500","size":5,"soft_fail":false,"target_va":5403593968,"va":5390644919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d70
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12949307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390644640,"raw_hex":"e83b97c500","size":5,"soft_fail":false,"target_va":5403594096,"va":5390644784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d80
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12949354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390644640,"raw_hex":"e86a97c500","size":5,"soft_fail":false,"target_va":5403594112,"va":5390644753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-19340169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390644640,"raw_hex":"e877e4d8fe","size":5,"soft_fail":false,"target_va":5371305056,"va":5390645220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12334023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390644640,"raw_hex":"e8c733bc00","size":5,"soft_fail":false,"target_va":5402979344,"va":5390645316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2289,"record_offset":5624,"parent_record_offset":4416,"ancestor_record_offset":4416,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":5160,"parent_record_offset":4416,"ancestor_record_offset":4416,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::ceil
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":5336,"parent_record_offset":4416,"ancestor_record_offset":4416,"inlinee":24393,"source_line":600,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":4972,"parent_record_offset":4416,"ancestor_record_offset":4416,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::floor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":4816,"parent_record_offset":4416,"ancestor_record_offset":4416,"inlinee":24391,"source_line":589,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":4672,"parent_record_offset":4416,"ancestor_record_offset":4416,"inlinee":24386,"source_line":771,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::clutter::impl$1::add_region_to_lazy_grid::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/mod.rs
//     evidence={"module":2289,"record_offset":5876,"parent_record_offset":4416,"ancestor_record_offset":4416,"inlinee":81288,"source_line":268,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2289,"record_offset":5976,"parent_record_offset":5952,"ancestor_record_offset":4416,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2289,"record_offset":5952,"parent_record_offset":5912,"ancestor_record_offset":4416,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2289,"record_offset":5912,"parent_record_offset":5876,"ancestor_record_offset":4416,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$60::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2289,"record_offset":5684,"parent_record_offset":5652,"ancestor_record_offset":4416,"inlinee":5261,"source_line":1553,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2289,"record_offset":5040,"parent_record_offset":5000,"ancestor_record_offset":4416,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2289,"record_offset":5068,"parent_record_offset":5000,"ancestor_record_offset":4416,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2289,"record_offset":5716,"parent_record_offset":5652,"ancestor_record_offset":4416,"inlinee":5988,"source_line":207,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2289,"record_offset":5652,"parent_record_offset":5624,"ancestor_record_offset":4416,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2289,"record_offset":5740,"parent_record_offset":5716,"ancestor_record_offset":4416,"inlinee":5989,"source_line":520,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2289,"record_offset":6008,"parent_record_offset":5912,"ancestor_record_offset":4416,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::ceil
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":2289,"record_offset":5364,"parent_record_offset":5336,"ancestor_record_offset":4416,"inlinee":24394,"source_line":212,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::floor
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":2289,"record_offset":4848,"parent_record_offset":4816,"ancestor_record_offset":4416,"inlinee":24392,"source_line":217,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":2289,"record_offset":4700,"parent_record_offset":4672,"ancestor_record_offset":4416,"inlinee":24387,"source_line":232,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":5000,"parent_record_offset":4972,"ancestor_record_offset":4416,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2289,"record_offset":5388,"parent_record_offset":5364,"ancestor_record_offset":4416,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2289,"record_offset":4872,"parent_record_offset":4848,"ancestor_record_offset":4416,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2289,"record_offset":4724,"parent_record_offset":4700,"ancestor_record_offset":4416,"inlinee":24388,"source_line":292,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::clutter::impl$1::remove_region_from_lazy_grid::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2289,"record_offset":2192,"rva":21934592,"va":5390643712}
// 编译实例 RVAs: 0x14eb200
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12949972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390643712,"raw_hex":"e8d499c500","size":5,"soft_fail":false,"target_va":5403593968,"va":5390643991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d70
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12950235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390643712,"raw_hex":"e8db9ac500","size":5,"soft_fail":false,"target_va":5403594096,"va":5390643856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d80
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12950282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390643712,"raw_hex":"e80a9bc500","size":5,"soft_fail":false,"target_va":5403594112,"va":5390643825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12334718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390643712,"raw_hex":"e87e36bc00","size":5,"soft_fail":false,"target_va":5402979344,"va":5390644621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2289,"record_offset":3404,"parent_record_offset":2192,"ancestor_record_offset":2192,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":2940,"parent_record_offset":2192,"ancestor_record_offset":2192,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::ceil
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":3116,"parent_record_offset":2192,"ancestor_record_offset":2192,"inlinee":24393,"source_line":600,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":2752,"parent_record_offset":2192,"ancestor_record_offset":2192,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::floor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":2596,"parent_record_offset":2192,"ancestor_record_offset":2192,"inlinee":24391,"source_line":589,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":2452,"parent_record_offset":2192,"ancestor_record_offset":2192,"inlinee":24386,"source_line":771,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::clutter::impl$1::remove_region_from_lazy_grid::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/mod.rs
//     evidence={"module":2289,"record_offset":3656,"parent_record_offset":2192,"ancestor_record_offset":2192,"inlinee":81286,"source_line":283,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2289,"record_offset":4168,"parent_record_offset":4144,"ancestor_record_offset":2192,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2289,"record_offset":4144,"parent_record_offset":4068,"ancestor_record_offset":2192,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2289,"record_offset":3876,"parent_record_offset":3720,"ancestor_record_offset":2192,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2289,"record_offset":3692,"parent_record_offset":3656,"ancestor_record_offset":2192,"inlinee":6475,"source_line":1653,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2289,"record_offset":3720,"parent_record_offset":3692,"ancestor_record_offset":2192,"inlinee":6476,"source_line":1679,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2289,"record_offset":3836,"parent_record_offset":3812,"ancestor_record_offset":2192,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2289,"record_offset":3936,"parent_record_offset":3904,"ancestor_record_offset":2192,"inlinee":6480,"source_line":1657,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2289,"record_offset":3812,"parent_record_offset":3788,"ancestor_record_offset":2192,"inlinee":6487,"source_line":1707,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::process_loop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2289,"record_offset":3904,"parent_record_offset":3720,"ancestor_record_offset":2192,"inlinee":6478,"source_line":1727,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2289,"record_offset":3992,"parent_record_offset":3964,"ancestor_record_offset":2192,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$60::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2289,"record_offset":3464,"parent_record_offset":3432,"ancestor_record_offset":2192,"inlinee":5261,"source_line":1553,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2289,"record_offset":2820,"parent_record_offset":2780,"ancestor_record_offset":2192,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2289,"record_offset":2848,"parent_record_offset":2780,"ancestor_record_offset":2192,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2289,"record_offset":4360,"parent_record_offset":4068,"ancestor_record_offset":2192,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2289,"record_offset":3496,"parent_record_offset":3432,"ancestor_record_offset":2192,"inlinee":5988,"source_line":207,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2289,"record_offset":3432,"parent_record_offset":3404,"ancestor_record_offset":2192,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2289,"record_offset":3764,"parent_record_offset":3720,"ancestor_record_offset":2192,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2289,"record_offset":3520,"parent_record_offset":3496,"ancestor_record_offset":2192,"inlinee":5989,"source_line":520,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2289,"record_offset":3788,"parent_record_offset":3764,"ancestor_record_offset":2192,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::ceil
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":2289,"record_offset":3144,"parent_record_offset":3116,"ancestor_record_offset":2192,"inlinee":24394,"source_line":212,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::floor
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":2289,"record_offset":2628,"parent_record_offset":2596,"ancestor_record_offset":2192,"inlinee":24392,"source_line":217,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":2289,"record_offset":2480,"parent_record_offset":2452,"ancestor_record_offset":2192,"inlinee":24387,"source_line":232,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":2780,"parent_record_offset":2752,"ancestor_record_offset":2192,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2289,"record_offset":3168,"parent_record_offset":3144,"ancestor_record_offset":2192,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2289,"record_offset":2652,"parent_record_offset":2628,"ancestor_record_offset":2192,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2289,"record_offset":2504,"parent_record_offset":2480,"ancestor_record_offset":2192,"inlinee":24388,"source_line":292,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::clutter::impl$1::remove_region_from_lazy_grid::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/mod.rs
//     evidence={"module":2289,"record_offset":3964,"parent_record_offset":3936,"ancestor_record_offset":2192,"inlinee":81260,"source_line":287,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::clutter::impl$22::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/mod.rs
//     evidence={"module":2289,"record_offset":4020,"parent_record_offset":3992,"ancestor_record_offset":2192,"inlinee":48733,"source_line":331,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::clutter::read_bush_positions_from_gpu::impl$0::bushes_near_point::closure_env$0<system_clutter::clutter::populate_clutter_regions::filter_clutter_object::closure_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2289,"record_offset":6048,"rva":21936208,"va":5390645328}
// 编译实例 RVAs: 0x14eb850
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14064a3d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15341171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390645328,"raw_hex":"e88de915ff","size":5,"soft_fail":false,"target_va":5375304656,"va":5390645822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12948353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390645328,"raw_hex":"e88193c500","size":5,"soft_fail":false,"target_va":5403593968,"va":5390645610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d70
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12948616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390645328,"raw_hex":"e88894c500","size":5,"soft_fail":false,"target_va":5403594096,"va":5390645475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d80
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12948663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390645328,"raw_hex":"e8b794c500","size":5,"soft_fail":false,"target_va":5403594112,"va":5390645444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12333258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390645328,"raw_hex":"e8ca30bc00","size":5,"soft_fail":false,"target_va":5402979344,"va":5390646081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2289,"record_offset":7368,"parent_record_offset":6048,"ancestor_record_offset":6048,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":6904,"parent_record_offset":6048,"ancestor_record_offset":6048,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::ceil
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":7080,"parent_record_offset":6048,"ancestor_record_offset":6048,"inlinee":24393,"source_line":600,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":6716,"parent_record_offset":6048,"ancestor_record_offset":6048,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::floor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":6560,"parent_record_offset":6048,"ancestor_record_offset":6048,"inlinee":24391,"source_line":589,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":6416,"parent_record_offset":6048,"ancestor_record_offset":6048,"inlinee":24386,"source_line":771,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::clutter::read_bush_positions_from_gpu::impl$0::bushes_near_point::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/read_bush_positions_from_gpu.rs
//     evidence={"module":2289,"record_offset":7640,"parent_record_offset":6048,"ancestor_record_offset":6048,"inlinee":81290,"source_line":46,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2289,"record_offset":7684,"parent_record_offset":7640,"ancestor_record_offset":6048,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$60::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2289,"record_offset":7444,"parent_record_offset":7404,"ancestor_record_offset":6048,"inlinee":5261,"source_line":1553,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2289,"record_offset":6784,"parent_record_offset":6744,"ancestor_record_offset":6048,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2289,"record_offset":6812,"parent_record_offset":6744,"ancestor_record_offset":6048,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2289,"record_offset":7480,"parent_record_offset":7404,"ancestor_record_offset":6048,"inlinee":5988,"source_line":207,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2289,"record_offset":7404,"parent_record_offset":7368,"ancestor_record_offset":6048,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2289,"record_offset":7504,"parent_record_offset":7480,"ancestor_record_offset":6048,"inlinee":5989,"source_line":520,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2289,"record_offset":7932,"parent_record_offset":7904,"ancestor_record_offset":6048,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2289,"record_offset":7812,"parent_record_offset":7780,"ancestor_record_offset":6048,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2289,"record_offset":7712,"parent_record_offset":7684,"ancestor_record_offset":6048,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2289,"record_offset":7740,"parent_record_offset":7712,"ancestor_record_offset":6048,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2289,"record_offset":7780,"parent_record_offset":7640,"ancestor_record_offset":6048,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::ceil
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":2289,"record_offset":7108,"parent_record_offset":7080,"ancestor_record_offset":6048,"inlinee":24394,"source_line":212,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::floor
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":2289,"record_offset":6592,"parent_record_offset":6560,"ancestor_record_offset":6048,"inlinee":24392,"source_line":217,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":2289,"record_offset":6444,"parent_record_offset":6416,"ancestor_record_offset":6048,"inlinee":24387,"source_line":232,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":7880,"parent_record_offset":7844,"ancestor_record_offset":6048,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":7960,"parent_record_offset":7904,"ancestor_record_offset":6048,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":6744,"parent_record_offset":6716,"ancestor_record_offset":6048,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":7904,"parent_record_offset":7880,"ancestor_record_offset":6048,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2289,"record_offset":7132,"parent_record_offset":7108,"ancestor_record_offset":6048,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2289,"record_offset":6616,"parent_record_offset":6592,"ancestor_record_offset":6048,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2289,"record_offset":6468,"parent_record_offset":6444,"ancestor_record_offset":6048,"inlinee":24388,"source_line":292,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2289,"record_offset":7992,"parent_record_offset":7960,"ancestor_record_offset":6048,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::clutter::populate_clutter_regions::filter_clutter_object::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/populate_clutter_regions.rs
//     evidence={"module":2289,"record_offset":7844,"parent_record_offset":7640,"ancestor_record_offset":6048,"inlinee":81292,"source_line":597,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::clutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2289,"record_offset":8044,"rva":21936976,"va":5390646096}
// 编译实例 RVAs: 0x14ebb50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14064a3d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15341956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390646096,"raw_hex":"e87ce615ff","size":5,"soft_fail":false,"target_va":5375304656,"va":5390646607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12947592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390646096,"raw_hex":"e88890c500","size":5,"soft_fail":false,"target_va":5403593968,"va":5390646371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d70
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12947851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390646096,"raw_hex":"e88b91c500","size":5,"soft_fail":false,"target_va":5403594096,"va":5390646240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d80
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12947898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390646096,"raw_hex":"e8ba91c500","size":5,"soft_fail":false,"target_va":5403594112,"va":5390646209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-19339542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390646096,"raw_hex":"e8eae6d8fe","size":5,"soft_fail":false,"target_va":5371307104,"va":5390646641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12332605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390646096,"raw_hex":"e83d2ebc00","size":5,"soft_fail":false,"target_va":5402979344,"va":5390646734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2289,"record_offset":9280,"parent_record_offset":8044,"ancestor_record_offset":8044,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":8816,"parent_record_offset":8044,"ancestor_record_offset":8044,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::ceil
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":8992,"parent_record_offset":8044,"ancestor_record_offset":8044,"inlinee":24393,"source_line":600,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":8628,"parent_record_offset":8044,"ancestor_record_offset":8044,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::floor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":8472,"parent_record_offset":8044,"ancestor_record_offset":8044,"inlinee":24391,"source_line":589,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::mul_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":8328,"parent_record_offset":8044,"ancestor_record_offset":8044,"inlinee":24386,"source_line":771,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::clutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/read_bush_positions_from_gpu.rs
//     evidence={"module":2289,"record_offset":9532,"parent_record_offset":8044,"ancestor_record_offset":8044,"inlinee":81293,"source_line":142,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2289,"record_offset":9628,"parent_record_offset":9604,"ancestor_record_offset":8044,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2289,"record_offset":9604,"parent_record_offset":9564,"ancestor_record_offset":8044,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2289,"record_offset":9564,"parent_record_offset":9532,"ancestor_record_offset":8044,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2289,"record_offset":9692,"parent_record_offset":9532,"ancestor_record_offset":8044,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$60::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2289,"record_offset":9340,"parent_record_offset":9308,"ancestor_record_offset":8044,"inlinee":5261,"source_line":1553,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2289,"record_offset":8696,"parent_record_offset":8656,"ancestor_record_offset":8044,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2289,"record_offset":8724,"parent_record_offset":8656,"ancestor_record_offset":8044,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2289,"record_offset":9372,"parent_record_offset":9308,"ancestor_record_offset":8044,"inlinee":5988,"source_line":207,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2289,"record_offset":9308,"parent_record_offset":9280,"ancestor_record_offset":8044,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2289,"record_offset":9396,"parent_record_offset":9372,"ancestor_record_offset":8044,"inlinee":5989,"source_line":520,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2289,"record_offset":9660,"parent_record_offset":9564,"ancestor_record_offset":8044,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2289,"record_offset":9720,"parent_record_offset":9692,"ancestor_record_offset":8044,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2289,"record_offset":9748,"parent_record_offset":9720,"ancestor_record_offset":8044,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::ceil
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":2289,"record_offset":9020,"parent_record_offset":8992,"ancestor_record_offset":8044,"inlinee":24394,"source_line":212,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::floor
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":2289,"record_offset":8504,"parent_record_offset":8472,"ancestor_record_offset":8044,"inlinee":24392,"source_line":217,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::mul_add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":2289,"record_offset":8356,"parent_record_offset":8328,"ancestor_record_offset":8044,"inlinee":24387,"source_line":232,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2289,"record_offset":8656,"parent_record_offset":8628,"ancestor_record_offset":8044,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2289,"record_offset":9044,"parent_record_offset":9020,"ancestor_record_offset":8044,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2289,"record_offset":8528,"parent_record_offset":8504,"ancestor_record_offset":8044,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2289,"record_offset":8380,"parent_record_offset":8356,"ancestor_record_offset":8044,"inlinee":24388,"source_line":292,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_rect
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5DB4
// 首条位置: {"module":417,"inlinee":23988}
// 编译实例 RVAs: 0xc96870
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x1405a0b40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-7298366","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381908592,"raw_hex":"e9c2a290ff","size":5,"soft_fail":false,"target_va":5374610240,"va":5381908601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F4C
// 首条位置: {"module":421,"inlinee":24396}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":421,"record_offset":14628,"parent_record_offset":14600,"ancestor_record_offset":14600,"inlinee":24398,"source_line":192,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::intersects_world_space_rect
// 记录形式: compiled_procedure; instances=1
// 源码行: 103
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1207,"record_offset":15752,"rva":13199488,"va":5381908608}
// 编译实例 RVAs: 0xc96880
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405a0e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7297548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381908608,"raw_hex":"e8f4a590ff","size":5,"soft_fail":false,"target_va":5374611088,"va":5381908631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F50
// 首条位置: {"module":421,"inlinee":24400}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::bitmap_intersects
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":421,"record_offset":16180,"parent_record_offset":16140,"ancestor_record_offset":16140,"inlinee":24401,"source_line":209,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::clear
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=22
// 源码行: 132, 133
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x649F
// 首条位置: {"module":448,"inlinee":25759}
// 编译实例 RVAs: 0xc968b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t21684723","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381908656,"raw_hex":"e9f3e14a01","size":5,"soft_fail":false,"target_va":5403593392,"va":5381908664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::iter_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":421,"record_offset":16384,"rva":5902560,"va":5374611680}
// 编译实例 RVAs: 0x5a10e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4998461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374611680,"raw_hex":"e8c3bab3ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5374611976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a6f6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5038910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374611680,"raw_hex":"e83ee34c00","size":5,"soft_fail":false,"target_va":5379651280,"va":5374612365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28366618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374611680,"raw_hex":"e81ad7b001","size":5,"soft_fail":false,"target_va":5402979152,"va":5374612529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28366834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374611680,"raw_hex":"e8f2d7b001","size":5,"soft_fail":false,"target_va":5402979344,"va":5374612505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":421,"record_offset":16644,"parent_record_offset":16384,"ancestor_record_offset":16384,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":421,"record_offset":16788,"parent_record_offset":16384,"ancestor_record_offset":16384,"inlinee":6317,"source_line":169,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/publish_public_wall_state.rs
//     evidence={"module":421,"record_offset":16816,"parent_record_offset":16384,"ancestor_record_offset":16384,"inlinee":24412,"source_line":183,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":421,"record_offset":16916,"parent_record_offset":16888,"ancestor_record_offset":16384,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":421,"record_offset":18020,"parent_record_offset":17996,"ancestor_record_offset":16384,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":421,"record_offset":17488,"parent_record_offset":17464,"ancestor_record_offset":16384,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":421,"record_offset":17572,"parent_record_offset":17548,"ancestor_record_offset":16384,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":421,"record_offset":17512,"parent_record_offset":17488,"ancestor_record_offset":16384,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":421,"record_offset":16668,"parent_record_offset":16644,"ancestor_record_offset":16384,"inlinee":8603,"source_line":47,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":421,"record_offset":17684,"parent_record_offset":17660,"ancestor_record_offset":16384,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":421,"record_offset":16752,"parent_record_offset":16668,"ancestor_record_offset":16384,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":421,"record_offset":19364,"parent_record_offset":16816,"ancestor_record_offset":16384,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":421,"record_offset":17932,"parent_record_offset":17908,"ancestor_record_offset":16384,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":421,"record_offset":17824,"parent_record_offset":17800,"ancestor_record_offset":16384,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":421,"record_offset":16720,"parent_record_offset":16692,"ancestor_record_offset":16384,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":16948,"parent_record_offset":16916,"ancestor_record_offset":16384,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":16980,"parent_record_offset":16948,"ancestor_record_offset":16384,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":421,"record_offset":16692,"parent_record_offset":16668,"ancestor_record_offset":16384,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":421,"record_offset":18044,"parent_record_offset":18020,"ancestor_record_offset":16384,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":421,"record_offset":17152,"parent_record_offset":16816,"ancestor_record_offset":16384,"inlinee":22068,"source_line":100,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":421,"record_offset":18296,"parent_record_offset":16816,"ancestor_record_offset":16384,"inlinee":22381,"source_line":33,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/publish_public_wall_state.rs
//     evidence={"module":421,"record_offset":19392,"parent_record_offset":19364,"ancestor_record_offset":16384,"inlinee":24417,"source_line":191,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::GridCell::walls
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":421,"record_offset":17028,"parent_record_offset":16816,"ancestor_record_offset":16384,"inlinee":24415,"source_line":76,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cell_at_index
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":421,"record_offset":16888,"parent_record_offset":16816,"ancestor_record_offset":16384,"inlinee":24413,"source_line":521,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":421,"record_offset":17996,"parent_record_offset":17972,"ancestor_record_offset":16384,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":421,"record_offset":17232,"parent_record_offset":17192,"ancestor_record_offset":16384,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":421,"record_offset":17272,"parent_record_offset":17232,"ancestor_record_offset":16384,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":421,"record_offset":17972,"parent_record_offset":17752,"ancestor_record_offset":16384,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":421,"record_offset":17316,"parent_record_offset":17272,"ancestor_record_offset":16384,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":421,"record_offset":17884,"parent_record_offset":17860,"ancestor_record_offset":16384,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":421,"record_offset":17860,"parent_record_offset":17752,"ancestor_record_offset":16384,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":421,"record_offset":17800,"parent_record_offset":17776,"ancestor_record_offset":16384,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":421,"record_offset":18180,"parent_record_offset":17392,"ancestor_record_offset":16384,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":421,"record_offset":17776,"parent_record_offset":17752,"ancestor_record_offset":16384,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":421,"record_offset":17368,"parent_record_offset":17344,"ancestor_record_offset":16384,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":421,"record_offset":17344,"parent_record_offset":17272,"ancestor_record_offset":16384,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":421,"record_offset":18224,"parent_record_offset":17272,"ancestor_record_offset":16384,"inlinee":17280,"source_line":1571,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::len
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":421,"record_offset":18248,"parent_record_offset":18224,"ancestor_record_offset":16384,"inlinee":17281,"source_line":1565,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":421,"record_offset":17392,"parent_record_offset":17368,"ancestor_record_offset":16384,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":421,"record_offset":17632,"parent_record_offset":17604,"ancestor_record_offset":16384,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":421,"record_offset":17660,"parent_record_offset":17632,"ancestor_record_offset":16384,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":421,"record_offset":17720,"parent_record_offset":17604,"ancestor_record_offset":16384,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":421,"record_offset":17604,"parent_record_offset":17392,"ancestor_record_offset":16384,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":421,"record_offset":17436,"parent_record_offset":17392,"ancestor_record_offset":16384,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":421,"record_offset":17752,"parent_record_offset":17392,"ancestor_record_offset":16384,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":421,"record_offset":17464,"parent_record_offset":17392,"ancestor_record_offset":16384,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":421,"record_offset":17548,"parent_record_offset":17392,"ancestor_record_offset":16384,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":421,"record_offset":18088,"parent_record_offset":17392,"ancestor_record_offset":16384,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":421,"record_offset":19448,"parent_record_offset":19424,"ancestor_record_offset":16384,"inlinee":6708,"source_line":1114,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":421,"record_offset":17192,"parent_record_offset":17152,"ancestor_record_offset":16384,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":421,"record_offset":19424,"parent_record_offset":16816,"ancestor_record_offset":16384,"inlinee":8705,"source_line":889,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":421,"record_offset":19332,"parent_record_offset":18296,"ancestor_record_offset":16384,"inlinee":22072,"source_line":141,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::iter_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":421,"record_offset":19488,"rva":5903424,"va":5374612544}
// 编译实例 RVAs: 0x5a1440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=3
//     target_aliases=9 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4999385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374612544,"raw_hex":"e827b7b3ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5374612900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a6f6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5038417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374612544,"raw_hex":"e851e14c00","size":5,"soft_fail":false,"target_va":5379651280,"va":5374612858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28365540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374612544,"raw_hex":"e8e4d2b001","size":5,"soft_fail":false,"target_va":5402979152,"va":5374613607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28365756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374612544,"raw_hex":"e8bcd3b001","size":5,"soft_fail":false,"target_va":5402979344,"va":5374613583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":421,"record_offset":19748,"parent_record_offset":19488,"ancestor_record_offset":19488,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":421,"record_offset":19896,"parent_record_offset":19488,"ancestor_record_offset":19488,"inlinee":6317,"source_line":169,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/publish_public_wall_state.rs
//     evidence={"module":421,"record_offset":19924,"parent_record_offset":19488,"ancestor_record_offset":19488,"inlinee":24418,"source_line":205,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":421,"record_offset":20036,"parent_record_offset":20008,"ancestor_record_offset":19488,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":421,"record_offset":21092,"parent_record_offset":21068,"ancestor_record_offset":19488,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":421,"record_offset":20560,"parent_record_offset":20536,"ancestor_record_offset":19488,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":421,"record_offset":20644,"parent_record_offset":20620,"ancestor_record_offset":19488,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":421,"record_offset":20584,"parent_record_offset":20560,"ancestor_record_offset":19488,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":421,"record_offset":19772,"parent_record_offset":19748,"ancestor_record_offset":19488,"inlinee":8603,"source_line":47,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":421,"record_offset":20756,"parent_record_offset":20732,"ancestor_record_offset":19488,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":421,"record_offset":19860,"parent_record_offset":19772,"ancestor_record_offset":19488,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":421,"record_offset":22432,"parent_record_offset":19924,"ancestor_record_offset":19488,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":421,"record_offset":21004,"parent_record_offset":20980,"ancestor_record_offset":19488,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":421,"record_offset":20896,"parent_record_offset":20872,"ancestor_record_offset":19488,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":421,"record_offset":19828,"parent_record_offset":19796,"ancestor_record_offset":19488,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":20068,"parent_record_offset":20036,"ancestor_record_offset":19488,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":20100,"parent_record_offset":20068,"ancestor_record_offset":19488,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":421,"record_offset":19796,"parent_record_offset":19772,"ancestor_record_offset":19488,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":421,"record_offset":21116,"parent_record_offset":21092,"ancestor_record_offset":19488,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":421,"record_offset":20272,"parent_record_offset":19924,"ancestor_record_offset":19488,"inlinee":22068,"source_line":100,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":421,"record_offset":21368,"parent_record_offset":19924,"ancestor_record_offset":19488,"inlinee":22381,"source_line":33,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::GridCell::walls
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":421,"record_offset":20148,"parent_record_offset":19924,"ancestor_record_offset":19488,"inlinee":24415,"source_line":76,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cell_at_index
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":421,"record_offset":20008,"parent_record_offset":19924,"ancestor_record_offset":19488,"inlinee":24413,"source_line":521,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":421,"record_offset":21068,"parent_record_offset":21044,"ancestor_record_offset":19488,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":421,"record_offset":22508,"parent_record_offset":22484,"ancestor_record_offset":19488,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":421,"record_offset":20328,"parent_record_offset":20300,"ancestor_record_offset":19488,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":421,"record_offset":20356,"parent_record_offset":20328,"ancestor_record_offset":19488,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":421,"record_offset":21044,"parent_record_offset":20824,"ancestor_record_offset":19488,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":421,"record_offset":20388,"parent_record_offset":20356,"ancestor_record_offset":19488,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":421,"record_offset":20956,"parent_record_offset":20932,"ancestor_record_offset":19488,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":421,"record_offset":20932,"parent_record_offset":20824,"ancestor_record_offset":19488,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":421,"record_offset":20872,"parent_record_offset":20848,"ancestor_record_offset":19488,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":421,"record_offset":21252,"parent_record_offset":20464,"ancestor_record_offset":19488,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":421,"record_offset":20848,"parent_record_offset":20824,"ancestor_record_offset":19488,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":421,"record_offset":20440,"parent_record_offset":20416,"ancestor_record_offset":19488,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":421,"record_offset":20416,"parent_record_offset":20356,"ancestor_record_offset":19488,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":421,"record_offset":20464,"parent_record_offset":20440,"ancestor_record_offset":19488,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":421,"record_offset":20704,"parent_record_offset":20676,"ancestor_record_offset":19488,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":421,"record_offset":20732,"parent_record_offset":20704,"ancestor_record_offset":19488,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":421,"record_offset":20792,"parent_record_offset":20676,"ancestor_record_offset":19488,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":421,"record_offset":20676,"parent_record_offset":20464,"ancestor_record_offset":19488,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":421,"record_offset":20508,"parent_record_offset":20464,"ancestor_record_offset":19488,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":421,"record_offset":20824,"parent_record_offset":20464,"ancestor_record_offset":19488,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":421,"record_offset":20536,"parent_record_offset":20464,"ancestor_record_offset":19488,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":421,"record_offset":20620,"parent_record_offset":20464,"ancestor_record_offset":19488,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":421,"record_offset":21160,"parent_record_offset":20464,"ancestor_record_offset":19488,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":421,"record_offset":22484,"parent_record_offset":22460,"ancestor_record_offset":19488,"inlinee":8703,"source_line":861,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":421,"record_offset":21336,"parent_record_offset":21312,"ancestor_record_offset":19488,"inlinee":6708,"source_line":1114,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":421,"record_offset":20300,"parent_record_offset":20272,"ancestor_record_offset":19488,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":421,"record_offset":22460,"parent_record_offset":19924,"ancestor_record_offset":19488,"inlinee":8702,"source_line":670,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":421,"record_offset":21312,"parent_record_offset":19924,"ancestor_record_offset":19488,"inlinee":8705,"source_line":889,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":421,"record_offset":22400,"parent_record_offset":21368,"ancestor_record_offset":19488,"inlinee":22072,"source_line":141,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::iter_tiles<system_clutter::clutter::populate_clutter_regions::populate_clutter_regions::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2289,"record_offset":252,"rva":21934080,"va":5390643200}
// 编译实例 RVAs: 0x14eb000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21029944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390643200,"raw_hex":"e8c81bbffe","size":5,"soft_fail":false,"target_va":5369613520,"va":5390643459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064a3d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15338696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390643200,"raw_hex":"e838f315ff","size":5,"soft_fail":false,"target_va":5375304656,"va":5390643347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-13254168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390643200,"raw_hex":"e8e8c135ff","size":5,"soft_fail":false,"target_va":5377389488,"va":5390643651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12335169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390643200,"raw_hex":"e84138bc00","size":5,"soft_fail":false,"target_va":5402978880,"va":5390643706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12335645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390643200,"raw_hex":"e81d3abc00","size":5,"soft_fail":false,"target_va":5402979344,"va":5390643694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2289,"record_offset":668,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2289,"record_offset":496,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2289,"record_offset":640,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":6317,"source_line":169,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/populate_clutter_regions.rs
//     evidence={"module":2289,"record_offset":696,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":81284,"source_line":149,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2289,"record_offset":1064,"parent_record_offset":1040,"ancestor_record_offset":252,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2289,"record_offset":1040,"parent_record_offset":1000,"ancestor_record_offset":252,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2289,"record_offset":1000,"parent_record_offset":724,"ancestor_record_offset":252,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2289,"record_offset":784,"parent_record_offset":724,"ancestor_record_offset":252,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2289,"record_offset":892,"parent_record_offset":724,"ancestor_record_offset":252,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2289,"record_offset":1908,"parent_record_offset":1884,"ancestor_record_offset":252,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2289,"record_offset":1376,"parent_record_offset":1352,"ancestor_record_offset":252,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2289,"record_offset":1460,"parent_record_offset":1436,"ancestor_record_offset":252,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2289,"record_offset":1400,"parent_record_offset":1376,"ancestor_record_offset":252,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":2289,"record_offset":520,"parent_record_offset":496,"ancestor_record_offset":252,"inlinee":8603,"source_line":47,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2289,"record_offset":1572,"parent_record_offset":1548,"ancestor_record_offset":252,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2289,"record_offset":604,"parent_record_offset":520,"ancestor_record_offset":252,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2289,"record_offset":2152,"parent_record_offset":724,"ancestor_record_offset":252,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2289,"record_offset":1820,"parent_record_offset":1796,"ancestor_record_offset":252,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2289,"record_offset":1712,"parent_record_offset":1688,"ancestor_record_offset":252,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2289,"record_offset":572,"parent_record_offset":544,"ancestor_record_offset":252,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2289,"record_offset":1096,"parent_record_offset":1000,"ancestor_record_offset":252,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2289,"record_offset":816,"parent_record_offset":784,"ancestor_record_offset":252,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2289,"record_offset":848,"parent_record_offset":816,"ancestor_record_offset":252,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2289,"record_offset":544,"parent_record_offset":520,"ancestor_record_offset":252,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":2289,"record_offset":1884,"parent_record_offset":1860,"ancestor_record_offset":252,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2289,"record_offset":1176,"parent_record_offset":1152,"ancestor_record_offset":252,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2289,"record_offset":1152,"parent_record_offset":1128,"ancestor_record_offset":252,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2289,"record_offset":1860,"parent_record_offset":1640,"ancestor_record_offset":252,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2289,"record_offset":1204,"parent_record_offset":1176,"ancestor_record_offset":252,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2289,"record_offset":1772,"parent_record_offset":1748,"ancestor_record_offset":252,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2289,"record_offset":1748,"parent_record_offset":1640,"ancestor_record_offset":252,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2289,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":252,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2289,"record_offset":2068,"parent_record_offset":1280,"ancestor_record_offset":252,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2289,"record_offset":1664,"parent_record_offset":1640,"ancestor_record_offset":252,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2289,"record_offset":1256,"parent_record_offset":1232,"ancestor_record_offset":252,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2289,"record_offset":1232,"parent_record_offset":1176,"ancestor_record_offset":252,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2289,"record_offset":1280,"parent_record_offset":1256,"ancestor_record_offset":252,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2289,"record_offset":1520,"parent_record_offset":1492,"ancestor_record_offset":252,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2289,"record_offset":1548,"parent_record_offset":1520,"ancestor_record_offset":252,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2289,"record_offset":1608,"parent_record_offset":1492,"ancestor_record_offset":252,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2289,"record_offset":1492,"parent_record_offset":1280,"ancestor_record_offset":252,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2289,"record_offset":1324,"parent_record_offset":1280,"ancestor_record_offset":252,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2289,"record_offset":1640,"parent_record_offset":1280,"ancestor_record_offset":252,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2289,"record_offset":1352,"parent_record_offset":1280,"ancestor_record_offset":252,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2289,"record_offset":1436,"parent_record_offset":1280,"ancestor_record_offset":252,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2289,"record_offset":1976,"parent_record_offset":1280,"ancestor_record_offset":252,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2289,"record_offset":1128,"parent_record_offset":724,"ancestor_record_offset":252,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::clutter::ClutterCandidateIndex::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/mod.rs
//     evidence={"module":2289,"record_offset":2124,"parent_record_offset":724,"ancestor_record_offset":252,"inlinee":48746,"source_line":356,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::clutter::ClutterState::mark_dirty
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/mod.rs
//     evidence={"module":2289,"record_offset":724,"parent_record_offset":696,"ancestor_record_offset":252,"inlinee":48759,"source_line":223,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::clutter::impl$22::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/mod.rs
//     evidence={"module":2289,"record_offset":1932,"parent_record_offset":1908,"ancestor_record_offset":252,"inlinee":48733,"source_line":331,"archive":"_pdb/A/modules/02289.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$1::copy_to_staging::closure_env$0<512> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 148
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":421,"record_offset":23296,"rva":5904496,"va":5374613616}
// 编译实例 RVAs: 0x5a1870, 0x1db7870
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14064a3d0
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24565872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399869552,"raw_hex":"e8902789fe","size":5,"soft_fail":false,"target_va":5375304656,"va":5399870523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28364190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374613616,"raw_hex":"e89ecdb001","size":5,"soft_fail":false,"target_va":5402979344,"va":5374615149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::copy_from_slice::len_mismatch_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28364540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374613616,"raw_hex":"e8fcceb001","size":5,"soft_fail":false,"target_va":5402980464,"va":5374615919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=22
//     evidence={"assembly":"\tcallq\t28365072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374613616,"raw_hex":"e810d1b001","size":5,"soft_fail":false,"target_va":5402980304,"va":5374615227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28363841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374613616,"raw_hex":"e841ccb001","size":5,"soft_fail":false,"target_va":5402980288,"va":5374616442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::world_raster::build_tile_hasher
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25262488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399869552,"raw_hex":"e868867efe","size":5,"soft_fail":false,"target_va":5374608304,"va":5399870787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":4205,"record_offset":4864,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/04205.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":421,"record_offset":26192,"parent_record_offset":23296,"ancestor_record_offset":23296,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":421,"record_offset":26440,"parent_record_offset":23296,"ancestor_record_offset":23296,"inlinee":6317,"source_line":169,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::impl$1::copy_to_staging::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":421,"record_offset":23528,"parent_record_offset":23296,"ancestor_record_offset":23296,"inlinee":24420,"source_line":383,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] ahash::convert::impl$0::read_u128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ahash","version":"0.8.11","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ahash-0.8.11/src/convert.rs
//     evidence={"module":421,"record_offset":25576,"parent_record_offset":25412,"ancestor_record_offset":23296,"inlinee":8786,"source_line":110,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] ahash::fallback_hash::AHasher::large_update
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ahash","version":"0.8.11","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ahash-0.8.11/src/fallback_hash.rs
//     evidence={"module":421,"record_offset":25448,"parent_record_offset":25412,"ancestor_record_offset":23296,"inlinee":8781,"source_line":111,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] ahash::fallback_hash::impl$1::finish
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ahash","version":"0.8.11","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ahash-0.8.11/src/fallback_hash.rs
//     evidence={"module":421,"record_offset":25856,"parent_record_offset":23528,"ancestor_record_offset":23296,"inlinee":5839,"source_line":196,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] ahash::fallback_hash::impl$1::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ahash","version":"0.8.11","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ahash-0.8.11/src/fallback_hash.rs
//     evidence={"module":421,"record_offset":25412,"parent_record_offset":23528,"ancestor_record_offset":23296,"inlinee":8799,"source_line":171,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] ahash::operations::folded_multiply
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"ahash","version":"0.8.11","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ahash-0.8.11/src/operations.rs
//     evidence={"module":421,"record_offset":25476,"parent_record_offset":25448,"ancestor_record_offset":23296,"inlinee":5828,"source_line":17,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":421,"record_offset":25956,"parent_record_offset":23528,"ancestor_record_offset":23296,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":421,"record_offset":24744,"parent_record_offset":24688,"ancestor_record_offset":23296,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::wrapping_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":421,"record_offset":25504,"parent_record_offset":25476,"ancestor_record_offset":23296,"inlinee":5830,"source_line":1703,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":421,"record_offset":25540,"parent_record_offset":25448,"ancestor_record_offset":23296,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":421,"record_offset":25812,"parent_record_offset":25716,"ancestor_record_offset":23296,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":421,"record_offset":25672,"parent_record_offset":25648,"ancestor_record_offset":23296,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":421,"record_offset":26372,"parent_record_offset":26248,"ancestor_record_offset":23296,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":421,"record_offset":24688,"parent_record_offset":23656,"ancestor_record_offset":23296,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":421,"record_offset":25600,"parent_record_offset":25576,"ancestor_record_offset":23296,"inlinee":6165,"source_line":1886,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":421,"record_offset":25624,"parent_record_offset":25600,"ancestor_record_offset":23296,"inlinee":6166,"source_line":2086,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":421,"record_offset":25648,"parent_record_offset":25624,"ancestor_record_offset":23296,"inlinee":6167,"source_line":1973,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":24140,"parent_record_offset":23656,"ancestor_record_offset":23296,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":24016,"parent_record_offset":23656,"ancestor_record_offset":23296,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":26012,"parent_record_offset":25984,"ancestor_record_offset":23296,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":24472,"parent_record_offset":24272,"ancestor_record_offset":23296,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":24072,"parent_record_offset":24016,"ancestor_record_offset":23296,"inlinee":5972,"source_line":424,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":24200,"parent_record_offset":24140,"ancestor_record_offset":23296,"inlinee":5472,"source_line":501,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":421,"record_offset":26248,"parent_record_offset":26192,"ancestor_record_offset":23296,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::WorldRaster::prepare_tile
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":421,"record_offset":23656,"parent_record_offset":23528,"ancestor_record_offset":23296,"inlinee":24421,"source_line":175,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::DirtyTiles::extend
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=21
// 源码行: 165, 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE844
// 首条位置: {"module":1207,"record_offset":16032,"rva":13199552,"va":5381908672}
// 编译实例 RVAs: 0xc968c0
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
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 180
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE7F7
// 首条位置: {"module":1202,"inlinee":59383}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::bitmap_set
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1202,"record_offset":12960,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":24398,"source_line":192,"archive":"_pdb/A/modules/01202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::bitmap_set
// 记录形式: inline_instance, inlinee_declaration; instances=32
// 源码行: 192
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F4E
// 首条位置: {"module":421,"inlinee":24398}
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
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 209
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F51
// 首条位置: {"module":421,"inlinee":24401}
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
// 源码行: 214
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE83D
// 首条位置: {"module":1207,"inlinee":59453}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::impl$1::fold
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF002
// 首条位置: {"module":1308,"inlinee":61442}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1308,"record_offset":57916,"parent_record_offset":57892,"ancestor_record_offset":57892,"inlinee":59460,"source_line":165,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::impl$2::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 285
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":498,"record_offset":26692,"rva":7147984,"va":5375857104}
// 编译实例 RVAs: 0x6d11d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cfa90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4200387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375857104,"raw_hex":"e83de8bfff","size":5,"soft_fail":false,"target_va":5371656848,"va":5375857230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405a10e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1245493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375857104,"raw_hex":"e8cbfeecff","size":5,"soft_fail":false,"target_va":5374611680,"va":5375857168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406cf170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375857104,"raw_hex":"e87edfffff","size":5,"soft_fail":false,"target_va":5375848816,"va":5375857133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f1c80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t133624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375857104,"raw_hex":"e8f8090200","size":5,"soft_fail":false,"target_va":5375990912,"va":5375857283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t27111104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375857104,"raw_hex":"e8c0ae9d01","size":5,"soft_fail":false,"target_va":5402968352,"va":5375857243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":498,"record_offset":26948,"parent_record_offset":26692,"ancestor_record_offset":26692,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":498,"record_offset":27272,"parent_record_offset":26972,"ancestor_record_offset":26692,"inlinee":9141,"source_line":1394,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":498,"record_offset":27296,"parent_record_offset":27272,"ancestor_record_offset":26692,"inlinee":9142,"source_line":1588,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":498,"record_offset":27432,"parent_record_offset":27412,"ancestor_record_offset":26692,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":498,"record_offset":27008,"parent_record_offset":26972,"ancestor_record_offset":26692,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":498,"record_offset":27188,"parent_record_offset":26972,"ancestor_record_offset":26692,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":498,"record_offset":27320,"parent_record_offset":27296,"ancestor_record_offset":26692,"inlinee":9143,"source_line":1077,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":498,"record_offset":27368,"parent_record_offset":27344,"ancestor_record_offset":26692,"inlinee":9145,"source_line":468,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":498,"record_offset":27344,"parent_record_offset":27320,"ancestor_record_offset":26692,"inlinee":9144,"source_line":1045,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":498,"record_offset":27032,"parent_record_offset":27008,"ancestor_record_offset":26692,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":498,"record_offset":27212,"parent_record_offset":27188,"ancestor_record_offset":26692,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":498,"record_offset":27240,"parent_record_offset":27188,"ancestor_record_offset":26692,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":27116,"parent_record_offset":27084,"ancestor_record_offset":26692,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":27140,"parent_record_offset":27116,"ancestor_record_offset":26692,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":27520,"parent_record_offset":27432,"ancestor_record_offset":26692,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$43::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":498,"record_offset":27056,"parent_record_offset":27032,"ancestor_record_offset":26692,"inlinee":5734,"source_line":2431,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":498,"record_offset":27084,"parent_record_offset":27056,"ancestor_record_offset":26692,"inlinee":5735,"source_line":129,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] tabloid::impl$2::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tabloid/src/lib.rs
//     evidence={"module":498,"record_offset":26972,"parent_record_offset":26948,"ancestor_record_offset":26692,"inlinee":21325,"source_line":140,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::dirty_tiles::impl$2::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 285
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":498,"record_offset":25816,"rva":7147792,"va":5375856912}
// 编译实例 RVAs: 0x6d1110
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cfa90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4200195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375856912,"raw_hex":"e8fde8bfff","size":5,"soft_fail":false,"target_va":5371656848,"va":5375857038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405a1440
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1244437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375856912,"raw_hex":"e8eb02edff","size":5,"soft_fail":false,"target_va":5374612544,"va":5375856976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406cf170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375856912,"raw_hex":"e83ee0ffff","size":5,"soft_fail":false,"target_va":5375848816,"va":5375856941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f1c80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dirty_tiles.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t133816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375856912,"raw_hex":"e8b80a0200","size":5,"soft_fail":false,"target_va":5375990912,"va":5375857091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t27111296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375856912,"raw_hex":"e880af9d01","size":5,"soft_fail":false,"target_va":5402968352,"va":5375857051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":498,"record_offset":26072,"parent_record_offset":25816,"ancestor_record_offset":25816,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":498,"record_offset":26396,"parent_record_offset":26096,"ancestor_record_offset":25816,"inlinee":9141,"source_line":1394,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":498,"record_offset":26420,"parent_record_offset":26396,"ancestor_record_offset":25816,"inlinee":9142,"source_line":1588,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":498,"record_offset":26556,"parent_record_offset":26536,"ancestor_record_offset":25816,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":498,"record_offset":26132,"parent_record_offset":26096,"ancestor_record_offset":25816,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":498,"record_offset":26312,"parent_record_offset":26096,"ancestor_record_offset":25816,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":498,"record_offset":26444,"parent_record_offset":26420,"ancestor_record_offset":25816,"inlinee":9143,"source_line":1077,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":498,"record_offset":26492,"parent_record_offset":26468,"ancestor_record_offset":25816,"inlinee":9145,"source_line":468,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":498,"record_offset":26468,"parent_record_offset":26444,"ancestor_record_offset":25816,"inlinee":9144,"source_line":1045,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":498,"record_offset":26156,"parent_record_offset":26132,"ancestor_record_offset":25816,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":498,"record_offset":26336,"parent_record_offset":26312,"ancestor_record_offset":25816,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":498,"record_offset":26364,"parent_record_offset":26312,"ancestor_record_offset":25816,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":26240,"parent_record_offset":26208,"ancestor_record_offset":25816,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":26264,"parent_record_offset":26240,"ancestor_record_offset":25816,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":26644,"parent_record_offset":26556,"ancestor_record_offset":25816,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$43::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":498,"record_offset":26180,"parent_record_offset":26156,"ancestor_record_offset":25816,"inlinee":5734,"source_line":2431,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":498,"record_offset":26208,"parent_record_offset":26180,"ancestor_record_offset":25816,"inlinee":5735,"source_line":129,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] tabloid::impl$2::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tabloid/src/lib.rs
//     evidence={"module":498,"record_offset":26096,"parent_record_offset":26072,"ancestor_record_offset":25816,"inlinee":21325,"source_line":140,"archive":"_pdb/A/modules/00498.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 14; 过程引用数: 87
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Subscription", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Publication", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "lock_api::rwlock::RwLock", "arguments": ["parking_lot::raw_rwlock::RawRwLock", "tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Publisher", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure_env$0>", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells", "arguments": ["country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure_env$0>", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells", "arguments": ["country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::iter_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0>", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::iter_tiles", "arguments": ["country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::iter_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1>", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::iter_tiles", "arguments": ["country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$1::copy_to_staging::closure_env$0<512> >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::retain", "arguments": ["country_core::resources::world_raster::impl$1::copy_to_staging::closure_env$0<512>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Publisher::publish", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publisher::subscribe<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Publisher::subscribe", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Subscriber::take<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Subscriber::take", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::PublicationReader<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::PublicationReader", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tabloid::PublicationReader<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tabloid::PublicationReader<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::impl$1::publish::closure_env$0", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "alloc::vec::Vec::retain", "arguments": ["alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "alloc::alloc::Global", "tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::iter_tiles<system_clutter::clutter::populate_clutter_regions::populate_clutter_regions::closure_env$0>", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::iter_tiles", "arguments": ["system_clutter::clutter::populate_clutter_regions::populate_clutter_regions::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::clutter::impl$1::remove_region_from_lazy_grid::closure_env$0>", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells", "arguments": ["system_clutter::clutter::impl$1::remove_region_from_lazy_grid::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::clutter::impl$1::add_region_to_lazy_grid::closure_env$0>", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells", "arguments": ["system_clutter::clutter::impl$1::add_region_to_lazy_grid::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::clutter::read_bush_positions_from_gpu::impl$0::bushes_near_point::closure_env$0<system_clutter::clutter::populate_clutter_regions::filter_clutter_object::closure_env$0> >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells", "arguments": ["system_clutter::clutter::read_bush_positions_from_gpu::impl$0::bushes_near_point::closure_env$0<system_clutter::clutter::populate_clutter_regions::filter_clutter_object::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::clutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu::closure_env$1>", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells", "arguments": ["system_clutter::clutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 23; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::clear
// 信息 ID: 1d76516f1048bbdd4a06f882
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 16 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3> @ 0x1a8e283 -> 0xc968b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3> @ 0x1a8e87a -> 0xc968b0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_from_cpu; module=448, inline_record=36124; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::draw_path_mask::draw_path_mask_compute; module=459, inline_record=60656; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu; module=498, inline_record=21612; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>; module=1339, inline_record=9324; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>; module=1339, inline_record=10668; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>; module=1339, inline_record=11228; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=1339, inline_record=25676; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=1339, inline_record=27540; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::extend
// 信息 ID: 8db48196611a24d53cc5c3fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 17 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x4d4591 -> 0xc968c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles> > | alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles> > @ 0x14e51ea -> 0xc968c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles> > | alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles> > @ 0x14e532b -> 0xc968c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3> @ 0x1a8e2c5 -> 0xc968c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3> @ 0x1a8e2d1 -> 0xc968c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3> @ 0x1a8e872 -> 0xc968c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad548e -> 0xc968c0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles> >; module=1308, inline_record=57916; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles> >; module=1308, inline_record=58908; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>; module=1339, inline_record=10860; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>; module=1339, inline_record=17476; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>; module=1339, inline_record=17532; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=1339, inline_record=27732; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=1339, inline_record=34332; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=1339, inline_record=34388; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::intersects_world_space_rect
// 信息 ID: 395bca856614bc192b15f6b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x4d4975 -> 0xc96880 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x4d4a76 -> 0xc96880 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::iter_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0>
// 信息 ID: 3c939e3f155cddb254c7d02c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::iter_tiles -> ["country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::dirty_tiles::impl$2::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0> @ 0x6d1210 -> 0x5a10e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::iter_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1>
// 信息 ID: 4534355d86678f57c6789b4a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::iter_tiles -> ["country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::dirty_tiles::impl$2::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1> @ 0x6d1150 -> 0x5a1440 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::iter_tiles<system_clutter::clutter::populate_clutter_regions::populate_clutter_regions::closure_env$0>
// 信息 ID: 0b7b328a5e55cfde2dbad2a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::iter_tiles -> ["system_clutter::clutter::populate_clutter_regions::populate_clutter_regions::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad5503 -> 0x14eb000 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure_env$0>
// 信息 ID: ac5928ce664bf0d928813e82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells -> ["country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::intersects_world_space_rect @ 0xc96897 -> 0x5a0e90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure_env$0>
// 信息 ID: 8a253f1d4541fc1bed8d080d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells -> ["country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::dispose_collider_mesh_instance @ 0x592da9 -> 0x5a0b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_rect @ 0xc96879 -> 0x5a0b40 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::upsert_collider_entry<bevy_ecs::entity::Entity,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0xf2c837 -> 0x5a0b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::upsert_collider_entry<bevy_ecs::entity::Entity,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0xf2ccb2 -> 0x5a0b40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::clutter::impl$1::add_region_to_lazy_grid::closure_env$0>
// 信息 ID: c29829803e47dbf6fb751686
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells -> ["system_clutter::clutter::impl$1::add_region_to_lazy_grid::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::clutter::ClutterState::add_region @ 0x9bdd46 -> 0x14eb5a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::ClutterState::add_region @ 0x9bdefd -> 0x14eb5a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::clutter::impl$1::remove_region_from_lazy_grid::closure_env$0>
// 信息 ID: 384a21ccb7350f1962360608
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells -> ["system_clutter::clutter::impl$1::remove_region_from_lazy_grid::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::clutter::ClutterState::add_region @ 0x9bdc6e -> 0x14eb200 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::ClutterState::remove_region_from_lazy_grid @ 0x9be114 -> 0x14eb200 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::clutter::read_bush_positions_from_gpu::impl$0::bushes_near_point::closure_env$0<system_clutter::clutter::populate_clutter_regions::filter_clutter_object::closure_env$0> >
// 信息 ID: b350464dd375d23796e957a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_clutter::clutter::read_bush_positions_from_gpu::impl$0::bushes_near_point::closure_env$0 -> ["system_clutter::clutter::populate_clutter_regions::filter_clutter_object::closure_env$0"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells -> ["system_clutter::clutter::read_bush_positions_from_gpu::impl$0::bushes_near_point::closure_env$0<system_clutter::clutter::populate_clutter_regions::filter_clutter_object::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad8dff -> 0x14eb850 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::clutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu::closure_env$1>
// 信息 ID: b5927972365ad1fcd083c5ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells -> ["system_clutter::clutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::clutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu @ 0x14e48a8 -> 0x14ebb50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::new_clear
// 信息 ID: 262b985a3e4629e1fda07586
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x4d44f9 -> 0xc96850 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu @ 0x14e48e0 -> 0xc96850 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad5404 -> 0xc96850 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::resources::terrain_height::TerrainHeights::copy_from_gpu; module=1202, inline_record=12732; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_rect
// 信息 ID: f46035e366ea41d5490b6fb8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::WallId,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0xcbe4aa -> 0xc96870 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::WallId,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0xcbe75d -> 0xc96870 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::decorator::DecoratorId,country_core::systems::collision::world::OffsetColliderTransform> @ 0xcfb48a -> 0xc96870 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::decorator::DecoratorId,country_core::systems::collision::world::OffsetColliderTransform> @ 0xcfb73d -> 0xc96870 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::decorator::DecoratorId,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0xcfc58a -> 0xc96870 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::decorator::DecoratorId,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0xcfc83d -> 0xc96870 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::collision::world::dispose_collider_mesh_instance; module=417, inline_record=113740; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::world::upsert_collider_entry<bevy_ecs::entity::Entity,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> >; module=1369, inline_record=91812; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::world::upsert_collider_entry<bevy_ecs::entity::Entity,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> >; module=1369, inline_record=95984; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> >
// 信息 ID: 72ba2ed33e8753056e893c4e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> @ 0xec7ad4 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> @ 0xec8397 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> @ 0xec85d2 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xeca169 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xecaa24 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xecac6a -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xecc814 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xecd0d7 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xecd312 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$0,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2> @ 0xecf134 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$0,country_core::systems::paths::draw_path::draw_path::closure_env$1,country_core::systems::paths::draw_path::draw_path::closure_env$2> @ 0xed0424 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3> @ 0x1a8e2ae -> 0x1db7650 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > >
// 信息 ID: daef9264789e28519f637d98
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster::closure_env$1 -> ["country_core::resources::world_raster_defs::GardenRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_garden::replay_garden::closure_env$2"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 11 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> @ 0xec7ad4 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> @ 0xec8397 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> @ 0xec85d2 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xeca169 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xecaa24 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xecac6a -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xecc814 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xecd0d7 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xecd312 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$0,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2> @ 0xecf134 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$0,country_core::systems::paths::draw_path::draw_path::closure_env$1,country_core::systems::paths::draw_path::draw_path::closure_env$2> @ 0xed0424 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > >
// 信息 ID: 61519573135f7bbee0e8845d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster::closure_env$1 -> ["country_core::resources::world_raster_defs::PathRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$2"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 11 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> @ 0xec7ad4 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> @ 0xec8397 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> @ 0xec85d2 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xeca169 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xecaa24 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xecac6a -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xecc814 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xecd0d7 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xecd312 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$0,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2> @ 0xecf134 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$0,country_core::systems::paths::draw_path::draw_path::closure_env$1,country_core::systems::paths::draw_path::draw_path::closure_env$2> @ 0xed0424 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> > >
// 信息 ID: 84f52c2d99498b3f53155291
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster::WorldRaster -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::systems::history::replay_raster::replay_raster::closure_env$1 -> ["country_core::resources::world_raster_defs::WaterRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_water::replay_water::closure_env$2"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 11 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> @ 0xec7ad4 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> @ 0xec8397 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> @ 0xec85d2 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xeca169 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xecaa24 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xecac6a -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xecc814 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xecd0d7 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xecd312 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$0,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2> @ 0xecf134 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$0,country_core::systems::paths::draw_path::draw_path::closure_env$1,country_core::systems::paths::draw_path::draw_path::closure_env$2> @ 0xed0424 -> 0x5a0920 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$1::copy_to_staging::closure_env$0<512> >
// 信息 ID: 3cc5748d6257954af7a742b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::world_raster::impl$1::copy_to_staging::closure_env$0 -> ["512"]; 字段布局 = unknown。
// [类型关系] country_core::utils::dirty_tiles::DirtyTiles::retain -> ["country_core::resources::world_raster::impl$1::copy_to_staging::closure_env$0<512>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::world_raster::impl$1::copy_to_staging<512> @ 0x1032f02 -> 0x5a1870 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster::impl$1::copy_to_staging<512> @ 0x1db8852 -> 0x1db7870 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::DirtyTiles::set
// 信息 ID: 7dec3574c90f4afa3bae5704
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::terrain_height::TerrainHeights::copy_from_gpu; module=1202, inline_record=12908; 不是独立调用边。
// [PDB 内联宿主] system_clutter::clutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu; module=2280, inline_record=1468; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::bitmap_intersects
// 信息 ID: 1421fdbfac37901af4816e35
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure_env$0>; module=421, inline_record=16180; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::bitmap_set
// 信息 ID: a7fa374f95ffa988639761e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 27 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure_env$0>; module=421, inline_record=14628; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=1448; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=1836; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=2168; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=2684; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=3100; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=3460; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=3936; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::dilate
// 信息 ID: ef14b7773e5ccf4f4677c69e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure$0
// 信息 ID: 963696ada03b960af3ed0552
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure_env$0 <- country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::intersects_world_space_rect::closure_env$0>; module=421, inline_record=16140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure$0
// 信息 ID: 6f15eb06d2db2b705402d853
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure_env$0 <- country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure_env$0>; module=421, inline_record=14600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure$0
// 信息 ID: a91f97f61e80d625cc1030cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0 <- conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0 <- conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 24 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=1404; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=1792; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=2124; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=2644; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=3060; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=3420; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=3892; 不是独立调用边。
// [PDB 内联宿主] conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >; module=479, inline_record=4280; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::impl$1::fold
// 信息 ID: b958b8ead8e61ba2a6df3f34
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles> >; module=1308, inline_record=57892; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles> >; module=1308, inline_record=58884; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::impl$2::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0>
// 信息 ID: b97007fb759b613172bf0062
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::dirty_tiles::impl$2::iter_dirty_tiles -> ["country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state @ 0x5fa612 -> 0x6d11d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::impl$2::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1>
// 信息 ID: fc0c39698720bd4ff4a99c7c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::dirty_tiles::impl$2::iter_dirty_tiles -> ["country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state @ 0x5fa69c -> 0x6d1110 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::dirty_tiles::impl$3::clone
// 信息 ID: c6c8ae438cc93cf762cfe097
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::world_raster::impl$1::copy_to_staging<512>; module=1398, inline_record=26108; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster::impl$1::copy_to_staging<512>; module=4206, inline_record=3168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

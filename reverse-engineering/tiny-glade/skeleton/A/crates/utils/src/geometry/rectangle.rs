// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/utils/src/geometry/rectangle.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: db74a53b6ced5039b6fec826e334d80b4bf1b849786097c49248e9b7f5be9b83
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: rectangle.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 182, 383, 397, 460, 486, 740, 1226, 1249, 1282, 1286, 1287, 1326, 1338, 1355, 1421, 1426, 1905, 2269, 2699, 2749, 2772, 2773, 2775, 2776, 2777, 3185, 3194, 3515, 3842, 3849, 3850, 3855, 3856, 3858, 3859, 3863, 3865, 4378, 4379, 4467
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::_get_normal_at_u
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":12864,"rva":9248592,"va":5377957712}
// 编译实例 RVAs: 0x8d1f50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25020662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377957712,"raw_hex":"e8f6c87d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377958485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25020478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377957712,"raw_hex":"e83ec87d01","size":5,"soft_fail":false,"target_va":5402978944,"va":5377958461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":15212,"parent_record_offset":12864,"ancestor_record_offset":12864,"inlinee":10010,"source_line":120,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":740,"record_offset":13660,"parent_record_offset":12864,"ancestor_record_offset":12864,"inlinee":24539,"source_line":6,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_corner_normal
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":13716,"parent_record_offset":12864,"ancestor_record_offset":12864,"inlinee":45792,"source_line":324,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_edge_normal
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":15324,"parent_record_offset":12864,"ancestor_record_offset":12864,"inlinee":45794,"source_line":374,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":13024,"parent_record_offset":12864,"ancestor_record_offset":12864,"inlinee":45790,"source_line":241,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":15596,"parent_record_offset":15572,"ancestor_record_offset":12864,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":13160,"parent_record_offset":13136,"ancestor_record_offset":12864,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":13184,"parent_record_offset":13160,"ancestor_record_offset":12864,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":15644,"parent_record_offset":15620,"ancestor_record_offset":12864,"inlinee":7043,"source_line":1506,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":14172,"parent_record_offset":14108,"ancestor_record_offset":12864,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":13896,"parent_record_offset":13872,"ancestor_record_offset":12864,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":14140,"parent_record_offset":14108,"ancestor_record_offset":12864,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":13220,"parent_record_offset":13084,"ancestor_record_offset":12864,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":740,"record_offset":16392,"parent_record_offset":16364,"ancestor_record_offset":12864,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":14956,"parent_record_offset":14828,"ancestor_record_offset":12864,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":13136,"parent_record_offset":13112,"ancestor_record_offset":12864,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":13112,"parent_record_offset":13084,"ancestor_record_offset":12864,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":13816,"parent_record_offset":13764,"ancestor_record_offset":12864,"inlinee":26062,"source_line":61,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":13872,"parent_record_offset":13848,"ancestor_record_offset":12864,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":13848,"parent_record_offset":13740,"ancestor_record_offset":12864,"inlinee":21770,"source_line":427,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":14888,"parent_record_offset":14856,"ancestor_record_offset":12864,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":14856,"parent_record_offset":14828,"ancestor_record_offset":12864,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":14828,"parent_record_offset":14680,"ancestor_record_offset":12864,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":13788,"parent_record_offset":13764,"ancestor_record_offset":12864,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":740,"record_offset":14920,"parent_record_offset":14888,"ancestor_record_offset":12864,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":740,"record_offset":13084,"parent_record_offset":13024,"ancestor_record_offset":12864,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$13::partial_cmp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":740,"record_offset":15620,"parent_record_offset":15596,"ancestor_record_offset":12864,"inlinee":24549,"source_line":6,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal
//     owner=private_source_ownership_candidate; count=10
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":13740,"parent_record_offset":13716,"ancestor_record_offset":12864,"inlinee":45793,"source_line":404,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":13764,"parent_record_offset":13740,"ancestor_record_offset":12864,"inlinee":45779,"source_line":203,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":13052,"parent_record_offset":13024,"ancestor_record_offset":12864,"inlinee":45789,"source_line":199,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::expand_along_edge
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":35536,"rva":9257008,"va":5377966128}
// 编译实例 RVAs: 0x8d4030
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25012653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377966128,"raw_hex":"e8ada97d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377966494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25012469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377966128,"raw_hex":"e8f5a87d01","size":5,"soft_fail":false,"target_va":5402978944,"va":5377966470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":35692,"parent_record_offset":35536,"ancestor_record_offset":35536,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":36168,"parent_record_offset":35536,"ancestor_record_offset":35536,"inlinee":14662,"source_line":1012,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":36232,"parent_record_offset":35536,"ancestor_record_offset":35536,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_edge_normal
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":35756,"parent_record_offset":35536,"ancestor_record_offset":35536,"inlinee":45794,"source_line":374,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":35848,"parent_record_offset":35824,"ancestor_record_offset":35536,"inlinee":5479,"source_line":1672,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":35872,"parent_record_offset":35848,"ancestor_record_offset":35536,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":36124,"parent_record_offset":36040,"ancestor_record_offset":35536,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":36068,"parent_record_offset":36040,"ancestor_record_offset":35536,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":36096,"parent_record_offset":36040,"ancestor_record_offset":35536,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":740,"record_offset":36328,"parent_record_offset":36288,"ancestor_record_offset":35536,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":36256,"parent_record_offset":36232,"ancestor_record_offset":35536,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":36196,"parent_record_offset":36168,"ancestor_record_offset":35536,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":35800,"parent_record_offset":35756,"ancestor_record_offset":35536,"inlinee":10010,"source_line":120,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":35824,"parent_record_offset":35800,"ancestor_record_offset":35536,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":35984,"parent_record_offset":35932,"ancestor_record_offset":35536,"inlinee":26062,"source_line":61,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":36040,"parent_record_offset":36016,"ancestor_record_offset":35536,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":36016,"parent_record_offset":35908,"ancestor_record_offset":35536,"inlinee":21770,"source_line":427,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":35956,"parent_record_offset":35932,"ancestor_record_offset":35536,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":35908,"parent_record_offset":35756,"ancestor_record_offset":35536,"inlinee":45793,"source_line":404,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":35932,"parent_record_offset":35908,"ancestor_record_offset":35536,"inlinee":45779,"source_line":203,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":27472,"rva":9252864,"va":5377961984}
// 编译实例 RVAs: 0x8d3000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t25016263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377961984,"raw_hex":"e8c7b77d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377962884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t25016079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377961984,"raw_hex":"e80fb77d01","size":5,"soft_fail":false,"target_va":5402978944,"va":5377962860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377961984,"raw_hex":"e8f0f9ffff","size":5,"soft_fail":false,"target_va":5377960512,"va":5377962059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":740,"record_offset":28524,"parent_record_offset":27472,"ancestor_record_offset":27472,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":740,"record_offset":28664,"parent_record_offset":27472,"ancestor_record_offset":27472,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":27632,"parent_record_offset":27472,"ancestor_record_offset":27472,"inlinee":45791,"source_line":223,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":27868,"parent_record_offset":27472,"ancestor_record_offset":27472,"inlinee":45790,"source_line":241,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":28068,"parent_record_offset":28044,"ancestor_record_offset":27472,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":28092,"parent_record_offset":28068,"ancestor_record_offset":27472,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":28132,"parent_record_offset":27992,"ancestor_record_offset":27472,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":740,"record_offset":28844,"parent_record_offset":28664,"ancestor_record_offset":27472,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":28576,"parent_record_offset":28524,"ancestor_record_offset":27472,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":28632,"parent_record_offset":28524,"ancestor_record_offset":27472,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":28548,"parent_record_offset":28524,"ancestor_record_offset":27472,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":28044,"parent_record_offset":28020,"ancestor_record_offset":27472,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":28020,"parent_record_offset":27992,"ancestor_record_offset":27472,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":740,"record_offset":27992,"parent_record_offset":27868,"ancestor_record_offset":27472,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":27740,"parent_record_offset":27688,"ancestor_record_offset":27472,"inlinee":45784,"source_line":195,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":27688,"parent_record_offset":27632,"ancestor_record_offset":27472,"inlinee":45785,"source_line":126,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":27908,"parent_record_offset":27868,"ancestor_record_offset":27472,"inlinee":45789,"source_line":199,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":17128,"rva":9249584,"va":5377958704}
// 编译实例 RVAs: 0x8d2330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t25020026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377958704,"raw_hex":"e87ac67d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377959121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":17472,"parent_record_offset":17128,"ancestor_record_offset":17128,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":17416,"parent_record_offset":17128,"ancestor_record_offset":17128,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":17292,"parent_record_offset":17128,"ancestor_record_offset":17128,"inlinee":45786,"source_line":155,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":17624,"parent_record_offset":17496,"ancestor_record_offset":17128,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":17380,"parent_record_offset":17356,"ancestor_record_offset":17128,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":17440,"parent_record_offset":17416,"ancestor_record_offset":17128,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":17556,"parent_record_offset":17524,"ancestor_record_offset":17128,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":17524,"parent_record_offset":17496,"ancestor_record_offset":17128,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":17496,"parent_record_offset":17472,"ancestor_record_offset":17128,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":17356,"parent_record_offset":17292,"ancestor_record_offset":17128,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":740,"record_offset":17588,"parent_record_offset":17556,"ancestor_record_offset":17128,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":17328,"parent_record_offset":17292,"ancestor_record_offset":17128,"inlinee":45784,"source_line":195,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_edge_id_at_u
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":24248,"rva":9252032,"va":5377961152}
// 编译实例 RVAs: 0x8d2cc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25017649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377961152,"raw_hex":"e831bd7d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377961498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25017465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377961152,"raw_hex":"e879bc7d01","size":5,"soft_fail":false,"target_va":5402978944,"va":5377961474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":25032,"parent_record_offset":24248,"ancestor_record_offset":24248,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":24396,"parent_record_offset":24248,"ancestor_record_offset":24248,"inlinee":45790,"source_line":241,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":24532,"parent_record_offset":24508,"ancestor_record_offset":24248,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":24556,"parent_record_offset":24532,"ancestor_record_offset":24248,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":24592,"parent_record_offset":24456,"ancestor_record_offset":24248,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":740,"record_offset":25372,"parent_record_offset":25344,"ancestor_record_offset":24248,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":24508,"parent_record_offset":24484,"ancestor_record_offset":24248,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":24484,"parent_record_offset":24456,"ancestor_record_offset":24248,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":740,"record_offset":24456,"parent_record_offset":24396,"ancestor_record_offset":24248,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":24424,"parent_record_offset":24396,"ancestor_record_offset":24248,"inlinee":45789,"source_line":199,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_pos_at_u
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":8216,"rva":9247024,"va":5377956144}
// 编译实例 RVAs: 0x8d1930
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t25022246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377956144,"raw_hex":"e826cf7d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377956901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25022062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377956144,"raw_hex":"e86ece7d01","size":5,"soft_fail":false,"target_va":5402978944,"va":5377956877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377956144,"raw_hex":"e89b000000","size":5,"soft_fail":false,"target_va":5377956944,"va":5377956784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":10020,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":10010,"source_line":120,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":740,"record_offset":9000,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":24539,"source_line":6,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":9056,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":45791,"source_line":223,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":8368,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":45790,"source_line":241,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":10208,"parent_record_offset":10184,"ancestor_record_offset":8216,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":8500,"parent_record_offset":8476,"ancestor_record_offset":8216,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":8524,"parent_record_offset":8500,"ancestor_record_offset":8216,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":10256,"parent_record_offset":10232,"ancestor_record_offset":8216,"inlinee":7043,"source_line":1506,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":9556,"parent_record_offset":9440,"ancestor_record_offset":8216,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":9500,"parent_record_offset":9440,"ancestor_record_offset":8216,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":9472,"parent_record_offset":9440,"ancestor_record_offset":8216,"inlinee":21773,"source_line":943,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":9528,"parent_record_offset":9440,"ancestor_record_offset":8216,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":8560,"parent_record_offset":8424,"ancestor_record_offset":8216,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":740,"record_offset":10528,"parent_record_offset":10500,"ancestor_record_offset":8216,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":9588,"parent_record_offset":9396,"ancestor_record_offset":8216,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":8476,"parent_record_offset":8452,"ancestor_record_offset":8216,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":8452,"parent_record_offset":8424,"ancestor_record_offset":8216,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":9176,"parent_record_offset":9152,"ancestor_record_offset":8216,"inlinee":26062,"source_line":61,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":9440,"parent_record_offset":9396,"ancestor_record_offset":8216,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":9396,"parent_record_offset":9368,"ancestor_record_offset":8216,"inlinee":21770,"source_line":427,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":740,"record_offset":8424,"parent_record_offset":8368,"ancestor_record_offset":8216,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$13::partial_cmp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":740,"record_offset":10232,"parent_record_offset":10208,"ancestor_record_offset":8216,"inlinee":24549,"source_line":6,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":9120,"parent_record_offset":9088,"ancestor_record_offset":8216,"inlinee":45784,"source_line":195,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":9088,"parent_record_offset":9056,"ancestor_record_offset":8216,"inlinee":45785,"source_line":126,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":9152,"parent_record_offset":9056,"ancestor_record_offset":8216,"inlinee":45779,"source_line":203,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":8396,"parent_record_offset":8368,"ancestor_record_offset":8216,"inlinee":45789,"source_line":199,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":29024,"rva":9253920,"va":5377963040}
// 编译实例 RVAs: 0x8d3420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25015453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377963040,"raw_hex":"e89db47d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377963694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":740,"record_offset":29180,"parent_record_offset":29024,"ancestor_record_offset":29024,"inlinee":45797,"source_line":72,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::aabb::Aabb2::intersects
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":740,"record_offset":30264,"parent_record_offset":29024,"ancestor_record_offset":29024,"inlinee":45801,"source_line":132,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::local_aabb_of
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":29460,"parent_record_offset":29024,"ancestor_record_offset":29024,"inlinee":45798,"source_line":536,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":29744,"parent_record_offset":29628,"ancestor_record_offset":29024,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":29688,"parent_record_offset":29628,"ancestor_record_offset":29024,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":29660,"parent_record_offset":29628,"ancestor_record_offset":29024,"inlinee":21773,"source_line":943,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":29716,"parent_record_offset":29628,"ancestor_record_offset":29024,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":29396,"parent_record_offset":29364,"ancestor_record_offset":29024,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":29332,"parent_record_offset":29308,"ancestor_record_offset":29024,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":29804,"parent_record_offset":29780,"ancestor_record_offset":29024,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":29572,"parent_record_offset":29516,"ancestor_record_offset":29024,"inlinee":26062,"source_line":61,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":29628,"parent_record_offset":29604,"ancestor_record_offset":29024,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":29604,"parent_record_offset":29460,"ancestor_record_offset":29024,"inlinee":21770,"source_line":427,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":29240,"parent_record_offset":29216,"ancestor_record_offset":29024,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":29216,"parent_record_offset":29180,"ancestor_record_offset":29024,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":29544,"parent_record_offset":29516,"ancestor_record_offset":29024,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":29364,"parent_record_offset":29180,"ancestor_record_offset":29024,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":29780,"parent_record_offset":29460,"ancestor_record_offset":29024,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":29308,"parent_record_offset":29180,"ancestor_record_offset":29024,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":740,"record_offset":30136,"parent_record_offset":29992,"ancestor_record_offset":29024,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":740,"record_offset":29272,"parent_record_offset":29240,"ancestor_record_offset":29024,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::aabb
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":29992,"parent_record_offset":29460,"ancestor_record_offset":29024,"inlinee":45799,"source_line":597,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::half_dimensions
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":30040,"parent_record_offset":29992,"ancestor_record_offset":29024,"inlinee":45800,"source_line":592,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":29516,"parent_record_offset":29460,"ancestor_record_offset":29024,"inlinee":45779,"source_line":203,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":10568,"rva":9247824,"va":5377956944}
// 编译实例 RVAs: 0x8d1c50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t25021511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377956944,"raw_hex":"e847cc7d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377957636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25021327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377956944,"raw_hex":"e88fcb7d01","size":5,"soft_fail":false,"target_va":5402978944,"va":5377957612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":12160,"parent_record_offset":10568,"ancestor_record_offset":10568,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":11320,"parent_record_offset":10568,"ancestor_record_offset":10568,"inlinee":45791,"source_line":223,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":10724,"parent_record_offset":10568,"ancestor_record_offset":10568,"inlinee":45790,"source_line":241,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::inverse_lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":740,"record_offset":11808,"parent_record_offset":10568,"ancestor_record_offset":10568,"inlinee":36056,"source_line":254,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":10928,"parent_record_offset":10904,"ancestor_record_offset":10568,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":10952,"parent_record_offset":10928,"ancestor_record_offset":10568,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":11680,"parent_record_offset":11592,"ancestor_record_offset":10568,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":11624,"parent_record_offset":11592,"ancestor_record_offset":10568,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":11652,"parent_record_offset":11592,"ancestor_record_offset":10568,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":10992,"parent_record_offset":10852,"ancestor_record_offset":10568,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":740,"record_offset":12452,"parent_record_offset":12424,"ancestor_record_offset":10568,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":12280,"parent_record_offset":12184,"ancestor_record_offset":10568,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":11744,"parent_record_offset":11716,"ancestor_record_offset":10568,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":12224,"parent_record_offset":12184,"ancestor_record_offset":10568,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":10904,"parent_record_offset":10880,"ancestor_record_offset":10568,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":10880,"parent_record_offset":10852,"ancestor_record_offset":10568,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":11532,"parent_record_offset":11476,"ancestor_record_offset":10568,"inlinee":26062,"source_line":61,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":11592,"parent_record_offset":11568,"ancestor_record_offset":10568,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":11568,"parent_record_offset":11320,"ancestor_record_offset":10568,"inlinee":21770,"source_line":427,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":11504,"parent_record_offset":11476,"ancestor_record_offset":10568,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":11716,"parent_record_offset":11320,"ancestor_record_offset":10568,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":12184,"parent_record_offset":12160,"ancestor_record_offset":10568,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":740,"record_offset":10852,"parent_record_offset":10724,"ancestor_record_offset":10568,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":11440,"parent_record_offset":11384,"ancestor_record_offset":10568,"inlinee":45784,"source_line":195,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":11384,"parent_record_offset":11320,"ancestor_record_offset":10568,"inlinee":45785,"source_line":126,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":11476,"parent_record_offset":11320,"ancestor_record_offset":10568,"inlinee":45779,"source_line":203,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":10768,"parent_record_offset":10724,"ancestor_record_offset":10568,"inlinee":45789,"source_line":199,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::impl$3::clone
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x66D6
// 首条位置: {"module":460,"inlinee":26326}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":244,"rva":9243136,"va":5377952256}
// 编译实例 RVAs: 0x8d0a00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t25026631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377952256,"raw_hex":"e847e07d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377952516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":372,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":45773,"source_line":113,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":440,"parent_record_offset":416,"ancestor_record_offset":244,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":416,"parent_record_offset":372,"ancestor_record_offset":244,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":584,"parent_record_offset":552,"ancestor_record_offset":244,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::is_normalized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":528,"parent_record_offset":372,"ancestor_record_offset":244,"inlinee":45774,"source_line":509,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":552,"parent_record_offset":528,"ancestor_record_offset":244,"inlinee":21909,"source_line":390,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":740,"record_offset":612,"parent_record_offset":552,"ancestor_record_offset":244,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::translate
// 记录形式: compiled_procedure; instances=1
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":656,"rva":9243456,"va":5377952576}
// 编译实例 RVAs: 0x8d0b40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":792,"parent_record_offset":656,"ancestor_record_offset":656,"inlinee":14662,"source_line":1012,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":816,"parent_record_offset":792,"ancestor_record_offset":656,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::rotate_2d
// 记录形式: compiled_procedure; instances=1
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":856,"rva":9243504,"va":5377952624}
// 编译实例 RVAs: 0x8d0b70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":992,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":1016,"parent_record_offset":992,"ancestor_record_offset":856,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::set_orientation_xz
// 记录形式: compiled_procedure; instances=1
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":1104,"rva":9243552,"va":5377952672}
// 编译实例 RVAs: 0x8d0ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::overlaps_rectangle3d
// 记录形式: compiled_procedure; instances=1
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":1240,"rva":9243568,"va":5377952688}
// 编译实例 RVAs: 0x8d0bb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::impl$6::intersects
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":740,"record_offset":1388,"parent_record_offset":1240,"ancestor_record_offset":1240,"inlinee":45776,"source_line":479,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::contains_point
// 记录形式: compiled_procedure; instances=1
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":1416,"rva":9243600,"va":5377952720}
// 编译实例 RVAs: 0x8d0bd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":1560,"parent_record_offset":1416,"ancestor_record_offset":1416,"inlinee":10010,"source_line":120,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::contains_point
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":1664,"parent_record_offset":1416,"ancestor_record_offset":1416,"inlinee":45777,"source_line":408,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":1608,"parent_record_offset":1584,"ancestor_record_offset":1416,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":1628,"parent_record_offset":1608,"ancestor_record_offset":1416,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":2020,"parent_record_offset":1936,"ancestor_record_offset":1416,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_cmple_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":2080,"parent_record_offset":2056,"ancestor_record_offset":1416,"inlinee":45781,"source_line":505,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":1964,"parent_record_offset":1936,"ancestor_record_offset":1416,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":1992,"parent_record_offset":1936,"ancestor_record_offset":1416,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":1880,"parent_record_offset":1856,"ancestor_record_offset":1416,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":1584,"parent_record_offset":1560,"ancestor_record_offset":1416,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::bool::sse2::bvec4a::BVec4A::all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/bool/sse2/bvec4a.rs
//     evidence={"module":740,"record_offset":2112,"parent_record_offset":1688,"ancestor_record_offset":1416,"inlinee":45782,"source_line":85,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":1792,"parent_record_offset":1728,"ancestor_record_offset":1416,"inlinee":26062,"source_line":61,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":1416,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":1912,"parent_record_offset":1688,"ancestor_record_offset":1416,"inlinee":21770,"source_line":427,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::cmple
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":740,"record_offset":2056,"parent_record_offset":1688,"ancestor_record_offset":1416,"inlinee":45780,"source_line":354,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":740,"record_offset":1824,"parent_record_offset":1688,"ancestor_record_offset":1416,"inlinee":14703,"source_line":92,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":1760,"parent_record_offset":1728,"ancestor_record_offset":1416,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":1856,"parent_record_offset":1688,"ancestor_record_offset":1416,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::contains_point_padding
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":1416,"inlinee":45778,"source_line":413,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":1728,"parent_record_offset":1688,"ancestor_record_offset":1416,"inlinee":45779,"source_line":203,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::center
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x590B
// 首条位置: {"module":397,"inlinee":22795}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::height
// 记录形式: inline_instance, inlinee_declaration; instances=54
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x54E8
// 首条位置: {"module":383,"inlinee":21736}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::width
// 记录形式: inline_instance, inlinee_declaration; instances=51
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x54E7
// 首条位置: {"module":383,"inlinee":21735}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::length
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2D7
// 首条位置: {"module":740,"inlinee":45783}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle3d::extent
// 记录形式: compiled_procedure; instances=1
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":2152,"rva":9243728,"va":5377952848}
// 编译实例 RVAs: 0x8d0c50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":740,"record_offset":2372,"parent_record_offset":2152,"ancestor_record_offset":2152,"inlinee":14693,"source_line":76,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":2296,"parent_record_offset":2152,"ancestor_record_offset":2152,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":2344,"parent_record_offset":2152,"ancestor_record_offset":2152,"inlinee":45783,"source_line":75,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":2320,"parent_record_offset":2152,"ancestor_record_offset":2152,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B11
// 首条位置: {"module":1326,"inlinee":27409}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::deserialize::impl$0::expecting
// 记录形式: compiled_procedure; instances=1
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":38768,"rva":9258672,"va":5377967792}
// 编译实例 RVAs: 0x8d46b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-383893","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377967792,"raw_hex":"e96b24faff","size":5,"soft_fail":false,"target_va":5377583920,"va":5377967808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_bytes<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":486,"record_offset":2584,"rva":6909648,"va":5375618768}
// 编译实例 RVAs: 0x696ed0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":486,"record_offset":2760,"rva":6909968,"va":5375619088}
// 编译实例 RVAs: 0x697010
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":486,"record_offset":2940,"parent_record_offset":2760,"ancestor_record_offset":2760,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":486,"record_offset":2960,"parent_record_offset":2940,"ancestor_record_offset":2760,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":486,"record_offset":2980,"parent_record_offset":2960,"ancestor_record_offset":2760,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":486,"record_offset":3000,"parent_record_offset":2980,"ancestor_record_offset":2760,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00486.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_u64
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6AFF
// 首条位置: {"module":486,"inlinee":27391}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1024A
// 首条位置: {"module":1421,"inlinee":66122}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::deserialize::impl$2::expecting
// 记录形式: compiled_procedure; instances=1
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":38916,"rva":9258704,"va":5377967824}
// 编译实例 RVAs: 0x8d46d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-383925","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377967824,"raw_hex":"e94b24faff","size":5,"soft_fail":false,"target_va":5377583920,"va":5377967840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10247
// 首条位置: {"module":1421,"inlinee":66119}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1421,"record_offset":13612,"parent_record_offset":12188,"ancestor_record_offset":12188,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1421,"record_offset":14676,"parent_record_offset":12188,"ancestor_record_offset":12188,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":14888,"parent_record_offset":12188,"ancestor_record_offset":12188,"inlinee":6896,"source_line":23,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":12244,"parent_record_offset":12188,"ancestor_record_offset":12188,"inlinee":9789,"source_line":1936,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":12756,"parent_record_offset":12188,"ancestor_record_offset":12188,"inlinee":9791,"source_line":1944,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":1421,"record_offset":13388,"parent_record_offset":13360,"ancestor_record_offset":12188,"inlinee":9813,"source_line":750,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":1421,"record_offset":13360,"parent_record_offset":13324,"ancestor_record_offset":12188,"inlinee":9812,"source_line":719,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1421,"record_offset":12440,"parent_record_offset":12404,"ancestor_record_offset":12188,"inlinee":5090,"source_line":358,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1421,"record_offset":12404,"parent_record_offset":12364,"ancestor_record_offset":12188,"inlinee":5088,"source_line":49,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1421,"record_offset":12516,"parent_record_offset":12440,"ancestor_record_offset":12188,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1421,"record_offset":12476,"parent_record_offset":12440,"ancestor_record_offset":12188,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1421,"record_offset":12588,"parent_record_offset":12564,"ancestor_record_offset":12188,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1421,"record_offset":12704,"parent_record_offset":12316,"ancestor_record_offset":12188,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1421,"record_offset":12564,"parent_record_offset":12540,"ancestor_record_offset":12188,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1421,"record_offset":12540,"parent_record_offset":12516,"ancestor_record_offset":12188,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$13::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/features/impl_serde.rs
//     evidence={"module":1421,"record_offset":14628,"parent_record_offset":14600,"ancestor_record_offset":12188,"inlinee":24930,"source_line":16,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":13188,"parent_record_offset":13116,"ancestor_record_offset":12188,"inlinee":51269,"source_line":1665,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_f32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":13116,"parent_record_offset":13044,"ancestor_record_offset":12188,"inlinee":51268,"source_line":1826,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":14960,"parent_record_offset":14936,"ancestor_record_offset":12188,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1421,"record_offset":15048,"parent_record_offset":15024,"ancestor_record_offset":12188,"inlinee":6871,"source_line":126,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_tuple_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1421,"record_offset":14936,"parent_record_offset":14912,"ancestor_record_offset":12188,"inlinee":11581,"source_line":126,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":12280,"parent_record_offset":12244,"ancestor_record_offset":12188,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":12828,"parent_record_offset":12756,"ancestor_record_offset":12188,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":12640,"parent_record_offset":12316,"ancestor_record_offset":12188,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":13044,"parent_record_offset":12972,"ancestor_record_offset":12188,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":13324,"parent_record_offset":13188,"ancestor_record_offset":12188,"inlinee":9811,"source_line":227,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":13524,"parent_record_offset":13188,"ancestor_record_offset":12188,"inlinee":9820,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":13492,"parent_record_offset":13188,"ancestor_record_offset":12188,"inlinee":9817,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":13556,"parent_record_offset":13188,"ancestor_record_offset":12188,"inlinee":9815,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":13460,"parent_record_offset":13188,"ancestor_record_offset":12188,"inlinee":9816,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::MapDeserializer::next_pair
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1421,"record_offset":12364,"parent_record_offset":12316,"ancestor_record_offset":12188,"inlinee":7217,"source_line":1146,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$57::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1421,"record_offset":12316,"parent_record_offset":12280,"ancestor_record_offset":12188,"inlinee":7249,"source_line":1210,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$57::next_value_seed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1421,"record_offset":12900,"parent_record_offset":12828,"ancestor_record_offset":12188,"inlinee":7251,"source_line":1223,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::copysign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1421,"record_offset":13424,"parent_record_offset":13324,"ancestor_record_offset":12188,"inlinee":9814,"source_line":251,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1421,"record_offset":12668,"parent_record_offset":12640,"ancestor_record_offset":12188,"inlinee":66122,"source_line":91,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10248
// 首条位置: {"module":1421,"inlinee":66120}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1421,"record_offset":17624,"parent_record_offset":15604,"ancestor_record_offset":15604,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$6::next_element
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":15660,"parent_record_offset":15604,"ancestor_record_offset":15604,"inlinee":9782,"source_line":1753,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":1421,"record_offset":16380,"parent_record_offset":16356,"ancestor_record_offset":15604,"inlinee":9813,"source_line":750,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":1421,"record_offset":16356,"parent_record_offset":16328,"ancestor_record_offset":15604,"inlinee":9812,"source_line":719,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1421,"record_offset":15768,"parent_record_offset":15744,"ancestor_record_offset":15604,"inlinee":5090,"source_line":358,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1421,"record_offset":15744,"parent_record_offset":15716,"ancestor_record_offset":15604,"inlinee":5088,"source_line":49,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1421,"record_offset":17404,"parent_record_offset":17348,"ancestor_record_offset":15604,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1421,"record_offset":15792,"parent_record_offset":15768,"ancestor_record_offset":15604,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1421,"record_offset":17476,"parent_record_offset":17452,"ancestor_record_offset":15604,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1421,"record_offset":15884,"parent_record_offset":15716,"ancestor_record_offset":15604,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1421,"record_offset":17452,"parent_record_offset":17428,"ancestor_record_offset":15604,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1421,"record_offset":17428,"parent_record_offset":17404,"ancestor_record_offset":15604,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$13::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/features/impl_serde.rs
//     evidence={"module":1421,"record_offset":15852,"parent_record_offset":15828,"ancestor_record_offset":15604,"inlinee":24930,"source_line":16,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":16252,"parent_record_offset":16212,"ancestor_record_offset":15604,"inlinee":51269,"source_line":1665,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_f32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1421,"record_offset":16212,"parent_record_offset":16172,"ancestor_record_offset":15604,"inlinee":51268,"source_line":1826,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":15688,"parent_record_offset":15660,"ancestor_record_offset":15604,"inlinee":9784,"source_line":1724,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1421,"record_offset":15828,"parent_record_offset":15716,"ancestor_record_offset":15604,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":16172,"parent_record_offset":16132,"ancestor_record_offset":15604,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":16328,"parent_record_offset":16252,"ancestor_record_offset":15604,"inlinee":9811,"source_line":227,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":16528,"parent_record_offset":16252,"ancestor_record_offset":15604,"inlinee":9820,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":16500,"parent_record_offset":16252,"ancestor_record_offset":15604,"inlinee":9817,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":16444,"parent_record_offset":16252,"ancestor_record_offset":15604,"inlinee":9815,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1421,"record_offset":16472,"parent_record_offset":16252,"ancestor_record_offset":15604,"inlinee":9816,"source_line":193,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$45::next_element_seed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1421,"record_offset":15716,"parent_record_offset":15688,"ancestor_record_offset":15604,"inlinee":7253,"source_line":963,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::copysign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1421,"record_offset":16412,"parent_record_offset":16328,"ancestor_record_offset":15604,"inlinee":9814,"source_line":251,"archive":"_pdb/A/modules/01421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=2
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":182,"record_offset":34060,"rva":3058336,"va":5371767456}
// 编译实例 RVAs: 0x2eaaa0, 0x697310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140017e80
//     owner=unresolved_namespace; count=1
//     target_aliases=13 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2960908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371767456,"raw_hex":"e8f4d1d2ff","size":5,"soft_fail":false,"target_va":5368807040,"va":5371767943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401565c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1656159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371767456,"raw_hex":"e8a1bae6ff","size":5,"soft_fail":false,"target_va":5370111424,"va":5371767578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140156880
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1655420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371767456,"raw_hex":"e884bde6ff","size":5,"soft_fail":false,"target_va":5370112128,"va":5371767543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140361670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t486055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371767456,"raw_hex":"e8a76a0700","size":5,"soft_fail":false,"target_va":5372253808,"va":5371767748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t981930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371767456,"raw_hex":"e8aafb0e00","size":5,"soft_fail":false,"target_va":5372749712,"va":5371767777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405cac50
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-837804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375619856,"raw_hex":"e85437f3ff","size":5,"soft_fail":false,"target_va":5374782544,"va":5375620343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d99a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t271719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375619856,"raw_hex":"e867250400","size":5,"soft_fail":false,"target_va":5375891872,"va":5375620148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406fa580
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4258147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371767456,"raw_hex":"e863f94000","size":5,"soft_fail":false,"target_va":5376025984,"va":5371767832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141096f40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10484657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375619856,"raw_hex":"e8b1fb9f00","size":5,"soft_fail":false,"target_va":5386104640,"va":5375619978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14109a3b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10498116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375619856,"raw_hex":"e84430a000","size":5,"soft_fail":false,"target_va":5386118064,"va":5375619943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":182,"record_offset":34260,"parent_record_offset":34060,"ancestor_record_offset":34060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":182,"record_offset":34232,"parent_record_offset":34060,"ancestor_record_offset":34060,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":182,"record_offset":34644,"parent_record_offset":34620,"ancestor_record_offset":34060,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":182,"record_offset":34620,"parent_record_offset":34564,"ancestor_record_offset":34060,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":182,"record_offset":34336,"parent_record_offset":34312,"ancestor_record_offset":34060,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":182,"record_offset":34392,"parent_record_offset":34336,"ancestor_record_offset":34060,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":182,"record_offset":34592,"parent_record_offset":34564,"ancestor_record_offset":34060,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":182,"record_offset":34564,"parent_record_offset":34536,"ancestor_record_offset":34060,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":182,"record_offset":34424,"parent_record_offset":34336,"ancestor_record_offset":34060,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":182,"record_offset":34288,"parent_record_offset":34260,"ancestor_record_offset":34060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":182,"record_offset":34360,"parent_record_offset":34336,"ancestor_record_offset":34060,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::impl$10::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D24
// 首条位置: {"module":2269,"inlinee":81188}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$46::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2269,"record_offset":15868,"parent_record_offset":15832,"ancestor_record_offset":15832,"inlinee":5676,"source_line":18,"archive":"_pdb/A/modules/02269.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::impl$7::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6AFC
// 首条位置: {"module":486,"inlinee":27388}
// 编译实例 RVAs: 0xec33a0, 0xf02c50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field4_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6604272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384188832,"raw_hex":"e8103a9bff","size":5,"soft_fail":false,"target_va":5377584768,"va":5384189035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::new_centered_axis_aligned
// 记录形式: compiled_procedure; instances=1
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":2404,"rva":9243776,"va":5377952896}
// 编译实例 RVAs: 0x8d0c80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t25026161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377952896,"raw_hex":"e871de7d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377952986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":2560,"parent_record_offset":2404,"ancestor_record_offset":2404,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":2664,"parent_record_offset":2404,"ancestor_record_offset":2404,"inlinee":45773,"source_line":113,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":2580,"parent_record_offset":2560,"ancestor_record_offset":2404,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::new_axis_aligned
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB305
// 首条位置: {"module":740,"inlinee":45829}
// 编译实例 RVAs: 0x8d0d00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t25026017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377953024,"raw_hex":"e8e1dd7d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377953130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":2844,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":2948,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":45773,"source_line":113,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":2864,"parent_record_offset":2844,"ancestor_record_offset":2696,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2CD
// 首条位置: {"module":740,"inlinee":45773}
// 编译实例 RVAs: 0x8d0d90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t25025817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377953168,"raw_hex":"e819dd7d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377953330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":416,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::is_normalized
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":528,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":45774,"source_line":509,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":440,"parent_record_offset":416,"ancestor_record_offset":372,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":584,"parent_record_offset":552,"ancestor_record_offset":372,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":552,"parent_record_offset":528,"ancestor_record_offset":372,"inlinee":21909,"source_line":390,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":740,"record_offset":612,"parent_record_offset":552,"ancestor_record_offset":372,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=19
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2D9
// 首条位置: {"module":740,"inlinee":45785}
// 编译实例 RVAs: 0x8d0e70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25025671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377953392,"raw_hex":"e887dc7d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377953476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=9
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":3492,"parent_record_offset":3340,"ancestor_record_offset":3340,"inlinee":45784,"source_line":195,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::as_points2
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 155
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2DA
// 首条位置: {"module":740,"inlinee":45786}
// 编译实例 RVAs: 0x8d0ed0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25025441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377953488,"raw_hex":"e8a1db7d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377953706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":3820,"parent_record_offset":3520,"ancestor_record_offset":3520,"inlinee":21770,"source_line":427,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":3980,"parent_record_offset":3520,"ancestor_record_offset":3520,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":3672,"parent_record_offset":3520,"ancestor_record_offset":3520,"inlinee":45784,"source_line":195,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":3780,"parent_record_offset":3520,"ancestor_record_offset":3520,"inlinee":45785,"source_line":126,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":3696,"parent_record_offset":3520,"ancestor_record_offset":3520,"inlinee":45779,"source_line":203,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":4672,"parent_record_offset":4532,"ancestor_record_offset":4396,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":3932,"parent_record_offset":3852,"ancestor_record_offset":3520,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":3892,"parent_record_offset":3852,"ancestor_record_offset":3520,"inlinee":21773,"source_line":943,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":4644,"parent_record_offset":4532,"ancestor_record_offset":4396,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":4008,"parent_record_offset":3980,"ancestor_record_offset":3520,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":3748,"parent_record_offset":3696,"ancestor_record_offset":3520,"inlinee":26062,"source_line":61,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":3852,"parent_record_offset":3820,"ancestor_record_offset":3520,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":3720,"parent_record_offset":3696,"ancestor_record_offset":3520,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::as_points3
// 记录形式: compiled_procedure; instances=1
// 源码行: 171
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":4048,"rva":9244592,"va":5377953712}
// 编译实例 RVAs: 0x8d0fb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25025098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377953712,"raw_hex":"e84ada7d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377954049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":4184,"parent_record_offset":4048,"ancestor_record_offset":4048,"inlinee":45784,"source_line":195,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::as_curve2
// 记录形式: compiled_procedure; instances=1
// 源码行: 181
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":4216,"rva":9244944,"va":5377954064}
// 编译实例 RVAs: 0x8d1110
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5204482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377954064,"raw_hex":"e8fe95b0ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5377954173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5204896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377954064,"raw_hex":"e86094b0ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5377954603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412c5700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10437741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377954064,"raw_hex":"e86d449f00","size":5,"soft_fail":false,"target_va":5388392192,"va":5377954446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25023618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377954064,"raw_hex":"e882d47d01","size":5,"soft_fail":false,"target_va":5402978144,"va":5377954521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6647288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377954064,"raw_hex":"e808929aff","size":5,"soft_fail":false,"target_va":5371307104,"va":5377954387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25024641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377954064,"raw_hex":"e881d87d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377954506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":740,"record_offset":5148,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":740,"record_offset":4784,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":740,"record_offset":5268,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":4368,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":45784,"source_line":195,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":4396,"parent_record_offset":4216,"ancestor_record_offset":4216,"inlinee":45786,"source_line":155,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":740,"record_offset":4960,"parent_record_offset":4936,"ancestor_record_offset":4216,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":740,"record_offset":4984,"parent_record_offset":4960,"ancestor_record_offset":4216,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":740,"record_offset":5444,"parent_record_offset":5420,"ancestor_record_offset":4216,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":740,"record_offset":4904,"parent_record_offset":4864,"ancestor_record_offset":4216,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":740,"record_offset":4936,"parent_record_offset":4904,"ancestor_record_offset":4216,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":740,"record_offset":5420,"parent_record_offset":5320,"ancestor_record_offset":4216,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":740,"record_offset":4864,"parent_record_offset":4824,"ancestor_record_offset":4216,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":740,"record_offset":5352,"parent_record_offset":5320,"ancestor_record_offset":4216,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":740,"record_offset":5204,"parent_record_offset":5180,"ancestor_record_offset":4216,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":740,"record_offset":5320,"parent_record_offset":5292,"ancestor_record_offset":4216,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":740,"record_offset":5080,"parent_record_offset":5056,"ancestor_record_offset":4216,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":740,"record_offset":5056,"parent_record_offset":4824,"ancestor_record_offset":4216,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":740,"record_offset":5180,"parent_record_offset":5148,"ancestor_record_offset":4216,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":740,"record_offset":5104,"parent_record_offset":5080,"ancestor_record_offset":4216,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$35::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":740,"record_offset":4824,"parent_record_offset":4784,"ancestor_record_offset":4216,"inlinee":45788,"source_line":3337,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":4672,"parent_record_offset":4532,"ancestor_record_offset":4216,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":4608,"parent_record_offset":4532,"ancestor_record_offset":4216,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":4572,"parent_record_offset":4532,"ancestor_record_offset":4216,"inlinee":21773,"source_line":943,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":4644,"parent_record_offset":4532,"ancestor_record_offset":4216,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":740,"record_offset":5384,"parent_record_offset":5352,"ancestor_record_offset":4216,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":4740,"parent_record_offset":4708,"ancestor_record_offset":4216,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":740,"record_offset":5292,"parent_record_offset":5268,"ancestor_record_offset":4216,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":740,"record_offset":5008,"parent_record_offset":4984,"ancestor_record_offset":4216,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":740,"record_offset":5236,"parent_record_offset":5148,"ancestor_record_offset":4216,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":4532,"parent_record_offset":4504,"ancestor_record_offset":4216,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":4504,"parent_record_offset":4396,"ancestor_record_offset":4216,"inlinee":21770,"source_line":427,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":4708,"parent_record_offset":4396,"ancestor_record_offset":4216,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":4472,"parent_record_offset":4396,"ancestor_record_offset":4216,"inlinee":45785,"source_line":126,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":4444,"parent_record_offset":4396,"ancestor_record_offset":4216,"inlinee":45779,"source_line":203,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::as_curve3
// 记录形式: compiled_procedure; instances=1
// 源码行: 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":5492,"rva":9245536,"va":5377954656}
// 编译实例 RVAs: 0x8d1360
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5205084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377954656,"raw_hex":"e8a493b0ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5377954775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5205645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377954656,"raw_hex":"e87391b0ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5377955352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412c59e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10437752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377954656,"raw_hex":"e878449f00","size":5,"soft_fail":false,"target_va":5388392928,"va":5377955171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25022888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377954656,"raw_hex":"e8a8d17d01","size":5,"soft_fail":false,"target_va":5402978144,"va":5377955251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6648943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377954656,"raw_hex":"e8918b9aff","size":5,"soft_fail":false,"target_va":5371306160,"va":5377955098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25023911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377954656,"raw_hex":"e8a7d57d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377955236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":740,"record_offset":6000,"parent_record_offset":5492,"ancestor_record_offset":5492,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":740,"record_offset":5668,"parent_record_offset":5492,"ancestor_record_offset":5492,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":740,"record_offset":6132,"parent_record_offset":5492,"ancestor_record_offset":5492,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":5640,"parent_record_offset":5492,"ancestor_record_offset":5492,"inlinee":45784,"source_line":195,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":740,"record_offset":5808,"parent_record_offset":5784,"ancestor_record_offset":5492,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":740,"record_offset":5832,"parent_record_offset":5808,"ancestor_record_offset":5492,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":740,"record_offset":6308,"parent_record_offset":6284,"ancestor_record_offset":5492,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":740,"record_offset":5752,"parent_record_offset":5724,"ancestor_record_offset":5492,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":740,"record_offset":5784,"parent_record_offset":5752,"ancestor_record_offset":5492,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":740,"record_offset":6284,"parent_record_offset":6184,"ancestor_record_offset":5492,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":740,"record_offset":5724,"parent_record_offset":5696,"ancestor_record_offset":5492,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":740,"record_offset":6216,"parent_record_offset":6184,"ancestor_record_offset":5492,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":740,"record_offset":6068,"parent_record_offset":6032,"ancestor_record_offset":5492,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":740,"record_offset":6184,"parent_record_offset":6156,"ancestor_record_offset":5492,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":740,"record_offset":5932,"parent_record_offset":5908,"ancestor_record_offset":5492,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":740,"record_offset":5908,"parent_record_offset":5696,"ancestor_record_offset":5492,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":740,"record_offset":6032,"parent_record_offset":6000,"ancestor_record_offset":5492,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":740,"record_offset":5956,"parent_record_offset":5932,"ancestor_record_offset":5492,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$35::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":740,"record_offset":5696,"parent_record_offset":5668,"ancestor_record_offset":5492,"inlinee":45788,"source_line":3337,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":740,"record_offset":6248,"parent_record_offset":6216,"ancestor_record_offset":5492,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":740,"record_offset":6156,"parent_record_offset":6132,"ancestor_record_offset":5492,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":740,"record_offset":5860,"parent_record_offset":5832,"ancestor_record_offset":5492,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":740,"record_offset":6100,"parent_record_offset":6000,"ancestor_record_offset":5492,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::area
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=18
// 源码行: 195, 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2D8
// 首条位置: {"module":740,"inlinee":45784}
// 编译实例 RVAs: 0x8d1650
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::perimeter
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=20
// 源码行: 199, 200
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2DD
// 首条位置: {"module":740,"inlinee":45789}
// 编译实例 RVAs: 0x8d1660
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::orientation_mat2
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=30
// 源码行: 203
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2D3
// 首条位置: {"module":740,"inlinee":45779}
// 编译实例 RVAs: 0x8d1670
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=28
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":1792,"parent_record_offset":1728,"ancestor_record_offset":1728,"inlinee":26062,"source_line":61,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":15112,"parent_record_offset":15048,"ancestor_record_offset":15048,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":1760,"parent_record_offset":1728,"ancestor_record_offset":1728,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":15168,"parent_record_offset":15112,"ancestor_record_offset":15048,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":15140,"parent_record_offset":15112,"ancestor_record_offset":15048,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::orientation_mat3
// 记录形式: compiled_procedure; instances=1
// 源码行: 208
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":6784,"rva":9246368,"va":5377955488}
// 编译实例 RVAs: 0x8d16a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::mat3::Mat3::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/mat3.rs
//     evidence={"module":740,"record_offset":6988,"parent_record_offset":6784,"ancestor_record_offset":6784,"inlinee":36060,"source_line":82,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":6960,"parent_record_offset":6784,"ancestor_record_offset":6784,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::swizzlers::impl$1::xvy
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/swizzlers.rs
//     evidence={"module":740,"record_offset":6936,"parent_record_offset":6784,"ancestor_record_offset":6784,"inlinee":36112,"source_line":18,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::as_transformation_and_size
// 记录形式: compiled_procedure, inlinee_declaration; instances=2
// 源码行: 216
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2F0
// 首条位置: {"module":740,"inlinee":45808}
// 编译实例 RVAs: 0x8d16f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":7172,"parent_record_offset":7020,"ancestor_record_offset":7020,"inlinee":45779,"source_line":203,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":7192,"parent_record_offset":7172,"ancestor_record_offset":7020,"inlinee":26062,"source_line":61,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_point
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=17
// 源码行: 223
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2DF
// 首条位置: {"module":740,"inlinee":45791}
// 编译实例 RVAs: 0x8d1730
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t25023340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377955632,"raw_hex":"e86cd37d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377955807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":7520,"parent_record_offset":7224,"ancestor_record_offset":7224,"inlinee":21770,"source_line":427,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":7664,"parent_record_offset":7224,"ancestor_record_offset":7224,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=15
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":7372,"parent_record_offset":7224,"ancestor_record_offset":7224,"inlinee":45785,"source_line":126,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":7436,"parent_record_offset":7224,"ancestor_record_offset":7224,"inlinee":45779,"source_line":203,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":7628,"parent_record_offset":7544,"ancestor_record_offset":7224,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":7572,"parent_record_offset":7544,"ancestor_record_offset":7224,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":9472,"parent_record_offset":9440,"ancestor_record_offset":9368,"inlinee":21773,"source_line":943,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":7600,"parent_record_offset":7544,"ancestor_record_offset":7224,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":7692,"parent_record_offset":7664,"ancestor_record_offset":7224,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":7488,"parent_record_offset":7436,"ancestor_record_offset":7224,"inlinee":26062,"source_line":61,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":7544,"parent_record_offset":7520,"ancestor_record_offset":7224,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":7460,"parent_record_offset":7436,"ancestor_record_offset":7224,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":7408,"parent_record_offset":7372,"ancestor_record_offset":7224,"inlinee":45784,"source_line":195,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_point_u
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=23
// 源码行: 241
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2DE
// 首条位置: {"module":740,"inlinee":45790}
// 编译实例 RVAs: 0x8d1800
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t25023128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377955840,"raw_hex":"e898d27d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377956019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25022832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377955840,"raw_hex":"e870d17d01","size":5,"soft_fail":false,"target_va":5402978944,"va":5377956107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=22
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":740,"record_offset":7980,"parent_record_offset":7756,"ancestor_record_offset":7756,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=private_source_ownership_candidate; count=18
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":7896,"parent_record_offset":7756,"ancestor_record_offset":7756,"inlinee":45789,"source_line":199,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":8064,"parent_record_offset":8040,"ancestor_record_offset":7756,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":8088,"parent_record_offset":8064,"ancestor_record_offset":7756,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":8124,"parent_record_offset":7980,"ancestor_record_offset":7756,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":740,"record_offset":8152,"parent_record_offset":7980,"ancestor_record_offset":7756,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":8040,"parent_record_offset":8016,"ancestor_record_offset":7756,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":8016,"parent_record_offset":7980,"ancestor_record_offset":7756,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_normal_at_u
// 记录形式: compiled_procedure; instances=1
// 源码行: 290
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":12576,"rva":9248560,"va":5377957680}
// 编译实例 RVAs: 0x8d1f30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t19","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377957680,"raw_hex":"e913000000","size":5,"soft_fail":false,"target_va":5377957712,"va":5377957688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_normal_at_u_non_normalized_corners
// 记录形式: compiled_procedure; instances=1
// 源码行: 294
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":12708,"rva":9248576,"va":5377957696}
// 编译实例 RVAs: 0x8d1f40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t3","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377957696,"raw_hex":"e903000000","size":5,"soft_fail":false,"target_va":5377957712,"va":5377957704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_corner_normal
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 324
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2E0
// 首条位置: {"module":740,"inlinee":45792}
// 编译实例 RVAs: 0x8d2260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25020461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377958496,"raw_hex":"e82dc87d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377958686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":14992,"parent_record_offset":14636,"ancestor_record_offset":14636,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":16876,"parent_record_offset":16432,"ancestor_record_offset":16432,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":13740,"parent_record_offset":13716,"ancestor_record_offset":13716,"inlinee":45793,"source_line":404,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":14172,"parent_record_offset":14108,"ancestor_record_offset":13940,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":13896,"parent_record_offset":13872,"ancestor_record_offset":13716,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":16752,"parent_record_offset":16720,"ancestor_record_offset":16432,"inlinee":21773,"source_line":943,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":14140,"parent_record_offset":14108,"ancestor_record_offset":13940,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":14956,"parent_record_offset":14828,"ancestor_record_offset":14636,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":13816,"parent_record_offset":13764,"ancestor_record_offset":13716,"inlinee":26062,"source_line":61,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":13872,"parent_record_offset":13848,"ancestor_record_offset":13716,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":13848,"parent_record_offset":13740,"ancestor_record_offset":13716,"inlinee":21770,"source_line":427,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":14888,"parent_record_offset":14856,"ancestor_record_offset":14636,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":14856,"parent_record_offset":14828,"ancestor_record_offset":14636,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":14828,"parent_record_offset":14680,"ancestor_record_offset":14636,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":13788,"parent_record_offset":13764,"ancestor_record_offset":13716,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":740,"record_offset":14920,"parent_record_offset":14888,"ancestor_record_offset":14636,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":13764,"parent_record_offset":13740,"ancestor_record_offset":13716,"inlinee":45779,"source_line":203,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_edge_pts
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 363
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB303
// 首条位置: {"module":740,"inlinee":45827}
// 编译实例 RVAs: 0x8d24f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t25019727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377959152,"raw_hex":"e84fc57d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377959420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":18352,"parent_record_offset":18212,"ancestor_record_offset":18212,"inlinee":45786,"source_line":155,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":18380,"parent_record_offset":18352,"ancestor_record_offset":18212,"inlinee":45784,"source_line":195,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_edge_normal
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 374
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2E2
// 首条位置: {"module":740,"inlinee":45794}
// 编译实例 RVAs: 0x8d2620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25019569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377959456,"raw_hex":"e8b1c47d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377959578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":18588,"parent_record_offset":18444,"ancestor_record_offset":18444,"inlinee":10010,"source_line":120,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":15348,"parent_record_offset":15324,"ancestor_record_offset":15324,"inlinee":45793,"source_line":404,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":18636,"parent_record_offset":18612,"ancestor_record_offset":18444,"inlinee":5479,"source_line":1672,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":18656,"parent_record_offset":18636,"ancestor_record_offset":18444,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":18908,"parent_record_offset":18824,"ancestor_record_offset":18444,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":15476,"parent_record_offset":15452,"ancestor_record_offset":15324,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":18880,"parent_record_offset":18824,"ancestor_record_offset":18444,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":18612,"parent_record_offset":18588,"ancestor_record_offset":18444,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":15396,"parent_record_offset":15372,"ancestor_record_offset":15324,"inlinee":26062,"source_line":61,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":15452,"parent_record_offset":15428,"ancestor_record_offset":15324,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":15428,"parent_record_offset":15348,"ancestor_record_offset":15324,"inlinee":21770,"source_line":427,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":18740,"parent_record_offset":18716,"ancestor_record_offset":18444,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":15372,"parent_record_offset":15348,"ancestor_record_offset":15324,"inlinee":45779,"source_line":203,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 404
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2E1
// 首条位置: {"module":740,"inlinee":45793}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":13848,"parent_record_offset":13740,"ancestor_record_offset":13740,"inlinee":21770,"source_line":427,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":14828,"parent_record_offset":14680,"ancestor_record_offset":14680,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=10
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":13764,"parent_record_offset":13740,"ancestor_record_offset":13740,"inlinee":45779,"source_line":203,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":14172,"parent_record_offset":14108,"ancestor_record_offset":13964,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":13896,"parent_record_offset":13872,"ancestor_record_offset":13740,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":16752,"parent_record_offset":16720,"ancestor_record_offset":16580,"inlinee":21773,"source_line":943,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":14140,"parent_record_offset":14108,"ancestor_record_offset":13964,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":14956,"parent_record_offset":14828,"ancestor_record_offset":14680,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":13816,"parent_record_offset":13764,"ancestor_record_offset":13740,"inlinee":26062,"source_line":61,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":13872,"parent_record_offset":13848,"ancestor_record_offset":13740,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":14888,"parent_record_offset":14856,"ancestor_record_offset":14680,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":14856,"parent_record_offset":14828,"ancestor_record_offset":14680,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":13788,"parent_record_offset":13764,"ancestor_record_offset":13740,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":740,"record_offset":14920,"parent_record_offset":14888,"ancestor_record_offset":14680,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::contains_point
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 408
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2D1
// 首条位置: {"module":740,"inlinee":45777}
// 编译实例 RVAs: 0x8d26a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::contains_point_padding
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":1664,"inlinee":45778,"source_line":413,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":2020,"parent_record_offset":1936,"ancestor_record_offset":1664,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_cmple_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":2080,"parent_record_offset":2056,"ancestor_record_offset":1664,"inlinee":45781,"source_line":505,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":1964,"parent_record_offset":1936,"ancestor_record_offset":1664,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":1992,"parent_record_offset":1936,"ancestor_record_offset":1664,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":1880,"parent_record_offset":1856,"ancestor_record_offset":1664,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::bool::sse2::bvec4a::BVec4A::all
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/bool/sse2/bvec4a.rs
//     evidence={"module":740,"record_offset":2112,"parent_record_offset":1688,"ancestor_record_offset":1664,"inlinee":45782,"source_line":85,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":1792,"parent_record_offset":1728,"ancestor_record_offset":1664,"inlinee":26062,"source_line":61,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":1664,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":1912,"parent_record_offset":1688,"ancestor_record_offset":1664,"inlinee":21770,"source_line":427,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::cmple
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":740,"record_offset":2056,"parent_record_offset":1688,"ancestor_record_offset":1664,"inlinee":45780,"source_line":354,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":740,"record_offset":1824,"parent_record_offset":1688,"ancestor_record_offset":1664,"inlinee":14703,"source_line":92,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":1760,"parent_record_offset":1728,"ancestor_record_offset":1664,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":1856,"parent_record_offset":1688,"ancestor_record_offset":1664,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":1728,"parent_record_offset":1688,"ancestor_record_offset":1664,"inlinee":45779,"source_line":203,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::contains_point_padding
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 413
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2D2
// 首条位置: {"module":740,"inlinee":45778}
// 编译实例 RVAs: 0x8d2710
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::bool::sse2::bvec4a::BVec4A::all
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/bool/sse2/bvec4a.rs
//     evidence={"module":740,"record_offset":2112,"parent_record_offset":1688,"ancestor_record_offset":1688,"inlinee":45782,"source_line":85,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":1912,"parent_record_offset":1688,"ancestor_record_offset":1688,"inlinee":21770,"source_line":427,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec4::Vec4::cmple
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":740,"record_offset":2056,"parent_record_offset":1688,"ancestor_record_offset":1688,"inlinee":45780,"source_line":354,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":740,"record_offset":1824,"parent_record_offset":1688,"ancestor_record_offset":1688,"inlinee":14703,"source_line":92,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":1856,"parent_record_offset":1688,"ancestor_record_offset":1688,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":1728,"parent_record_offset":1688,"ancestor_record_offset":1688,"inlinee":45779,"source_line":203,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":2020,"parent_record_offset":1936,"ancestor_record_offset":1688,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_cmple_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":2080,"parent_record_offset":2056,"ancestor_record_offset":1688,"inlinee":45781,"source_line":505,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":1964,"parent_record_offset":1936,"ancestor_record_offset":1688,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":1992,"parent_record_offset":1936,"ancestor_record_offset":1688,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":1880,"parent_record_offset":1856,"ancestor_record_offset":1688,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":1792,"parent_record_offset":1728,"ancestor_record_offset":1688,"inlinee":26062,"source_line":61,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":1688,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":1760,"parent_record_offset":1728,"ancestor_record_offset":1688,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::with_expanded_size
// 记录形式: compiled_procedure; instances=1
// 源码行: 425
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":20144,"rva":9250720,"va":5377959840}
// 编译实例 RVAs: 0x8d27a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::distance_to_closest_edge
// 记录形式: compiled_procedure; instances=1
// 源码行: 435
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":20280,"rva":9250752,"va":5377959872}
// 编译实例 RVAs: 0x8d27c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144b80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25633549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377959872,"raw_hex":"e80d238701","size":5,"soft_fail":false,"target_va":5403593600,"va":5377960046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":21000,"parent_record_offset":20280,"ancestor_record_offset":20280,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::signed_distance_to_closest_edge
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":20436,"parent_record_offset":20280,"ancestor_record_offset":20280,"inlinee":45795,"source_line":439,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":20908,"parent_record_offset":20824,"ancestor_record_offset":20280,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":20680,"parent_record_offset":20564,"ancestor_record_offset":20280,"inlinee":21870,"source_line":100,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":20624,"parent_record_offset":20564,"ancestor_record_offset":20280,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":20596,"parent_record_offset":20564,"ancestor_record_offset":20280,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":20940,"parent_record_offset":20436,"ancestor_record_offset":20280,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":20792,"parent_record_offset":20768,"ancestor_record_offset":20280,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":20532,"parent_record_offset":20480,"ancestor_record_offset":20280,"inlinee":26062,"source_line":61,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::inverse
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":20564,"parent_record_offset":20436,"ancestor_record_offset":20280,"inlinee":45796,"source_line":255,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":20824,"parent_record_offset":20436,"ancestor_record_offset":20280,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":20504,"parent_record_offset":20480,"ancestor_record_offset":20280,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":20768,"parent_record_offset":20436,"ancestor_record_offset":20280,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::hypot
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":740,"record_offset":20968,"parent_record_offset":20436,"ancestor_record_offset":20280,"inlinee":27085,"source_line":690,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":20480,"parent_record_offset":20436,"ancestor_record_offset":20280,"inlinee":45779,"source_line":203,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::signed_distance_to_closest_edge
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 439
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2E3
// 首条位置: {"module":740,"inlinee":45795}
// 编译实例 RVAs: 0x8d2890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x142144b80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t25633327","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377960080,"raw_hex":"e92f228701","size":5,"soft_fail":false,"target_va":5403593600,"va":5377960268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":20940,"parent_record_offset":20436,"ancestor_record_offset":20436,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::inverse
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":20564,"parent_record_offset":20436,"ancestor_record_offset":20436,"inlinee":45796,"source_line":255,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":20824,"parent_record_offset":20436,"ancestor_record_offset":20436,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":20768,"parent_record_offset":20436,"ancestor_record_offset":20436,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::hypot
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":740,"record_offset":20968,"parent_record_offset":20436,"ancestor_record_offset":20436,"inlinee":27085,"source_line":690,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":20480,"parent_record_offset":20436,"ancestor_record_offset":20436,"inlinee":45779,"source_line":203,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":20908,"parent_record_offset":20824,"ancestor_record_offset":20436,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":20680,"parent_record_offset":20564,"ancestor_record_offset":20436,"inlinee":21870,"source_line":100,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":20624,"parent_record_offset":20564,"ancestor_record_offset":20436,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":20596,"parent_record_offset":20564,"ancestor_record_offset":20436,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":20792,"parent_record_offset":20768,"ancestor_record_offset":20436,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":20532,"parent_record_offset":20480,"ancestor_record_offset":20436,"inlinee":26062,"source_line":61,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":20504,"parent_record_offset":20480,"ancestor_record_offset":20436,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::closest_edge_normal
// 记录形式: compiled_procedure; instances=1
// 源码行: 453
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":21728,"rva":9251168,"va":5377960288}
// 编译实例 RVAs: 0x8d2960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::inverse
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":21972,"parent_record_offset":21728,"ancestor_record_offset":21728,"inlinee":45796,"source_line":255,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":22232,"parent_record_offset":21728,"ancestor_record_offset":21728,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":22176,"parent_record_offset":21728,"ancestor_record_offset":21728,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":740,"record_offset":22352,"parent_record_offset":21728,"ancestor_record_offset":21728,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":21888,"parent_record_offset":21728,"ancestor_record_offset":21728,"inlinee":45779,"source_line":203,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":22320,"parent_record_offset":22232,"ancestor_record_offset":21728,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":22088,"parent_record_offset":21972,"ancestor_record_offset":21728,"inlinee":21870,"source_line":100,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":22032,"parent_record_offset":21972,"ancestor_record_offset":21728,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":22004,"parent_record_offset":21972,"ancestor_record_offset":21728,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":22200,"parent_record_offset":22176,"ancestor_record_offset":21728,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":21940,"parent_record_offset":21888,"ancestor_record_offset":21728,"inlinee":26062,"source_line":61,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":21912,"parent_record_offset":21888,"ancestor_record_offset":21728,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices
// 记录形式: compiled_procedure; instances=1
// 源码行: 472
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":22504,"rva":9251392,"va":5377960512}
// 编译实例 RVAs: 0x8d2a40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25018028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377960512,"raw_hex":"e8acbe7d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377961119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":740,"record_offset":22944,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::distance_segment_point
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":740,"record_offset":23384,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":22272,"source_line":166,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":22664,"parent_record_offset":22504,"ancestor_record_offset":22504,"inlinee":45791,"source_line":223,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":23048,"parent_record_offset":23008,"ancestor_record_offset":22504,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":22848,"parent_record_offset":22764,"ancestor_record_offset":22504,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":22792,"parent_record_offset":22764,"ancestor_record_offset":22504,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":22820,"parent_record_offset":22764,"ancestor_record_offset":22504,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":23736,"parent_record_offset":23416,"ancestor_record_offset":22504,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":23704,"parent_record_offset":23448,"ancestor_record_offset":22504,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":740,"record_offset":23008,"parent_record_offset":22976,"ancestor_record_offset":22504,"inlinee":7624,"source_line":1153,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":740,"record_offset":22976,"parent_record_offset":22944,"ancestor_record_offset":22504,"inlinee":7622,"source_line":1248,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":23856,"parent_record_offset":23824,"ancestor_record_offset":22504,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":22908,"parent_record_offset":22884,"ancestor_record_offset":22504,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":23504,"parent_record_offset":23480,"ancestor_record_offset":22504,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":23080,"parent_record_offset":23008,"ancestor_record_offset":22504,"inlinee":7625,"source_line":545,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":22764,"parent_record_offset":22740,"ancestor_record_offset":22504,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":22740,"parent_record_offset":22664,"ancestor_record_offset":22504,"inlinee":21770,"source_line":427,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":23616,"parent_record_offset":23592,"ancestor_record_offset":22504,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":24008,"parent_record_offset":23984,"ancestor_record_offset":22504,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":23796,"parent_record_offset":23768,"ancestor_record_offset":22504,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":22884,"parent_record_offset":22664,"ancestor_record_offset":22504,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":23480,"parent_record_offset":23448,"ancestor_record_offset":22504,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":23824,"parent_record_offset":23796,"ancestor_record_offset":22504,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":740,"record_offset":24064,"parent_record_offset":24032,"ancestor_record_offset":22504,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":22700,"parent_record_offset":22664,"ancestor_record_offset":22504,"inlinee":45785,"source_line":126,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":740,"record_offset":23592,"parent_record_offset":23448,"ancestor_record_offset":22504,"inlinee":22388,"source_line":80,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":740,"record_offset":23984,"parent_record_offset":23384,"ancestor_record_offset":22504,"inlinee":23097,"source_line":55,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":740,"record_offset":23768,"parent_record_offset":23416,"ancestor_record_offset":22504,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::project_point_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":740,"record_offset":23416,"parent_record_offset":23384,"ancestor_record_offset":22504,"inlinee":21612,"source_line":181,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":740,"record_offset":23448,"parent_record_offset":23416,"ancestor_record_offset":22504,"inlinee":21613,"source_line":149,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::get_closest_edge_id
// 记录形式: compiled_procedure; instances=1
// 源码行: 491
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":24112,"rva":9252016,"va":5377961136}
// 编译实例 RVAs: 0x8d2cb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t-629","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377961136,"raw_hex":"e98bfdffff","size":5,"soft_fail":false,"target_va":5377960512,"va":5377961136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::closest_point_on_perimeter
// 记录形式: compiled_procedure; instances=1
// 源码行: 510
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":25412,"rva":9252384,"va":5377961504}
// 编译实例 RVAs: 0x8d2e20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25017181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377961504,"raw_hex":"e85dbb7d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377961966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377961504,"raw_hex":"e8fdfbffff","size":5,"soft_fail":false,"target_va":5377960512,"va":5377961534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":26900,"parent_record_offset":25412,"ancestor_record_offset":25412,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":27228,"parent_record_offset":25412,"ancestor_record_offset":25412,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":25576,"parent_record_offset":25412,"ancestor_record_offset":25412,"inlinee":45791,"source_line":223,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":740,"record_offset":26604,"parent_record_offset":25412,"ancestor_record_offset":25412,"inlinee":21613,"source_line":149,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":25916,"parent_record_offset":25832,"ancestor_record_offset":25412,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":25860,"parent_record_offset":25832,"ancestor_record_offset":25412,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":25888,"parent_record_offset":25832,"ancestor_record_offset":25412,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":26868,"parent_record_offset":26604,"ancestor_record_offset":25412,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":27348,"parent_record_offset":27252,"ancestor_record_offset":25412,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":25976,"parent_record_offset":25952,"ancestor_record_offset":25412,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":26660,"parent_record_offset":26636,"ancestor_record_offset":25412,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":25776,"parent_record_offset":25724,"ancestor_record_offset":25412,"inlinee":26062,"source_line":61,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":25832,"parent_record_offset":25808,"ancestor_record_offset":25412,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":25808,"parent_record_offset":25576,"ancestor_record_offset":25412,"inlinee":21770,"source_line":427,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":26776,"parent_record_offset":26748,"ancestor_record_offset":25412,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":25748,"parent_record_offset":25724,"ancestor_record_offset":25412,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":25952,"parent_record_offset":25576,"ancestor_record_offset":25412,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":26636,"parent_record_offset":26604,"ancestor_record_offset":25412,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":25692,"parent_record_offset":25640,"ancestor_record_offset":25412,"inlinee":45784,"source_line":195,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":25640,"parent_record_offset":25576,"ancestor_record_offset":25412,"inlinee":45785,"source_line":126,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":25724,"parent_record_offset":25576,"ancestor_record_offset":25412,"inlinee":45779,"source_line":203,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":740,"record_offset":26748,"parent_record_offset":26604,"ancestor_record_offset":25412,"inlinee":22388,"source_line":80,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::local_aabb_of
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 536
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2E6
// 首条位置: {"module":740,"inlinee":45798}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":29604,"parent_record_offset":29460,"ancestor_record_offset":29460,"inlinee":21770,"source_line":427,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":29780,"parent_record_offset":29460,"ancestor_record_offset":29460,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::aabb
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":29992,"parent_record_offset":29460,"ancestor_record_offset":29460,"inlinee":45799,"source_line":597,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":29516,"parent_record_offset":29460,"ancestor_record_offset":29460,"inlinee":45779,"source_line":203,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":29744,"parent_record_offset":29628,"ancestor_record_offset":29460,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":29688,"parent_record_offset":29628,"ancestor_record_offset":29460,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":29660,"parent_record_offset":29628,"ancestor_record_offset":29460,"inlinee":21773,"source_line":943,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":740,"record_offset":29716,"parent_record_offset":29628,"ancestor_record_offset":29460,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":31316,"parent_record_offset":31292,"ancestor_record_offset":30604,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":30096,"parent_record_offset":30068,"ancestor_record_offset":29460,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":29804,"parent_record_offset":29780,"ancestor_record_offset":29460,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":29572,"parent_record_offset":29516,"ancestor_record_offset":29460,"inlinee":26062,"source_line":61,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":29628,"parent_record_offset":29604,"ancestor_record_offset":29460,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":29544,"parent_record_offset":29516,"ancestor_record_offset":29460,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":31292,"parent_record_offset":31128,"ancestor_record_offset":30604,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":30164,"parent_record_offset":29992,"ancestor_record_offset":29460,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":30068,"parent_record_offset":30040,"ancestor_record_offset":29460,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":740,"record_offset":30136,"parent_record_offset":29992,"ancestor_record_offset":29460,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::half_dimensions
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":30040,"parent_record_offset":29992,"ancestor_record_offset":29460,"inlinee":45800,"source_line":592,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::half_dimensions
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 592
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2E8
// 首条位置: {"module":740,"inlinee":45800}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":30068,"parent_record_offset":30040,"ancestor_record_offset":30040,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":30096,"parent_record_offset":30068,"ancestor_record_offset":30040,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::aabb
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 597
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2E7
// 首条位置: {"module":740,"inlinee":45799}
// 编译实例 RVAs: 0x8d36c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":31292,"parent_record_offset":31128,"ancestor_record_offset":31128,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":30164,"parent_record_offset":29992,"ancestor_record_offset":29992,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":740,"record_offset":30136,"parent_record_offset":29992,"ancestor_record_offset":29992,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::half_dimensions
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":30040,"parent_record_offset":29992,"ancestor_record_offset":29992,"inlinee":45800,"source_line":592,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":31316,"parent_record_offset":31292,"ancestor_record_offset":31128,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":30096,"parent_record_offset":30068,"ancestor_record_offset":29992,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":30196,"parent_record_offset":30164,"ancestor_record_offset":29992,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":30068,"parent_record_offset":30040,"ancestor_record_offset":29992,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::impl$1::intersects::closure$0
// 记录形式: inlinee_declaration; instances=3
// 源码行: 623
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB301
// 首条位置: {"module":740,"inlinee":45825}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::circular_slice
// 记录形式: compiled_procedure; instances=1
// 源码行: 644
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":31764,"rva":9254688,"va":5377963808}
// 编译实例 RVAs: 0x8d3720
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5214205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377963808,"raw_hex":"e80370b0ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5377963896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5216197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377963808,"raw_hex":"e83b68b0ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5377965904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25013229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377963808,"raw_hex":"e8edab7d01","size":5,"soft_fail":false,"target_va":5402978144,"va":5377964910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t-6655781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377963808,"raw_hex":"e8db709aff","size":5,"soft_fail":false,"target_va":5371308432,"va":5377964208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t25014122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377963808,"raw_hex":"e86aaf7d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377965025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t25013947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377963808,"raw_hex":"e8bbae7d01","size":5,"soft_fail":false,"target_va":5402978944,"va":5377964992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":740,"record_offset":31928,"parent_record_offset":31764,"ancestor_record_offset":31764,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":740,"record_offset":32348,"parent_record_offset":31764,"ancestor_record_offset":31764,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":740,"record_offset":33208,"parent_record_offset":31764,"ancestor_record_offset":31764,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":740,"record_offset":32076,"parent_record_offset":31764,"ancestor_record_offset":31764,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":740,"record_offset":35200,"parent_record_offset":31764,"ancestor_record_offset":31764,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":740,"record_offset":33808,"parent_record_offset":31764,"ancestor_record_offset":31764,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":32432,"parent_record_offset":31764,"ancestor_record_offset":31764,"inlinee":45790,"source_line":241,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":740,"record_offset":31984,"parent_record_offset":31960,"ancestor_record_offset":31764,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":740,"record_offset":32008,"parent_record_offset":31984,"ancestor_record_offset":31764,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":740,"record_offset":35272,"parent_record_offset":35248,"ancestor_record_offset":31764,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":740,"record_offset":31960,"parent_record_offset":31928,"ancestor_record_offset":31764,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":740,"record_offset":35248,"parent_record_offset":35224,"ancestor_record_offset":31764,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":740,"record_offset":35224,"parent_record_offset":35200,"ancestor_record_offset":31764,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":740,"record_offset":35396,"parent_record_offset":35364,"ancestor_record_offset":31764,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":740,"record_offset":33264,"parent_record_offset":33240,"ancestor_record_offset":31764,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":740,"record_offset":35364,"parent_record_offset":35336,"ancestor_record_offset":31764,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":740,"record_offset":32372,"parent_record_offset":32348,"ancestor_record_offset":31764,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":740,"record_offset":33240,"parent_record_offset":33208,"ancestor_record_offset":31764,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":740,"record_offset":32396,"parent_record_offset":32372,"ancestor_record_offset":31764,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":32220,"parent_record_offset":32196,"ancestor_record_offset":31764,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":740,"record_offset":32244,"parent_record_offset":32220,"ancestor_record_offset":31764,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":32280,"parent_record_offset":32132,"ancestor_record_offset":31764,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":740,"record_offset":32308,"parent_record_offset":32132,"ancestor_record_offset":31764,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":740,"record_offset":35428,"parent_record_offset":35396,"ancestor_record_offset":31764,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":32196,"parent_record_offset":32172,"ancestor_record_offset":31764,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":740,"record_offset":32172,"parent_record_offset":32132,"ancestor_record_offset":31764,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":740,"record_offset":35336,"parent_record_offset":35312,"ancestor_record_offset":31764,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":740,"record_offset":32032,"parent_record_offset":32008,"ancestor_record_offset":31764,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":740,"record_offset":33296,"parent_record_offset":33208,"ancestor_record_offset":31764,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":740,"record_offset":32132,"parent_record_offset":32104,"ancestor_record_offset":31764,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":740,"record_offset":32104,"parent_record_offset":32076,"ancestor_record_offset":31764,"inlinee":26825,"source_line":115,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":32464,"parent_record_offset":32432,"ancestor_record_offset":31764,"inlinee":45789,"source_line":199,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::lerp
// 记录形式: compiled_procedure; instances=1
// 源码行: 720
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":36364,"rva":9257392,"va":5377966512}
// 编译实例 RVAs: 0x8d41b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25627003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377966512,"raw_hex":"e87b098701","size":5,"soft_fail":false,"target_va":5403594000,"va":5377966992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25627351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377966512,"raw_hex":"e8d70a8701","size":5,"soft_fail":false,"target_va":5403594336,"va":5377966980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t25011878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377966512,"raw_hex":"e8a6a67d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377967269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::from_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":740,"record_offset":37176,"parent_record_offset":36364,"ancestor_record_offset":36364,"inlinee":23090,"source_line":124,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::angle_between
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":36512,"parent_record_offset":36364,"ancestor_record_offset":36364,"inlinee":23080,"source_line":803,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":37696,"parent_record_offset":36364,"ancestor_record_offset":36364,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":740,"record_offset":37036,"parent_record_offset":36364,"ancestor_record_offset":36364,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":37404,"parent_record_offset":36364,"ancestor_record_offset":36364,"inlinee":45773,"source_line":113,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":37480,"parent_record_offset":37456,"ancestor_record_offset":36364,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":37456,"parent_record_offset":37404,"ancestor_record_offset":36364,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":37088,"parent_record_offset":37036,"ancestor_record_offset":36364,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":37144,"parent_record_offset":37036,"ancestor_record_offset":36364,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":37060,"parent_record_offset":37036,"ancestor_record_offset":36364,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::acos_approx_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":740,"record_offset":36820,"parent_record_offset":36792,"ancestor_record_offset":36364,"inlinee":23083,"source_line":3,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::abs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":740,"record_offset":36904,"parent_record_offset":36820,"ancestor_record_offset":36364,"inlinee":21561,"source_line":151,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::acos_approx
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":740,"record_offset":36792,"parent_record_offset":36512,"ancestor_record_offset":36364,"inlinee":23081,"source_line":156,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":740,"record_offset":37200,"parent_record_offset":37176,"ancestor_record_offset":36364,"inlinee":21674,"source_line":177,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":740,"record_offset":36968,"parent_record_offset":36820,"ancestor_record_offset":36364,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":36704,"parent_record_offset":36652,"ancestor_record_offset":36364,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::is_normalized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":37568,"parent_record_offset":37404,"ancestor_record_offset":36364,"inlinee":45774,"source_line":509,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":36652,"parent_record_offset":36512,"ancestor_record_offset":36364,"inlinee":21909,"source_line":390,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp_dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":36568,"parent_record_offset":36512,"ancestor_record_offset":36364,"inlinee":45826,"source_line":828,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":37720,"parent_record_offset":37696,"ancestor_record_offset":36364,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":740,"record_offset":36932,"parent_record_offset":36904,"ancestor_record_offset":36364,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::signum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":740,"record_offset":36616,"parent_record_offset":36568,"ancestor_record_offset":36364,"inlinee":21926,"source_line":222,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":740,"record_offset":37248,"parent_record_offset":37224,"ancestor_record_offset":36364,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":740,"record_offset":37224,"parent_record_offset":37200,"ancestor_record_offset":36364,"inlinee":21675,"source_line":915,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":740,"record_offset":36760,"parent_record_offset":36652,"ancestor_record_offset":36364,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::Rectangle2d::edge_length
// 记录形式: compiled_procedure; instances=1
// 源码行: 732
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":37848,"rva":9258208,"va":5377967328}
// 编译实例 RVAs: 0x8d44e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t25011529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377967328,"raw_hex":"e849a57d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377967618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":38092,"parent_record_offset":37848,"ancestor_record_offset":37848,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::get_edge_pts
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":37988,"parent_record_offset":37848,"ancestor_record_offset":37848,"inlinee":45827,"source_line":363,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":38144,"parent_record_offset":38116,"ancestor_record_offset":37848,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":38172,"parent_record_offset":38116,"ancestor_record_offset":37848,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":38116,"parent_record_offset":38092,"ancestor_record_offset":37848,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":740,"record_offset":38204,"parent_record_offset":38172,"ancestor_record_offset":37848,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::area
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":38056,"parent_record_offset":38028,"ancestor_record_offset":37848,"inlinee":45784,"source_line":195,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":38028,"parent_record_offset":37988,"ancestor_record_offset":37848,"inlinee":45786,"source_line":155,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::rectangle::impl$2::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 745
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":740,"record_offset":38276,"rva":9258528,"va":5377967648}
// 编译实例 RVAs: 0x8d4620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t25011386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377967648,"raw_hex":"e8baa47d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5377967761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::geometry::aabb::Aabb2::center
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":740,"record_offset":38624,"parent_record_offset":38276,"ancestor_record_offset":38276,"inlinee":45830,"source_line":100,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::aabb::Aabb2::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":740,"record_offset":38408,"parent_record_offset":38276,"ancestor_record_offset":38276,"inlinee":45828,"source_line":173,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle2d::new_axis_aligned
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":38432,"parent_record_offset":38276,"ancestor_record_offset":38276,"inlinee":45829,"source_line":107,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":38500,"parent_record_offset":38476,"ancestor_record_offset":38276,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":740,"record_offset":38476,"parent_record_offset":38432,"ancestor_record_offset":38276,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":38728,"parent_record_offset":38704,"ancestor_record_offset":38276,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":740,"record_offset":38672,"parent_record_offset":38648,"ancestor_record_offset":38276,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":38648,"parent_record_offset":38624,"ancestor_record_offset":38276,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":38704,"parent_record_offset":38624,"ancestor_record_offset":38276,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle2d::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":740,"record_offset":38588,"parent_record_offset":38432,"ancestor_record_offset":38276,"inlinee":45773,"source_line":113,"archive":"_pdb/A/modules/00740.jsonl.gz"}
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
// 完整结构及原始类型记录: rectangle.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::rectangle::Rectangle2d
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 47; 过程引用数: 44
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$1::serialize_newtype_variant<utils::geometry::rectangle::Rectangle2d>", "constructor": "serde_json::value::ser::impl$1::serialize_newtype_variant", "arguments": ["utils::geometry::rectangle::Rectangle2d"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<utils::geometry::rectangle::Rectangle2d>", "constructor": "core::fmt::impl$51::fmt", "arguments": ["utils::geometry::rectangle::Rectangle2d"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::rectangle::Rectangle3d
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 13; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<utils::geometry::rectangle::Rectangle3d,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["utils::geometry::rectangle::Rectangle3d", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::collision::world::RaycastWorld::shape_overlaps_with<utils::geometry::rectangle::Rectangle3d>", "constructor": "country_core::systems::collision::world::RaycastWorld::shape_overlaps_with", "arguments": ["utils::geometry::rectangle::Rectangle3d"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: rectangle.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: rectangle.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 33; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

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
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::system::system_param::Local<country_core::systems::ui::debug_visualize_volumes::DebugRoofs>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >", "bevy_ecs::system::commands::Commands", "bevy_ecs::system::system_param::Local<country_core::systems::ui::debug_visualize_volumes::DebugRoofs>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::ui::debug_visualize_volumes::DebugRoofs>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::EventReader<country_core::resources::window::WindowFocusEvent>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::event::EventReader<country_core::resources::window::WindowFocusEvent>", "bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>", "bevy_ecs::event::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>", "bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::window::WindowFocusEvent>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::_get_normal_at_u
// 信息 ID: 7bf6814ca0696be7e2dde6a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::get_normal_at_u @ 0x8d1f38 -> 0x8d1f50 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::get_normal_at_u_non_normalized_corners @ 0x8d1f48 -> 0x8d1f50 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::aabb
// 信息 ID: a885cb5a8d1cbc03d41f5ce3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 13 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x4d4969 -> 0x8d36c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate @ 0x550820 -> 0x8d36c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::ClutterState::add_region @ 0x9bdc51 -> 0x8d36c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::ClutterState::add_region @ 0x9bdd2e -> 0x8d36c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::ClutterState::add_region @ 0x9bdee0 -> 0x8d36c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::ClutterState::remove_region_from_lazy_grid @ 0x9be0f3 -> 0x8d36c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_rectangle2d<country_core::systems::wall::wall_state::wall_spatial_hash::impl$2::query_with_rectangle2d::closure_env$0> @ 0x1044d61 -> 0x8d36c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_rectangle2d<country_core::systems::wall::wall_state::wall_spatial_hash::impl$2::query_with_rectangle2d::closure_env$0> @ 0x104513a -> 0x8d36c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$2::query_with_segment::closure_env$0> @ 0x1045b62 -> 0x8d36c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$2::add_segment::closure_env$0> @ 0x10467cd -> 0x8d36c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::ui_edit_rectangle_dims @ 0x112ecbe -> 0x8d36c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad96f0 -> 0x8d36c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::dormer_holes::dormer_holes @ 0x1c4c95c -> 0x8d36c0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=740, inline_record=29992; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=740, inline_record=31128; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal
// 信息 ID: 0c786618291564bdda7b6818
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 13 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=740, inline_record=13740; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=740, inline_record=13964; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=740, inline_record=14268; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=740, inline_record=14436; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=740, inline_record=14680; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=740, inline_record=15020; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=740, inline_record=15348; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=740, inline_record=15716; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::area
// 信息 ID: 1533df608cfaf32922557aec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 16 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio @ 0x617112 -> 0x8d1650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio @ 0x6174f7 -> 0x8d1650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::get_building_size_01 @ 0x618180 -> 0x8d1650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_create_rectangle::ui_create_rectangle @ 0xf04a24 -> 0x8d1650 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned; module=740, inline_record=3492; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_points2; module=740, inline_record=3672; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_points3; module=740, inline_record=4184; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_curve2; module=740, inline_record=4368; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_curve3; module=740, inline_record=5640; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_point; module=740, inline_record=7408; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=740, inline_record=9120; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=740, inline_record=9276; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::as_curve2
// 信息 ID: 0fb9c1d31ee6f18a8bd54fdb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 15 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::wall_outline::wall_outline @ 0x570a7a -> 0x8d1110 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::edit_rectangle::edit_rectangle @ 0x61b7fe -> 0x8d1110 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2 @ 0x6b0947 -> 0x8d1110 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::calc_shape_height::calc_shape_height @ 0x6bcddd -> 0x8d1110 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dbfc9 -> 0x8d1110 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dbff0 -> 0x8d1110 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0xc92e71 -> 0x8d1110 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0xc9391d -> 0x8d1110 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0xc93e6d -> 0x8d1110 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::rectangle_signifier::closure$0::closure_env$0,tuple$<> > | country_core::systems::wall::shapes::rectangle::ui_create_rectangle::rectangle_signifier::closure$0::closure$0 @ 0xf02bd4 -> 0x8d1110 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::create_rectangle::create_rectangle @ 0x112d626 -> 0x8d1110 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x12eaa5d -> 0x8d1110 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x12eaa7e -> 0x8d1110 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x1984d88 -> 0x8d1110 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x1984d98 -> 0x8d1110 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::as_curve3
// 信息 ID: f4541c69a2c52a9b40264880
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve3 @ 0x6b098d -> 0x8d1360 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers @ 0xd227cd -> 0x8d1360 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::debug_viz::DebugRectangle3d::show @ 0xec42ab -> 0x8d1360 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::debug_viz::DebugRectangle2d::show @ 0xec591f -> 0x8d1360 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x12e9b49 -> 0x8d1360 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x12e9f31 -> 0x8d1360 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x12ea62f -> 0x8d1360 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_full_glade_size::demo_show_full_glade_size @ 0x165c3ee -> 0x8d1360 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::construct_inter_roof_merlons::construct_inter_roof_merlons @ 0x1acd355 -> 0x8d1360 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::as_points2
// 信息 ID: b08759d946c2d86591e81371
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate @ 0x5508b2 -> 0x8d0ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dc0a0 -> 0x8d0ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dc0c0 -> 0x8d0ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dc155 -> 0x8d0ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x9b7802 -> 0x8d0ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$3> @ 0x9b8911 -> 0x8d0ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::find_stone_column_start_y @ 0xca3b36 -> 0x8d0ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0xccd13f -> 0x8d0ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0xccd15f -> 0x8d0ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0xccd1ec -> 0x8d0ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::is_shape_inside @ 0xefcfd4 -> 0x8d0ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::is_shape_inside @ 0xefd082 -> 0x8d0ed0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_curve2; module=740, inline_record=4396; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id; module=740, inline_record=17292; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_edge_pts; module=740, inline_record=18352; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::edge_length; module=740, inline_record=38028; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
// 信息 ID: 93378c0941c18b2578e7b444
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 17 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x9b726f -> 0x8d0e70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$3> @ 0x9b7e6a -> 0x8d0e70 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_points2; module=740, inline_record=3780; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_curve2; module=740, inline_record=4472; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_point; module=740, inline_record=7372; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=740, inline_record=9088; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=740, inline_record=9244; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=740, inline_record=9712; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=740, inline_record=9896; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=740, inline_record=11384; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::as_points3
// 信息 ID: 25abf8d1ce81e51655eab83e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::as_transformation_and_size
// 信息 ID: 23c0eabbfb1083f5eabe4f43
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x9b7168 -> 0x8d16f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x9b71a5 -> 0x8d16f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$3> @ 0x9b7d63 -> 0x8d16f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$3> @ 0x9b7d99 -> 0x8d16f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::circular_slice
// 信息 ID: d0e337548eeccc1faf03133c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4f0ad5 -> 0x8d3720 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::closest_edge_normal
// 信息 ID: 155590c4f430e99cb75c2d1f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x4fd3a0 -> 0x8d2960 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::closest_point_on_perimeter
// 信息 ID: b4dc53de8e4625624889340d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds @ 0x502770 -> 0x8d2e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] forest::manage_tree_prefabs::manage_tree_prefabs @ 0x143ff87 -> 0x8d2e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::add_birdnests::add_birdnests @ 0x1ad2d06 -> 0x8d2e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6> @ 0x1c41074 -> 0x8d2e20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::contains_point
// 信息 ID: 553cca28c85aa2e1184c2aca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 25 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::ui_draw_wall::ui_draw_wall @ 0x545b45 -> 0x8d26a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::ui_draw_wall::ui_draw_wall @ 0x5461e8 -> 0x8d26a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::ui_draw_wall::ui_draw_wall @ 0x545ec6 -> 0x8d26a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_pruner::ivy_pruner @ 0x5d10ec -> 0x8d26a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_pruner::ivy_pruner @ 0x5d1148 -> 0x8d26a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0a04 -> 0x8d26a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0a27 -> 0x8d26a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0a4a -> 0x8d26a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0a73 -> 0x8d26a0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0b1b -> 0x8d26a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0b41 -> 0x8d26a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0b67 -> 0x8d26a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0b8d -> 0x8d26a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0x6b0bb3 -> 0x8d26a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::ui_continue_wall::ui_continue_wall @ 0x6b7e55 -> 0x8d26a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0xc93c02 -> 0x8d26a0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::contains_point; module=740, inline_record=1664; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::contains_point_padding
// 信息 ID: fea67e894e2dd0d73f506aa7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door @ 0xd0ba44 -> 0x8d2710 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door @ 0xd0ba69 -> 0x8d2710 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad8f21 -> 0x8d2710 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::trap_doors::generate_trap_doors @ 0x1c5e78e -> 0x8d2710 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::contains_point; module=740, inline_record=1688; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::contains_point; module=740, inline_record=19092; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::distance_to_closest_edge
// 信息 ID: 39d0f951220563b7c2826d98
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0xc9ef6d -> 0x8d27c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::edge_length
// 信息 ID: 3c9d05dfae1ec8e19195941c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position @ 0x17d583d -> 0x8d44e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<system_decorator::collision_detection::DecoratorIntersection> >,system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure_env$0> @ 0x17d9be6 -> 0x8d44e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id @ 0x1e670d5 -> 0x8d44e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$1 @ 0x1e68d88 -> 0x8d44e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::expand_along_edge
// 信息 ID: e0b7f04cf6f4f0b9bfc49cca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_closest_edge_id
// 信息 ID: 2a77044266b47cf4097465c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x501dd2 -> 0x8d2cb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position @ 0x17d5832 -> 0x8d2cb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::derive_decorator_subtype::is_half_dormer @ 0x17d79d7 -> 0x8d2cb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<system_decorator::collision_detection::DecoratorIntersection> >,system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure_env$0> @ 0x17d9bdb -> 0x8d2cb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id @ 0x1e670ca -> 0x8d2cb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$1 @ 0x1e68d7d -> 0x8d2cb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices
// 信息 ID: 42f0faa50173f8613b08e79a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::get_closest_edge_id @ 0x8d2cb0 -> 0x8d2a40 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::closest_point_on_perimeter @ 0x8d2e3e -> 0x8d2a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos @ 0x8d304b -> 0x8d2a40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos
// 信息 ID: 43a09082868ac8f1d8d32df1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 14 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4f089d -> 0x8d3000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4f08b6 -> 0x8d3000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x502f45 -> 0x8d3000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x503074 -> 0x8d3000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x9b75d0 -> 0x8d3000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x9b75e7 -> 0x8d3000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x9b7719 -> 0x8d3000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x9b7730 -> 0x8d3000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$3> @ 0x9b82ec -> 0x8d3000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$3> @ 0x9b82ff -> 0x8d3000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$3> @ 0x9b868f -> 0x8d3000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$3> @ 0x9b86a2 -> 0x8d3000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds @ 0xcf6a6f -> 0x8d3000 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds @ 0xcf6c6c -> 0x8d3000 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_corner_normal
// 信息 ID: ce2a01d1fb565ed277061dbc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 10 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::walls::public_wall_state::PublicWallState::corner_pos_and_normal_xz @ 0xc9c8e3 -> 0x8d2260 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars @ 0xca326a -> 0x8d2260 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars @ 0xca3516 -> 0x8d2260 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0xca47ac -> 0x8d2260 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0xca75f5 -> 0x8d2260 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd3949 -> 0x8d2260 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::widget @ 0x10214f4 -> 0x8d2260 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::signifier @ 0x10219f8 -> 0x8d2260 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::signifier @ 0x1021d40 -> 0x8d2260 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x1c542b9 -> 0x8d2260 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=740, inline_record=13716; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=740, inline_record=13940; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=740, inline_record=14244; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=740, inline_record=14412; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=740, inline_record=14636; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id
// 信息 ID: 11b941837769038e3e4994ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars @ 0xca34dc -> 0x8d2330 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd3edb -> 0x8d2330 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::widget @ 0x1021512 -> 0x8d2330 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x1c542d5 -> 0x8d2330 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_edge_id_at_u
// 信息 ID: 093ce1b74cb71796dddf77c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd13d1 -> 0x8d2cc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd14db -> 0x8d2cc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_edge_normal
// 信息 ID: e6d913183345eba94eb2ebf8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::UiRectangleDimsEdit::signifier @ 0xce08fd -> 0x8d2620 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::UiRectangleDimsEdit::widget @ 0xce1990 -> 0x8d2620 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::ui_edit_rectangle_dims @ 0x112ef9b -> 0x8d2620 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::ui_edit_rectangle_dims @ 0x112effb -> 0x8d2620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::chimney_assemble_bricks::construct_crossbrick_chimney<system_decorator::decorator_visual::chimney_assemble_bricks::chimney_assemble_bricks::closure_env$2> @ 0x17e4362 -> 0x8d2620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::util_cross_brick_pillar::construct_crossbrick_pillar<ref_mut$<system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base::closure_env$1> > @ 0x1c31e62 -> 0x8d2620 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=740, inline_record=15324; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=740, inline_record=15692; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=740, inline_record=16060; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=740, inline_record=16200; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::expand_along_edge; module=740, inline_record=35756; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_edge_pts
// 信息 ID: 37f14137ba693466f3e4196b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::wren::update_roof_bird_spots @ 0x1b3c9b5 -> 0x8d24f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_roof_bird_spots @ 0x1b3ce4f -> 0x8d24f0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::edge_length; module=740, inline_record=37988; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_normal_at_u
// 信息 ID: e67f7431fa687ae38ea593be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x9b75f6 -> 0x8d1f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x9b7608 -> 0x8d1f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x9b773f -> 0x8d1f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x9b7751 -> 0x8d1f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$3> @ 0x9b830f -> 0x8d1f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$3> @ 0x9b8323 -> 0x8d1f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$3> @ 0x9b86b2 -> 0x8d1f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$3> @ 0x9b86c6 -> 0x8d1f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> > @ 0xce39a3 -> 0x8d1f30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_normal_at_u_non_normalized_corners
// 信息 ID: 166b60ca05be6c3121283590
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4f111a -> 0x8d1f40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_point
// 信息 ID: 71f3e2b867f81529c3713c2d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 111 条；内联宿主 15 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x502f05 -> 0x8d1730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge @ 0x519f52 -> 0x8d1730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge @ 0x519f7d -> 0x8d1730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge @ 0x51a02f -> 0x8d1730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge @ 0x51a05f -> 0x8d1730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_outline::wall_outline @ 0x5712e1 -> 0x8d1730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_outline::wall_outline @ 0x571316 -> 0x8d1730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_outline::wall_outline @ 0x572183 -> 0x8d1730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_outline::wall_outline @ 0x57219d -> 0x8d1730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_outline::wall_outline @ 0x572676 -> 0x8d1730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_outline::wall_outline @ 0x57268d -> 0x8d1730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_outline::wall_outline @ 0x572cb6 -> 0x8d1730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_outline::wall_outline @ 0x572ccd -> 0x8d1730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x58d206 -> 0x8d1730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x58d239 -> 0x8d1730 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x58d26c -> 0x8d1730 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=740, inline_record=9056; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=740, inline_record=9212; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=740, inline_record=9368; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=740, inline_record=9680; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=740, inline_record=9864; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=740, inline_record=11320; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=740, inline_record=11836; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices; module=740, inline_record=22664; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_point_u
// 信息 ID: d4a72e29749d38b08a9c5b7b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 68 条；内联宿主 21 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x4ea87f -> 0x8d1800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x4ea892 -> 0x8d1800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x4ea8ad -> 0x8d1800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x4ea8c0 -> 0x8d1800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x4ee964 -> 0x8d1800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x4ee9af -> 0x8d1800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x4eeb05 -> 0x8d1800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x4eeb49 -> 0x8d1800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x4eec80 -> 0x8d1800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x4eecc4 -> 0x8d1800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x4eedf9 -> 0x8d1800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x4eee38 -> 0x8d1800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x4fb804 -> 0x8d1800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x4fb817 -> 0x8d1800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x4fb826 -> 0x8d1800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x4fb839 -> 0x8d1800 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=740, inline_record=8368; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=740, inline_record=8596; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=740, inline_record=8796; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=740, inline_record=10476; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=740, inline_record=10724; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=740, inline_record=11028; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=740, inline_record=12400; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=740, inline_record=13024; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::get_pos_at_u
// 信息 ID: 1e76d726cd10379870967b93
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::half_dimensions
// 信息 ID: d92270110dfb447caf77f849
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=740, inline_record=30040; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::aabb; module=740, inline_record=31620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::lerp
// 信息 ID: 9f7549f84ef2ff93f5ebef98
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::search_lerped_shape_that_fits_into_glade @ 0xefd392 -> 0x8d41b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::search_lerped_shape_that_fits_into_glade @ 0xefd487 -> 0x8d41b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::local_aabb_of
// 信息 ID: 2ee0868f604c4a0d54155945
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=740, inline_record=29460; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d; module=740, inline_record=30604; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::new
// 信息 ID: ae229651cb494c223fe38a7a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 17 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0x58630c -> 0x8d0d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_shape::Roof::roof_shape_at_intersection_y @ 0x6baa91 -> 0x8d0d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::ClutterState::add_region @ 0x9bdc41 -> 0x8d0d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::ClutterState::add_region @ 0x9bdd1e -> 0x8d0d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::ClutterState::add_region @ 0x9bded0 -> 0x8d0d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::ClutterState::remove_region_from_lazy_grid @ 0x9be0e3 -> 0x8d0d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::find_stone_column_start_y @ 0xca3b29 -> 0x8d0d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::init_glade_border @ 0xefd7ec -> 0x8d0d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::glade_themes::olden::startup_olden @ 0x1043c55 -> 0x8d0d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_rectangle2d<country_core::systems::wall::wall_state::wall_spatial_hash::impl$2::query_with_rectangle2d::closure_env$0> @ 0x1044df9 -> 0x8d0d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$2::query_with_segment::closure_env$0> @ 0x10458b3 -> 0x8d0d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$2::add_segment::closure_env$0> @ 0x1046530 -> 0x8d0d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_full_glade_size::demo_show_full_glade_size @ 0x165c3db -> 0x8d0d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::chimney_assemble_bricks::construct_crossbrick_chimney<system_decorator::decorator_visual::chimney_assemble_bricks::chimney_assemble_bricks::closure_env$2> @ 0x17e41dd -> 0x8d0d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad798d -> 0x8d0d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad983d -> 0x8d0d90 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::new; module=740, inline_record=372; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::new_centered_axis_aligned; module=740, inline_record=2664; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::new_axis_aligned; module=740, inline_record=2948; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::lerp; module=740, inline_record=37404; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$2::from; module=740, inline_record=38588; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::new_axis_aligned
// 信息 ID: bd47a7a581d4fa893f468529
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dbef0 -> 0x8d0d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0xccd02f -> 0x8d0d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::collision_bbx @ 0xf20026 -> 0x8d0d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x1983933 -> 0x8d0d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_roof_bird_spots @ 0x1b3c982 -> 0x8d0d00 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::impl$2::from; module=740, inline_record=38432; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::new_centered_axis_aligned
// 信息 ID: e6d81c1c07b29fa1f14e4e76
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::wall_outline::wall_outline @ 0x5720c0 -> 0x8d0c80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_outline::wall_outline @ 0x5720d7 -> 0x8d0c80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dbeb3 -> 0x8d0c80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0xccd005 -> 0x8d0c80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x19838fb -> 0x8d0c80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::orientation_mat2
// 信息 ID: cb7325610682a8873b1515cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 19 条；内联宿主 28 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x502cb7 -> 0x8d1670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x502cd8 -> 0x8d1670 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space @ 0x59b87f -> 0x8d1670 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space @ 0x59b938 -> 0x8d1670 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space @ 0x59babe -> 0x8d1670 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space @ 0x59bb67 -> 0x8d1670 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space @ 0x59b9da -> 0x8d1670 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_shape::Roof::roof_tip_offset_xz @ 0x6ba87d -> 0x8d1670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dbf03 -> 0x8d1670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0xca4e4a -> 0x8d1670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0xca4e6d -> 0x8d1670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0xca7b83 -> 0x8d1670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0xca7ba6 -> 0x8d1670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0xccd077 -> 0x8d1670 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::utils_intersect::line_x_shape_overlap @ 0x10449cc -> 0x8d1670 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::utils_intersect::line_x_shape_overlap @ 0x1044a1f -> 0x8d1670 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::contains_point; module=740, inline_record=1728; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_points2; module=740, inline_record=3696; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_curve2; module=740, inline_record=4444; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::as_transformation_and_size; module=740, inline_record=7172; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_point; module=740, inline_record=7436; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=740, inline_record=9152; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=740, inline_record=9308; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=740, inline_record=9776; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::orientation_mat3
// 信息 ID: 136955409489bd6efb4aaec0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0x101e421 -> 0x8d16a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::widget @ 0x102160f -> 0x8d16a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x12e9ba0 -> 0x8d16a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x12ea184 -> 0x8d16a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d
// 信息 ID: 1d4f78671fa61399a71ccafc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad984d -> 0x8d3420 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::perimeter
// 信息 ID: 858cbe6280bfc8677919d42d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 20 条；内联宿主 18 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x502f51 -> 0x8d1660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited @ 0x50307f -> 0x8d1660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_outline::wall_outline @ 0x570d10 -> 0x8d1660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0xc93876 -> 0x8d1660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0xc938bb -> 0x8d1660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0xc938c7 -> 0x8d1660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0xc93dfc -> 0x8d1660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0xc93e0c -> 0x8d1660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_bottom_layer::construct_rect_bottom_layer @ 0xca2b65 -> 0x8d1660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_bottom_layer::construct_rect_bottom_layer @ 0xca2ba5 -> 0x8d1660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd2df8 -> 0x8d1660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd3a0e -> 0x8d1660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds @ 0xcf6a7b -> 0x8d1660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds @ 0xcf6c78 -> 0x8d1660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::debug_viz::DebugRectangle3d::show @ 0xec42fc -> 0x8d1660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_create_rectangle::rectangle_signifier::closure$0 @ 0xf05377 -> 0x8d1660 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_point_u; module=740, inline_record=7896; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_point_u; module=740, inline_record=7924; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_point_u; module=740, inline_record=7952; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=740, inline_record=8396; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=740, inline_record=10768; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=740, inline_record=10796; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=740, inline_record=10824; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=740, inline_record=11084; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::signed_distance_to_closest_edge
// 信息 ID: cc3f74501a3bacc4c23ecf5d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0xc93ff0 -> 0x8d2890 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::distance_to_closest_edge; module=740, inline_record=20436; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle2d::with_expanded_size
// 信息 ID: 865e6c5f4f5edbbbd8b7a448
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_tree::hide_trees::hide_trees @ 0x1a92cc5 -> 0x8d27a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::center
// 信息 ID: 88321b515bd217ff38cac225
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::world::impl$32::into_transformed_shape; module=397, inline_record=34376; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>; module=1226, inline_record=4588; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers; module=1287, inline_record=14512; 不是独立调用边。
// [PDB 内联宿主] system_decorator::derive_decorator_subtype::is_half_dormer; module=2773, inline_record=19964; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows; module=3842, inline_record=19748; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_collision_as_bbx; module=3849, inline_record=20032; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_collision_as_bbx; module=3849, inline_record=20172; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits; module=3856, inline_record=16668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::contains_point
// 信息 ID: a524836aec1b18d753ac5e87
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::extent
// 信息 ID: c648fb99ad285224a8ddd3ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::impl$32::into_transformed_shape @ 0x53e27e -> 0x8d0c50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers @ 0xd0d5a6 -> 0x8d0c50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers @ 0xd0d67d -> 0x8d0c50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows @ 0x1c3f9e7 -> 0x8d0c50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows @ 0x1c3fd73 -> 0x8d0c50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors @ 0x1c49f80 -> 0x8d0c50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits @ 0x1c51ffc -> 0x8d0c50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits @ 0x1c5214e -> 0x8d0c50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x1c60d1a -> 0x8d0c50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x1c61b8f -> 0x8d0c50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x1c61c84 -> 0x8d0c50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x1c61ca9 -> 0x8d0c50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::height
// 信息 ID: b797207dad9fb51894caf466
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 31 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=23148; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=29116; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=29288; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::extent; module=740, inline_record=2296; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>; module=1226, inline_record=4504; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=56916; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::get_window_aabb; module=1249, inline_record=71600; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1286, inline_record=3220; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::length
// 信息 ID: cd8c40f9587e9a61a0ec1568
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::extent; module=740, inline_record=2344; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>; module=1226, inline_record=4560; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_collision_as_bbx; module=3849, inline_record=20004; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_collision_as_bbx; module=3849, inline_record=20144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::new
// 信息 ID: da5b7d411a8296c54df96215
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0xc9f398 -> 0x8d0a00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars @ 0xca375a -> 0x8d0a00 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators::closure_env$0<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$0> > @ 0x17da040 -> 0x8d0a00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$0> @ 0x17da221 -> 0x8d0a00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::load_clutter_meshes::load_clutter_meshes::closure$0 @ 0x1ac9ce7 -> 0x8d0a00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad948c -> 0x8d0a00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad90ae -> 0x8d0a00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::overlaps_rectangle3d
// 信息 ID: 2affa590e195db335a844ae7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators::closure_env$0<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$0> > @ 0x17da04c -> 0x8d0bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::process_clutter_candidates::process_clutter_candidates @ 0x1acb92e -> 0x8d0bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad9517 -> 0x8d0bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad9139 -> 0x8d0bb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::rotate_2d
// 信息 ID: 2b090eed50e73a451fadc1eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::set_orientation_xz
// 信息 ID: c00a39505948df127a4b82e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad835c -> 0x8d0ba0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::translate
// 信息 ID: dea9d302f6a274520697115c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad8314 -> 0x8d0b40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::Rectangle3d::width
// 信息 ID: e2990ef21a3cee0701bf660d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 29 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=23120; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::extent; module=740, inline_record=2320; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>; module=1226, inline_record=4532; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=56884; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::get_window_aabb; module=1249, inline_record=71572; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1286, inline_record=3248; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1286, inline_record=6732; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers; module=1287, inline_record=10396; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize
// 信息 ID: 12b9c6a7806f8659da00f810
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum<serde::__private::de::content::EnumRefDeserializer<serde_json::error::Error> >; module=1326, inline_record=23332; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$0::expecting
// 信息 ID: 3cbe79b0c2ac33f579e1d536
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impl$1::fmt<utils::geometry::rectangle::_::impl$0::deserialize::__FieldVisitor> @ 0x697120 -> 0x8d46b0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_bytes<serde_json::error::Error>
// 信息 ID: e04a90f0a58996f4ec8ae1b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_bytes -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: 794008e0411853490e288803
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_u64
// 信息 ID: f802a65f174aa1b27023a718
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_identifier<serde_json::error::Error,utils::geometry::rectangle::_::impl$0::deserialize::__FieldVisitor>; module=1421, inline_record=19116; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_identifier<serde_json::error::Error,utils::geometry::rectangle::_::impl$0::deserialize::__FieldVisitor>; module=1421, inline_record=19172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 67dbaec1a06893a334fe04d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::geometry::rectangle::_::impl$0::deserialize::__Visitor>; module=1421, inline_record=12668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: dce0e59d1879acffe7925527
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impl$1::fmt<utils::geometry::rectangle::_::impl$0::deserialize::__Visitor> @ 0x697110 -> 0x8d46d0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: ba3d9ed35b48616e7ced9d1a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::geometry::rectangle::_::impl$0::deserialize::__Visitor>; module=1421, inline_record=12188; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: a9110db14378a13ce5099007
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<serde_json::error::Error,utils::geometry::rectangle::_::impl$0::deserialize::__Visitor>; module=1421, inline_record=15604; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: 1c611f46e46ac1446dd5ba8f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::geometry::rectangle::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$1::serialize_newtype_variant<utils::geometry::rectangle::Rectangle2d> @ 0x14da2a -> 0x2eaaa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$1::serialize_newtype_variant<utils::geometry::rectangle::Rectangle2d> @ 0x1092c9a -> 0x697310 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$10::eq
// 信息 ID: e9b031cc8e8bf825f073b5e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners; module=2269, inline_record=15832; 不是独立调用边。
// [PDB 内联宿主] system_roof::update_roof_spatial::update_roof_spatial; module=3194, inline_record=1912; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0
// 信息 ID: 3286151fd5f313d32f4a6d0b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::geometry::rectangle::impl$1::get_pos_at_u", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::geometry::rectangle::Rectangle2d::get_pos_at_u @ 0x8d1bb0 -> 0x8d1c50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$1::intersects::closure$0
// 信息 ID: 6a5d08034443614d3573f912
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::geometry::rectangle::impl$1::intersects", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$2::from
// 信息 ID: 00f45e332006531f27e90230
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0x585d4d -> 0x8d4620 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$3::clone
// 信息 ID: ec832e8deaf15c33cd0552a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::insert_collision_and_interaction_as_same; module=460, inline_record=3128; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::insert; module=460, inline_record=5996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::rectangle::impl$7::fmt
// 信息 ID: 38fb1ff026c120f1159fcd7a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<utils::geometry::rectangle::Rectangle2d>; module=486, inline_record=3972; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$51::fmt<utils::geometry::rectangle::Rectangle2d>; module=4467, inline_record=396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/construct_elevation_supports/circle_pillars.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 5bbc91ad3f80e2bc8b6a624a9587a4910354d37a0ea49688638fa11368f735c6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: circle_pillars.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1219
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars
// 记录形式: compiled_procedure; instances=1
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1219,"record_offset":260,"rva":13232784,"va":5381941904}
// 编译实例 RVAs: 0xc9ea90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: circle_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12130320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e8f0e746ff","size":5,"soft_fail":false,"target_va":5369813552,"va":5381943867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: circle_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9194710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e82ab373ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5381944417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14051aa00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7883255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e809b687ff","size":5,"soft_fail":false,"target_va":5374061056,"va":5381944306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b09b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6218658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e85e1ca1ff","size":5,"soft_fail":false,"target_va":5375723952,"va":5381942605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d9780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5002220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e814acb3ff","size":5,"soft_fail":false,"target_va":5376939904,"va":5381942119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4998763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e895b9b3ff","size":5,"soft_fail":false,"target_va":5376943968,"va":5381942726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: circle_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4998678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e8eab9b3ff","size":5,"soft_fail":false,"target_va":5376944000,"va":5381942673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144da0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: circle_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21650718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e81e5d4a01","size":5,"soft_fail":false,"target_va":5403594144,"va":5381943421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21652097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e881624a01","size":5,"soft_fail":false,"target_va":5403594288,"va":5381942186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10638083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e8fdac5dff","size":5,"soft_fail":false,"target_va":5371305600,"va":5381943678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t21034263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e817f54001","size":5,"soft_fail":false,"target_va":5402978880,"va":5381944612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t21034385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e891f54001","size":5,"soft_fail":false,"target_va":5402979152,"va":5381944762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t21034213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e8e5f44001","size":5,"soft_fail":false,"target_va":5402978944,"va":5381944726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-9795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e8bdd9ffff","size":5,"soft_fail":false,"target_va":5381932784,"va":5381942574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-6507579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e8c5b39cff","size":5,"soft_fail":false,"target_va":5375434976,"va":5381942550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_elevation_supports::rectangle_pillars::find_stone_column_start_y
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t19389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e8bd4b0000","size":5,"soft_fail":false,"target_va":5381962240,"va":5381942846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_brick_columns
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11360204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e8cc57ad00","size":5,"soft_fail":false,"target_va":5393304704,"va":5381944495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11356220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e83c48ad00","size":5,"soft_fail":false,"target_va":5393300608,"va":5381944383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-3974317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e8535bc3ff","size":5,"soft_fail":false,"target_va":5377968384,"va":5381942696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-3974564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e85c5ac3ff","size":5,"soft_fail":false,"target_va":5377967920,"va":5381942479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::perimeter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3973629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e8035ec3ff","size":5,"soft_fail":false,"target_va":5377968464,"va":5381942088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::signed_distance
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3973111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e80960c3ff","size":5,"soft_fail":false,"target_va":5377970096,"va":5381943202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::distance_to_closest_edge
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3983282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e84e38c3ff","size":5,"soft_fail":false,"target_va":5377959872,"va":5381943149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3991965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e86316c3ff","size":5,"soft_fail":false,"target_va":5377952256,"va":5381944216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t311689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381941904,"raw_hex":"e889c10400","size":5,"soft_fail":false,"target_va":5382255664,"va":5381943970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1219,"record_offset":524,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1219,"record_offset":3140,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1219,"record_offset":2440,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1219,"record_offset":552,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1219,"record_offset":1740,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1219,"record_offset":752,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1219,"record_offset":1112,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1219,"record_offset":2012,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1219,"record_offset":3672,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1219,"record_offset":3276,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1219,"record_offset":1048,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1219,"record_offset":1824,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1219,"record_offset":1584,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1219,"record_offset":1472,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1219,"record_offset":3644,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14693,"source_line":76,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1219,"record_offset":1236,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1219,"record_offset":1148,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1219,"record_offset":584,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1219,"record_offset":724,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6599,"source_line":101,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/circle_pillars.rs
//     evidence={"module":1219,"record_offset":1324,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59523,"source_line":50,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1219,"record_offset":836,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1219,"record_offset":2316,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21570,"source_line":31,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1219,"record_offset":3848,"parent_record_offset":3824,"ancestor_record_offset":260,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1219,"record_offset":3824,"parent_record_offset":3724,"ancestor_record_offset":260,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1219,"record_offset":3756,"parent_record_offset":3724,"ancestor_record_offset":260,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1219,"record_offset":3212,"parent_record_offset":3180,"ancestor_record_offset":260,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1219,"record_offset":3724,"parent_record_offset":3696,"ancestor_record_offset":260,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1219,"record_offset":3180,"parent_record_offset":3140,"ancestor_record_offset":260,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1219,"record_offset":924,"parent_record_offset":900,"ancestor_record_offset":260,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1219,"record_offset":948,"parent_record_offset":924,"ancestor_record_offset":260,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1219,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":260,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1219,"record_offset":1376,"parent_record_offset":1348,"ancestor_record_offset":260,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1219,"record_offset":988,"parent_record_offset":836,"ancestor_record_offset":260,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1219,"record_offset":1016,"parent_record_offset":836,"ancestor_record_offset":260,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1219,"record_offset":776,"parent_record_offset":752,"ancestor_record_offset":260,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1219,"record_offset":3788,"parent_record_offset":3756,"ancestor_record_offset":260,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1219,"record_offset":636,"parent_record_offset":584,"ancestor_record_offset":260,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1219,"record_offset":692,"parent_record_offset":584,"ancestor_record_offset":260,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1219,"record_offset":608,"parent_record_offset":584,"ancestor_record_offset":260,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1219,"record_offset":900,"parent_record_offset":876,"ancestor_record_offset":260,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1219,"record_offset":876,"parent_record_offset":836,"ancestor_record_offset":260,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1219,"record_offset":3696,"parent_record_offset":3672,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1219,"record_offset":3324,"parent_record_offset":3300,"ancestor_record_offset":260,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1219,"record_offset":1080,"parent_record_offset":1048,"ancestor_record_offset":260,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1219,"record_offset":3244,"parent_record_offset":3140,"ancestor_record_offset":260,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1219,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":260,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1219,"record_offset":1908,"parent_record_offset":1876,"ancestor_record_offset":260,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1219,"record_offset":1876,"parent_record_offset":1848,"ancestor_record_offset":260,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1219,"record_offset":1848,"parent_record_offset":1824,"ancestor_record_offset":260,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1219,"record_offset":2120,"parent_record_offset":2040,"ancestor_record_offset":260,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1219,"record_offset":2064,"parent_record_offset":2040,"ancestor_record_offset":260,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1219,"record_offset":1412,"parent_record_offset":1324,"ancestor_record_offset":260,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1219,"record_offset":1940,"parent_record_offset":1908,"ancestor_record_offset":260,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/circle_pillars.rs
//     evidence={"module":1219,"record_offset":2040,"parent_record_offset":2012,"ancestor_record_offset":260,"inlinee":59524,"source_line":127,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1219,"record_offset":2404,"parent_record_offset":2380,"ancestor_record_offset":260,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1219,"record_offset":2504,"parent_record_offset":2472,"ancestor_record_offset":260,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1219,"record_offset":2380,"parent_record_offset":2316,"ancestor_record_offset":260,"inlinee":21571,"source_line":90,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1219,"record_offset":2472,"parent_record_offset":2440,"ancestor_record_offset":260,"inlinee":26825,"source_line":115,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1219,"record_offset":1348,"parent_record_offset":1324,"ancestor_record_offset":260,"inlinee":21527,"source_line":230,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE883
// 首条位置: {"module":1219,"inlinee":59523}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1219,"record_offset":1412,"parent_record_offset":1324,"ancestor_record_offset":1324,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1219,"record_offset":1348,"parent_record_offset":1324,"ancestor_record_offset":1324,"inlinee":21527,"source_line":230,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1219,"record_offset":1376,"parent_record_offset":1348,"ancestor_record_offset":1324,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1219,"record_offset":1436,"parent_record_offset":1412,"ancestor_record_offset":1324,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE884
// 首条位置: {"module":1219,"inlinee":59524}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1219,"record_offset":2120,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1219,"record_offset":2064,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1219,"record_offset":2272,"parent_record_offset":2144,"ancestor_record_offset":2040,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1219,"record_offset":2088,"parent_record_offset":2064,"ancestor_record_offset":2040,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1219,"record_offset":2204,"parent_record_offset":2172,"ancestor_record_offset":2040,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1219,"record_offset":2172,"parent_record_offset":2144,"ancestor_record_offset":2040,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1219,"record_offset":2144,"parent_record_offset":2120,"ancestor_record_offset":2040,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/01219.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1219,"record_offset":2236,"parent_record_offset":2204,"ancestor_record_offset":2040,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01219.jsonl.gz"}
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
// 完整结构及原始类型记录: circle_pillars.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: circle_pillars.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: circle_pillars.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars
// 信息 ID: 6f897609cd2d3cd5f93ec33c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::construct_circle_supports @ 0x4d5742 -> 0xc9ea90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars::closure$0
// 信息 ID: 19ab0829bd6608324a3cf6fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars; module=1219, inline_record=1324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars::closure$1
// 信息 ID: 2cd304ebe939b89c3dbcfe41
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars; module=1219, inline_record=2040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

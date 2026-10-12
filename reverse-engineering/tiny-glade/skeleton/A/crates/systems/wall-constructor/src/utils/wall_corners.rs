// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/utils/wall_corners.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 7858f7dd984fcce473f47733f75e7227215e61f08bd4ece41020b296783e03f9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_corners.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1248, 1263, 1264
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
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
// 函数符号: system_wall_constructor::utils::wall_corners::add_wall_corners
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1248,"record_offset":260,"rva":13412352,"va":5382121472}
// 编译实例 RVAs: 0xcca800
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=2
//     target_aliases=1889 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13402239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e8817f33ff","size":5,"soft_fail":false,"target_va":5368720672,"va":5382122906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12314019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e85d1a44ff","size":5,"soft_fail":false,"target_va":5369813552,"va":5382127566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9374510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e8d2f470ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5382124201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=16
//     target_aliases=1 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9376271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e8f1ed70ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382125978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14056dd60
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7725532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e8241e8aff","size":5,"soft_fail":false,"target_va":5374401888,"va":5382127415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=3
//     target_aliases=28 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6952039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e899eb95ff","size":5,"soft_fail":false,"target_va":5375171056,"va":5382123090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cacf70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-121153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e8bf26feff","size":5,"soft_fail":false,"target_va":5382000496,"va":5382121644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb4820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-91435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e8d59afeff","size":5,"soft_fail":false,"target_va":5382031392,"va":5382122822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb4bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-89296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e830a3feff","size":5,"soft_fail":false,"target_va":5382032368,"va":5382121659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb560
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t131867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e81b030200","size":5,"soft_fail":false,"target_va":5382255968,"va":5382124096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417768f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11189814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e836beaa00","size":5,"soft_fail":false,"target_va":5393311984,"va":5382122165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177faa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11221001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e80938ab00","size":5,"soft_fail":false,"target_va":5393349280,"va":5382128274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21470633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e8a99d4701","size":5,"soft_fail":false,"target_va":5403593968,"va":5382123330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20849406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e8fe223e01","size":5,"soft_fail":false,"target_va":5402978144,"va":5382128733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-10816994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e81ef25aff","size":5,"soft_fail":false,"target_va":5371305056,"va":5382122045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20850083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e8a3253e01","size":5,"soft_fail":false,"target_va":5402978880,"va":5382128792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20850683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e8fb273e01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382128464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t20850855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e8a7283e01","size":5,"soft_fail":false,"target_va":5402979344,"va":5382128484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20851272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e8482a3e01","size":5,"soft_fail":false,"target_va":5402980128,"va":5382128851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::render::Affine3Packed::from_mat3_translation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-257210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e84613fcff","size":5,"soft_fail":false,"target_va":5381870112,"va":5382127317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-191119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e87115fdff","size":5,"soft_fail":false,"target_va":5381932864,"va":5382123978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-188751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e8b11efdff","size":5,"soft_fail":false,"target_va":5381933056,"va":5382121802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3726243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e8a3db3800","size":5,"soft_fail":false,"target_va":5385850032,"va":5382123784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t11218339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e8a32dab00","size":5,"soft_fail":false,"target_va":5393343808,"va":5382125464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::wall_corners::add_wall_corners::closure$1
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t4407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e837110000","size":5,"soft_fail":false,"target_va":5382130752,"va":5382126340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=unresolved_namespace; count=27
//     evidence={"assembly":"\tcallq\t2295606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e836072300","size":5,"soft_fail":false,"target_va":5384420192,"va":5382124581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t2295816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e808082300","size":5,"soft_fail":false,"target_va":5384420000,"va":5382124179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t2293677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e8adff2200","size":5,"soft_fail":false,"target_va":5384420752,"va":5382127070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersects
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2294925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e88d042300","size":5,"soft_fail":false,"target_va":5384420368,"va":5382125438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t2295888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e850082300","size":5,"soft_fail":false,"target_va":5384420736,"va":5382124843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5183834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e8a6e6b0ff","size":5,"soft_fail":false,"target_va":5376939616,"va":5382123445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t129429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e895f90100","size":5,"soft_fail":false,"target_va":5382256592,"va":5382127158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::with_y
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t133469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e85d090200","size":5,"soft_fail":false,"target_va":5382257616,"va":5382124142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_mat3
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5184435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382121472,"raw_hex":"e84de4b0ff","size":5,"soft_fail":false,"target_va":5376942752,"va":5382127182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1248,"record_offset":5420,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$21::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1248,"record_offset":5572,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":23774,"source_line":2875,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::into_iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\into_iter.rs
//     evidence={"module":1248,"record_offset":7276,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":10715,"source_line":46,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1248,"record_offset":2888,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1248,"record_offset":7508,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1248,"record_offset":3064,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1248,"record_offset":2916,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1248,"record_offset":2972,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1248,"record_offset":5980,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1248,"record_offset":5712,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1248,"record_offset":4928,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1248,"record_offset":5840,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":10010,"source_line":120,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1248,"record_offset":7848,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1248,"record_offset":9784,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1248,"record_offset":6532,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1248,"record_offset":5600,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7698,"source_line":24,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1248,"record_offset":4324,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1248,"record_offset":800,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21382,"source_line":101,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::mat3::Mat3::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/mat3.rs
//     evidence={"module":1248,"record_offset":7760,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":36060,"source_line":82,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$11::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1248,"record_offset":5364,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":27855,"source_line":991,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1248,"record_offset":5240,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1248,"record_offset":7648,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1248,"record_offset":4460,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1248,"record_offset":5072,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1248,"record_offset":4404,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1248,"record_offset":4432,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::wall_corners::find_corners
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_corners.rs
//     evidence={"module":1248,"record_offset":828,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59880,"source_line":492,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1248,"record_offset":8708,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21348,"source_line":525,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1248,"record_offset":6036,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21590,"source_line":145,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1248,"record_offset":3036,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21364,"source_line":871,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1248,"record_offset":3256,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":27900,"source_line":133,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::to_curve2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1248,"record_offset":548,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21343,"source_line":938,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1248,"record_offset":3376,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21356,"source_line":1168,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::as_vec2
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1248,"record_offset":5812,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":26222,"source_line":146,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::with_offset_clamped
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1248,"record_offset":5332,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21740,"source_line":241,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::x
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1248,"record_offset":7908,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59886,"source_line":141,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1248,"record_offset":5476,"parent_record_offset":5452,"ancestor_record_offset":260,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1248,"record_offset":5500,"parent_record_offset":5476,"ancestor_record_offset":260,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1248,"record_offset":2840,"parent_record_offset":2816,"ancestor_record_offset":260,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1248,"record_offset":5452,"parent_record_offset":5420,"ancestor_record_offset":260,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1248,"record_offset":2816,"parent_record_offset":2716,"ancestor_record_offset":260,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1248,"record_offset":7444,"parent_record_offset":7308,"ancestor_record_offset":260,"inlinee":10717,"source_line":165,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1248,"record_offset":7308,"parent_record_offset":7276,"ancestor_record_offset":260,"inlinee":10716,"source_line":1610,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1248,"record_offset":7348,"parent_record_offset":7308,"ancestor_record_offset":260,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1248,"record_offset":7372,"parent_record_offset":7348,"ancestor_record_offset":260,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1248,"record_offset":2748,"parent_record_offset":2716,"ancestor_record_offset":260,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1248,"record_offset":1816,"parent_record_offset":1792,"ancestor_record_offset":260,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1248,"record_offset":2716,"parent_record_offset":2688,"ancestor_record_offset":260,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1248,"record_offset":1792,"parent_record_offset":1760,"ancestor_record_offset":260,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1248,"record_offset":916,"parent_record_offset":828,"ancestor_record_offset":260,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1248,"record_offset":1760,"parent_record_offset":828,"ancestor_record_offset":260,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1248,"record_offset":688,"parent_record_offset":664,"ancestor_record_offset":260,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1248,"record_offset":9048,"parent_record_offset":9024,"ancestor_record_offset":260,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1248,"record_offset":9096,"parent_record_offset":9072,"ancestor_record_offset":260,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1248,"record_offset":1692,"parent_record_offset":1668,"ancestor_record_offset":260,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1248,"record_offset":1716,"parent_record_offset":1692,"ancestor_record_offset":260,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1248,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":260,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1248,"record_offset":1588,"parent_record_offset":1304,"ancestor_record_offset":260,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1248,"record_offset":1264,"parent_record_offset":1240,"ancestor_record_offset":260,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1248,"record_offset":2008,"parent_record_offset":828,"ancestor_record_offset":260,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1248,"record_offset":4528,"parent_record_offset":4488,"ancestor_record_offset":260,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1248,"record_offset":1240,"parent_record_offset":1072,"ancestor_record_offset":260,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1248,"record_offset":664,"parent_record_offset":548,"ancestor_record_offset":260,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1248,"record_offset":2780,"parent_record_offset":2748,"ancestor_record_offset":260,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1248,"record_offset":7396,"parent_record_offset":7372,"ancestor_record_offset":260,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1248,"record_offset":3636,"parent_record_offset":3376,"ancestor_record_offset":260,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1248,"record_offset":3664,"parent_record_offset":3376,"ancestor_record_offset":260,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1248,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":260,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1248,"record_offset":4716,"parent_record_offset":4692,"ancestor_record_offset":260,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::Range::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1248,"record_offset":1644,"parent_record_offset":1304,"ancestor_record_offset":260,"inlinee":10010,"source_line":120,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1248,"record_offset":1668,"parent_record_offset":1644,"ancestor_record_offset":260,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1248,"record_offset":3336,"parent_record_offset":3256,"ancestor_record_offset":260,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1248,"record_offset":4692,"parent_record_offset":4528,"ancestor_record_offset":260,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1248,"record_offset":2664,"parent_record_offset":828,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1248,"record_offset":628,"parent_record_offset":604,"ancestor_record_offset":260,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1248,"record_offset":1204,"parent_record_offset":1180,"ancestor_record_offset":260,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1248,"record_offset":1152,"parent_record_offset":1112,"ancestor_record_offset":260,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1248,"record_offset":5528,"parent_record_offset":5500,"ancestor_record_offset":260,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1248,"record_offset":1848,"parent_record_offset":1760,"ancestor_record_offset":260,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1248,"record_offset":752,"parent_record_offset":720,"ancestor_record_offset":260,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1248,"record_offset":3280,"parent_record_offset":3256,"ancestor_record_offset":260,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1248,"record_offset":580,"parent_record_offset":548,"ancestor_record_offset":260,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1248,"record_offset":3092,"parent_record_offset":3064,"ancestor_record_offset":260,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1248,"record_offset":3304,"parent_record_offset":3280,"ancestor_record_offset":260,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1248,"record_offset":3120,"parent_record_offset":3092,"ancestor_record_offset":260,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1248,"record_offset":604,"parent_record_offset":580,"ancestor_record_offset":260,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1248,"record_offset":1180,"parent_record_offset":1112,"ancestor_record_offset":260,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1248,"record_offset":1112,"parent_record_offset":1072,"ancestor_record_offset":260,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1248,"record_offset":1472,"parent_record_offset":1448,"ancestor_record_offset":260,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1248,"record_offset":1500,"parent_record_offset":1448,"ancestor_record_offset":260,"inlinee":21909,"source_line":390,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1248,"record_offset":1336,"parent_record_offset":1304,"ancestor_record_offset":260,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1248,"record_offset":1040,"parent_record_offset":828,"ancestor_record_offset":260,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1248,"record_offset":2044,"parent_record_offset":2008,"ancestor_record_offset":260,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1248,"record_offset":1072,"parent_record_offset":1040,"ancestor_record_offset":260,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1248,"record_offset":4764,"parent_record_offset":4740,"ancestor_record_offset":260,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1248,"record_offset":1556,"parent_record_offset":1448,"ancestor_record_offset":260,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::wall_corners::add_wall_corners::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_corners.rs
//     evidence={"module":1248,"record_offset":4740,"parent_record_offset":4716,"ancestor_record_offset":260,"inlinee":59884,"source_line":142,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::wall_corners::add_wall_corners::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_corners.rs
//     evidence={"module":1248,"record_offset":7876,"parent_record_offset":7848,"ancestor_record_offset":260,"inlinee":59885,"source_line":448,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::wall_corners::check_angle
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_corners.rs
//     evidence={"module":1248,"record_offset":1304,"parent_record_offset":828,"ancestor_record_offset":260,"inlinee":59881,"source_line":518,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] utils::cos_angle_between
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1248,"record_offset":1448,"parent_record_offset":1304,"ancestor_record_offset":260,"inlinee":59882,"source_line":447,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::new_from_points
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1248,"record_offset":720,"parent_record_offset":548,"ancestor_record_offset":260,"inlinee":21342,"source_line":216,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1248,"record_offset":3424,"parent_record_offset":3376,"ancestor_record_offset":260,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1248,"record_offset":3516,"parent_record_offset":3376,"ancestor_record_offset":260,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_corners::add_wall_corners::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 142
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9EC
// 首条位置: {"module":1248,"inlinee":59884}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1248,"record_offset":4764,"parent_record_offset":4740,"ancestor_record_offset":4740,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1248,"record_offset":4816,"parent_record_offset":4764,"ancestor_record_offset":4740,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1248,"record_offset":4872,"parent_record_offset":4764,"ancestor_record_offset":4740,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1248,"record_offset":4788,"parent_record_offset":4764,"ancestor_record_offset":4740,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_corners::add_wall_corners::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 352
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1248,"record_offset":11460,"rva":13421632,"va":5382130752}
// 编译实例 RVAs: 0xcccc40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb4820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_corners.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-99435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382130752,"raw_hex":"e8957bfeff","size":5,"soft_fail":false,"target_va":5382031392,"va":5382130822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20847957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382130752,"raw_hex":"e8551d3e01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382131190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t20848330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382130752,"raw_hex":"e8ca1e3e01","size":5,"soft_fail":false,"target_va":5402979344,"va":5382131009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20847773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382130752,"raw_hex":"e89d1c3e01","size":5,"soft_fail":false,"target_va":5402978944,"va":5382131166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1248,"record_offset":11864,"parent_record_offset":11460,"ancestor_record_offset":11460,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::i0_t
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1248,"record_offset":11836,"parent_record_offset":11460,"ancestor_record_offset":11460,"inlinee":59887,"source_line":1123,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1248,"record_offset":11628,"parent_record_offset":11460,"ancestor_record_offset":11460,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1248,"record_offset":11716,"parent_record_offset":11692,"ancestor_record_offset":11460,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1248,"record_offset":11740,"parent_record_offset":11716,"ancestor_record_offset":11460,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1248,"record_offset":11776,"parent_record_offset":11628,"ancestor_record_offset":11460,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1248,"record_offset":11804,"parent_record_offset":11628,"ancestor_record_offset":11460,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1248,"record_offset":11896,"parent_record_offset":11864,"ancestor_record_offset":11460,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1248,"record_offset":11996,"parent_record_offset":11864,"ancestor_record_offset":11460,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1248,"record_offset":11968,"parent_record_offset":11864,"ancestor_record_offset":11460,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1248,"record_offset":11692,"parent_record_offset":11668,"ancestor_record_offset":11460,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1248,"record_offset":11668,"parent_record_offset":11628,"ancestor_record_offset":11460,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_corners::add_wall_corners::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 448
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9ED
// 首条位置: {"module":1248,"inlinee":59885}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_corners::find_corners
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 492
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9E8
// 首条位置: {"module":1248,"inlinee":59880}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1248,"record_offset":916,"parent_record_offset":828,"ancestor_record_offset":828,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1248,"record_offset":1760,"parent_record_offset":828,"ancestor_record_offset":828,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1248,"record_offset":2008,"parent_record_offset":828,"ancestor_record_offset":828,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1248,"record_offset":2664,"parent_record_offset":828,"ancestor_record_offset":828,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1248,"record_offset":944,"parent_record_offset":828,"ancestor_record_offset":828,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1248,"record_offset":1040,"parent_record_offset":828,"ancestor_record_offset":828,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::wall_corners::check_angle
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/wall_corners.rs
//     evidence={"module":1248,"record_offset":1304,"parent_record_offset":828,"ancestor_record_offset":828,"inlinee":59881,"source_line":518,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1248,"record_offset":2840,"parent_record_offset":2816,"ancestor_record_offset":828,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1248,"record_offset":2816,"parent_record_offset":2716,"ancestor_record_offset":828,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1248,"record_offset":2748,"parent_record_offset":2716,"ancestor_record_offset":828,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1248,"record_offset":1816,"parent_record_offset":1792,"ancestor_record_offset":828,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1248,"record_offset":2716,"parent_record_offset":2688,"ancestor_record_offset":828,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1248,"record_offset":1792,"parent_record_offset":1760,"ancestor_record_offset":828,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1248,"record_offset":1692,"parent_record_offset":1668,"ancestor_record_offset":828,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1248,"record_offset":1716,"parent_record_offset":1692,"ancestor_record_offset":828,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1248,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":828,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1248,"record_offset":1588,"parent_record_offset":1304,"ancestor_record_offset":828,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1248,"record_offset":1264,"parent_record_offset":1240,"ancestor_record_offset":828,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1248,"record_offset":1240,"parent_record_offset":1072,"ancestor_record_offset":828,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1248,"record_offset":2780,"parent_record_offset":2748,"ancestor_record_offset":828,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1248,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":828,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::Range::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1248,"record_offset":1644,"parent_record_offset":1304,"ancestor_record_offset":828,"inlinee":10010,"source_line":120,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1248,"record_offset":1668,"parent_record_offset":1644,"ancestor_record_offset":828,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1248,"record_offset":2688,"parent_record_offset":2664,"ancestor_record_offset":828,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1248,"record_offset":1000,"parent_record_offset":972,"ancestor_record_offset":828,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1248,"record_offset":1204,"parent_record_offset":1180,"ancestor_record_offset":828,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1248,"record_offset":1152,"parent_record_offset":1112,"ancestor_record_offset":828,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1248,"record_offset":1848,"parent_record_offset":1760,"ancestor_record_offset":828,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1248,"record_offset":972,"parent_record_offset":944,"ancestor_record_offset":828,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1248,"record_offset":1180,"parent_record_offset":1112,"ancestor_record_offset":828,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1248,"record_offset":1112,"parent_record_offset":1072,"ancestor_record_offset":828,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1248,"record_offset":1472,"parent_record_offset":1448,"ancestor_record_offset":828,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1248,"record_offset":1500,"parent_record_offset":1448,"ancestor_record_offset":828,"inlinee":21909,"source_line":390,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1248,"record_offset":1336,"parent_record_offset":1304,"ancestor_record_offset":828,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1248,"record_offset":2044,"parent_record_offset":2008,"ancestor_record_offset":828,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1248,"record_offset":1072,"parent_record_offset":1040,"ancestor_record_offset":828,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1248,"record_offset":1556,"parent_record_offset":1448,"ancestor_record_offset":828,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] utils::cos_angle_between
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1248,"record_offset":1448,"parent_record_offset":1304,"ancestor_record_offset":828,"inlinee":59882,"source_line":447,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::wall_corners::check_angle
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 518
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9E9
// 首条位置: {"module":1248,"inlinee":59881}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1248,"record_offset":1588,"parent_record_offset":1304,"ancestor_record_offset":1304,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1248,"record_offset":1644,"parent_record_offset":1304,"ancestor_record_offset":1304,"inlinee":10010,"source_line":120,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1248,"record_offset":1336,"parent_record_offset":1304,"ancestor_record_offset":1304,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] utils::cos_angle_between
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1248,"record_offset":1448,"parent_record_offset":1304,"ancestor_record_offset":1304,"inlinee":59882,"source_line":447,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1248,"record_offset":1692,"parent_record_offset":1668,"ancestor_record_offset":1304,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1248,"record_offset":1716,"parent_record_offset":1692,"ancestor_record_offset":1304,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1248,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":1304,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1248,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":1304,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1248,"record_offset":1668,"parent_record_offset":1644,"ancestor_record_offset":1304,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1248,"record_offset":1472,"parent_record_offset":1448,"ancestor_record_offset":1304,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1248,"record_offset":1500,"parent_record_offset":1448,"ancestor_record_offset":1304,"inlinee":21909,"source_line":390,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1248,"record_offset":1556,"parent_record_offset":1448,"ancestor_record_offset":1304,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01248.jsonl.gz"}
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
// 完整结构及原始类型记录: wall_corners.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_corners.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_corners.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_corners::add_wall_corners
// 信息 ID: b7dc58461d9412b1fab3abb3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x4eabea -> 0xcca800 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_corners::add_wall_corners::closure$0
// 信息 ID: d6e9fef1c36001758feaa658
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::utils::wall_corners::add_wall_corners", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners; module=1248, inline_record=4740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_corners::add_wall_corners::closure$1
// 信息 ID: fbb5aa82619aa8652e364f4a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::utils::wall_corners::add_wall_corners", "captures": "unknown"}
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccbb04 -> 0xcccc40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccbb38 -> 0xcccc40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccbb69 -> 0xcccc40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_corners::add_wall_corners::closure$2
// 信息 ID: b7bdbe964f414328330aa733
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::utils::wall_corners::add_wall_corners", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners; module=1248, inline_record=7876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_corners::check_angle
// 信息 ID: 599f5707928a12b3fed0755f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners; module=1248, inline_record=1304; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners; module=1248, inline_record=2208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::wall_corners::find_corners
// 信息 ID: 3d08f0c9ca88c2eee9153ced
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners; module=1248, inline_record=828; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

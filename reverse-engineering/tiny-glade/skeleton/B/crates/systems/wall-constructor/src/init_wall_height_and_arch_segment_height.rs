// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/init_wall_height_and_arch_segment_height.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: fc6ddbac324a7c419656d6274ebbe5256051c85871454d317726a2ec5cc9cf71
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: init_wall_height_and_arch_segment_height.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 413, 1415, 1417
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":413,"record_offset":384,"rva":7910736,"va":5376619856}
// 编译实例 RVAs: 0x78b550
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ba9e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init_wall_height_and_arch_segment_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5051992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376619856,"raw_hex":"e8a8e9b2ff","size":5,"soft_fail":false,"target_va":5371570656,"va":5376622643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: init_wall_height_and_arch_segment_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1943997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376619856,"raw_hex":"e84356e2ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5376620328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d24f0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: init_wall_height_and_arch_segment_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1811055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376619856,"raw_hex":"e8915de4ff","size":5,"soft_fail":false,"target_va":5374813424,"va":5376624474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: init_wall_height_and_arch_segment_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1539110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376619856,"raw_hex":"e8da83e8ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376620609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: init_wall_height_and_arch_segment_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1539028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376619856,"raw_hex":"e82c84e8ff","size":5,"soft_fail":false,"target_va":5375081536,"va":5376620559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408c5670
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: init_wall_height_and_arch_segment_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1283774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376619856,"raw_hex":"e8be961300","size":5,"soft_fail":false,"target_va":5377906288,"va":5376622509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c904a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init_wall_height_and_arch_segment_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5260476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376619856,"raw_hex":"e8bc445000","size":5,"soft_fail":false,"target_va":5381883040,"va":5376622559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f9920
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init_wall_height_and_arch_segment_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10934489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376619856,"raw_hex":"e8d9d8a600","size":5,"soft_fail":false,"target_va":5387557152,"va":5376622658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f70
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: init_wall_height_and_arch_segment_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35224984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376619856,"raw_hex":"e8987d1902","size":5,"soft_fail":false,"target_va":5411848048,"va":5376623059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34716036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376619856,"raw_hex":"e884b91102","size":5,"soft_fail":false,"target_va":5411339744,"va":5376623703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34916732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376619856,"raw_hex":"e87cc91402","size":5,"soft_fail":false,"target_va":5411540963,"va":5376624226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34917675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376619856,"raw_hex":"e82bcd1402","size":5,"soft_fail":false,"target_va":5411541776,"va":5376624096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34917934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376619856,"raw_hex":"e82ece1402","size":5,"soft_fail":false,"target_va":5411542296,"va":5376624357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34918582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376619856,"raw_hex":"e8b6d01402","size":5,"soft_fail":false,"target_va":5411542880,"va":5376624293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::height
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1405130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376619856,"raw_hex":"e8ca701500","size":5,"soft_fail":false,"target_va":5378028128,"va":5376622993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_gable
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1406485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376619856,"raw_hex":"e815761500","size":5,"soft_fail":false,"target_va":5378028464,"va":5376621974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::RoofShapeParams::eave_length_ws
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1408110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376619856,"raw_hex":"e86e7c1500","size":5,"soft_fail":false,"target_va":5378030640,"va":5376622525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::is_rectangle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3795697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376619856,"raw_hex":"e8f1ea3900","size":5,"soft_fail":false,"target_va":5380417424,"va":5376621722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3171817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376619856,"raw_hex":"e8e9653000","size":5,"soft_fail":false,"target_va":5379793584,"va":5376621762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5298421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376619856,"raw_hex":"e8f5d85000","size":5,"soft_fail":false,"target_va":5381920464,"va":5376622038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":413,"record_offset":3560,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":413,"record_offset":3588,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":5294,"source_line":3002,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":413,"record_offset":2872,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":413,"record_offset":10056,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":413,"record_offset":5040,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":413,"record_offset":1788,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":413,"record_offset":668,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":413,"record_offset":6048,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":413,"record_offset":9296,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":413,"record_offset":3500,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":413,"record_offset":1364,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":413,"record_offset":9084,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":413,"record_offset":8944,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":413,"record_offset":9168,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":7522,"source_line":120,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":413,"record_offset":10272,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":413,"record_offset":3124,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":413,"record_offset":10328,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":413,"record_offset":3412,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":413,"record_offset":3384,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":413,"record_offset":8648,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":413,"record_offset":1844,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":413,"record_offset":9336,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":11915,"source_line":339,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_to_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":413,"record_offset":8704,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":24511,"source_line":1147,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":413,"record_offset":9140,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":413,"record_offset":3044,"parent_record_offset":3020,"ancestor_record_offset":384,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":413,"record_offset":3068,"parent_record_offset":3044,"ancestor_record_offset":384,"inlinee":4772,"source_line":173,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":413,"record_offset":3336,"parent_record_offset":3312,"ancestor_record_offset":384,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":413,"record_offset":3020,"parent_record_offset":2988,"ancestor_record_offset":384,"inlinee":4771,"source_line":259,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":413,"record_offset":3312,"parent_record_offset":3208,"ancestor_record_offset":384,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":413,"record_offset":3868,"parent_record_offset":3812,"ancestor_record_offset":384,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":413,"record_offset":3712,"parent_record_offset":3684,"ancestor_record_offset":384,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":413,"record_offset":2928,"parent_record_offset":2900,"ancestor_record_offset":384,"inlinee":4769,"source_line":206,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":413,"record_offset":3244,"parent_record_offset":3208,"ancestor_record_offset":384,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":413,"record_offset":3208,"parent_record_offset":3180,"ancestor_record_offset":384,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":413,"record_offset":3772,"parent_record_offset":3740,"ancestor_record_offset":384,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":413,"record_offset":3924,"parent_record_offset":3896,"ancestor_record_offset":384,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":413,"record_offset":3896,"parent_record_offset":3868,"ancestor_record_offset":384,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":413,"record_offset":3740,"parent_record_offset":3712,"ancestor_record_offset":384,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":413,"record_offset":2988,"parent_record_offset":2956,"ancestor_record_offset":384,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":413,"record_offset":2956,"parent_record_offset":2928,"ancestor_record_offset":384,"inlinee":4770,"source_line":442,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":413,"record_offset":3180,"parent_record_offset":3152,"ancestor_record_offset":384,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":413,"record_offset":3812,"parent_record_offset":3620,"ancestor_record_offset":384,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":413,"record_offset":6288,"parent_record_offset":6264,"ancestor_record_offset":384,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":413,"record_offset":1008,"parent_record_offset":984,"ancestor_record_offset":384,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":413,"record_offset":5120,"parent_record_offset":5080,"ancestor_record_offset":384,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":413,"record_offset":3620,"parent_record_offset":3588,"ancestor_record_offset":384,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":413,"record_offset":3684,"parent_record_offset":3620,"ancestor_record_offset":384,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":413,"record_offset":8840,"parent_record_offset":8816,"ancestor_record_offset":384,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":413,"record_offset":5624,"parent_record_offset":5596,"ancestor_record_offset":384,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":413,"record_offset":984,"parent_record_offset":756,"ancestor_record_offset":384,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":413,"record_offset":4172,"parent_record_offset":3620,"ancestor_record_offset":384,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":413,"record_offset":6008,"parent_record_offset":5120,"ancestor_record_offset":384,"inlinee":6531,"source_line":13,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":413,"record_offset":5824,"parent_record_offset":5800,"ancestor_record_offset":384,"inlinee":6529,"source_line":30,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":413,"record_offset":5080,"parent_record_offset":5040,"ancestor_record_offset":384,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":413,"record_offset":2900,"parent_record_offset":2872,"ancestor_record_offset":384,"inlinee":4768,"source_line":24,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":413,"record_offset":6464,"parent_record_offset":6132,"ancestor_record_offset":384,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":413,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":384,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":413,"record_offset":6132,"parent_record_offset":6080,"ancestor_record_offset":384,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":413,"record_offset":732,"parent_record_offset":692,"ancestor_record_offset":384,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":413,"record_offset":756,"parent_record_offset":732,"ancestor_record_offset":384,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":413,"record_offset":1388,"parent_record_offset":1364,"ancestor_record_offset":384,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":413,"record_offset":1412,"parent_record_offset":1388,"ancestor_record_offset":384,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":413,"record_offset":7412,"parent_record_offset":7388,"ancestor_record_offset":384,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":413,"record_offset":6936,"parent_record_offset":6080,"ancestor_record_offset":384,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":413,"record_offset":6960,"parent_record_offset":6936,"ancestor_record_offset":384,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":413,"record_offset":7388,"parent_record_offset":6080,"ancestor_record_offset":384,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":413,"record_offset":7436,"parent_record_offset":7412,"ancestor_record_offset":384,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":413,"record_offset":7336,"parent_record_offset":7312,"ancestor_record_offset":384,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":413,"record_offset":7060,"parent_record_offset":7036,"ancestor_record_offset":384,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":413,"record_offset":7036,"parent_record_offset":7012,"ancestor_record_offset":384,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":413,"record_offset":7196,"parent_record_offset":7172,"ancestor_record_offset":384,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":413,"record_offset":7012,"parent_record_offset":6984,"ancestor_record_offset":384,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":413,"record_offset":6984,"parent_record_offset":6960,"ancestor_record_offset":384,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":413,"record_offset":6676,"parent_record_offset":6080,"ancestor_record_offset":384,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":413,"record_offset":7312,"parent_record_offset":7288,"ancestor_record_offset":384,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":413,"record_offset":7288,"parent_record_offset":7264,"ancestor_record_offset":384,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":413,"record_offset":6080,"parent_record_offset":6048,"ancestor_record_offset":384,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":413,"record_offset":692,"parent_record_offset":668,"ancestor_record_offset":384,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":413,"record_offset":7460,"parent_record_offset":7436,"ancestor_record_offset":384,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":413,"record_offset":6524,"parent_record_offset":6496,"ancestor_record_offset":384,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":413,"record_offset":9040,"parent_record_offset":9016,"ancestor_record_offset":384,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":413,"record_offset":9224,"parent_record_offset":9196,"ancestor_record_offset":384,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":413,"record_offset":6432,"parent_record_offset":6408,"ancestor_record_offset":384,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":413,"record_offset":9252,"parent_record_offset":9224,"ancestor_record_offset":384,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":413,"record_offset":4040,"parent_record_offset":4004,"ancestor_record_offset":384,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":413,"record_offset":2628,"parent_record_offset":2604,"ancestor_record_offset":384,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":413,"record_offset":9016,"parent_record_offset":8992,"ancestor_record_offset":384,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":413,"record_offset":2136,"parent_record_offset":2112,"ancestor_record_offset":384,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":413,"record_offset":2220,"parent_record_offset":2196,"ancestor_record_offset":384,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":413,"record_offset":5560,"parent_record_offset":5528,"ancestor_record_offset":384,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":413,"record_offset":2160,"parent_record_offset":2136,"ancestor_record_offset":384,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":413,"record_offset":2696,"parent_record_offset":2000,"ancestor_record_offset":384,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":413,"record_offset":1464,"parent_record_offset":1440,"ancestor_record_offset":384,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":413,"record_offset":1440,"parent_record_offset":1412,"ancestor_record_offset":384,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":413,"record_offset":1544,"parent_record_offset":1520,"ancestor_record_offset":384,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":413,"record_offset":5348,"parent_record_offset":5304,"ancestor_record_offset":384,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":413,"record_offset":5496,"parent_record_offset":5392,"ancestor_record_offset":384,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":413,"record_offset":8968,"parent_record_offset":8944,"ancestor_record_offset":384,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":413,"record_offset":8992,"parent_record_offset":8968,"ancestor_record_offset":384,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":413,"record_offset":9108,"parent_record_offset":9084,"ancestor_record_offset":384,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":413,"record_offset":4004,"parent_record_offset":3968,"ancestor_record_offset":384,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":413,"record_offset":3968,"parent_record_offset":3620,"ancestor_record_offset":384,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":413,"record_offset":8816,"parent_record_offset":8704,"ancestor_record_offset":384,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":413,"record_offset":5304,"parent_record_offset":5120,"ancestor_record_offset":384,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":413,"record_offset":5596,"parent_record_offset":5496,"ancestor_record_offset":384,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":413,"record_offset":940,"parent_record_offset":912,"ancestor_record_offset":384,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":413,"record_offset":800,"parent_record_offset":756,"ancestor_record_offset":384,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":413,"record_offset":3280,"parent_record_offset":3244,"ancestor_record_offset":384,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":413,"record_offset":2368,"parent_record_offset":2340,"ancestor_record_offset":384,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":413,"record_offset":2340,"parent_record_offset":2312,"ancestor_record_offset":384,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":413,"record_offset":6408,"parent_record_offset":6132,"ancestor_record_offset":384,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":413,"record_offset":9492,"parent_record_offset":9368,"ancestor_record_offset":384,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":413,"record_offset":8672,"parent_record_offset":8648,"ancestor_record_offset":384,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":413,"record_offset":9396,"parent_record_offset":9368,"ancestor_record_offset":384,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":413,"record_offset":7108,"parent_record_offset":7084,"ancestor_record_offset":384,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":413,"record_offset":1752,"parent_record_offset":1728,"ancestor_record_offset":384,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":413,"record_offset":9196,"parent_record_offset":9168,"ancestor_record_offset":384,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":413,"record_offset":7084,"parent_record_offset":7060,"ancestor_record_offset":384,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":413,"record_offset":1488,"parent_record_offset":1464,"ancestor_record_offset":384,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":413,"record_offset":7132,"parent_record_offset":7108,"ancestor_record_offset":384,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":413,"record_offset":1728,"parent_record_offset":1364,"ancestor_record_offset":384,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":413,"record_offset":1520,"parent_record_offset":1440,"ancestor_record_offset":384,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":413,"record_offset":3152,"parent_record_offset":3124,"ancestor_record_offset":384,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":413,"record_offset":1256,"parent_record_offset":1232,"ancestor_record_offset":384,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":413,"record_offset":2488,"parent_record_offset":2464,"ancestor_record_offset":384,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":413,"record_offset":1668,"parent_record_offset":1644,"ancestor_record_offset":384,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":413,"record_offset":3460,"parent_record_offset":3412,"ancestor_record_offset":384,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":413,"record_offset":4084,"parent_record_offset":3620,"ancestor_record_offset":384,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":413,"record_offset":8900,"parent_record_offset":8872,"ancestor_record_offset":384,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":413,"record_offset":7528,"parent_record_offset":6080,"ancestor_record_offset":384,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":413,"record_offset":856,"parent_record_offset":756,"ancestor_record_offset":384,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":413,"record_offset":1208,"parent_record_offset":756,"ancestor_record_offset":384,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":413,"record_offset":6232,"parent_record_offset":6204,"ancestor_record_offset":384,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":413,"record_offset":10088,"parent_record_offset":10056,"ancestor_record_offset":384,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":413,"record_offset":6204,"parent_record_offset":6164,"ancestor_record_offset":384,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":413,"record_offset":10120,"parent_record_offset":10088,"ancestor_record_offset":384,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":413,"record_offset":912,"parent_record_offset":884,"ancestor_record_offset":384,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":413,"record_offset":884,"parent_record_offset":856,"ancestor_record_offset":384,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":413,"record_offset":1232,"parent_record_offset":1208,"ancestor_record_offset":384,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":413,"record_offset":1644,"parent_record_offset":1620,"ancestor_record_offset":384,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":413,"record_offset":5392,"parent_record_offset":5348,"ancestor_record_offset":384,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":413,"record_offset":1596,"parent_record_offset":1544,"ancestor_record_offset":384,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":413,"record_offset":1620,"parent_record_offset":1596,"ancestor_record_offset":384,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":413,"record_offset":2652,"parent_record_offset":2628,"ancestor_record_offset":384,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":413,"record_offset":6496,"parent_record_offset":6080,"ancestor_record_offset":384,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":413,"record_offset":6572,"parent_record_offset":6548,"ancestor_record_offset":384,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":413,"record_offset":6596,"parent_record_offset":6572,"ancestor_record_offset":384,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":413,"record_offset":6548,"parent_record_offset":6524,"ancestor_record_offset":384,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":413,"record_offset":2280,"parent_record_offset":2252,"ancestor_record_offset":384,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":413,"record_offset":2312,"parent_record_offset":2280,"ancestor_record_offset":384,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":413,"record_offset":2252,"parent_record_offset":2000,"ancestor_record_offset":384,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":413,"record_offset":2112,"parent_record_offset":2000,"ancestor_record_offset":384,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":413,"record_offset":2724,"parent_record_offset":2000,"ancestor_record_offset":384,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":413,"record_offset":2196,"parent_record_offset":2000,"ancestor_record_offset":384,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":413,"record_offset":2084,"parent_record_offset":2000,"ancestor_record_offset":384,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":413,"record_offset":2604,"parent_record_offset":2580,"ancestor_record_offset":384,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":413,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":384,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":413,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":384,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":413,"record_offset":2580,"parent_record_offset":2412,"ancestor_record_offset":384,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":413,"record_offset":1924,"parent_record_offset":1892,"ancestor_record_offset":384,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":413,"record_offset":2520,"parent_record_offset":2436,"ancestor_record_offset":384,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":413,"record_offset":2464,"parent_record_offset":2436,"ancestor_record_offset":384,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":413,"record_offset":2816,"parent_record_offset":2000,"ancestor_record_offset":384,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":413,"record_offset":2436,"parent_record_offset":2412,"ancestor_record_offset":384,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":413,"record_offset":1976,"parent_record_offset":1952,"ancestor_record_offset":384,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":413,"record_offset":1952,"parent_record_offset":1892,"ancestor_record_offset":384,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":413,"record_offset":2000,"parent_record_offset":1976,"ancestor_record_offset":384,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":413,"record_offset":2056,"parent_record_offset":2000,"ancestor_record_offset":384,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":413,"record_offset":2412,"parent_record_offset":2000,"ancestor_record_offset":384,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":413,"record_offset":7220,"parent_record_offset":7196,"ancestor_record_offset":384,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/init_wall_height_and_arch_segment_height.rs
//     evidence={"module":413,"record_offset":5528,"parent_record_offset":5496,"ancestor_record_offset":384,"inlinee":24778,"source_line":180,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":413,"record_offset":8872,"parent_record_offset":8704,"ancestor_record_offset":384,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/00413.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 180
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x60CA
// 首条位置: {"module":413,"inlinee":24778}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":413,"record_offset":5560,"parent_record_offset":5528,"ancestor_record_offset":5528,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00413.jsonl.gz"}
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
// 完整结构及原始类型记录: init_wall_height_and_arch_segment_height.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: init_wall_height_and_arch_segment_height.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: init_wall_height_and_arch_segment_height.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >

// ------------------------------------------------------------------
// 函数: system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height
// 信息 ID: 235e787c289203a9f107c372
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 579d5c91db6336a8c0cba7d6: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)> > > @ 0x43948c -> 0x78b550 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)> > @ 0x5374d8 -> 0x78b550 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height::closure$1
// 信息 ID: c076fad276deae78d1bdaaa0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height; module=413, inline_record=5528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

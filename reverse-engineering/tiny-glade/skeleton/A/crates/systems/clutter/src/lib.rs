// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/clutter/src/lib.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 37cd3928fbc5921dc74261ce23e8f6cb589afe52b4c9fc075112987cc83270a2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 295
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use system_clutter; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_clutter::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":295,"record_offset":532,"rva":4263456,"va":5372972576}
// 编译实例 RVAs: 0x410e20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$T$u20$as$u20$alloc..vec..spec_from_elem..SpecFromElem$GT$::from_elem
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5941436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372972576,"raw_hex":"e8bca85a00","size":5,"soft_fail":false,"target_va":5378915648,"va":5372974207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400379d0
//     owner=unresolved_namespace; count=5
//     target_aliases=84 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4037568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372972576,"raw_hex":"e84064c2ff","size":5,"soft_fail":false,"target_va":5368936912,"va":5372974475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140037d90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4035474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372972576,"raw_hex":"e86e6cc2ff","size":5,"soft_fail":false,"target_va":5368937872,"va":5372973341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=2
//     target_aliases=16 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-348610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372972576,"raw_hex":"e83eaefaff","size":5,"soft_fail":false,"target_va":5372625024,"va":5372973629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409b0cc0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5895729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372972576,"raw_hex":"e831f65900","size":5,"soft_fail":false,"target_va":5378870464,"va":5372974730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409b6ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5921860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372972576,"raw_hex":"e8445c5a00","size":5,"soft_fail":false,"target_va":5378894528,"va":5372972663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409b9da0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5934714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372972576,"raw_hex":"e87a8e5a00","size":5,"soft_fail":false,"target_va":5378907552,"va":5372972833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409baee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5938960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372972576,"raw_hex":"e8109f5a00","size":5,"soft_fail":false,"target_va":5378911968,"va":5372973003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409be790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5952750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372972576,"raw_hex":"e8eed45a00","size":5,"soft_fail":false,"target_va":5378926480,"va":5372973725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=unresolved_namespace; count=11
//     evidence={"assembly":"\tcallq\t5957488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372972576,"raw_hex":"e870e75a00","size":5,"soft_fail":false,"target_va":5378931376,"va":5372973883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::intern::Interner$LT$T$GT$::intern
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5945760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372972576,"raw_hex":"e8a0b95a00","size":5,"soft_fail":false,"target_va":5378919088,"va":5372973323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoSystemConfigs::chain
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5941022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372972576,"raw_hex":"e81ea75a00","size":5,"soft_fail":false,"target_va":5378914144,"va":5372973117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoSystemConfigs::chain_ignore_deferred
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5942721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372972576,"raw_hex":"e8c1ad5a00","size":5,"soft_fail":false,"target_va":5378916016,"va":5372973290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoSystemConfigs::run_if
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5936172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372972576,"raw_hex":"e82c945a00","size":5,"soft_fail":false,"target_va":5378909168,"va":5372972991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::schedule::Schedules::add_systems
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t5914921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372972576,"raw_hex":"e829415a00","size":5,"soft_fail":false,"target_va":5378887568,"va":5372972642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::World::resource_mut::heb8c2d950f48aee6 (.llvm.7181808648565651538)
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t5908612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372972576,"raw_hex":"e884285a00","size":5,"soft_fail":false,"target_va":5378881232,"va":5372972615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4392612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372972576,"raw_hex":"e8a4064300","size":5,"soft_fail":false,"target_va":5377366224,"va":5372973607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":295,"record_offset":688,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$4::in_set
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/schedule/config.rs
//     evidence={"module":295,"record_offset":1084,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":6385,"source_line":455,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::shape_intersections::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/shape_intersections.rs
//     evidence={"module":295,"record_offset":2032,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":15872,"source_line":8,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision.rs
//     evidence={"module":295,"record_offset":2644,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":15880,"source_line":57,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::impl$9::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision.rs
//     evidence={"module":295,"record_offset":2672,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":15882,"source_line":69,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::clutter::impl$17::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/mod.rs
//     evidence={"module":295,"record_offset":1416,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":15864,"source_line":148,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::clutter::populate_clutter_regions::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/populate_clutter_regions.rs
//     evidence={"module":295,"record_offset":2700,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":15884,"source_line":74,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::clutter::read_bush_positions_from_gpu::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/read_bush_positions_from_gpu.rs
//     evidence={"module":295,"record_offset":2556,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":15876,"source_line":110,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/inter_roof_merlons/detect_inter_roof_merlons.rs
//     evidence={"module":295,"record_offset":2528,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":15874,"source_line":42,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::inter_shape_stitches::detect_intra_shape_corners::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/inter_shape_stitches/detect_intra_shape_corners.rs
//     evidence={"module":295,"record_offset":2004,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":15870,"source_line":30,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::inter_shape_stitches::stitch_bricks::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/inter_shape_stitches/stitch_bricks.rs
//     evidence={"module":295,"record_offset":1976,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":15868,"source_line":136,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":295,"record_offset":1944,"parent_record_offset":1416,"ancestor_record_offset":532,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_systems
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/sub_app.rs
//     evidence={"module":295,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":532,"inlinee":13630,"source_line":180,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":295,"record_offset":764,"parent_record_offset":736,"ancestor_record_offset":532,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":295,"record_offset":736,"parent_record_offset":712,"ancestor_record_offset":532,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/label.rs
//     evidence={"module":295,"record_offset":1124,"parent_record_offset":1084,"ancestor_record_offset":532,"inlinee":6375,"source_line":129,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":295,"record_offset":1724,"parent_record_offset":1696,"ancestor_record_offset":532,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":295,"record_offset":1776,"parent_record_offset":1752,"ancestor_record_offset":532,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":295,"record_offset":1752,"parent_record_offset":1696,"ancestor_record_offset":532,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":295,"record_offset":1800,"parent_record_offset":1776,"ancestor_record_offset":532,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":295,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":532,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":295,"record_offset":1824,"parent_record_offset":1800,"ancestor_record_offset":532,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":295,"record_offset":1884,"parent_record_offset":1452,"ancestor_record_offset":532,"inlinee":5553,"source_line":283,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":295,"record_offset":1452,"parent_record_offset":1416,"ancestor_record_offset":532,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":295,"record_offset":1500,"parent_record_offset":1476,"ancestor_record_offset":532,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":295,"record_offset":1576,"parent_record_offset":1548,"ancestor_record_offset":532,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":295,"record_offset":1696,"parent_record_offset":1548,"ancestor_record_offset":532,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":295,"record_offset":1476,"parent_record_offset":1452,"ancestor_record_offset":532,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":295,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":532,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":295,"record_offset":1628,"parent_record_offset":1600,"ancestor_record_offset":532,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":295,"record_offset":1548,"parent_record_offset":1524,"ancestor_record_offset":532,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":295,"record_offset":1524,"parent_record_offset":1500,"ancestor_record_offset":532,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::clutter::impl$15::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/mod.rs
//     evidence={"module":295,"record_offset":1916,"parent_record_offset":1416,"ancestor_record_offset":532,"inlinee":15866,"source_line":142,"archive":"_pdb/A/modules/00295.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::clutter::read_bush_positions_from_gpu::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/read_bush_positions_from_gpu.rs
//     evidence={"module":295,"record_offset":2584,"parent_record_offset":2556,"ancestor_record_offset":532,"inlinee":15878,"source_line":18,"archive":"_pdb/A/modules/00295.jsonl.gz"}
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
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_clutter::ClutterPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<system_clutter::ClutterPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<system_clutter::ClutterPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<system_clutter::ClutterPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<system_clutter::ClutterPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<system_clutter::ClutterPlugin>", "constructor": "core::ptr::drop_in_place", "arguments": ["system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<system_clutter::ClutterPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<system_clutter::ClutterPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<system_clutter::ClutterPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::app::App::add_plugins<bevy_app::plugin::sealed::PluginMarker,system_clutter::ClutterPlugin>", "constructor": "bevy_app::app::App::add_plugins", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 函数: system_clutter::impl$0::build
// 信息 ID: 467d2d9ac005c5c5f2c019ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

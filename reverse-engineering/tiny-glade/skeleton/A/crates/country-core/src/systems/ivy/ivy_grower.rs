// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/ivy/ivy_grower.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 62cac253aaf02498c5354818abb6fd3714ca8c4fbcc78173d19f3065cb77180d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ivy_grower.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 454, 1308, 1336, 1368
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::ivy_grower
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":454,"record_offset":4104,"rva":6418512,"va":5375127632}
// 编译实例 RVAs: 0x61f050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5515834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e8c6d5abff","size":5,"soft_fail":false,"target_va":5369613520,"va":5375129349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2382973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e883a3dbff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375132680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1812141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e8ada61b00","size":5,"soft_fail":false,"target_va":5376943968,"va":5375131822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d85100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7753977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e8f9507600","size":5,"soft_fail":false,"target_va":5382885632,"va":5375131650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8179988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e814d17c00","size":5,"soft_fail":false,"target_va":5383308144,"va":5375128151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e87b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8813029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e8e5798600","size":5,"soft_fail":false,"target_va":5383945008,"va":5375131974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e88710
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8817492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e8548b8600","size":5,"soft_fail":false,"target_va":5383948048,"va":5375130551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e88f50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8818235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e83b8e8600","size":5,"soft_fail":false,"target_va":5383950160,"va":5375131920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e88fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8819257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e839928600","size":5,"soft_fail":false,"target_va":5383950272,"va":5375131010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e8a0b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8824187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e87ba58600","size":5,"soft_fail":false,"target_va":5383954608,"va":5375130416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e8e830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8840789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e855e68600","size":5,"soft_fail":false,"target_va":5383972912,"va":5375132118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ea8960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8947999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e81f898800","size":5,"soft_fail":false,"target_va":5384079712,"va":5375131708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140eb12f0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8983699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e893148900","size":5,"soft_fail":false,"target_va":5384114928,"va":5375131224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141045340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10638640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e83055a200","size":5,"soft_fail":false,"target_va":5385769792,"va":5375131147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d70
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28465079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e8b757b201","size":5,"soft_fail":false,"target_va":5403594096,"va":5375129012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3824570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e846a4c5ff","size":5,"soft_fail":false,"target_va":5371305232,"va":5375129797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27845256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e888e2a801","size":5,"soft_fail":false,"target_va":5402978688,"va":5375133427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t27845436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e83ce3a801","size":5,"soft_fail":false,"target_va":5402978880,"va":5375133439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27845679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e82fe4a801","size":5,"soft_fail":false,"target_va":5402979152,"va":5375133468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t27845983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e85fe5a801","size":5,"soft_fail":false,"target_va":5402979344,"va":5375133356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e860e3ffff","size":5,"soft_fail":false,"target_va":5375125920,"va":5375133243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27851920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e890fca801","size":5,"soft_fail":false,"target_va":5402980128,"va":5375128203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e877f9ffff","size":5,"soft_fail":false,"target_va":5375126112,"va":5375127780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27852794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e8faffa801","size":5,"soft_fail":false,"target_va":5402980656,"va":5375127857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer::get_direction
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-821198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e83278f3ff","size":5,"soft_fail":false,"target_va":5374309120,"va":5375130313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::ivy::ivy_storage::Ivy::add_point
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-564060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e8a464f7ff","size":5,"soft_fail":false,"target_va":5374569072,"va":5375133127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::ivy::ivy_storage::Ivy::last_segment_point_ids
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-560538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e86672f7ff","size":5,"soft_fail":false,"target_va":5374569216,"va":5375129749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::ivy::wall_coord::WallCoord::into_world_space
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-39072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e86067ffff","size":5,"soft_fail":false,"target_va":5375091968,"va":5375131035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t303227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e87ba00400","size":5,"soft_fail":false,"target_va":5375434976,"va":5375131744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ivy::ivy_grower::closest_segment_index
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e8b9130000","size":5,"soft_fail":false,"target_va":5375134304,"va":5375129250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ivy::ivy_grower::get_wall_double_sided_normal
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e8db070000","size":5,"soft_fail":false,"target_va":5375134976,"va":5375132960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1228535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e8f7be1200","size":5,"soft_fail":false,"target_va":5376356912,"va":5375128372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1232424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e828ce1200","size":5,"soft_fail":false,"target_va":5376360560,"va":5375128131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1228379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375127632,"raw_hex":"e85bbe1200","size":5,"soft_fail":false,"target_va":5376356144,"va":5375127760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":7088,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":454,"record_offset":7032,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":454,"record_offset":10404,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":454,"record_offset":9860,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":454,"record_offset":10664,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":454,"record_offset":11788,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":454,"record_offset":18304,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":454,"record_offset":8764,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::action_set::DigitalActionState::just_pressed
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/action_set.rs
//     evidence={"module":454,"record_offset":7004,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":21894,"source_line":148,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::Ivy::last_point
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":454,"record_offset":8056,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":24193,"source_line":54,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::Ivy::point_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":454,"record_offset":8028,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":23351,"source_line":50,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::Ivy::set_last_point_position
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":454,"record_offset":10144,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":24194,"source_line":58,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::IvyStorage::iter_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":454,"record_offset":7172,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":25002,"source_line":159,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::max_ivy_grow_height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/wall_coord.rs
//     evidence={"module":454,"record_offset":17744,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":25005,"source_line":60,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":454,"record_offset":8792,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":22381,"source_line":33,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ivy::ivy_grower::check_for_wall_jump
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_grower.rs
//     evidence={"module":454,"record_offset":14200,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":26058,"source_line":238,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ivy::ivy_grower::get_growth_point_proposal
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_grower.rs
//     evidence={"module":454,"record_offset":11852,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":26056,"source_line":184,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":454,"record_offset":8328,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":22384,"source_line":115,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":454,"record_offset":18108,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":454,"record_offset":9892,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":454,"record_offset":17876,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":454,"record_offset":5936,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":454,"record_offset":4348,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":454,"record_offset":4924,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$56::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":454,"record_offset":7560,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":8349,"source_line":2427,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":11824,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":22072,"source_line":141,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":454,"record_offset":17816,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":21347,"source_line":205,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::is_within_01_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":454,"record_offset":17716,"parent_record_offset":4104,"ancestor_record_offset":4104,"inlinee":22073,"source_line":216,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":454,"record_offset":17240,"parent_record_offset":17216,"ancestor_record_offset":4104,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":454,"record_offset":17216,"parent_record_offset":17124,"ancestor_record_offset":4104,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::back
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":454,"record_offset":8080,"parent_record_offset":8056,"ancestor_record_offset":4104,"inlinee":24156,"source_line":1565,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":454,"record_offset":8104,"parent_record_offset":8080,"ancestor_record_offset":4104,"inlinee":7687,"source_line":667,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":454,"record_offset":10196,"parent_record_offset":10172,"ancestor_record_offset":4104,"inlinee":21857,"source_line":697,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":454,"record_offset":8248,"parent_record_offset":8104,"ancestor_record_offset":4104,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":454,"record_offset":8140,"parent_record_offset":8104,"ancestor_record_offset":4104,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":454,"record_offset":8168,"parent_record_offset":8140,"ancestor_record_offset":4104,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$13::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":454,"record_offset":10172,"parent_record_offset":10144,"ancestor_record_offset":4104,"inlinee":24158,"source_line":2779,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":454,"record_offset":17152,"parent_record_offset":17124,"ancestor_record_offset":4104,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":454,"record_offset":8284,"parent_record_offset":8248,"ancestor_record_offset":4104,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":454,"record_offset":17124,"parent_record_offset":17100,"ancestor_record_offset":4104,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":454,"record_offset":15552,"parent_record_offset":14200,"ancestor_record_offset":4104,"inlinee":6488,"source_line":263,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":454,"record_offset":15576,"parent_record_offset":15552,"ancestor_record_offset":4104,"inlinee":6490,"source_line":842,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":10560,"parent_record_offset":10520,"ancestor_record_offset":4104,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":14388,"parent_record_offset":14200,"ancestor_record_offset":4104,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":14544,"parent_record_offset":14416,"ancestor_record_offset":4104,"inlinee":8654,"source_line":1815,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":15524,"parent_record_offset":14200,"ancestor_record_offset":4104,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":14604,"parent_record_offset":14544,"ancestor_record_offset":4104,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":10520,"parent_record_offset":10484,"ancestor_record_offset":4104,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":14752,"parent_record_offset":14200,"ancestor_record_offset":4104,"inlinee":6475,"source_line":1653,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":14780,"parent_record_offset":14752,"ancestor_record_offset":4104,"inlinee":6476,"source_line":1679,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":14716,"parent_record_offset":14544,"ancestor_record_offset":4104,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":13120,"parent_record_offset":13028,"ancestor_record_offset":4104,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":15464,"parent_record_offset":15440,"ancestor_record_offset":4104,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":14848,"parent_record_offset":14816,"ancestor_record_offset":4104,"inlinee":6480,"source_line":1657,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":15284,"parent_record_offset":15260,"ancestor_record_offset":4104,"inlinee":6487,"source_line":1707,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::process_loop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":14816,"parent_record_offset":14780,"ancestor_record_offset":4104,"inlinee":6478,"source_line":1727,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":454,"record_offset":7056,"parent_record_offset":7032,"ancestor_record_offset":4104,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":454,"record_offset":10428,"parent_record_offset":10404,"ancestor_record_offset":4104,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":454,"record_offset":10484,"parent_record_offset":10404,"ancestor_record_offset":4104,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":454,"record_offset":6356,"parent_record_offset":6328,"ancestor_record_offset":4104,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":454,"record_offset":6408,"parent_record_offset":6384,"ancestor_record_offset":4104,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":454,"record_offset":6384,"parent_record_offset":6356,"ancestor_record_offset":4104,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":454,"record_offset":6296,"parent_record_offset":6264,"ancestor_record_offset":4104,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":454,"record_offset":6328,"parent_record_offset":6296,"ancestor_record_offset":4104,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":454,"record_offset":6500,"parent_record_offset":6328,"ancestor_record_offset":4104,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":454,"record_offset":6612,"parent_record_offset":6588,"ancestor_record_offset":4104,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":454,"record_offset":14904,"parent_record_offset":14876,"ancestor_record_offset":4104,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":454,"record_offset":9612,"parent_record_offset":9588,"ancestor_record_offset":4104,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":454,"record_offset":7408,"parent_record_offset":7380,"ancestor_record_offset":4104,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":454,"record_offset":9080,"parent_record_offset":9056,"ancestor_record_offset":4104,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":454,"record_offset":7468,"parent_record_offset":7436,"ancestor_record_offset":4104,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":454,"record_offset":12944,"parent_record_offset":12920,"ancestor_record_offset":4104,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":454,"record_offset":16864,"parent_record_offset":14200,"ancestor_record_offset":4104,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":454,"record_offset":9104,"parent_record_offset":9080,"ancestor_record_offset":4104,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":454,"record_offset":15488,"parent_record_offset":15464,"ancestor_record_offset":4104,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":454,"record_offset":15440,"parent_record_offset":14200,"ancestor_record_offset":4104,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":454,"record_offset":15236,"parent_record_offset":14780,"ancestor_record_offset":4104,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":454,"record_offset":6432,"parent_record_offset":6408,"ancestor_record_offset":4104,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::leading_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":454,"record_offset":14504,"parent_record_offset":14476,"ancestor_record_offset":4104,"inlinee":17359,"source_line":148,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":454,"record_offset":17180,"parent_record_offset":17152,"ancestor_record_offset":4104,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":454,"record_offset":8196,"parent_record_offset":8168,"ancestor_record_offset":4104,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":454,"record_offset":4560,"parent_record_offset":4464,"ancestor_record_offset":4104,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":454,"record_offset":9276,"parent_record_offset":9252,"ancestor_record_offset":4104,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":9996,"parent_record_offset":9920,"ancestor_record_offset":4104,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":8476,"parent_record_offset":8452,"ancestor_record_offset":4104,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":8420,"parent_record_offset":8396,"ancestor_record_offset":4104,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$286::neg
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":15888,"parent_record_offset":15856,"ancestor_record_offset":4104,"inlinee":14668,"source_line":694,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":9968,"parent_record_offset":9920,"ancestor_record_offset":4104,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":454,"record_offset":6188,"parent_record_offset":6160,"ancestor_record_offset":4104,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":454,"record_offset":10688,"parent_record_offset":10664,"ancestor_record_offset":4104,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":454,"record_offset":5520,"parent_record_offset":5060,"ancestor_record_offset":4104,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":454,"record_offset":8724,"parent_record_offset":8328,"ancestor_record_offset":4104,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":454,"record_offset":6236,"parent_record_offset":6056,"ancestor_record_offset":4104,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":454,"record_offset":12240,"parent_record_offset":11972,"ancestor_record_offset":4104,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":454,"record_offset":4872,"parent_record_offset":4844,"ancestor_record_offset":4104,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":454,"record_offset":6564,"parent_record_offset":6264,"ancestor_record_offset":4104,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":454,"record_offset":15404,"parent_record_offset":15380,"ancestor_record_offset":4104,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":454,"record_offset":7944,"parent_record_offset":7920,"ancestor_record_offset":4104,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":454,"record_offset":7920,"parent_record_offset":7896,"ancestor_record_offset":4104,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":454,"record_offset":5296,"parent_record_offset":5272,"ancestor_record_offset":4104,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":454,"record_offset":5272,"parent_record_offset":5248,"ancestor_record_offset":4104,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":454,"record_offset":5480,"parent_record_offset":5448,"ancestor_record_offset":4104,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":454,"record_offset":6456,"parent_record_offset":6432,"ancestor_record_offset":4104,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":454,"record_offset":6964,"parent_record_offset":6016,"ancestor_record_offset":4104,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":454,"record_offset":4496,"parent_record_offset":4464,"ancestor_record_offset":4104,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":454,"record_offset":15356,"parent_record_offset":14200,"ancestor_record_offset":4104,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::sort_unstable_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":454,"record_offset":14452,"parent_record_offset":14416,"ancestor_record_offset":4104,"inlinee":17357,"source_line":2967,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":454,"record_offset":13148,"parent_record_offset":13120,"ancestor_record_offset":4104,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":454,"record_offset":4524,"parent_record_offset":4496,"ancestor_record_offset":4104,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":454,"record_offset":13176,"parent_record_offset":13148,"ancestor_record_offset":4104,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":454,"record_offset":15380,"parent_record_offset":15356,"ancestor_record_offset":4104,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":454,"record_offset":5248,"parent_record_offset":5224,"ancestor_record_offset":4104,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":454,"record_offset":5576,"parent_record_offset":5552,"ancestor_record_offset":4104,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":454,"record_offset":5224,"parent_record_offset":5116,"ancestor_record_offset":4104,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::quicksort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\sort.rs
//     evidence={"module":454,"record_offset":14476,"parent_record_offset":14452,"ancestor_record_offset":4104,"inlinee":17358,"source_line":892,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":454,"record_offset":4464,"parent_record_offset":4436,"ancestor_record_offset":4104,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":454,"record_offset":4956,"parent_record_offset":4924,"ancestor_record_offset":4104,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":454,"record_offset":5088,"parent_record_offset":5060,"ancestor_record_offset":4104,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":454,"record_offset":5060,"parent_record_offset":5028,"ancestor_record_offset":4104,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":454,"record_offset":5004,"parent_record_offset":4980,"ancestor_record_offset":4104,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":454,"record_offset":5028,"parent_record_offset":5004,"ancestor_record_offset":4104,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":454,"record_offset":4980,"parent_record_offset":4956,"ancestor_record_offset":4104,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":454,"record_offset":5552,"parent_record_offset":5028,"ancestor_record_offset":4104,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":454,"record_offset":4660,"parent_record_offset":4628,"ancestor_record_offset":4104,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":454,"record_offset":4820,"parent_record_offset":4660,"ancestor_record_offset":4104,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":454,"record_offset":4628,"parent_record_offset":4604,"ancestor_record_offset":4104,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":454,"record_offset":4380,"parent_record_offset":4348,"ancestor_record_offset":4104,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":454,"record_offset":4844,"parent_record_offset":4820,"ancestor_record_offset":4104,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":454,"record_offset":4436,"parent_record_offset":4404,"ancestor_record_offset":4104,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":454,"record_offset":4404,"parent_record_offset":4380,"ancestor_record_offset":4104,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":454,"record_offset":5116,"parent_record_offset":5088,"ancestor_record_offset":4104,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":454,"record_offset":5364,"parent_record_offset":5116,"ancestor_record_offset":4104,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":454,"record_offset":5336,"parent_record_offset":5116,"ancestor_record_offset":4104,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":454,"record_offset":5168,"parent_record_offset":5116,"ancestor_record_offset":4104,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::ivy::ivy_storage::Ivy::last_point
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":454,"record_offset":11972,"parent_record_offset":11852,"ancestor_record_offset":4104,"inlinee":24193,"source_line":54,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::ivy::wall_coord::WallCoord::from_wall_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/wall_coord.rs
//     evidence={"module":454,"record_offset":12892,"parent_record_offset":11852,"ancestor_record_offset":4104,"inlinee":25961,"source_line":24,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":454,"record_offset":9636,"parent_record_offset":9612,"ancestor_record_offset":4104,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":454,"record_offset":13860,"parent_record_offset":13820,"ancestor_record_offset":4104,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::max_ivy_grow_height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/wall_coord.rs
//     evidence={"module":454,"record_offset":13820,"parent_record_offset":11852,"ancestor_record_offset":4104,"inlinee":25005,"source_line":60,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_grower.rs
//     evidence={"module":454,"record_offset":14876,"parent_record_offset":14848,"ancestor_record_offset":4104,"inlinee":26060,"source_line":250,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ivy::ivy_grower::get_ivy_x_direction_relative_to_wall_direction
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_grower.rs
//     evidence={"module":454,"record_offset":12980,"parent_record_offset":11852,"ancestor_record_offset":4104,"inlinee":26057,"source_line":421,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ivy::ivy_grower::get_wall_double_sided_normal
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_grower.rs
//     evidence={"module":454,"record_offset":15668,"parent_record_offset":14200,"ancestor_record_offset":4104,"inlinee":26061,"source_line":401,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_size
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":454,"record_offset":8508,"parent_record_offset":8360,"ancestor_record_offset":4104,"inlinee":22386,"source_line":525,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::grid_coord_to_cell_index
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":454,"record_offset":8628,"parent_record_offset":8328,"ancestor_record_offset":4104,"inlinee":22387,"source_line":508,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":454,"record_offset":14416,"parent_record_offset":14200,"ancestor_record_offset":4104,"inlinee":25037,"source_line":131,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::world_space_to_grid_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":454,"record_offset":8360,"parent_record_offset":8328,"ancestor_record_offset":4104,"inlinee":22385,"source_line":492,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::impl$2::query_with_segment::closure$2
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":454,"record_offset":14632,"parent_record_offset":14544,"ancestor_record_offset":4104,"inlinee":25020,"source_line":144,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":16712,"parent_record_offset":14200,"ancestor_record_offset":4104,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":13788,"parent_record_offset":12980,"ancestor_record_offset":4104,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":16620,"parent_record_offset":14200,"ancestor_record_offset":4104,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":13560,"parent_record_offset":13532,"ancestor_record_offset":4104,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":15712,"parent_record_offset":15668,"ancestor_record_offset":4104,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":8452,"parent_record_offset":8360,"ancestor_record_offset":4104,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":13588,"parent_record_offset":13560,"ancestor_record_offset":4104,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":8540,"parent_record_offset":8360,"ancestor_record_offset":4104,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":8396,"parent_record_offset":8360,"ancestor_record_offset":4104,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":454,"record_offset":16344,"parent_record_offset":14200,"ancestor_record_offset":4104,"inlinee":14686,"source_line":209,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":454,"record_offset":16892,"parent_record_offset":14200,"ancestor_record_offset":4104,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":454,"record_offset":16144,"parent_record_offset":16108,"ancestor_record_offset":4104,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":454,"record_offset":16108,"parent_record_offset":16072,"ancestor_record_offset":4104,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":454,"record_offset":16072,"parent_record_offset":16044,"ancestor_record_offset":4104,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":454,"record_offset":16044,"parent_record_offset":14200,"ancestor_record_offset":4104,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":454,"record_offset":9920,"parent_record_offset":9892,"ancestor_record_offset":4104,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$33::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":454,"record_offset":15856,"parent_record_offset":15668,"ancestor_record_offset":4104,"inlinee":14666,"source_line":1299,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":454,"record_offset":9588,"parent_record_offset":9564,"ancestor_record_offset":4104,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":454,"record_offset":8852,"parent_record_offset":8828,"ancestor_record_offset":4104,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":454,"record_offset":8876,"parent_record_offset":8852,"ancestor_record_offset":4104,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":454,"record_offset":7244,"parent_record_offset":7220,"ancestor_record_offset":4104,"inlinee":6419,"source_line":801,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":454,"record_offset":9564,"parent_record_offset":9344,"ancestor_record_offset":4104,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$63::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":454,"record_offset":7608,"parent_record_offset":7584,"ancestor_record_offset":4104,"inlinee":8353,"source_line":4766,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":454,"record_offset":8908,"parent_record_offset":8876,"ancestor_record_offset":4104,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":454,"record_offset":9476,"parent_record_offset":9452,"ancestor_record_offset":4104,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":454,"record_offset":9452,"parent_record_offset":9344,"ancestor_record_offset":4104,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":454,"record_offset":9392,"parent_record_offset":9368,"ancestor_record_offset":4104,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":454,"record_offset":7896,"parent_record_offset":7660,"ancestor_record_offset":4104,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":454,"record_offset":9772,"parent_record_offset":8984,"ancestor_record_offset":4104,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":454,"record_offset":7352,"parent_record_offset":7292,"ancestor_record_offset":4104,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":454,"record_offset":7660,"parent_record_offset":7632,"ancestor_record_offset":4104,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":454,"record_offset":9368,"parent_record_offset":9344,"ancestor_record_offset":4104,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":454,"record_offset":8960,"parent_record_offset":8936,"ancestor_record_offset":4104,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":454,"record_offset":8936,"parent_record_offset":8876,"ancestor_record_offset":4104,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":454,"record_offset":7268,"parent_record_offset":7244,"ancestor_record_offset":4104,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":454,"record_offset":7324,"parent_record_offset":7292,"ancestor_record_offset":4104,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":454,"record_offset":8984,"parent_record_offset":8960,"ancestor_record_offset":4104,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":454,"record_offset":7292,"parent_record_offset":7268,"ancestor_record_offset":4104,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":454,"record_offset":7728,"parent_record_offset":7700,"ancestor_record_offset":4104,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":454,"record_offset":9252,"parent_record_offset":9224,"ancestor_record_offset":4104,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":454,"record_offset":9312,"parent_record_offset":9196,"ancestor_record_offset":4104,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":454,"record_offset":7700,"parent_record_offset":7660,"ancestor_record_offset":4104,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":454,"record_offset":9028,"parent_record_offset":8984,"ancestor_record_offset":4104,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":454,"record_offset":7632,"parent_record_offset":7608,"ancestor_record_offset":4104,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":454,"record_offset":9344,"parent_record_offset":8984,"ancestor_record_offset":4104,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":454,"record_offset":9056,"parent_record_offset":8984,"ancestor_record_offset":4104,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":454,"record_offset":7380,"parent_record_offset":7352,"ancestor_record_offset":4104,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":454,"record_offset":9140,"parent_record_offset":8984,"ancestor_record_offset":4104,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":454,"record_offset":9680,"parent_record_offset":8984,"ancestor_record_offset":4104,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":454,"record_offset":7436,"parent_record_offset":7380,"ancestor_record_offset":4104,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":454,"record_offset":5976,"parent_record_offset":5936,"ancestor_record_offset":4104,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":454,"record_offset":6100,"parent_record_offset":6056,"ancestor_record_offset":4104,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":454,"record_offset":6264,"parent_record_offset":6056,"ancestor_record_offset":4104,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":454,"record_offset":6536,"parent_record_offset":6264,"ancestor_record_offset":4104,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":454,"record_offset":8828,"parent_record_offset":8792,"ancestor_record_offset":4104,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":454,"record_offset":7220,"parent_record_offset":7196,"ancestor_record_offset":4104,"inlinee":6418,"source_line":553,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":454,"record_offset":7196,"parent_record_offset":7172,"ancestor_record_offset":4104,"inlinee":6439,"source_line":456,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$42::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":454,"record_offset":7584,"parent_record_offset":7560,"ancestor_record_offset":4104,"inlinee":8351,"source_line":2269,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":454,"record_offset":8596,"parent_record_offset":8360,"ancestor_record_offset":4104,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":454,"record_offset":16180,"parent_record_offset":16144,"ancestor_record_offset":4104,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":454,"record_offset":6128,"parent_record_offset":6100,"ancestor_record_offset":4104,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":454,"record_offset":6160,"parent_record_offset":6128,"ancestor_record_offset":4104,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":454,"record_offset":6056,"parent_record_offset":6016,"ancestor_record_offset":4104,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":454,"record_offset":6016,"parent_record_offset":5976,"ancestor_record_offset":4104,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":13060,"parent_record_offset":13028,"ancestor_record_offset":4104,"inlinee":22390,"source_line":519,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":13252,"parent_record_offset":13220,"ancestor_record_offset":4104,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":13220,"parent_record_offset":12980,"ancestor_record_offset":4104,"inlinee":22382,"source_line":562,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":13028,"parent_record_offset":12980,"ancestor_record_offset":4104,"inlinee":22389,"source_line":615,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":9828,"parent_record_offset":8792,"ancestor_record_offset":4104,"inlinee":22072,"source_line":141,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":13088,"parent_record_offset":13060,"ancestor_record_offset":4104,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":13408,"parent_record_offset":13252,"ancestor_record_offset":4104,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":454,"record_offset":12920,"parent_record_offset":11852,"ancestor_record_offset":4104,"inlinee":21347,"source_line":205,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":454,"record_offset":13532,"parent_record_offset":13252,"ancestor_record_offset":4104,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::get_growth_point_proposal
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 184
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x65C8
// 首条位置: {"module":454,"inlinee":26056}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::Ivy::last_point
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":454,"record_offset":11972,"parent_record_offset":11852,"ancestor_record_offset":11852,"inlinee":24193,"source_line":54,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::wall_coord::WallCoord::from_wall_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/wall_coord.rs
//     evidence={"module":454,"record_offset":12892,"parent_record_offset":11852,"ancestor_record_offset":11852,"inlinee":25961,"source_line":24,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::max_ivy_grow_height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/wall_coord.rs
//     evidence={"module":454,"record_offset":13820,"parent_record_offset":11852,"ancestor_record_offset":11852,"inlinee":25005,"source_line":60,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ivy::ivy_grower::get_ivy_x_direction_relative_to_wall_direction
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_grower.rs
//     evidence={"module":454,"record_offset":12980,"parent_record_offset":11852,"ancestor_record_offset":11852,"inlinee":26057,"source_line":421,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":454,"record_offset":12920,"parent_record_offset":11852,"ancestor_record_offset":11852,"inlinee":21347,"source_line":205,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":454,"record_offset":12000,"parent_record_offset":11972,"ancestor_record_offset":11852,"inlinee":24156,"source_line":1565,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":454,"record_offset":12024,"parent_record_offset":12000,"ancestor_record_offset":11852,"inlinee":7687,"source_line":667,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":454,"record_offset":12176,"parent_record_offset":12024,"ancestor_record_offset":11852,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":454,"record_offset":12056,"parent_record_offset":12024,"ancestor_record_offset":11852,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":454,"record_offset":12088,"parent_record_offset":12056,"ancestor_record_offset":11852,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":454,"record_offset":12200,"parent_record_offset":12176,"ancestor_record_offset":11852,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":13120,"parent_record_offset":13028,"ancestor_record_offset":11852,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":454,"record_offset":12944,"parent_record_offset":12920,"ancestor_record_offset":11852,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":454,"record_offset":12120,"parent_record_offset":12088,"ancestor_record_offset":11852,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":13656,"parent_record_offset":13588,"ancestor_record_offset":11852,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":13684,"parent_record_offset":13588,"ancestor_record_offset":11852,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":13624,"parent_record_offset":13588,"ancestor_record_offset":11852,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":454,"record_offset":12240,"parent_record_offset":11972,"ancestor_record_offset":11852,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":454,"record_offset":13148,"parent_record_offset":13120,"ancestor_record_offset":11852,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":454,"record_offset":13176,"parent_record_offset":13148,"ancestor_record_offset":11852,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":454,"record_offset":13860,"parent_record_offset":13820,"ancestor_record_offset":11852,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":13788,"parent_record_offset":12980,"ancestor_record_offset":11852,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":13560,"parent_record_offset":13532,"ancestor_record_offset":11852,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":13588,"parent_record_offset":13560,"ancestor_record_offset":11852,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":13060,"parent_record_offset":13028,"ancestor_record_offset":11852,"inlinee":22390,"source_line":519,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":13252,"parent_record_offset":13220,"ancestor_record_offset":11852,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":13220,"parent_record_offset":12980,"ancestor_record_offset":11852,"inlinee":22382,"source_line":562,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":13028,"parent_record_offset":12980,"ancestor_record_offset":11852,"inlinee":22389,"source_line":615,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":13088,"parent_record_offset":13060,"ancestor_record_offset":11852,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":13408,"parent_record_offset":13252,"ancestor_record_offset":11852,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":454,"record_offset":13532,"parent_record_offset":13252,"ancestor_record_offset":11852,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::check_for_wall_jump
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x65CA
// 首条位置: {"module":454,"inlinee":26058}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":454,"record_offset":15552,"parent_record_offset":14200,"ancestor_record_offset":14200,"inlinee":6488,"source_line":263,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":14388,"parent_record_offset":14200,"ancestor_record_offset":14200,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":15524,"parent_record_offset":14200,"ancestor_record_offset":14200,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":14752,"parent_record_offset":14200,"ancestor_record_offset":14200,"inlinee":6475,"source_line":1653,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":454,"record_offset":16864,"parent_record_offset":14200,"ancestor_record_offset":14200,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":454,"record_offset":15440,"parent_record_offset":14200,"ancestor_record_offset":14200,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":454,"record_offset":17076,"parent_record_offset":14200,"ancestor_record_offset":14200,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":454,"record_offset":15356,"parent_record_offset":14200,"ancestor_record_offset":14200,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ivy::ivy_grower::get_wall_double_sided_normal
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_grower.rs
//     evidence={"module":454,"record_offset":15668,"parent_record_offset":14200,"ancestor_record_offset":14200,"inlinee":26061,"source_line":401,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":454,"record_offset":14416,"parent_record_offset":14200,"ancestor_record_offset":14200,"inlinee":25037,"source_line":131,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":16712,"parent_record_offset":14200,"ancestor_record_offset":14200,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":16620,"parent_record_offset":14200,"ancestor_record_offset":14200,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":16280,"parent_record_offset":14200,"ancestor_record_offset":14200,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":454,"record_offset":16344,"parent_record_offset":14200,"ancestor_record_offset":14200,"inlinee":14686,"source_line":209,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":454,"record_offset":16892,"parent_record_offset":14200,"ancestor_record_offset":14200,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":454,"record_offset":16044,"parent_record_offset":14200,"ancestor_record_offset":14200,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":454,"record_offset":15952,"parent_record_offset":14200,"ancestor_record_offset":14200,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":454,"record_offset":15608,"parent_record_offset":14200,"ancestor_record_offset":14200,"inlinee":21347,"source_line":205,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":454,"record_offset":17240,"parent_record_offset":17216,"ancestor_record_offset":14200,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":454,"record_offset":17216,"parent_record_offset":17124,"ancestor_record_offset":14200,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":454,"record_offset":17152,"parent_record_offset":17124,"ancestor_record_offset":14200,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":454,"record_offset":17124,"parent_record_offset":17100,"ancestor_record_offset":14200,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":454,"record_offset":15576,"parent_record_offset":15552,"ancestor_record_offset":14200,"inlinee":6490,"source_line":842,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":14544,"parent_record_offset":14416,"ancestor_record_offset":14200,"inlinee":8654,"source_line":1815,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":14604,"parent_record_offset":14544,"ancestor_record_offset":14200,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":14780,"parent_record_offset":14752,"ancestor_record_offset":14200,"inlinee":6476,"source_line":1679,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":14716,"parent_record_offset":14544,"ancestor_record_offset":14200,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":15464,"parent_record_offset":15440,"ancestor_record_offset":14200,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":14848,"parent_record_offset":14816,"ancestor_record_offset":14200,"inlinee":6480,"source_line":1657,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":15284,"parent_record_offset":15260,"ancestor_record_offset":14200,"inlinee":6487,"source_line":1707,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::process_loop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":14816,"parent_record_offset":14780,"ancestor_record_offset":14200,"inlinee":6478,"source_line":1727,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":454,"record_offset":14904,"parent_record_offset":14876,"ancestor_record_offset":14200,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":454,"record_offset":15632,"parent_record_offset":15608,"ancestor_record_offset":14200,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":454,"record_offset":14688,"parent_record_offset":14544,"ancestor_record_offset":14200,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":454,"record_offset":15488,"parent_record_offset":15464,"ancestor_record_offset":14200,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":454,"record_offset":15236,"parent_record_offset":14780,"ancestor_record_offset":14200,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::leading_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":454,"record_offset":14504,"parent_record_offset":14476,"ancestor_record_offset":14200,"inlinee":17359,"source_line":148,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":454,"record_offset":17180,"parent_record_offset":17152,"ancestor_record_offset":14200,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":16216,"parent_record_offset":16072,"ancestor_record_offset":14200,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$286::neg
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":15888,"parent_record_offset":15856,"ancestor_record_offset":14200,"inlinee":14668,"source_line":694,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":15768,"parent_record_offset":15740,"ancestor_record_offset":14200,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":454,"record_offset":15260,"parent_record_offset":15236,"ancestor_record_offset":14200,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":454,"record_offset":15404,"parent_record_offset":15380,"ancestor_record_offset":14200,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::sort_unstable_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":454,"record_offset":14452,"parent_record_offset":14416,"ancestor_record_offset":14200,"inlinee":17357,"source_line":2967,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":454,"record_offset":15380,"parent_record_offset":15356,"ancestor_record_offset":14200,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::quicksort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\sort.rs
//     evidence={"module":454,"record_offset":14476,"parent_record_offset":14452,"ancestor_record_offset":14200,"inlinee":17358,"source_line":892,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":454,"record_offset":14932,"parent_record_offset":14904,"ancestor_record_offset":14200,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_grower.rs
//     evidence={"module":454,"record_offset":14876,"parent_record_offset":14848,"ancestor_record_offset":14200,"inlinee":26060,"source_line":250,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::impl$2::query_with_segment::closure$2
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":454,"record_offset":14632,"parent_record_offset":14544,"ancestor_record_offset":14200,"inlinee":25020,"source_line":144,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":15828,"parent_record_offset":15668,"ancestor_record_offset":14200,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":15712,"parent_record_offset":15668,"ancestor_record_offset":14200,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":15740,"parent_record_offset":15668,"ancestor_record_offset":14200,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":454,"record_offset":16144,"parent_record_offset":16108,"ancestor_record_offset":14200,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":454,"record_offset":16108,"parent_record_offset":16072,"ancestor_record_offset":14200,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":454,"record_offset":16072,"parent_record_offset":16044,"ancestor_record_offset":14200,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":454,"record_offset":16916,"parent_record_offset":16892,"ancestor_record_offset":14200,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$33::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":454,"record_offset":15856,"parent_record_offset":15668,"ancestor_record_offset":14200,"inlinee":14666,"source_line":1299,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":454,"record_offset":16180,"parent_record_offset":16144,"ancestor_record_offset":14200,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 250
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x65CC
// 首条位置: {"module":454,"inlinee":26060}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":454,"record_offset":14904,"parent_record_offset":14876,"ancestor_record_offset":14876,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":454,"record_offset":14932,"parent_record_offset":14904,"ancestor_record_offset":14876,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 255
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF08C
// 首条位置: {"module":1308,"inlinee":61580}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1308,"record_offset":361576,"parent_record_offset":361540,"ancestor_record_offset":361540,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1308,"record_offset":361700,"parent_record_offset":361660,"ancestor_record_offset":361540,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_grower.rs
//     evidence={"module":1308,"record_offset":361612,"parent_record_offset":361576,"ancestor_record_offset":361540,"inlinee":61582,"source_line":261,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1308,"record_offset":361636,"parent_record_offset":361612,"ancestor_record_offset":361540,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1308,"record_offset":361728,"parent_record_offset":361660,"ancestor_record_offset":361540,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1308,"record_offset":361660,"parent_record_offset":361636,"ancestor_record_offset":361540,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1308,"record_offset":361764,"parent_record_offset":361728,"ancestor_record_offset":361540,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 261
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF08E
// 首条位置: {"module":1308,"inlinee":61582}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1308,"record_offset":361636,"parent_record_offset":361612,"ancestor_record_offset":361612,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1308,"record_offset":361700,"parent_record_offset":361660,"ancestor_record_offset":361612,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1308,"record_offset":361728,"parent_record_offset":361660,"ancestor_record_offset":361612,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1308,"record_offset":361660,"parent_record_offset":361636,"ancestor_record_offset":361612,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1308,"record_offset":361764,"parent_record_offset":361728,"ancestor_record_offset":361612,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF32B
// 首条位置: {"module":1336,"inlinee":62251}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1336,"record_offset":31212,"parent_record_offset":31184,"ancestor_record_offset":31184,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01336.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::closest_segment_index
// 记录形式: compiled_procedure; instances=1
// 源码行: 331
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":454,"record_offset":18748,"rva":6425184,"va":5375134304}
// 编译实例 RVAs: 0x620a60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27844183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375134304,"raw_hex":"e857dea801","size":5,"soft_fail":false,"target_va":5402979152,"va":5375134964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1805171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375134304,"raw_hex":"e8738b1b00","size":5,"soft_fail":false,"target_va":5376939952,"va":5375134776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":454,"record_offset":19356,"parent_record_offset":18748,"ancestor_record_offset":18748,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":454,"record_offset":18932,"parent_record_offset":18748,"ancestor_record_offset":18748,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":19728,"parent_record_offset":18748,"ancestor_record_offset":18748,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":19528,"parent_record_offset":18748,"ancestor_record_offset":18748,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":19640,"parent_record_offset":18748,"ancestor_record_offset":18748,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":19672,"parent_record_offset":18748,"ancestor_record_offset":18748,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":454,"record_offset":19388,"parent_record_offset":18748,"ancestor_record_offset":18748,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":454,"record_offset":19612,"parent_record_offset":18748,"ancestor_record_offset":18748,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":454,"record_offset":19784,"parent_record_offset":18748,"ancestor_record_offset":18748,"inlinee":21781,"source_line":134,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":454,"record_offset":19068,"parent_record_offset":18748,"ancestor_record_offset":18748,"inlinee":21613,"source_line":149,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":454,"record_offset":19868,"parent_record_offset":19844,"ancestor_record_offset":18748,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":454,"record_offset":19892,"parent_record_offset":19868,"ancestor_record_offset":18748,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":454,"record_offset":19324,"parent_record_offset":19068,"ancestor_record_offset":18748,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":19440,"parent_record_offset":19388,"ancestor_record_offset":18748,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":19496,"parent_record_offset":19388,"ancestor_record_offset":18748,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":19124,"parent_record_offset":19100,"ancestor_record_offset":18748,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":454,"record_offset":19844,"parent_record_offset":19820,"ancestor_record_offset":18748,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":454,"record_offset":19820,"parent_record_offset":19784,"ancestor_record_offset":18748,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":454,"record_offset":19024,"parent_record_offset":18968,"ancestor_record_offset":18748,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":454,"record_offset":18968,"parent_record_offset":18932,"ancestor_record_offset":18748,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":454,"record_offset":19552,"parent_record_offset":19528,"ancestor_record_offset":18748,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":19236,"parent_record_offset":19212,"ancestor_record_offset":18748,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":19100,"parent_record_offset":19068,"ancestor_record_offset":18748,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":454,"record_offset":19576,"parent_record_offset":19552,"ancestor_record_offset":18748,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":454,"record_offset":19212,"parent_record_offset":19068,"ancestor_record_offset":18748,"inlinee":22388,"source_line":80,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::get_wall_double_sided_normal
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 401
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x65CD
// 首条位置: {"module":454,"inlinee":26061}
// 编译实例 RVAs: 0x620d00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1808708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375134976,"raw_hex":"e844991b00","size":5,"soft_fail":false,"target_va":5376943968,"va":5375135255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e88710
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8813013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375134976,"raw_hex":"e8d5798600","size":5,"soft_fail":false,"target_va":5383948048,"va":5375135030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27843981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375134976,"raw_hex":"e88ddda801","size":5,"soft_fail":false,"target_va":5402979344,"va":5375135358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":15828,"parent_record_offset":15668,"ancestor_record_offset":15668,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":15712,"parent_record_offset":15668,"ancestor_record_offset":15668,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":15740,"parent_record_offset":15668,"ancestor_record_offset":15668,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$33::neg
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":454,"record_offset":15856,"parent_record_offset":15668,"ancestor_record_offset":15668,"inlinee":14666,"source_line":1299,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":20608,"parent_record_offset":19940,"ancestor_record_offset":19940,"inlinee":22382,"source_line":562,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":20112,"parent_record_offset":19940,"ancestor_record_offset":19940,"inlinee":22389,"source_line":615,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":20208,"parent_record_offset":20112,"ancestor_record_offset":19940,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":20536,"parent_record_offset":20408,"ancestor_record_offset":19940,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":21064,"parent_record_offset":20968,"ancestor_record_offset":19940,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$286::neg
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":15888,"parent_record_offset":15856,"ancestor_record_offset":15668,"inlinee":14668,"source_line":694,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":15768,"parent_record_offset":15740,"ancestor_record_offset":15668,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":454,"record_offset":20236,"parent_record_offset":20208,"ancestor_record_offset":19940,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":454,"record_offset":20264,"parent_record_offset":20236,"ancestor_record_offset":19940,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":20468,"parent_record_offset":20436,"ancestor_record_offset":19940,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":20436,"parent_record_offset":20408,"ancestor_record_offset":19940,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":20408,"parent_record_offset":20384,"ancestor_record_offset":19940,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":20940,"parent_record_offset":20912,"ancestor_record_offset":19940,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":20384,"parent_record_offset":20360,"ancestor_record_offset":19940,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":20304,"parent_record_offset":20112,"ancestor_record_offset":19940,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":454,"record_offset":20500,"parent_record_offset":20468,"ancestor_record_offset":19940,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":20152,"parent_record_offset":20112,"ancestor_record_offset":19940,"inlinee":22390,"source_line":519,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":20636,"parent_record_offset":20608,"ancestor_record_offset":19940,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":20176,"parent_record_offset":20152,"ancestor_record_offset":19940,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":20788,"parent_record_offset":20636,"ancestor_record_offset":19940,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":454,"record_offset":20912,"parent_record_offset":20636,"ancestor_record_offset":19940,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::normalize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":454,"record_offset":20360,"parent_record_offset":20112,"ancestor_record_offset":19940,"inlinee":22391,"source_line":65,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::get_ivy_x_direction_relative_to_wall_direction
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 421
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x65C9
// 首条位置: {"module":454,"inlinee":26057}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":13788,"parent_record_offset":12980,"ancestor_record_offset":12980,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":13732,"parent_record_offset":12980,"ancestor_record_offset":12980,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":13220,"parent_record_offset":12980,"ancestor_record_offset":12980,"inlinee":22382,"source_line":562,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":13028,"parent_record_offset":12980,"ancestor_record_offset":12980,"inlinee":22389,"source_line":615,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":13120,"parent_record_offset":13028,"ancestor_record_offset":12980,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":13656,"parent_record_offset":13588,"ancestor_record_offset":12980,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":13684,"parent_record_offset":13588,"ancestor_record_offset":12980,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":13624,"parent_record_offset":13588,"ancestor_record_offset":12980,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":454,"record_offset":13148,"parent_record_offset":13120,"ancestor_record_offset":12980,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":454,"record_offset":13176,"parent_record_offset":13148,"ancestor_record_offset":12980,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":13560,"parent_record_offset":13532,"ancestor_record_offset":12980,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":13588,"parent_record_offset":13560,"ancestor_record_offset":12980,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":13060,"parent_record_offset":13028,"ancestor_record_offset":12980,"inlinee":22390,"source_line":519,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":13252,"parent_record_offset":13220,"ancestor_record_offset":12980,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":13088,"parent_record_offset":13060,"ancestor_record_offset":12980,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":13408,"parent_record_offset":13252,"ancestor_record_offset":12980,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":454,"record_offset":13532,"parent_record_offset":13252,"ancestor_record_offset":12980,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::intersect_ivy_growth_w_wall_segment
// 记录形式: compiled_procedure; instances=1
// 源码行: 443
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":454,"record_offset":21300,"rva":6426288,"va":5375135408}
// 编译实例 RVAs: 0x620eb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1808144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375135408,"raw_hex":"e810971b00","size":5,"soft_fail":false,"target_va":5376943968,"va":5375135819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::intersection_aabb_segment2
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9286483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375135408,"raw_hex":"e853b38d00","size":5,"soft_fail":false,"target_va":5384422224,"va":5375135736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":454,"record_offset":22232,"parent_record_offset":21300,"ancestor_record_offset":21300,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":454,"record_offset":21852,"parent_record_offset":21300,"ancestor_record_offset":21300,"inlinee":26062,"source_line":61,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":454,"record_offset":21936,"parent_record_offset":21300,"ancestor_record_offset":21300,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":454,"record_offset":22496,"parent_record_offset":21300,"ancestor_record_offset":21300,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":23528,"parent_record_offset":21300,"ancestor_record_offset":21300,"inlinee":23092,"source_line":413,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":21676,"parent_record_offset":21300,"ancestor_record_offset":21300,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":22380,"parent_record_offset":21300,"ancestor_record_offset":21300,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":21824,"parent_record_offset":21300,"ancestor_record_offset":21300,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":21508,"parent_record_offset":21300,"ancestor_record_offset":21300,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":21620,"parent_record_offset":21300,"ancestor_record_offset":21300,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":21564,"parent_record_offset":21300,"ancestor_record_offset":21300,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":23304,"parent_record_offset":21300,"ancestor_record_offset":21300,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] utils::project_point_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":454,"record_offset":22924,"parent_record_offset":21300,"ancestor_record_offset":21300,"inlinee":21612,"source_line":181,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":454,"record_offset":22520,"parent_record_offset":22496,"ancestor_record_offset":21300,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":454,"record_offset":22024,"parent_record_offset":21936,"ancestor_record_offset":21300,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":454,"record_offset":22712,"parent_record_offset":22656,"ancestor_record_offset":21300,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":454,"record_offset":21968,"parent_record_offset":21936,"ancestor_record_offset":21300,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":454,"record_offset":21996,"parent_record_offset":21936,"ancestor_record_offset":21300,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":454,"record_offset":23124,"parent_record_offset":22924,"ancestor_record_offset":21300,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":454,"record_offset":23092,"parent_record_offset":22952,"ancestor_record_offset":21300,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::recip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":454,"record_offset":22432,"parent_record_offset":22404,"ancestor_record_offset":21300,"inlinee":21457,"source_line":867,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":22460,"parent_record_offset":22404,"ancestor_record_offset":21300,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":21532,"parent_record_offset":21508,"ancestor_record_offset":21300,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":21588,"parent_record_offset":21564,"ancestor_record_offset":21300,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":21644,"parent_record_offset":21620,"ancestor_record_offset":21300,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":454,"record_offset":22604,"parent_record_offset":22496,"ancestor_record_offset":21300,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":454,"record_offset":22804,"parent_record_offset":22604,"ancestor_record_offset":21300,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":454,"record_offset":22568,"parent_record_offset":22544,"ancestor_record_offset":21300,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":454,"record_offset":22544,"parent_record_offset":22520,"ancestor_record_offset":21300,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":21700,"parent_record_offset":21676,"ancestor_record_offset":21300,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":22404,"parent_record_offset":22380,"ancestor_record_offset":21300,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":23180,"parent_record_offset":23156,"ancestor_record_offset":21300,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":22980,"parent_record_offset":22952,"ancestor_record_offset":21300,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":23204,"parent_record_offset":23180,"ancestor_record_offset":21300,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":454,"record_offset":22656,"parent_record_offset":22632,"ancestor_record_offset":21300,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":454,"record_offset":22632,"parent_record_offset":22604,"ancestor_record_offset":21300,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":454,"record_offset":21732,"parent_record_offset":21700,"ancestor_record_offset":21300,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":454,"record_offset":23036,"parent_record_offset":22952,"ancestor_record_offset":21300,"inlinee":22388,"source_line":80,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":454,"record_offset":23156,"parent_record_offset":22924,"ancestor_record_offset":21300,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":454,"record_offset":22952,"parent_record_offset":22924,"ancestor_record_offset":21300,"inlinee":21613,"source_line":149,"archive":"_pdb/A/modules/00454.jsonl.gz"}
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
// 完整结构及原始类型记录: ivy_grower.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ivy_grower.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ivy_grower.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 14; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::ShapesDragMouseMoved>,bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,tuple$<bevy_ecs::event::EventWriter<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::event::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::timberframe_audio::PlayTimberframeSound>,bevy_ecs::event::EventWriter<country_core::systems::wall::shapes::circle::create_circle::CreateCircleCmd>,bevy_ecs::event::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorCaptureCmd> >,bevy_ecs::event::EventWriter<country_core::systems::wall::shapes::shape_placement::DontRecalculateShapePlacement> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::event::EventWriter<country_core::resources::glade_border::MaxDemoElevationReached>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle> >", "tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::ShapesDragMouseMoved>", "bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>", "tuple$<bevy_ecs::event::EventWriter<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::event::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::timberframe_audio::PlayTimberframeSound>,bevy_ecs::event::EventWriter<country_core::systems::wall::shapes::circle::create_circle::CreateCircleCmd>,bevy_ecs::event::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorCaptureCmd> >,bevy_ecs::event::EventWriter<country_core::systems::wall::shapes::shape_placement::DontRecalculateShapePlacement> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >", "bevy_ecs::event::EventWriter<country_core::resources::glade_border::MaxDemoElevationReached>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::ShapesDragMouseMoved>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::ConstructTimberframeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::timberframe_audio::PlayTimberframeSound>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::shapes::circle::create_circle::CreateCircleCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorCaptureCmd> >
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::shapes::shape_placement::DontRecalculateShapePlacement>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::glade_border::MaxDemoElevationReached>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<ref$<alloc::string::String> >,ref$<ref$<alloc::string::String> >)
// 候选参数类型: ["ref$<ref$<alloc::string::String> >", "ref$<ref$<alloc::string::String> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell)
// 候选参数类型: ["country_core::render::pass_builder::draw_builder::DrawBuilder", "ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>", "bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<std::path::PathBuf>,ref$<std::path::PathBuf>)
// 候选参数类型: ["ref$<std::path::PathBuf>", "ref$<std::path::PathBuf>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::check_for_wall_jump
// 信息 ID: b302f3b1747dc991469491f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::ivy_grower; module=454, inline_record=14200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$0
// 信息 ID: ee750378ffda4672454343be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ivy::ivy_grower::check_for_wall_jump", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::ivy_grower; module=454, inline_record=14876; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::ivy_grower; module=454, inline_record=15084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$1
// 信息 ID: 8d9257bdec88de13c9f41322
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::ivy::ivy_grower::check_for_wall_jump", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1> >; module=1308, inline_record=361540; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1> >; module=1308, inline_record=362444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$1::closure$0
// 信息 ID: e6e9224decf85deeade87faa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::ivy::ivy_grower::check_for_wall_jump", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1> >; module=1308, inline_record=361612; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1> >; module=1308, inline_record=362504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$2
// 信息 ID: 570443a62b24ce651a885cea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::ivy::ivy_grower::check_for_wall_jump", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2 <- core::slice::sort::merge_sort<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2>,alloc::slice::stable_sort::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$1<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$2<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$3<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2 <- core::slice::sort::merge_sort<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2>,alloc::slice::stable_sort::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$1<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$2<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$3<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2 <- core::slice::sort::merge_sort<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2>,alloc::slice::stable_sort::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$1<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$2<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$3<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2 <- core::slice::sort::merge_sort<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2>,alloc::slice::stable_sort::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$1<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$2<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$3<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2 <- core::slice::sort::merge_sort<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2>,alloc::slice::stable_sort::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$1<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$2<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$3<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2 <- core::slice::sort::insertion_sort_shift_left<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2>,alloc::slice::stable_sort::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$1<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$2<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$3<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> > >; module=1336, inline_record=31184; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2>,alloc::slice::stable_sort::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$1<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$2<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$3<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> > >; module=1336, inline_record=31316; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2>,alloc::slice::stable_sort::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$1<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$2<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$3<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> > >; module=1336, inline_record=31428; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2>,alloc::slice::stable_sort::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$1<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$2<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$3<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> > >; module=1336, inline_record=32468; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2>,alloc::slice::stable_sort::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$1<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$2<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >,alloc::slice::stable_sort::closure_env$3<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> > >; module=1336, inline_record=32804; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; module=1336, inline_record=108288; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; module=1336, inline_record=108688; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::closest_segment_index
// 信息 ID: 99a8a49a3bc7c0c5207fbd6b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ivy::ivy_spawner::ivy_spawner @ 0x51dc70 -> 0x620a60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_pruner::is_point_still_valid @ 0x5d1e49 -> 0x620a60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_grower::ivy_grower @ 0x61f6a2 -> 0x620a60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::get_growth_point_proposal
// 信息 ID: 216303b67648774023d2694f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::ivy_grower; module=454, inline_record=11852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::get_ivy_x_direction_relative_to_wall_direction
// 信息 ID: 5d18a4b38a082f8c02a52f2b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::ivy_grower; module=454, inline_record=12980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::get_wall_double_sided_normal
// 信息 ID: 01769718bf6f74b3a84e4cb4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ivy::ivy_grower::ivy_grower @ 0x620520 -> 0x620d00 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::ivy_grower; module=454, inline_record=15668; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::intersect_ivy_growth_w_wall_segment
// 信息 ID: 6af5cd0fff7e31cb495d0149
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1> > @ 0xd85191 -> 0x620eb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1> > @ 0xd8528a -> 0x620eb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::ivy_grower
// 信息 ID: fc962ec9423b0d086e476469
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] fed8f54234ea651b12922212: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>)> > @ 0xe653bb -> 0x61f050 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>)> @ 0xf26929 -> 0x61f050 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

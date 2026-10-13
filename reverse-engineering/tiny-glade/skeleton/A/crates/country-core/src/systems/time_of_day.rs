// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/time_of_day.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 27139b6ced3401e620065ee48b5a4f562691641da1b165047d10ceb16bedb528
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: time_of_day.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 403
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_input; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use time; // namespace candidate
//   package identities: [{"name":"time","source":"PDB_registry_path_only","version":"0.3.36"}]
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::time_of_day::time_of_day
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":403,"record_offset":10004,"rva":5600864,"va":5374309984}
// 编译实例 RVAs: 0x557660
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14066c360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1131937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e8a1451100","size":5,"soft_fail":false,"target_va":5375443808,"va":5374311866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d9780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2627030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e8d6152800","size":5,"soft_fail":false,"target_va":5376939904,"va":5374312869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8997639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e8074b8900","size":5,"soft_fail":false,"target_va":5383308144,"va":5374310500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29281456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e8b0ccbe01","size":5,"soft_fail":false,"target_va":5403593984,"va":5374312523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29281373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e85dccbe01","size":5,"soft_fail":false,"target_va":5403594000,"va":5374312622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29281843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e833cebe01","size":5,"soft_fail":false,"target_va":5403594048,"va":5374312200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d50
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29283068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e8fcd2be01","size":5,"soft_fail":false,"target_va":5403594064,"va":5374310991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144da0
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29283205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e885d3be01","size":5,"soft_fail":false,"target_va":5403594144,"va":5374310934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e50
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29281764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e8e4cdbe01","size":5,"soft_fail":false,"target_va":5403594320,"va":5374312551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e60
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29281721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e8b9cdbe01","size":5,"soft_fail":false,"target_va":5403594336,"va":5374312610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28664432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e87062b501","size":5,"soft_fail":false,"target_va":5402978688,"va":5374314251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e806e4ffff","size":5,"soft_fail":false,"target_va":5374306912,"va":5374314069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28669574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e88676b501","size":5,"soft_fail":false,"target_va":5402980128,"va":5374310549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e81bf4ffff","size":5,"soft_fail":false,"target_va":5374307104,"va":5374310144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28670430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e8de79b501","size":5,"soft_fail":false,"target_va":5402980656,"va":5374310221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::directional_light::DirectionalLight::set_direction_towards_light
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t561689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e819920800","size":5,"soft_fail":false,"target_va":5374875392,"va":5374313698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCoreInner::set_parameter
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t363427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e8a38b0500","size":5,"soft_fail":false,"target_va":5374674256,"va":5374310824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::time_of_day::TodSequence::snapshot_at_tod
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1072672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e8205e1000","size":5,"soft_fail":false,"target_va":5375385360,"va":5374312683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::time_of_day::remap_hour_to_fmod_hour
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e867090000","size":5,"soft_fail":false,"target_va":5374314720,"va":5374312308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2046192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e8f0381f00","size":5,"soft_fail":false,"target_va":5376356912,"va":5374310715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2050075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e81b481f00","size":5,"soft_fail":false,"target_va":5376360560,"va":5374310480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2046019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e843381f00","size":5,"soft_fail":false,"target_va":5376356144,"va":5374310120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] suncalc::get_position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11313485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e84da1ac00","size":5,"soft_fail":false,"target_va":5385625968,"va":5374312478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::date::Date::from_ordinal_date
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2682259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e893ed2800","size":5,"soft_fail":false,"target_va":5376994624,"va":5374312360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::offset_date_time::OffsetDateTime::unix_timestamp
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2676811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e84bd82800","size":5,"soft_fail":false,"target_va":5376989216,"va":5374312400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp_mut
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2627362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374309984,"raw_hex":"e822172800","size":5,"soft_fail":false,"target_va":5376940320,"va":5374312953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=16
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":403,"record_offset":13164,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":403,"record_offset":13848,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::last
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":403,"record_offset":14140,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":7511,"source_line":782,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] bevy_input::button_input::ButtonInput::just_pressed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_input/src/button_input.rs
//     evidence={"module":403,"record_offset":14836,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":23184,"source_line":220,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":403,"record_offset":13524,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":403,"record_offset":14948,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":403,"record_offset":15856,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":10010,"source_line":120,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":403,"record_offset":15352,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] country_core::game_config::developer_hotkeys_enabled
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":403,"record_offset":14808,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":23183,"source_line":97,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_parameter
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":403,"record_offset":12868,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":21885,"source_line":294,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::time_of_day::TimeOfDay::hour
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/time_of_day.rs
//     evidence={"module":403,"record_offset":15116,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":23186,"source_line":58,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::time_of_day::suncalc_pos_to_vec
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/time_of_day.rs
//     evidence={"module":403,"record_offset":15472,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":23191,"source_line":330,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::mat3::Mat3::from_axis_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/mat3.rs
//     evidence={"module":403,"record_offset":15624,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":23195,"source_line":197,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_rotation_y
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":403,"record_offset":15740,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":21673,"source_line":169,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":403,"record_offset":16084,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":403,"record_offset":11820,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":403,"record_offset":10232,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":403,"record_offset":10808,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":403,"record_offset":13284,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":403,"record_offset":13092,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":16818,"source_line":467,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp2
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":403,"record_offset":15148,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":6462,"source_line":491,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::rem_euclid
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":403,"record_offset":13620,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":6485,"source_line":363,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] time::primitive_date_time::PrimitiveDateTime::assume_utc
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/time-0.3.36/src/primitive_date_time.rs
//     evidence={"module":403,"record_offset":15388,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":23188,"source_line":491,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":403,"record_offset":13060,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] utils::comparative_space::impl$3::subtract
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":403,"record_offset":13120,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":23181,"source_line":165,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":403,"record_offset":12928,"parent_record_offset":12868,"ancestor_record_offset":10004,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=16
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":403,"record_offset":13192,"parent_record_offset":13164,"ancestor_record_offset":10004,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":403,"record_offset":13960,"parent_record_offset":13928,"ancestor_record_offset":10004,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":403,"record_offset":13928,"parent_record_offset":13880,"ancestor_record_offset":10004,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::last
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":403,"record_offset":14172,"parent_record_offset":14140,"ancestor_record_offset":10004,"inlinee":7512,"source_line":867,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":403,"record_offset":13880,"parent_record_offset":13848,"ancestor_record_offset":10004,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":403,"record_offset":12220,"parent_record_offset":12192,"ancestor_record_offset":10004,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":403,"record_offset":12272,"parent_record_offset":12248,"ancestor_record_offset":10004,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":403,"record_offset":12248,"parent_record_offset":12220,"ancestor_record_offset":10004,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":403,"record_offset":12160,"parent_record_offset":12132,"ancestor_record_offset":10004,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":403,"record_offset":12192,"parent_record_offset":12160,"ancestor_record_offset":10004,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":403,"record_offset":12364,"parent_record_offset":12192,"ancestor_record_offset":10004,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":403,"record_offset":12476,"parent_record_offset":12452,"ancestor_record_offset":10004,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":403,"record_offset":14204,"parent_record_offset":14172,"ancestor_record_offset":10004,"inlinee":8099,"source_line":143,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":403,"record_offset":12296,"parent_record_offset":12272,"ancestor_record_offset":10004,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":403,"record_offset":14004,"parent_record_offset":13960,"ancestor_record_offset":10004,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":403,"record_offset":10444,"parent_record_offset":10348,"ancestor_record_offset":10004,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":403,"record_offset":16116,"parent_record_offset":16084,"ancestor_record_offset":10004,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":403,"record_offset":15880,"parent_record_offset":15856,"ancestor_record_offset":10004,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":403,"record_offset":14232,"parent_record_offset":14204,"ancestor_record_offset":10004,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":403,"record_offset":12056,"parent_record_offset":12028,"ancestor_record_offset":10004,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":403,"record_offset":11404,"parent_record_offset":10944,"ancestor_record_offset":10004,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":403,"record_offset":12104,"parent_record_offset":11928,"ancestor_record_offset":10004,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":403,"record_offset":14068,"parent_record_offset":13960,"ancestor_record_offset":10004,"inlinee":4156,"source_line":1003,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":403,"record_offset":14292,"parent_record_offset":14172,"ancestor_record_offset":10004,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":403,"record_offset":10756,"parent_record_offset":10728,"ancestor_record_offset":10004,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":403,"record_offset":12428,"parent_record_offset":12132,"ancestor_record_offset":10004,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":403,"record_offset":11180,"parent_record_offset":11156,"ancestor_record_offset":10004,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":403,"record_offset":11156,"parent_record_offset":11132,"ancestor_record_offset":10004,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":403,"record_offset":11364,"parent_record_offset":11332,"ancestor_record_offset":10004,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":403,"record_offset":12320,"parent_record_offset":12296,"ancestor_record_offset":10004,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":403,"record_offset":12828,"parent_record_offset":11892,"ancestor_record_offset":10004,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":403,"record_offset":10380,"parent_record_offset":10348,"ancestor_record_offset":10004,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":403,"record_offset":10408,"parent_record_offset":10380,"ancestor_record_offset":10004,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":403,"record_offset":11132,"parent_record_offset":11108,"ancestor_record_offset":10004,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":403,"record_offset":11460,"parent_record_offset":11436,"ancestor_record_offset":10004,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":403,"record_offset":11108,"parent_record_offset":11000,"ancestor_record_offset":10004,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":403,"record_offset":10348,"parent_record_offset":10320,"ancestor_record_offset":10004,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":403,"record_offset":10840,"parent_record_offset":10808,"ancestor_record_offset":10004,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":403,"record_offset":10972,"parent_record_offset":10944,"ancestor_record_offset":10004,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":403,"record_offset":10944,"parent_record_offset":10912,"ancestor_record_offset":10004,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":403,"record_offset":10888,"parent_record_offset":10864,"ancestor_record_offset":10004,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":403,"record_offset":10912,"parent_record_offset":10888,"ancestor_record_offset":10004,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":403,"record_offset":10864,"parent_record_offset":10840,"ancestor_record_offset":10004,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":403,"record_offset":11436,"parent_record_offset":10912,"ancestor_record_offset":10004,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":403,"record_offset":10544,"parent_record_offset":10512,"ancestor_record_offset":10004,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":403,"record_offset":10704,"parent_record_offset":10544,"ancestor_record_offset":10004,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":403,"record_offset":10512,"parent_record_offset":10488,"ancestor_record_offset":10004,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":403,"record_offset":10264,"parent_record_offset":10232,"ancestor_record_offset":10004,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":403,"record_offset":10728,"parent_record_offset":10704,"ancestor_record_offset":10004,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":403,"record_offset":10320,"parent_record_offset":10288,"ancestor_record_offset":10004,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":403,"record_offset":10288,"parent_record_offset":10264,"ancestor_record_offset":10004,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":403,"record_offset":11000,"parent_record_offset":10972,"ancestor_record_offset":10004,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":403,"record_offset":11248,"parent_record_offset":11000,"ancestor_record_offset":10004,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":403,"record_offset":11220,"parent_record_offset":11000,"ancestor_record_offset":10004,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":403,"record_offset":11052,"parent_record_offset":11000,"ancestor_record_offset":10004,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":403,"record_offset":15648,"parent_record_offset":15624,"ancestor_record_offset":10004,"inlinee":21674,"source_line":177,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/set.rs
//     evidence={"module":403,"record_offset":14860,"parent_record_offset":14836,"ancestor_record_offset":10004,"inlinee":8703,"source_line":861,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":403,"record_offset":11856,"parent_record_offset":11820,"ancestor_record_offset":10004,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":403,"record_offset":11968,"parent_record_offset":11928,"ancestor_record_offset":10004,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":403,"record_offset":12132,"parent_record_offset":11928,"ancestor_record_offset":10004,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":403,"record_offset":12400,"parent_record_offset":12132,"ancestor_record_offset":10004,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":403,"record_offset":15696,"parent_record_offset":15672,"ancestor_record_offset":10004,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":403,"record_offset":15672,"parent_record_offset":15648,"ancestor_record_offset":10004,"inlinee":21675,"source_line":915,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] std::f64::impl$0::cos
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f64.rs
//     evidence={"module":403,"record_offset":15508,"parent_record_offset":15472,"ancestor_record_offset":10004,"inlinee":23192,"source_line":736,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] std::f64::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f64.rs
//     evidence={"module":403,"record_offset":15564,"parent_record_offset":15472,"ancestor_record_offset":10004,"inlinee":23193,"source_line":713,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":403,"record_offset":11996,"parent_record_offset":11968,"ancestor_record_offset":10004,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":403,"record_offset":12028,"parent_record_offset":11996,"ancestor_record_offset":10004,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":403,"record_offset":11928,"parent_record_offset":11892,"ancestor_record_offset":10004,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":403,"record_offset":11892,"parent_record_offset":11856,"ancestor_record_offset":10004,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] time::offset_date_time::OffsetDateTime::new_in_offset
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/time-0.3.36/src/offset_date_time.rs
//     evidence={"module":403,"record_offset":15436,"parent_record_offset":15412,"ancestor_record_offset":10004,"inlinee":9429,"source_line":143,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] time::primitive_date_time::PrimitiveDateTime::assume_offset
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/time-0.3.36/src/primitive_date_time.rs
//     evidence={"module":403,"record_offset":15412,"parent_record_offset":15388,"ancestor_record_offset":10004,"inlinee":23189,"source_line":477,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::time_of_day::remap_hour_to_fmod_hour
// 记录形式: compiled_procedure; instances=1
// 源码行: 285
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":403,"record_offset":16552,"rva":5605600,"va":5374314720}
// 编译实例 RVAs: 0x5588e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::fit
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t2629051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374314720,"raw_hex":"e8bb1d2800","size":5,"soft_fail":false,"target_va":5376943840,"va":5374314784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":403,"record_offset":16696,"parent_record_offset":16552,"ancestor_record_offset":16552,"inlinee":10010,"source_line":120,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":403,"record_offset":16776,"parent_record_offset":16736,"ancestor_record_offset":16552,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":403,"record_offset":16812,"parent_record_offset":16776,"ancestor_record_offset":16552,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":403,"record_offset":16736,"parent_record_offset":16696,"ancestor_record_offset":16552,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::time_of_day::suncalc_pos_to_vec
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 330
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5A97
// 首条位置: {"module":403,"inlinee":23191}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f64::impl$0::cos
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f64.rs
//     evidence={"module":403,"record_offset":15508,"parent_record_offset":15472,"ancestor_record_offset":15472,"inlinee":23192,"source_line":736,"archive":"_pdb/A/modules/00403.jsonl.gz"}
//   [pdb_inline_direct] std::f64::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f64.rs
//     evidence={"module":403,"record_offset":15564,"parent_record_offset":15472,"ancestor_record_offset":15472,"inlinee":23193,"source_line":713,"archive":"_pdb/A/modules/00403.jsonl.gz"}
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
// 完整结构及原始类型记录: time_of_day.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: time_of_day.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: time_of_day.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::event::EventReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>", "bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >", "bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>", "bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>", "bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>", "bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>", "bevy_ecs::event::EventReader<country_core::resources::session::NewSessionStartedCmd>", "bevy_ecs::event::EventReader<country_core::systems::save::load_game::LoadingGameComplete>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::session::NewSessionStartedCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::save::load_game::LoadingGameComplete>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>

// ------------------------------------------------------------------
// 函数: country_core::systems::time_of_day::remap_hour_to_fmod_hour
// 信息 ID: 3ee45769abb41f19347c72e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$8::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x3e4aa -> 0x5588e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_tod_sequence_entry::closure$0::closure$0::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x34d197 -> 0x5588e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::time_of_day::time_of_day @ 0x557f74 -> 0x5588e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::time_of_day::suncalc_pos_to_vec
// 信息 ID: d2c4fa146d23a54f69a57d2c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::time_of_day::time_of_day; module=403, inline_record=15472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::time_of_day::time_of_day
// 信息 ID: 817d267dc5f9b444d5bc3374
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0fdf44d867265e3a5759100d: void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::event::EventReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::event::EventReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::event::EventReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>)> > @ 0xa780e -> 0x557660 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::event::EventReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::event::EventReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>)> @ 0x3595f4 -> 0x557660 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/ui_focus_mode_shape_height.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: d452851564d213a76355a2ba74e7a2c9fb33c4e9b44ee6a8935425a08af3f330
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_focus_mode_shape_height.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1302
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
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1302,"record_offset":19140,"rva":13758400,"va":5382467520}
// 编译实例 RVAs: 0xd1efc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: ui_focus_mode_shape_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12854202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382467520,"raw_hex":"e846dc3bff","size":5,"soft_fail":false,"target_va":5369613520,"va":5382467717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce150
//     owner=unresolved_namespace; count=6
//     target_aliases=3 candidates; complete list: ui_focus_mode_shape_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10823384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382467520,"raw_hex":"e828d95aff","size":5,"soft_fail":false,"target_va":5371650384,"va":5382473763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: ui_focus_mode_shape_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9719157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382467520,"raw_hex":"e88bb26bff","size":5,"soft_fail":false,"target_va":5372749696,"va":5382468848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: ui_focus_mode_shape_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9720151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382467520,"raw_hex":"e8a9ae6bff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382469858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405caa20
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: ui_focus_mode_shape_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7687848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382467520,"raw_hex":"e858b18aff","size":5,"soft_fail":false,"target_va":5374781984,"va":5382469827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140685f80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_focus_mode_shape_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6919890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382467520,"raw_hex":"e82e6996ff","size":5,"soft_fail":false,"target_va":5375549312,"va":5382469197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ui_focus_mode_shape_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5524644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382467520,"raw_hex":"e85cb3abff","size":5,"soft_fail":false,"target_va":5376944000,"va":5382468639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: ui_focus_mode_shape_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21123568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382467520,"raw_hex":"e8f0514201","size":5,"soft_fail":false,"target_va":5403593360,"va":5382469787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t20504996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382467520,"raw_hex":"e8a4e13801","size":5,"soft_fail":false,"target_va":5402978144,"va":5382473143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t-4939187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382467520,"raw_hex":"e84da2b4ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5382469070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-11166015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382467520,"raw_hex":"e8c19e55ff","size":5,"soft_fail":false,"target_va":5371304880,"va":5382470890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-4946073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382467520,"raw_hex":"e86787b4ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5382473188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20505860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382467520,"raw_hex":"e804e53801","size":5,"soft_fail":false,"target_va":5402978880,"va":5382473015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20506096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382467520,"raw_hex":"e8f0e53801","size":5,"soft_fail":false,"target_va":5402979152,"va":5382473051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20506336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382467520,"raw_hex":"e8e0e63801","size":5,"soft_fail":false,"target_va":5402979344,"va":5382473003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20505811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382467520,"raw_hex":"e8d3e43801","size":5,"soft_fail":false,"target_va":5402978944,"va":5382473128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::show
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1731691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382467520,"raw_hex":"e86b6c1a00","size":5,"soft_fail":false,"target_va":5384200656,"va":5382468960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::camera::CameraMatrices::eye_side
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t693008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382467520,"raw_hex":"e810930a00","size":5,"soft_fail":false,"target_va":5383161616,"va":5382468603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-5526408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382467520,"raw_hex":"e878acabff","size":5,"soft_fail":false,"target_va":5376942944,"va":5382469347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":23576,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1302,"record_offset":23816,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1302,"record_offset":24372,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":21608,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":21704,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1302,"record_offset":23756,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1302,"record_offset":29600,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1302,"record_offset":21816,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1302,"record_offset":20472,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":26156,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1302,"record_offset":21844,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1302,"record_offset":23452,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":23582,"source_line":126,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":1302,"record_offset":19404,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":22381,"source_line":33,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":1302,"record_offset":29536,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":22964,"source_line":238,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":1302,"record_offset":23928,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":22959,"source_line":248,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":1302,"record_offset":23900,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":24894,"source_line":87,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1302,"record_offset":25396,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":22967,"source_line":102,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::send
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":1302,"record_offset":26128,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":22969,"source_line":24,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugLine::line_width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":1302,"record_offset":23728,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":24410,"source_line":400,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::world_space_to_screen_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":1302,"record_offset":21908,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":23593,"source_line":83,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$7::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1302,"record_offset":24908,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":59541,"source_line":846,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1302,"record_offset":24068,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1302,"record_offset":24040,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1302,"record_offset":23044,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1302,"record_offset":23364,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1302,"record_offset":23276,"parent_record_offset":19140,"ancestor_record_offset":19140,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":23632,"parent_record_offset":23608,"ancestor_record_offset":19140,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":23656,"parent_record_offset":23632,"ancestor_record_offset":19140,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":26308,"parent_record_offset":26284,"ancestor_record_offset":19140,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":24444,"parent_record_offset":24408,"ancestor_record_offset":19140,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":23608,"parent_record_offset":23576,"ancestor_record_offset":19140,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":26284,"parent_record_offset":26228,"ancestor_record_offset":19140,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1302,"record_offset":24408,"parent_record_offset":24372,"ancestor_record_offset":19140,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1302,"record_offset":23864,"parent_record_offset":23840,"ancestor_record_offset":19140,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":26256,"parent_record_offset":26228,"ancestor_record_offset":19140,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":29792,"parent_record_offset":29760,"ancestor_record_offset":19140,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":25692,"parent_record_offset":25664,"ancestor_record_offset":19140,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":25664,"parent_record_offset":25636,"ancestor_record_offset":19140,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":25848,"parent_record_offset":25664,"ancestor_record_offset":19140,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":26228,"parent_record_offset":26204,"ancestor_record_offset":19140,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":25600,"parent_record_offset":25572,"ancestor_record_offset":19140,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":25572,"parent_record_offset":25544,"ancestor_record_offset":19140,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":25516,"parent_record_offset":25488,"ancestor_record_offset":19140,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":25544,"parent_record_offset":25516,"ancestor_record_offset":19140,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":25488,"parent_record_offset":25460,"ancestor_record_offset":19140,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1302,"record_offset":25460,"parent_record_offset":25432,"ancestor_record_offset":19140,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1302,"record_offset":25432,"parent_record_offset":25396,"ancestor_record_offset":19140,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1302,"record_offset":25996,"parent_record_offset":25972,"ancestor_record_offset":19140,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":29760,"parent_record_offset":29720,"ancestor_record_offset":19140,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":29720,"parent_record_offset":29684,"ancestor_record_offset":19140,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":25636,"parent_record_offset":25600,"ancestor_record_offset":19140,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1302,"record_offset":23784,"parent_record_offset":23756,"ancestor_record_offset":19140,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1302,"record_offset":29624,"parent_record_offset":29600,"ancestor_record_offset":19140,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1302,"record_offset":29684,"parent_record_offset":29600,"ancestor_record_offset":19140,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1302,"record_offset":20224,"parent_record_offset":20200,"ancestor_record_offset":19140,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1302,"record_offset":19692,"parent_record_offset":19668,"ancestor_record_offset":19140,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1302,"record_offset":19776,"parent_record_offset":19752,"ancestor_record_offset":19140,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1302,"record_offset":22148,"parent_record_offset":22064,"ancestor_record_offset":19140,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1302,"record_offset":22032,"parent_record_offset":21980,"ancestor_record_offset":19140,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1302,"record_offset":22004,"parent_record_offset":21980,"ancestor_record_offset":19140,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1302,"record_offset":22444,"parent_record_offset":22412,"ancestor_record_offset":19140,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1302,"record_offset":23536,"parent_record_offset":23512,"ancestor_record_offset":19140,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1302,"record_offset":19716,"parent_record_offset":19692,"ancestor_record_offset":19140,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1302,"record_offset":19888,"parent_record_offset":19864,"ancestor_record_offset":19140,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1302,"record_offset":23212,"parent_record_offset":23068,"ancestor_record_offset":19140,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1302,"record_offset":22952,"parent_record_offset":22928,"ancestor_record_offset":19140,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1302,"record_offset":22840,"parent_record_offset":22816,"ancestor_record_offset":19140,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$364::mul_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1302,"record_offset":23008,"parent_record_offset":22984,"ancestor_record_offset":19140,"inlinee":14660,"source_line":886,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1302,"record_offset":24004,"parent_record_offset":23952,"ancestor_record_offset":19140,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1302,"record_offset":23840,"parent_record_offset":23816,"ancestor_record_offset":19140,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1302,"record_offset":23480,"parent_record_offset":23452,"ancestor_record_offset":19140,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1302,"record_offset":25888,"parent_record_offset":25600,"ancestor_record_offset":19140,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":25972,"parent_record_offset":25396,"ancestor_record_offset":19140,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1302,"record_offset":21776,"parent_record_offset":21752,"ancestor_record_offset":19140,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1302,"record_offset":20136,"parent_record_offset":20112,"ancestor_record_offset":19140,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1302,"record_offset":20028,"parent_record_offset":20004,"ancestor_record_offset":19140,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1302,"record_offset":21876,"parent_record_offset":21844,"ancestor_record_offset":19140,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":23684,"parent_record_offset":23656,"ancestor_record_offset":19140,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":29824,"parent_record_offset":29720,"ancestor_record_offset":19140,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1302,"record_offset":21728,"parent_record_offset":21704,"ancestor_record_offset":19140,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1302,"record_offset":21636,"parent_record_offset":21608,"ancestor_record_offset":19140,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1302,"record_offset":21664,"parent_record_offset":21636,"ancestor_record_offset":19140,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1302,"record_offset":21752,"parent_record_offset":21728,"ancestor_record_offset":19140,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1302,"record_offset":26032,"parent_record_offset":25996,"ancestor_record_offset":19140,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1302,"record_offset":26056,"parent_record_offset":26032,"ancestor_record_offset":19140,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1302,"record_offset":26088,"parent_record_offset":25996,"ancestor_record_offset":19140,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1302,"record_offset":20248,"parent_record_offset":20224,"ancestor_record_offset":19140,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::impl$0::max_y::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1302,"record_offset":23512,"parent_record_offset":23480,"ancestor_record_offset":19140,"inlinee":23592,"source_line":127,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::dragged
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":1302,"record_offset":29564,"parent_record_offset":29536,"ancestor_record_offset":19140,"inlinee":22965,"source_line":134,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::intersecting
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":1302,"record_offset":23976,"parent_record_offset":23952,"ancestor_record_offset":19140,"inlinee":22961,"source_line":152,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::is_active
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":1302,"record_offset":23952,"parent_record_offset":23928,"ancestor_record_offset":19140,"inlinee":22960,"source_line":167,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::mul_vec4
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":1302,"record_offset":22380,"parent_record_offset":22356,"ancestor_record_offset":19140,"inlinee":15693,"source_line":1157,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::transform_point3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":1302,"record_offset":21944,"parent_record_offset":21908,"ancestor_record_offset":19140,"inlinee":23594,"source_line":1098,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":1302,"record_offset":22356,"parent_record_offset":21908,"ancestor_record_offset":19140,"inlinee":15692,"source_line":1335,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1302,"record_offset":24932,"parent_record_offset":24908,"ancestor_record_offset":19140,"inlinee":22239,"source_line":675,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$10::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":1302,"record_offset":21980,"parent_record_offset":21944,"ancestor_record_offset":19140,"inlinee":21866,"source_line":975,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$13::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":1302,"record_offset":22296,"parent_record_offset":21944,"ancestor_record_offset":19140,"inlinee":15701,"source_line":998,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1302,"record_offset":24156,"parent_record_offset":24124,"ancestor_record_offset":19140,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1302,"record_offset":24124,"parent_record_offset":24092,"ancestor_record_offset":19140,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1302,"record_offset":24092,"parent_record_offset":24068,"ancestor_record_offset":19140,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1302,"record_offset":22928,"parent_record_offset":21908,"ancestor_record_offset":19140,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1302,"record_offset":22816,"parent_record_offset":21908,"ancestor_record_offset":19140,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$8::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1302,"record_offset":22984,"parent_record_offset":21908,"ancestor_record_offset":19140,"inlinee":23596,"source_line":963,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1302,"record_offset":23140,"parent_record_offset":23104,"ancestor_record_offset":19140,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1302,"record_offset":23104,"parent_record_offset":23068,"ancestor_record_offset":19140,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1302,"record_offset":23068,"parent_record_offset":23044,"ancestor_record_offset":19140,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xxxx
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":1302,"record_offset":22412,"parent_record_offset":22380,"ancestor_record_offset":19140,"inlinee":15695,"source_line":803,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::yyyy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":1302,"record_offset":22504,"parent_record_offset":22380,"ancestor_record_offset":19140,"inlinee":15696,"source_line":1313,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::zzzz
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":1302,"record_offset":22628,"parent_record_offset":22380,"ancestor_record_offset":19140,"inlinee":15698,"source_line":1823,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1302,"record_offset":20200,"parent_record_offset":20176,"ancestor_record_offset":19140,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1302,"record_offset":19464,"parent_record_offset":19440,"ancestor_record_offset":19140,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1302,"record_offset":19488,"parent_record_offset":19464,"ancestor_record_offset":19140,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1302,"record_offset":20176,"parent_record_offset":19956,"ancestor_record_offset":19140,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1302,"record_offset":19520,"parent_record_offset":19488,"ancestor_record_offset":19140,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":20088,"parent_record_offset":20064,"ancestor_record_offset":19140,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":20064,"parent_record_offset":19956,"ancestor_record_offset":19140,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":20004,"parent_record_offset":19980,"ancestor_record_offset":19140,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":20384,"parent_record_offset":19596,"ancestor_record_offset":19140,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":19980,"parent_record_offset":19956,"ancestor_record_offset":19140,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":19572,"parent_record_offset":19548,"ancestor_record_offset":19140,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":19548,"parent_record_offset":19488,"ancestor_record_offset":19140,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":19596,"parent_record_offset":19572,"ancestor_record_offset":19140,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1302,"record_offset":19836,"parent_record_offset":19808,"ancestor_record_offset":19140,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1302,"record_offset":19864,"parent_record_offset":19836,"ancestor_record_offset":19140,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1302,"record_offset":19924,"parent_record_offset":19808,"ancestor_record_offset":19140,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1302,"record_offset":19808,"parent_record_offset":19596,"ancestor_record_offset":19140,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":19640,"parent_record_offset":19596,"ancestor_record_offset":19140,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":19956,"parent_record_offset":19596,"ancestor_record_offset":19140,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1302,"record_offset":19668,"parent_record_offset":19596,"ancestor_record_offset":19140,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1302,"record_offset":19752,"parent_record_offset":19596,"ancestor_record_offset":19140,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1302,"record_offset":20292,"parent_record_offset":19596,"ancestor_record_offset":19140,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1302,"record_offset":19440,"parent_record_offset":19404,"ancestor_record_offset":19140,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1302,"record_offset":23176,"parent_record_offset":23140,"ancestor_record_offset":19140,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1302,"record_offset":20440,"parent_record_offset":19404,"ancestor_record_offset":19140,"inlinee":22072,"source_line":141,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1302,"record_offset":64780,"rva":13776912,"va":5382486032}
// 编译实例 RVAs: 0xd23810
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: ui_focus_mode_shape_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9736404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382486032,"raw_hex":"e82c6f6bff","size":5,"soft_fail":false,"target_va":5372749696,"va":5382486095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_focus_mode_shape_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9736860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382486032,"raw_hex":"e8646d6bff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382486567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140652f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_focus_mode_shape_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7146193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382486032,"raw_hex":"e82ff592ff","size":5,"soft_fail":false,"target_va":5375340432,"va":5382486620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t20491665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382486032,"raw_hex":"e891ad3801","size":5,"soft_fail":false,"target_va":5402978144,"va":5382486474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7129519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382486032,"raw_hex":"e8513693ff","size":5,"soft_fail":false,"target_va":5375356896,"va":5382486410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":64972,"parent_record_offset":64780,"ancestor_record_offset":64780,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":66276,"parent_record_offset":64780,"ancestor_record_offset":64780,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1302,"record_offset":65120,"parent_record_offset":64780,"ancestor_record_offset":64780,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":66360,"parent_record_offset":64780,"ancestor_record_offset":64780,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1302,"record_offset":65676,"parent_record_offset":64780,"ancestor_record_offset":64780,"inlinee":22973,"source_line":143,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1302,"record_offset":66248,"parent_record_offset":64780,"ancestor_record_offset":64780,"inlinee":26370,"source_line":179,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":65028,"parent_record_offset":65004,"ancestor_record_offset":64780,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":65052,"parent_record_offset":65028,"ancestor_record_offset":64780,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":66432,"parent_record_offset":66408,"ancestor_record_offset":64780,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":65004,"parent_record_offset":64972,"ancestor_record_offset":64780,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":66408,"parent_record_offset":66384,"ancestor_record_offset":64780,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1302,"record_offset":66384,"parent_record_offset":66360,"ancestor_record_offset":64780,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":65408,"parent_record_offset":65380,"ancestor_record_offset":64780,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":65380,"parent_record_offset":65352,"ancestor_record_offset":64780,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":65552,"parent_record_offset":65380,"ancestor_record_offset":64780,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":65316,"parent_record_offset":65288,"ancestor_record_offset":64780,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":66300,"parent_record_offset":66276,"ancestor_record_offset":64780,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":65288,"parent_record_offset":65260,"ancestor_record_offset":64780,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":65232,"parent_record_offset":65204,"ancestor_record_offset":64780,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":65260,"parent_record_offset":65232,"ancestor_record_offset":64780,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":65204,"parent_record_offset":65176,"ancestor_record_offset":64780,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1302,"record_offset":65176,"parent_record_offset":65148,"ancestor_record_offset":64780,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1302,"record_offset":65148,"parent_record_offset":65120,"ancestor_record_offset":64780,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":66324,"parent_record_offset":66300,"ancestor_record_offset":64780,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":65352,"parent_record_offset":65316,"ancestor_record_offset":64780,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1302,"record_offset":65612,"parent_record_offset":65588,"ancestor_record_offset":64780,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1302,"record_offset":65588,"parent_record_offset":65316,"ancestor_record_offset":64780,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":65076,"parent_record_offset":65052,"ancestor_record_offset":64780,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 226
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1302,"record_offset":37720,"rva":7036640,"va":5375745760}
// 编译实例 RVAs: 0x6b5ee0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: ui_focus_mode_shape_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2996132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375745760,"raw_hex":"e85c48d2ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375745823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_focus_mode_shape_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2996607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375745760,"raw_hex":"e88146d2ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375746314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140652f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_focus_mode_shape_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-405937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375745760,"raw_hex":"e84fcef9ff","size":5,"soft_fail":false,"target_va":5375340432,"va":5375746364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27231913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375745760,"raw_hex":"e8a9869f01","size":5,"soft_fail":false,"target_va":5402978144,"va":5375746226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-389268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375745760,"raw_hex":"e86c0ffaff","size":5,"soft_fail":false,"target_va":5375356896,"va":5375746159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":37916,"parent_record_offset":37720,"ancestor_record_offset":37720,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":39248,"parent_record_offset":37720,"ancestor_record_offset":37720,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1302,"record_offset":38064,"parent_record_offset":37720,"ancestor_record_offset":37720,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":39332,"parent_record_offset":37720,"ancestor_record_offset":37720,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1302,"record_offset":39220,"parent_record_offset":37720,"ancestor_record_offset":37720,"inlinee":23597,"source_line":201,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1302,"record_offset":38620,"parent_record_offset":37720,"ancestor_record_offset":37720,"inlinee":22973,"source_line":143,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1302,"record_offset":39192,"parent_record_offset":37720,"ancestor_record_offset":37720,"inlinee":26370,"source_line":179,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":37972,"parent_record_offset":37948,"ancestor_record_offset":37720,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":37996,"parent_record_offset":37972,"ancestor_record_offset":37720,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":39404,"parent_record_offset":39380,"ancestor_record_offset":37720,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":37948,"parent_record_offset":37916,"ancestor_record_offset":37720,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":39380,"parent_record_offset":39356,"ancestor_record_offset":37720,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1302,"record_offset":39356,"parent_record_offset":39332,"ancestor_record_offset":37720,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":38352,"parent_record_offset":38324,"ancestor_record_offset":37720,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":38324,"parent_record_offset":38296,"ancestor_record_offset":37720,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":38496,"parent_record_offset":38324,"ancestor_record_offset":37720,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":38260,"parent_record_offset":38232,"ancestor_record_offset":37720,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":39272,"parent_record_offset":39248,"ancestor_record_offset":37720,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":38232,"parent_record_offset":38204,"ancestor_record_offset":37720,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":38176,"parent_record_offset":38148,"ancestor_record_offset":37720,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":38204,"parent_record_offset":38176,"ancestor_record_offset":37720,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":38148,"parent_record_offset":38120,"ancestor_record_offset":37720,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1302,"record_offset":38120,"parent_record_offset":38092,"ancestor_record_offset":37720,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1302,"record_offset":38092,"parent_record_offset":38064,"ancestor_record_offset":37720,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":39296,"parent_record_offset":39272,"ancestor_record_offset":37720,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":38296,"parent_record_offset":38260,"ancestor_record_offset":37720,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1302,"record_offset":38556,"parent_record_offset":38532,"ancestor_record_offset":37720,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1302,"record_offset":38532,"parent_record_offset":38260,"ancestor_record_offset":37720,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":38020,"parent_record_offset":37996,"ancestor_record_offset":37720,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01302.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_focus_mode_shape_height.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_focus_mode_shape_height.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_focus_mode_shape_height.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::EventWriter<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>,bevy_ecs::event::EventWriter<country_core::resources::events::InitAdjWallHeightMode>,bevy_ecs::event::EventWriter<country_core::systems::ui_raise_shape::HandleRaiseShapeUiInteraction>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::event::EventWriter<country_core::resources::events::FinishAdjWallHeightMode>", "bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>", "bevy_ecs::event::EventWriter<country_core::resources::events::InitAdjWallHeightMode>", "bevy_ecs::event::EventWriter<country_core::systems::ui_raise_shape::HandleRaiseShapeUiInteraction>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::FinishAdjWallHeightMode>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::InitAdjWallHeightMode>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::ui_raise_shape::HandleRaiseShapeUiInteraction>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height
// 信息 ID: 05e211846652d88b5ab66522
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8ab1c8a547843ac4230c5f49: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::EventWriter<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>,bevy_ecs::event::EventWriter<country_core::resources::events::InitAdjWallHeightMode>,bevy_ecs::event::EventWriter<country_core::systems::ui_raise_shape::HandleRaiseShapeUiInteraction>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::EventWriter<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>,bevy_ecs::event::EventWriter<country_core::resources::events::InitAdjWallHeightMode>,bevy_ecs::event::EventWriter<country_core::systems::ui_raise_shape::HandleRaiseShapeUiInteraction>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::EventWriter<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>,bevy_ecs::event::EventWriter<country_core::resources::events::InitAdjWallHeightMode>,bevy_ecs::event::EventWriter<country_core::systems::ui_raise_shape::HandleRaiseShapeUiInteraction>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0xe68af6 -> 0xd1efc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::EventWriter<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>,bevy_ecs::event::EventWriter<country_core::resources::events::InitAdjWallHeightMode>,bevy_ecs::event::EventWriter<country_core::systems::ui_raise_shape::HandleRaiseShapeUiInteraction>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::EventWriter<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::event::EventWriter<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>,bevy_ecs::event::EventWriter<country_core::resources::events::InitAdjWallHeightMode>,bevy_ecs::event::EventWriter<country_core::systems::ui_raise_shape::HandleRaiseShapeUiInteraction>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x1080e58 -> 0xd1efc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height::closure$1
// 信息 ID: 8978e8b3080a3fb9d8ed538b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::ui_edit_freehand_walls::ui_edit_freehand_walls::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > | core::ops::function::FnOnce::call_once<country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x6b1ed2 -> 0x6b5ee0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height::closure$2
// 信息 ID: 8c5fb00e1ebe9009d7482de0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height::closure_env$2 <- core::ptr::drop_in_place<country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height::closure_env$5,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xd1e5a2 -> 0xd23810 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

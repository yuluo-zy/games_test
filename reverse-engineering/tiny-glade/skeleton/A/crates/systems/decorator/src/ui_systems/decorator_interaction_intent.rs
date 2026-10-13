// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: de8e04f7a8ee6de9029aadd1d23fad692dfe161a1fefa7dfdad0098b855716aa
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: decorator_interaction_intent.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1286, 2775, 2777
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_ptr; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"PDB_registry_path_only","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1286,"record_offset":1556,"rva":13672608,"va":5382381728}
// 编译实例 RVAs: 0xd0a0a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=1
//     target_aliases=1889 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13661816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e888892fff","size":5,"soft_fail":false,"target_va":5368720672,"va":5382382483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003390
//     owner=unresolved_namespace; count=3
//     target_aliases=1000 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13660180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e8ec8f2fff","size":5,"soft_fail":false,"target_va":5368722320,"va":5382382495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9633146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e886026dff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382382853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14051aa00
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8322519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e8290281ff","size":5,"soft_fail":false,"target_va":5374061056,"va":5382383570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140593250
//     owner=unresolved_namespace; count=2
//     target_aliases=15 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7827637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e84b8f88ff","size":5,"soft_fail":false,"target_va":5374554704,"va":5382382336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140597160
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7811081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e8f7cf88ff","size":5,"soft_fail":false,"target_va":5374570848,"va":5382381924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b9b40
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7669958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e83af78aff","size":5,"soft_fail":false,"target_va":5374712640,"va":5382382593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633130
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7172575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e8218e92ff","size":5,"soft_fail":false,"target_va":5375209776,"va":5382382346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5440933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e85bfaacff","size":5,"soft_fail":false,"target_va":5376944000,"va":5382384928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb4e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-129423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e87106feff","size":5,"soft_fail":false,"target_va":5382255840,"va":5382385258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb560
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-127585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e89f0dfeff","size":5,"soft_fail":false,"target_va":5382255968,"va":5382383548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb6a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-127360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e8800efeff","size":5,"soft_fail":false,"target_va":5382256288,"va":5382383643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cf2e10
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-97983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e84181feff","size":5,"soft_fail":false,"target_va":5382286864,"va":5382384842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cf3280
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-94449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e80f8ffeff","size":5,"soft_fail":false,"target_va":5382288000,"va":5382382444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06720
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e8f3c3ffff","size":5,"soft_fail":false,"target_va":5382367008,"va":5382382376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417d8970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11330825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e809e5ac00","size":5,"soft_fail":false,"target_va":5393713520,"va":5382382690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417da0f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11333273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e899eeac00","size":5,"soft_fail":false,"target_va":5393719536,"va":5382386258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21210787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e8a3a64301","size":5,"soft_fail":false,"target_va":5403593904,"va":5382383112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8345189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e89ba980ff","size":5,"soft_fail":false,"target_va":5374040304,"va":5382385488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::Option::and_then<ref$<country_core::resources::walls::decorator_storage::Decorators>,system_decorator::merge_proposal_roof_decorators::RoofDecoratorMergeProposal,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$1>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e88becffff","size":5,"soft_fail":false,"target_va":5382381376,"va":5382386352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t20592273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e891363a01","size":5,"soft_fail":false,"target_va":5402978880,"va":5382386602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20592764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e87c383a01","size":5,"soft_fail":false,"target_va":5402979344,"va":5382386575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20592248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e878363a01","size":5,"soft_fail":false,"target_va":5402978944,"va":5382386691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20593382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e8e63a3a01","size":5,"soft_fail":false,"target_va":5402980128,"va":5382386741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-6618609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e80f029bff","size":5,"soft_fail":false,"target_va":5375763552,"va":5382382156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorType::can_place_on_terrain
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7812093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e803cc88ff","size":5,"soft_fail":false,"target_va":5374570864,"va":5382382952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorType::max_rank
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7815174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e8fabf88ff","size":5,"soft_fail":false,"target_va":5374570784,"va":5382385953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-449213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e84325f9ff","size":5,"soft_fail":false,"target_va":5381933056,"va":5382382264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6947273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e837fe95ff","size":5,"soft_fail":false,"target_va":5375434976,"va":5382382244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-6945927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e8790396ff","size":5,"soft_fail":false,"target_va":5375436032,"va":5382381954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_attachment::WallCornerAttachment::from_rectangle_corner_id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6948176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e8b0fa95ff","size":5,"soft_fail":false,"target_va":5375436592,"va":5382384763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-8326051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e85df480ff","size":5,"soft_fail":false,"target_va":5374057984,"va":5382384030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::dashed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1814942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e89eb11b00","size":5,"soft_fail":false,"target_va":5384200560,"va":5382385613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::line_width
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1815001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e8d9b11b00","size":5,"soft_fail":false,"target_va":5384200608,"va":5382385602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1814819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e823b11b00","size":5,"soft_fail":false,"target_va":5384200400,"va":5382385576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::show
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1815023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e8efb11b00","size":5,"soft_fail":false,"target_va":5384200656,"va":5382385628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugSphere::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1805971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e8938e1b00","size":5,"soft_fail":false,"target_va":5384190080,"va":5382384104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugSphere::radius
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1806025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e8c98e1b00","size":5,"soft_fail":false,"target_va":5384190160,"va":5382384130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugSphere::show
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1806063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e8ef8e1b00","size":5,"soft_fail":false,"target_va":5384190224,"va":5382384156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugSphere::solid
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1806046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e8de8e1b00","size":5,"soft_fail":false,"target_va":5384190192,"va":5382384141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::cull_oob_decorators::should_snap_to_corner
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8422016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e8807d7fff","size":5,"soft_fail":false,"target_va":5373962736,"va":5382384747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::derive_decorator_subtype::derive_decorator_subtype
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t11325338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e89acfac00","size":5,"soft_fail":false,"target_va":5393707664,"va":5382382321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11331520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e8c0e7ac00","size":5,"soft_fail":false,"target_va":5393714336,"va":5382382811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e87e080000","size":5,"soft_fail":false,"target_va":5382387328,"va":5382385149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-4427668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382381728,"raw_hex":"e86c70bcff","size":5,"soft_fail":false,"target_va":5377955840,"va":5382383503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1286,"record_offset":7372,"parent_record_offset":1556,"ancestor_record_offset":1556,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1286,"record_offset":7456,"parent_record_offset":1556,"ancestor_record_offset":1556,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":1286,"record_offset":1820,"parent_record_offset":1556,"ancestor_record_offset":1556,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1286,"record_offset":1784,"parent_record_offset":1556,"ancestor_record_offset":1556,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1286,"record_offset":7524,"parent_record_offset":1556,"ancestor_record_offset":1556,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1286,"record_offset":7744,"parent_record_offset":1556,"ancestor_record_offset":1556,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$52::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1286,"record_offset":7484,"parent_record_offset":1556,"ancestor_record_offset":1556,"inlinee":59537,"source_line":188,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1286,"record_offset":6580,"parent_record_offset":1556,"ancestor_record_offset":1556,"inlinee":21730,"source_line":261,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1286,"record_offset":5900,"parent_record_offset":1556,"ancestor_record_offset":1556,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1286,"record_offset":7260,"parent_record_offset":1556,"ancestor_record_offset":1556,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::atan2
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1286,"record_offset":6516,"parent_record_offset":1556,"ancestor_record_offset":1556,"inlinee":13077,"source_line":886,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1286,"record_offset":2772,"parent_record_offset":1556,"ancestor_record_offset":1556,"inlinee":60272,"source_line":309,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1286,"record_offset":3684,"parent_record_offset":3660,"ancestor_record_offset":1556,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1286,"record_offset":3660,"parent_record_offset":3560,"ancestor_record_offset":1556,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1286,"record_offset":3592,"parent_record_offset":3560,"ancestor_record_offset":1556,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1286,"record_offset":3560,"parent_record_offset":3532,"ancestor_record_offset":1556,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1286,"record_offset":7396,"parent_record_offset":7372,"ancestor_record_offset":1556,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1286,"record_offset":7420,"parent_record_offset":7396,"ancestor_record_offset":1556,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1286,"record_offset":3412,"parent_record_offset":2772,"ancestor_record_offset":1556,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1286,"record_offset":4568,"parent_record_offset":4512,"ancestor_record_offset":1556,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":1286,"record_offset":2036,"parent_record_offset":1892,"ancestor_record_offset":1556,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":1286,"record_offset":1892,"parent_record_offset":1848,"ancestor_record_offset":1556,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1286,"record_offset":2680,"parent_record_offset":1848,"ancestor_record_offset":1556,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1286,"record_offset":2312,"parent_record_offset":1848,"ancestor_record_offset":1556,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1286,"record_offset":2336,"parent_record_offset":2312,"ancestor_record_offset":1556,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":1286,"record_offset":1848,"parent_record_offset":1820,"ancestor_record_offset":1556,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":1286,"record_offset":2640,"parent_record_offset":2616,"ancestor_record_offset":1556,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1286,"record_offset":2412,"parent_record_offset":2388,"ancestor_record_offset":1556,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1286,"record_offset":2388,"parent_record_offset":2364,"ancestor_record_offset":1556,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1286,"record_offset":2460,"parent_record_offset":2436,"ancestor_record_offset":1556,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1286,"record_offset":2548,"parent_record_offset":2524,"ancestor_record_offset":1556,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1286,"record_offset":2616,"parent_record_offset":2336,"ancestor_record_offset":1556,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1286,"record_offset":2364,"parent_record_offset":2336,"ancestor_record_offset":1556,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1286,"record_offset":2284,"parent_record_offset":1848,"ancestor_record_offset":1556,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":1286,"record_offset":2704,"parent_record_offset":2680,"ancestor_record_offset":1556,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":1286,"record_offset":2100,"parent_record_offset":2068,"ancestor_record_offset":1556,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1286,"record_offset":2004,"parent_record_offset":1980,"ancestor_record_offset":1556,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1286,"record_offset":5412,"parent_record_offset":4996,"ancestor_record_offset":1556,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1286,"record_offset":5320,"parent_record_offset":5264,"ancestor_record_offset":1556,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1286,"record_offset":5092,"parent_record_offset":5068,"ancestor_record_offset":1556,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1286,"record_offset":5124,"parent_record_offset":5040,"ancestor_record_offset":1556,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1286,"record_offset":5184,"parent_record_offset":4996,"ancestor_record_offset":1556,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1286,"record_offset":3344,"parent_record_offset":3320,"ancestor_record_offset":1556,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1286,"record_offset":3624,"parent_record_offset":3592,"ancestor_record_offset":1556,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1286,"record_offset":1980,"parent_record_offset":1892,"ancestor_record_offset":1556,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1286,"record_offset":4896,"parent_record_offset":4836,"ancestor_record_offset":1556,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1286,"record_offset":4924,"parent_record_offset":4836,"ancestor_record_offset":1556,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1286,"record_offset":4868,"parent_record_offset":4836,"ancestor_record_offset":1556,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1286,"record_offset":3440,"parent_record_offset":2772,"ancestor_record_offset":1556,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1286,"record_offset":2484,"parent_record_offset":2460,"ancestor_record_offset":1556,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1286,"record_offset":3728,"parent_record_offset":2772,"ancestor_record_offset":1556,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1286,"record_offset":3752,"parent_record_offset":3728,"ancestor_record_offset":1556,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1286,"record_offset":2436,"parent_record_offset":2412,"ancestor_record_offset":1556,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1286,"record_offset":3032,"parent_record_offset":2772,"ancestor_record_offset":1556,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1286,"record_offset":3132,"parent_record_offset":3104,"ancestor_record_offset":1556,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1286,"record_offset":5644,"parent_record_offset":5620,"ancestor_record_offset":1556,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1286,"record_offset":3508,"parent_record_offset":2772,"ancestor_record_offset":1556,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1286,"record_offset":2208,"parent_record_offset":2176,"ancestor_record_offset":1556,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1286,"record_offset":2256,"parent_record_offset":1848,"ancestor_record_offset":1556,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1286,"record_offset":1924,"parent_record_offset":1892,"ancestor_record_offset":1556,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1286,"record_offset":4596,"parent_record_offset":4568,"ancestor_record_offset":1556,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1286,"record_offset":1948,"parent_record_offset":1924,"ancestor_record_offset":1556,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1286,"record_offset":4624,"parent_record_offset":4596,"ancestor_record_offset":1556,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1286,"record_offset":3196,"parent_record_offset":3104,"ancestor_record_offset":1556,"inlinee":21733,"source_line":102,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1286,"record_offset":3104,"parent_record_offset":3068,"ancestor_record_offset":1556,"inlinee":21731,"source_line":217,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1286,"record_offset":3068,"parent_record_offset":2772,"ancestor_record_offset":1556,"inlinee":21730,"source_line":261,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1286,"record_offset":3164,"parent_record_offset":3132,"ancestor_record_offset":1556,"inlinee":21753,"source_line":220,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1286,"record_offset":3384,"parent_record_offset":2772,"ancestor_record_offset":1556,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1286,"record_offset":2068,"parent_record_offset":1848,"ancestor_record_offset":1556,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1286,"record_offset":2152,"parent_record_offset":2128,"ancestor_record_offset":1556,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1286,"record_offset":2176,"parent_record_offset":2152,"ancestor_record_offset":1556,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1286,"record_offset":2128,"parent_record_offset":2100,"ancestor_record_offset":1556,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1286,"record_offset":4972,"parent_record_offset":2772,"ancestor_record_offset":1556,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1286,"record_offset":4996,"parent_record_offset":4972,"ancestor_record_offset":1556,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1286,"record_offset":5440,"parent_record_offset":4996,"ancestor_record_offset":1556,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1286,"record_offset":4812,"parent_record_offset":4788,"ancestor_record_offset":1556,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1286,"record_offset":4836,"parent_record_offset":4812,"ancestor_record_offset":1556,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":1286,"record_offset":5068,"parent_record_offset":5040,"ancestor_record_offset":1556,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":1286,"record_offset":5040,"parent_record_offset":4996,"ancestor_record_offset":1556,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1286,"record_offset":2572,"parent_record_offset":2548,"ancestor_record_offset":1556,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1286,"record_offset":4100,"parent_record_offset":4032,"ancestor_record_offset":1556,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::collision_detection::DecoratorWorldSpace::project_to_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/collision_detection.rs
//     evidence={"module":1286,"record_offset":3288,"parent_record_offset":2772,"ancestor_record_offset":1556,"inlinee":60274,"source_line":117,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::cull_oob_decorators::should_snap_to_corner
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":1286,"record_offset":4032,"parent_record_offset":2772,"ancestor_record_offset":1556,"inlinee":21750,"source_line":899,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::BalconySnapResult::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1286,"record_offset":5588,"parent_record_offset":2772,"ancestor_record_offset":1556,"inlinee":60278,"source_line":645,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1286,"record_offset":3468,"parent_record_offset":3440,"ancestor_record_offset":1556,"inlinee":60276,"source_line":377,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1286,"record_offset":4512,"parent_record_offset":4484,"ancestor_record_offset":1556,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1286,"record_offset":4484,"parent_record_offset":2772,"ancestor_record_offset":1556,"inlinee":22382,"source_line":562,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1286,"record_offset":4540,"parent_record_offset":4512,"ancestor_record_offset":1556,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1286,"record_offset":4664,"parent_record_offset":4512,"ancestor_record_offset":1556,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1286,"record_offset":3320,"parent_record_offset":3288,"ancestor_record_offset":1556,"inlinee":21347,"source_line":205,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1286,"record_offset":3220,"parent_record_offset":3196,"ancestor_record_offset":1556,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1286,"record_offset":3248,"parent_record_offset":3196,"ancestor_record_offset":1556,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1286,"record_offset":4788,"parent_record_offset":4512,"ancestor_record_offset":1556,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::new
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1286,"record_offset":4004,"parent_record_offset":2772,"ancestor_record_offset":1556,"inlinee":21749,"source_line":106,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::subtract
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1286,"record_offset":4072,"parent_record_offset":4032,"ancestor_record_offset":1556,"inlinee":21751,"source_line":268,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::with_offset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1286,"record_offset":5620,"parent_record_offset":2772,"ancestor_record_offset":1556,"inlinee":21743,"source_line":197,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 219
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F79
// 首条位置: {"module":2777,"inlinee":89977}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2777,"record_offset":3492,"parent_record_offset":3472,"ancestor_record_offset":3472,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":2777,"record_offset":3516,"parent_record_offset":3492,"ancestor_record_offset":3472,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/02777.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 228
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEB6E
// 首条位置: {"module":1286,"inlinee":60270}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 309
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEB70
// 首条位置: {"module":1286,"inlinee":60272}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1286,"record_offset":3412,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1286,"record_offset":3440,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1286,"record_offset":3728,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1286,"record_offset":3032,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1286,"record_offset":3508,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1286,"record_offset":3068,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":21730,"source_line":261,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1286,"record_offset":3384,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1286,"record_offset":4972,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::collision_detection::DecoratorWorldSpace::project_to_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/collision_detection.rs
//     evidence={"module":1286,"record_offset":3288,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":60274,"source_line":117,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::cull_oob_decorators::should_snap_to_corner
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":1286,"record_offset":4032,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":21750,"source_line":899,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::decorator_interaction_intent::BalconySnapResult::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1286,"record_offset":5588,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":60278,"source_line":645,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1286,"record_offset":4484,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":22382,"source_line":562,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1286,"record_offset":4424,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":21347,"source_line":205,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::new
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1286,"record_offset":4004,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":21749,"source_line":106,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::with_offset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1286,"record_offset":5620,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":21743,"source_line":197,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1286,"record_offset":3684,"parent_record_offset":3660,"ancestor_record_offset":2772,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1286,"record_offset":3660,"parent_record_offset":3560,"ancestor_record_offset":2772,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1286,"record_offset":3592,"parent_record_offset":3560,"ancestor_record_offset":2772,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1286,"record_offset":3560,"parent_record_offset":3532,"ancestor_record_offset":2772,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1286,"record_offset":4568,"parent_record_offset":4512,"ancestor_record_offset":2772,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1286,"record_offset":5412,"parent_record_offset":4996,"ancestor_record_offset":2772,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1286,"record_offset":5320,"parent_record_offset":5264,"ancestor_record_offset":2772,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1286,"record_offset":5092,"parent_record_offset":5068,"ancestor_record_offset":2772,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1286,"record_offset":5124,"parent_record_offset":5040,"ancestor_record_offset":2772,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1286,"record_offset":5184,"parent_record_offset":4996,"ancestor_record_offset":2772,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1286,"record_offset":3344,"parent_record_offset":3320,"ancestor_record_offset":2772,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1286,"record_offset":3624,"parent_record_offset":3592,"ancestor_record_offset":2772,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1286,"record_offset":4896,"parent_record_offset":4836,"ancestor_record_offset":2772,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1286,"record_offset":4924,"parent_record_offset":4836,"ancestor_record_offset":2772,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1286,"record_offset":4868,"parent_record_offset":4836,"ancestor_record_offset":2772,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1286,"record_offset":3752,"parent_record_offset":3728,"ancestor_record_offset":2772,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1286,"record_offset":3132,"parent_record_offset":3104,"ancestor_record_offset":2772,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1286,"record_offset":5644,"parent_record_offset":5620,"ancestor_record_offset":2772,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1286,"record_offset":3532,"parent_record_offset":3508,"ancestor_record_offset":2772,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1286,"record_offset":4596,"parent_record_offset":4568,"ancestor_record_offset":2772,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1286,"record_offset":4624,"parent_record_offset":4596,"ancestor_record_offset":2772,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1286,"record_offset":3196,"parent_record_offset":3104,"ancestor_record_offset":2772,"inlinee":21733,"source_line":102,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1286,"record_offset":3104,"parent_record_offset":3068,"ancestor_record_offset":2772,"inlinee":21731,"source_line":217,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1286,"record_offset":3164,"parent_record_offset":3132,"ancestor_record_offset":2772,"inlinee":21753,"source_line":220,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1286,"record_offset":4996,"parent_record_offset":4972,"ancestor_record_offset":2772,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1286,"record_offset":5440,"parent_record_offset":4996,"ancestor_record_offset":2772,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1286,"record_offset":4812,"parent_record_offset":4788,"ancestor_record_offset":2772,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1286,"record_offset":4836,"parent_record_offset":4812,"ancestor_record_offset":2772,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":1286,"record_offset":5068,"parent_record_offset":5040,"ancestor_record_offset":2772,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":1286,"record_offset":5040,"parent_record_offset":4996,"ancestor_record_offset":2772,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1286,"record_offset":4100,"parent_record_offset":4032,"ancestor_record_offset":2772,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1286,"record_offset":3468,"parent_record_offset":3440,"ancestor_record_offset":2772,"inlinee":60276,"source_line":377,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1286,"record_offset":4512,"parent_record_offset":4484,"ancestor_record_offset":2772,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1286,"record_offset":4540,"parent_record_offset":4512,"ancestor_record_offset":2772,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1286,"record_offset":4664,"parent_record_offset":4512,"ancestor_record_offset":2772,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1286,"record_offset":3320,"parent_record_offset":3288,"ancestor_record_offset":2772,"inlinee":21347,"source_line":205,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1286,"record_offset":3220,"parent_record_offset":3196,"ancestor_record_offset":2772,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1286,"record_offset":3248,"parent_record_offset":3196,"ancestor_record_offset":2772,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1286,"record_offset":4788,"parent_record_offset":4512,"ancestor_record_offset":2772,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::subtract
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1286,"record_offset":4072,"parent_record_offset":4032,"ancestor_record_offset":2772,"inlinee":21751,"source_line":268,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 370
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F67
// 首条位置: {"module":2775,"inlinee":89959}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2775,"record_offset":3960,"parent_record_offset":3940,"ancestor_record_offset":3940,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":2775,"record_offset":3984,"parent_record_offset":3960,"ancestor_record_offset":3940,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 377
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEB74
// 首条位置: {"module":1286,"inlinee":60276}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door
// 记录形式: compiled_procedure; instances=1
// 源码行: 509
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1286,"record_offset":8052,"rva":13678208,"va":5382387328}
// 编译实例 RVAs: 0xd0b680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140633130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7177719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382387328,"raw_hex":"e8097a92ff","size":5,"soft_fail":false,"target_va":5375209776,"va":5382387490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c96d80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-477640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382387328,"raw_hex":"e838b6f8ff","size":5,"soft_fail":false,"target_va":5381909888,"va":5382387523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cf3750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-99257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382387328,"raw_hex":"e8477cfeff","size":5,"soft_fail":false,"target_va":5382289232,"va":5382388484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cf3880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-98990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382387328,"raw_hex":"e8527dfeff","size":5,"soft_fail":false,"target_va":5382289536,"va":5382388521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20589932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382387328,"raw_hex":"e86c2d3a01","size":5,"soft_fail":false,"target_va":5402978944,"va":5382389007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-6624574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382387328,"raw_hex":"e8c2ea9aff","size":5,"soft_fail":false,"target_va":5375763552,"va":5382388121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::signed_distance
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4418278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382387328,"raw_hex":"e81a95bcff","size":5,"soft_fail":false,"target_va":5377970096,"va":5382388369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::contains_point_padding
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4428601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382387328,"raw_hex":"e8c76cbcff","size":5,"soft_fail":false,"target_va":5377959696,"va":5382388292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":1286,"record_offset":10052,"parent_record_offset":8052,"ancestor_record_offset":8052,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":1286,"record_offset":9852,"parent_record_offset":8052,"ancestor_record_offset":8052,"inlinee":14606,"source_line":435,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1286,"record_offset":10632,"parent_record_offset":8052,"ancestor_record_offset":8052,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1286,"record_offset":10576,"parent_record_offset":8052,"ancestor_record_offset":8052,"inlinee":10010,"source_line":120,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1286,"record_offset":8248,"parent_record_offset":8052,"ancestor_record_offset":8052,"inlinee":21727,"source_line":199,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1286,"record_offset":10548,"parent_record_offset":8052,"ancestor_record_offset":8052,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1286,"record_offset":9764,"parent_record_offset":8052,"ancestor_record_offset":8052,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1286,"record_offset":9676,"parent_record_offset":8052,"ancestor_record_offset":8052,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1286,"record_offset":9648,"parent_record_offset":8052,"ancestor_record_offset":8052,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1286,"record_offset":10448,"parent_record_offset":10076,"ancestor_record_offset":8052,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1286,"record_offset":10364,"parent_record_offset":10076,"ancestor_record_offset":8052,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":1286,"record_offset":9904,"parent_record_offset":9880,"ancestor_record_offset":8052,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":1286,"record_offset":9928,"parent_record_offset":9904,"ancestor_record_offset":8052,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":1286,"record_offset":10076,"parent_record_offset":10052,"ancestor_record_offset":8052,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":1286,"record_offset":9880,"parent_record_offset":9852,"ancestor_record_offset":8052,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":1286,"record_offset":10412,"parent_record_offset":10388,"ancestor_record_offset":8052,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1286,"record_offset":10388,"parent_record_offset":10364,"ancestor_record_offset":8052,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1286,"record_offset":10336,"parent_record_offset":10076,"ancestor_record_offset":8052,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1286,"record_offset":10472,"parent_record_offset":10448,"ancestor_record_offset":8052,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1286,"record_offset":10252,"parent_record_offset":10076,"ancestor_record_offset":8052,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1286,"record_offset":9192,"parent_record_offset":9164,"ancestor_record_offset":8052,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1286,"record_offset":8564,"parent_record_offset":8536,"ancestor_record_offset":8052,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1286,"record_offset":8660,"parent_record_offset":8632,"ancestor_record_offset":8052,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1286,"record_offset":9608,"parent_record_offset":9584,"ancestor_record_offset":8052,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1286,"record_offset":8592,"parent_record_offset":8564,"ancestor_record_offset":8052,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1286,"record_offset":8808,"parent_record_offset":8780,"ancestor_record_offset":8052,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1286,"record_offset":9704,"parent_record_offset":9676,"ancestor_record_offset":8052,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1286,"record_offset":9792,"parent_record_offset":9764,"ancestor_record_offset":8052,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1286,"record_offset":10600,"parent_record_offset":10576,"ancestor_record_offset":8052,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1286,"record_offset":9560,"parent_record_offset":8248,"ancestor_record_offset":8052,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1286,"record_offset":10000,"parent_record_offset":9976,"ancestor_record_offset":8052,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1286,"record_offset":9092,"parent_record_offset":9064,"ancestor_record_offset":8052,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1286,"record_offset":8968,"parent_record_offset":8940,"ancestor_record_offset":8052,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1286,"record_offset":10216,"parent_record_offset":10192,"ancestor_record_offset":8052,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1286,"record_offset":10164,"parent_record_offset":10128,"ancestor_record_offset":8052,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1286,"record_offset":10276,"parent_record_offset":10252,"ancestor_record_offset":8052,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1286,"record_offset":9952,"parent_record_offset":9928,"ancestor_record_offset":8052,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1286,"record_offset":10300,"parent_record_offset":10276,"ancestor_record_offset":8052,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1286,"record_offset":9976,"parent_record_offset":9952,"ancestor_record_offset":8052,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1286,"record_offset":10192,"parent_record_offset":10128,"ancestor_record_offset":8052,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1286,"record_offset":10128,"parent_record_offset":10076,"ancestor_record_offset":8052,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$21::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1286,"record_offset":9220,"parent_record_offset":9192,"ancestor_record_offset":8052,"inlinee":26328,"source_line":78,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1286,"record_offset":9584,"parent_record_offset":9560,"ancestor_record_offset":8052,"inlinee":21755,"source_line":202,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1286,"record_offset":9164,"parent_record_offset":9136,"ancestor_record_offset":8052,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1286,"record_offset":8304,"parent_record_offset":8280,"ancestor_record_offset":8052,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1286,"record_offset":8328,"parent_record_offset":8304,"ancestor_record_offset":8052,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1286,"record_offset":9136,"parent_record_offset":8884,"ancestor_record_offset":8052,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1286,"record_offset":8356,"parent_record_offset":8328,"ancestor_record_offset":8052,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1286,"record_offset":9036,"parent_record_offset":9008,"ancestor_record_offset":8052,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1286,"record_offset":9008,"parent_record_offset":8884,"ancestor_record_offset":8052,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1286,"record_offset":8940,"parent_record_offset":8912,"ancestor_record_offset":8052,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1286,"record_offset":9384,"parent_record_offset":8436,"ancestor_record_offset":8052,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1286,"record_offset":8912,"parent_record_offset":8884,"ancestor_record_offset":8052,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1286,"record_offset":8408,"parent_record_offset":8384,"ancestor_record_offset":8052,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1286,"record_offset":8384,"parent_record_offset":8328,"ancestor_record_offset":8052,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1286,"record_offset":8436,"parent_record_offset":8408,"ancestor_record_offset":8052,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1286,"record_offset":8736,"parent_record_offset":8696,"ancestor_record_offset":8052,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1286,"record_offset":8780,"parent_record_offset":8736,"ancestor_record_offset":8052,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1286,"record_offset":8852,"parent_record_offset":8696,"ancestor_record_offset":8052,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1286,"record_offset":8696,"parent_record_offset":8436,"ancestor_record_offset":8052,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1286,"record_offset":8508,"parent_record_offset":8436,"ancestor_record_offset":8052,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1286,"record_offset":8884,"parent_record_offset":8436,"ancestor_record_offset":8052,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1286,"record_offset":8536,"parent_record_offset":8436,"ancestor_record_offset":8052,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1286,"record_offset":8632,"parent_record_offset":8436,"ancestor_record_offset":8052,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1286,"record_offset":9268,"parent_record_offset":8436,"ancestor_record_offset":8052,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1286,"record_offset":8280,"parent_record_offset":8248,"ancestor_record_offset":8052,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::BalconySnapResult::get
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 645
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEB76
// 首条位置: {"module":1286,"inlinee":60278}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: decorator_interaction_intent.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_decorator::ui_systems::decorator_interaction_intent::BalconySnapResult
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: decorator_interaction_intent.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: decorator_interaction_intent.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::BalconySnapResult::get
// 信息 ID: 9decb0bf1b2ebf22b9b0145c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1286, inline_record=5588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent
// 信息 ID: c3ff8eb6c08bec899121b19c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x1c6e0ee -> 0xd0a0a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$0
// 信息 ID: 28f88948ad164cf0868f79b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators::closure_env$0<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$0 <- system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators::closure_env$0<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$0> >; module=2777, inline_record=3472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$1
// 信息 ID: 68c1f0a3f20d2e7e75dab293
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$1 <- core::option::Option::and_then<ref$<country_core::resources::walls::decorator_storage::Decorators>,system_decorator::merge_proposal_roof_decorators::RoofDecoratorMergeProposal,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::option::Option::and_then<ref$<country_core::resources::walls::decorator_storage::Decorators>,system_decorator::merge_proposal_roof_decorators::RoofDecoratorMergeProposal,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$1>; module=1286, inline_record=1044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent
// 信息 ID: f0669a3f066a7d2189f54a26
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1286, inline_record=2772; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$0
// 信息 ID: f915e76c4bae3fb83b2b745e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0 <- system_decorator::collision_detection::intersect_wall_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; module=2775, inline_record=3940; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; module=2775, inline_record=5204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$1
// 信息 ID: 2b67ba6617520112f23ff390
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1286, inline_record=3468; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door
// 信息 ID: b2bf50702e79189c59a38037
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x501be4 -> 0xd0b680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0adfd -> 0xd0b680 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

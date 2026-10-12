// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/startup/startup_water.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 0522f03d9a0860d35610f3f153af472365ec3abe4dc875f468632250e16a130d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: startup_water.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 526
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_water::startup_water
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":526,"record_offset":45184,"rva":10429648,"va":5379138768}
// 编译实例 RVAs: 0x9f24d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14096eb80
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: startup_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-543209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379138768,"raw_hex":"e817b6f7ff","size":5,"soft_fail":false,"target_va":5378599808,"va":5379143012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: startup_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2437829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379138768,"raw_hex":"e8c5322500","size":5,"soft_fail":false,"target_va":5381577744,"va":5379139910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162d150
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: startup_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12822257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379138768,"raw_hex":"e8f1a6c300","size":5,"soft_fail":false,"target_va":5391962448,"va":5379140186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162f990
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12833558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379138768,"raw_hex":"e816d3c300","size":5,"soft_fail":false,"target_va":5391972752,"va":5379139189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416305c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12833385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379138768,"raw_hex":"e869d2c300","size":5,"soft_fail":false,"target_va":5391975872,"va":5379142482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416318f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12841433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379138768,"raw_hex":"e8d9f1c300","size":5,"soft_fail":false,"target_va":5391980784,"va":5379139346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141aed3d0
//     owner=unresolved_namespace; count=7
//     target_aliases=3 candidates; complete list: startup_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17804958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379138768,"raw_hex":"e89eae0f01","size":5,"soft_fail":false,"target_va":5396943824,"va":5379138861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b2cae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18061349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379138768,"raw_hex":"e825981301","size":5,"soft_fail":false,"target_va":5397203680,"va":5379142326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b2d1e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18065495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379138768,"raw_hex":"e857a81301","size":5,"soft_fail":false,"target_va":5397205472,"va":5379139972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b2dd20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18067784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379138768,"raw_hex":"e848b11301","size":5,"soft_fail":false,"target_va":5397208352,"va":5379140563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b2dfe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18067788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379138768,"raw_hex":"e84cb11301","size":5,"soft_fail":false,"target_va":5397209056,"va":5379141263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5481008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379138768,"raw_hex":"e830a25300","size":5,"soft_fail":false,"target_va":5384624080,"va":5379143067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t37478733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379138768,"raw_hex":"e84de13b02","size":5,"soft_fail":false,"target_va":5416621600,"va":5379142862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t37480695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379138768,"raw_hex":"e8f7e83b02","size":5,"soft_fail":false,"target_va":5416621632,"va":5379140932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_world_raster_pipelines
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t382691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379138768,"raw_hex":"e8e3d60500","size":5,"soft_fail":false,"target_va":5379521760,"va":5379139064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t138386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379138768,"raw_hex":"e8921c0200","size":5,"soft_fail":false,"target_va":5379278928,"va":5379140537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mesh_into_library
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t510641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379138768,"raw_hex":"e8b1ca0700","size":5,"soft_fail":false,"target_va":5379649664,"va":5379139018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4184213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379138768,"raw_hex":"e895d83f00","size":5,"soft_fail":false,"target_va":5383324736,"va":5379140518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5379138768,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5379139943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":526,"record_offset":45716,"parent_record_offset":45184,"ancestor_record_offset":45184,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":526,"record_offset":45344,"parent_record_offset":45184,"ancestor_record_offset":45184,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":526,"record_offset":45656,"parent_record_offset":45184,"ancestor_record_offset":45184,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":526,"record_offset":45920,"parent_record_offset":45184,"ancestor_record_offset":45184,"inlinee":12354,"source_line":298,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_autumn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":526,"record_offset":46288,"parent_record_offset":45184,"ancestor_record_offset":45184,"inlinee":31059,"source_line":301,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_winter
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":526,"record_offset":45444,"parent_record_offset":45184,"ancestor_record_offset":45184,"inlinee":16149,"source_line":296,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::identity
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":526,"record_offset":45416,"parent_record_offset":45184,"ancestor_record_offset":45184,"inlinee":27995,"source_line":15,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":526,"record_offset":45384,"parent_record_offset":45344,"ancestor_record_offset":45184,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":526,"record_offset":45680,"parent_record_offset":45656,"ancestor_record_offset":45184,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::is_autumn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":526,"record_offset":46312,"parent_record_offset":46288,"ancestor_record_offset":45184,"inlinee":31060,"source_line":109,"archive":"_pdb/C/modules/00526.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::is_winter
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":526,"record_offset":45468,"parent_record_offset":45444,"ancestor_record_offset":45184,"inlinee":27994,"source_line":117,"archive":"_pdb/C/modules/00526.jsonl.gz"}
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
// 完整结构及原始类型记录: startup_water.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: startup_water.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: startup_water.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::grow_grass::GrowGrass>,bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::grow_grass::GrowGrass>", "bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::grow_grass::GrowGrass>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_water::startup_water
// 信息 ID: 044703dd9cbb5c3c410fefe6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8c5b58af9a34c03d0028a981: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> > @ 0x190d44a -> 0x9f24d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> @ 0x1ab546b -> 0x9f24d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

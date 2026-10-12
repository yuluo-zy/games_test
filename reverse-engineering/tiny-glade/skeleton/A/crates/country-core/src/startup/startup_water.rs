// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/startup/startup_water.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: cabb6d788fbc7ba3d0583374f01dd3b2a41d49a0d337b8483fdc77d1a1621a64
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: startup_water.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 448
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_water::startup_water
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":448,"record_offset":29716,"rva":6310784,"va":5375019904}
// 编译实例 RVAs: 0x604b80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: startup_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2270435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375019904,"raw_hex":"e81d5bddff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375020126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b6e80
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: startup_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-318812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375019904,"raw_hex":"e8a422fbff","size":5,"soft_fail":false,"target_va":5374701184,"va":5375019991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b2730
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: startup_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t711056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375019904,"raw_hex":"e890d90a00","size":5,"soft_fail":false,"target_va":5375731504,"va":5375020443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f12d0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: startup_water.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t964947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375019904,"raw_hex":"e853b90e00","size":5,"soft_fail":false,"target_va":5375988432,"va":5375023480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t27955022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375019904,"raw_hex":"e84e8faa01","size":5,"soft_fail":false,"target_va":5402978144,"va":5375023117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t2340451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375019904,"raw_hex":"e863b62300","size":5,"soft_fail":false,"target_va":5377364336,"va":5375023880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t9357543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375019904,"raw_hex":"e8e7c88e00","size":5,"soft_fail":false,"target_va":5384378096,"va":5375020548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t27958610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375019904,"raw_hex":"e8529daa01","size":5,"soft_fail":false,"target_va":5402980128,"va":5375021513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::add
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-454282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375019904,"raw_hex":"e87611f9ff","size":5,"soft_fail":false,"target_va":5374566080,"va":5375020357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":448,"record_offset":29868,"parent_record_offset":29716,"ancestor_record_offset":29716,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_autumn
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":448,"record_offset":30788,"parent_record_offset":29716,"ancestor_record_offset":29716,"inlinee":22564,"source_line":210,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_winter
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":448,"record_offset":30584,"parent_record_offset":29716,"ancestor_record_offset":29716,"inlinee":12658,"source_line":205,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_mesh_into_library
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod.rs
//     evidence={"module":448,"record_offset":29940,"parent_record_offset":29716,"ancestor_record_offset":29716,"inlinee":22686,"source_line":234,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::identity
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":448,"record_offset":30648,"parent_record_offset":29716,"ancestor_record_offset":29716,"inlinee":22687,"source_line":14,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":30360,"parent_record_offset":30336,"ancestor_record_offset":29716,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":30384,"parent_record_offset":30360,"ancestor_record_offset":29716,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":30336,"parent_record_offset":30308,"ancestor_record_offset":29716,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":30308,"parent_record_offset":30280,"ancestor_record_offset":29716,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":30280,"parent_record_offset":30252,"ancestor_record_offset":29716,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":30456,"parent_record_offset":30280,"ancestor_record_offset":29716,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":448,"record_offset":30216,"parent_record_offset":30188,"ancestor_record_offset":29716,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":448,"record_offset":30188,"parent_record_offset":30160,"ancestor_record_offset":29716,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":448,"record_offset":30132,"parent_record_offset":30104,"ancestor_record_offset":29716,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":448,"record_offset":30160,"parent_record_offset":30132,"ancestor_record_offset":29716,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":448,"record_offset":30104,"parent_record_offset":30076,"ancestor_record_offset":29716,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":448,"record_offset":30076,"parent_record_offset":30048,"ancestor_record_offset":29716,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":448,"record_offset":30048,"parent_record_offset":30004,"ancestor_record_offset":29716,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":448,"record_offset":30252,"parent_record_offset":30216,"ancestor_record_offset":29716,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":448,"record_offset":30516,"parent_record_offset":30492,"ancestor_record_offset":29716,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":448,"record_offset":30492,"parent_record_offset":30216,"ancestor_record_offset":29716,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":448,"record_offset":29908,"parent_record_offset":29868,"ancestor_record_offset":29716,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":448,"record_offset":30412,"parent_record_offset":30384,"ancestor_record_offset":29716,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":448,"record_offset":30004,"parent_record_offset":29940,"ancestor_record_offset":29716,"inlinee":22491,"source_line":35,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":448,"record_offset":29976,"parent_record_offset":29940,"ancestor_record_offset":29716,"inlinee":21549,"source_line":31,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::is_autumn
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":448,"record_offset":30812,"parent_record_offset":30788,"ancestor_record_offset":29716,"inlinee":22565,"source_line":101,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::is_winter
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":448,"record_offset":30612,"parent_record_offset":30584,"ancestor_record_offset":29716,"inlinee":21928,"source_line":109,"archive":"_pdb/A/modules/00448.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_water::startup_water
// 信息 ID: 443fed6653222a53253aa4d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5ead4c28cdae23fe8b291595: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> > @ 0xe8365a -> 0x604b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> @ 0xf0cd59 -> 0x604b80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/audio/wall_audio_utils.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: c48ef5ad5773dd295590cd4d3bb0e65fcfed7350772e0e8999ee8102badceee4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_audio_utils.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 395, 472
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::request_play
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":395,"record_offset":62752,"rva":5462640,"va":5374171760}
// 编译实例 RVAs: 0x535a70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405af4a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t497929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374171760,"raw_hex":"e809990700","size":5,"soft_fail":false,"target_va":5374669984,"va":5374172050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b2850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t511342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374171760,"raw_hex":"e86ecd0700","size":5,"soft_fail":false,"target_va":5374683216,"va":5374171869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b2e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t512966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374171760,"raw_hex":"e8c6d30700","size":5,"soft_fail":false,"target_va":5374684816,"va":5374171845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28807403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374171760,"raw_hex":"e8eb90b701","size":5,"soft_fail":false,"target_va":5402979344,"va":5374171936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28808115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374171760,"raw_hex":"e8b393b701","size":5,"soft_fail":false,"target_va":5402980128,"va":5374172008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::set_volume
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8428900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374171760,"raw_hex":"e8649d8000","size":5,"soft_fail":false,"target_va":5382600800,"va":5374171895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::audio_resources::impl$10::set_volume
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":395,"record_offset":63060,"parent_record_offset":62752,"ancestor_record_offset":62752,"inlinee":22640,"source_line":487,"archive":"_pdb/A/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::audio_utils::AudioLoop::current
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/audio_utils.rs
//     evidence={"module":395,"record_offset":62920,"parent_record_offset":62752,"ancestor_record_offset":62752,"inlinee":22638,"source_line":83,"archive":"_pdb/A/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":62948,"parent_record_offset":62920,"ancestor_record_offset":62752,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":395,"record_offset":63092,"parent_record_offset":63060,"ancestor_record_offset":62752,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":395,"record_offset":63124,"parent_record_offset":63060,"ancestor_record_offset":62752,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":395,"record_offset":62980,"parent_record_offset":62948,"ancestor_record_offset":62752,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":395,"record_offset":63012,"parent_record_offset":62980,"ancestor_record_offset":62752,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00395.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::resolve
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x588F
// 首条位置: {"module":472,"inlinee":22671}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::new
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5890
// 首条位置: {"module":472,"inlinee":22672}
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
// 完整结构及原始类型记录: wall_audio_utils.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::audio::wall_audio_utils::AudioLoopPlayer
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_audio_utils.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_audio_utils.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 11; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>,bevy_ecs::event::EventReader<country_core::systems::backend::icc::IccProfileChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>", "bevy_ecs::event::EventReader<country_core::systems::backend::icc::IccProfileChanged>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::backend::icc::IccProfileChanged>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustClosed>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>", "bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustClosed>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustClosed>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::new
// 信息 ID: b3b4e3106185d9b9a39ebad9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=472, inline_record=6744; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=472, inline_record=6872; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=472, inline_record=7000; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=472, inline_record=7128; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=472, inline_record=7256; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=472, inline_record=7384; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=472, inline_record=7512; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=472, inline_record=7640; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::request_play
// 信息 ID: 2ec161fdc2e4c5347ec3f9fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x65f48c -> 0x535a70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::resolve
// 信息 ID: 53b18cea38eb93237c63c41b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 10 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=472, inline_record=13400; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=472, inline_record=13492; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=472, inline_record=13584; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=472, inline_record=13676; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=472, inline_record=13768; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=472, inline_record=13860; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=472, inline_record=13952; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=472, inline_record=14044; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

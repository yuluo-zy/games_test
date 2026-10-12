// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/raw/bytes.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: c7e151090ce5c975ceaa26e6a478358d11bcba15c72fb74fa623aba9aa67179e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: bytes.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 39, 47, 265, 400, 436, 446, 451, 455, 456, 469, 497, 1019, 1202, 1304, 1381, 1434, 1450, 1922, 2829, 3174, 3852, 4499, 4613
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// 当前没有可证实的 crate 引用信息。
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::bytes::slice_as_bytes
// 记录形式: inline_instance, inlinee_declaration; instances=90
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C9B
// 首条位置: {"module":39,"inlinee":7323}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::bytes::bytes_as_slice
// 记录形式: inline_instance, inlinee_declaration; instances=119
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6252
// 首条位置: {"module":436,"inlinee":25170}
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
// 完整结构及原始类型记录: bytes.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: bytes.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: bytes.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::blit::BlitResources>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::blit::BlitResources>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::blit::BlitResources>

// ------------------------------------------------------------------
// 函数: rhapsody::raw::bytes::bytes_as_slice
// 信息 ID: 52558c9ec374a08cd14443a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 51 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::load_texture::load_compiled_texture_image; module=436, inline_record=12560; 不是独立调用边。
// [PDB 内联宿主] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::geometry::instanced_wall::BrickDrawListSsbo>; module=1381, inline_record=46076; 不是独立调用边。
// [PDB 内联宿主] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>; module=1381, inline_record=46568; 不是独立调用边。
// [PDB 内联宿主] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>; module=1381, inline_record=47068; 不是独立调用边。
// [PDB 内联宿主] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo>; module=1381, inline_record=47600; 不是独立调用边。
// [PDB 内联宿主] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::geometry::instanced_wall::BrickNextVisibilityBitmaskSsbo>; module=1381, inline_record=49080; 不是独立调用边。
// [PDB 内联宿主] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>; module=1381, inline_record=50568; 不是独立调用边。
// [PDB 内联宿主] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::rocky_terrain::RockyTerrainTriangleSSBO>; module=1381, inline_record=52128; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::bytes::slice_as_bytes
// 信息 ID: 493d4aa0bab81913c866dd85
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 35 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::egui_backend::EguiBackend::draw; module=39, inline_record=12668; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::egui_backend::EguiBackend::draw; module=39, inline_record=13064; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::egui_backend::EguiBackend::draw; module=39, inline_record=20552; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::egui_backend::EguiBackend::draw; module=39, inline_record=20692; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::calc_dof; module=400, inline_record=54108; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::rt::upload_rt_world; module=446, inline_record=24100; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::rt::upload_rt_world; module=446, inline_record=24644; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::rt::upload_rt_world; module=446, inline_record=25192; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

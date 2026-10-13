// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _references/sources/87094e387a03775b/binding_set.rs
// Original source: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/binding_set.rs
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: binding_set.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: binding_set.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: binding_set.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] tuple$<i64,std::time::Instant> (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] u32 (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::render::RenderParams>", "bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::RenderParams>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>

// ------------------------------------------------------------------
// 函数: rhapsody::raw::binding_set::BindingSetLayoutCold::allocate
// 信息 ID: df55105b232ce3f5f17696a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_binding_set_impl @ 0xe4cbde -> 0xe296e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::binding_set::ResourceHandles::retain_buffer
// 信息 ID: 28dfc7ea15b3e34d9e942bc6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 228 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::solid_vertex_color_wooden_gate::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x147d6d6 -> 0xe29130 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::solid_vertex_color_wooden_gate::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x147d832 -> 0xe29130 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::solid_vertex_color_wooden_gate::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x147d992 -> 0xe29130 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_roof_cloth::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14e7ca6 -> 0xe29130 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_roof_cloth::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14e7e02 -> 0xe29130 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_roof_cloth::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14e7f62 -> 0xe29130 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_wall_cloth::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14e9bb6 -> 0xe29130 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_wall_cloth::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14e9d12 -> 0xe29130 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_wall_cloth::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14e9e72 -> 0xe29130 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_small::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14ebaa6 -> 0xe29130 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_small::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14ebc02 -> 0xe29130 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_small::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14ebd62 -> 0xe29130 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::rocky_terrain::stairs::stairs_indirect::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x1556936 -> 0xe29130 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::rocky_terrain::stairs::stairs_indirect::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x1556a92 -> 0xe29130 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::rocky_terrain::stairs::stairs_indirect::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x1556bf2 -> 0xe29130 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::nani_curved_halftimber::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x1558508 -> 0xe29130 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::binding_set::ResourceHandles::retain_texture
// 信息 ID: 980d6eeb4a45a2342a100cb5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 152 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::solid_vertex_color_wooden_gate::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x147dacb -> 0xe29410 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::solid_vertex_color_wooden_gate::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x147dbf3 -> 0xe29410 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_roof_cloth::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14e808a -> 0xe29410 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_roof_cloth::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14e81b3 -> 0xe29410 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_wall_cloth::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14e9f9a -> 0xe29410 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_wall_cloth::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14ea0c3 -> 0xe29410 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_small::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14ebe8a -> 0xe29410 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_small::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14ebfb3 -> 0xe29410 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::rocky_terrain::stairs::stairs_indirect::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x1556d1a -> 0xe29410 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::rocky_terrain::stairs::stairs_indirect::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x1556e43 -> 0xe29410 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::nani_curved_halftimber::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x15588d7 -> 0xe29410 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::nani_curved_halftimber::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x1558a01 -> 0xe29410 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::rocky_terrain::rocky_terrain::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x155a177 -> 0xe29410 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::rocky_terrain::rocky_terrain::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x155a2a1 -> 0xe29410 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::loading_screen::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x155bdaa -> 0xe29410 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::loading_screen::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x155bed3 -> 0xe29410 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::binding_set::impl$13::dims
// 信息 ID: e10daf37db032c032e4e951b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::shader::roof::inc::wooden_floor::impl$2::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder> @ 0xb7829d -> 0xe18020 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::roof::inc::stone_floor::impl$2::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder> @ 0xc4950e -> 0xe18020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::loading_screen::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x155b4e4 -> 0xe18020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::loading_screen::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x155b721 -> 0xe18020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::nani_stairs_floor::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x157e6e1 -> 0xe18020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::nani_stairs_floor::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x157e936 -> 0xe18020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::nani_plaster::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x15b5161 -> 0xe18020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::nani_plaster::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x15b5174 -> 0xe18020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::nani_plaster::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x15b53fe -> 0xe18020 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::nani_plaster::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x15b5418 -> 0xe18020 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::binding_set::impl$14::read_buf
// 信息 ID: ffa562c0febb537c54f98f6a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 74 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::shader::sheep::impl$9::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder> @ 0xa04a58 -> 0xe222a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::solid_vertex_color_wooden_gate::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x147ceb2 -> 0xe222a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::solid_vertex_color_wooden_gate::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x147d144 -> 0xe222a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::rocky_terrain::stairs::stairs_indirect::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x1556431 -> 0xe222a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::rocky_terrain::stairs::stairs_indirect::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x155668b -> 0xe222a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::garden_lowpoly_flower::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x155d437 -> 0xe222a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::nani_lantern_emissive::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x1560c60 -> 0xe222a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::critters::nani_wren::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x15653c5 -> 0xe222a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::nani_wooden_plank::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x156861c -> 0xe222a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::nani_solid_vertex_color_window_glass::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x1569f7c -> 0xe222a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::nani_solid_vertex_color_window_glass::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x156a1a4 -> 0xe222a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::wall::wall_brick_lod0::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x156be8f -> 0xe222a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::wall::wall_brick_lod0::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x156becc -> 0xe222a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::wall::wall_brick_lod0::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x156c158 -> 0xe222a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::wall::wall_brick_lod0::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x156c19c -> 0xe222a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::nani_flower::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x156dec0 -> 0xe222a0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::binding_set::impl$14::read_tex
// 信息 ID: 3070d703f5e25cfec6e239fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 156 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::shader::roof::inc::wooden_floor::impl$2::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder> @ 0xb782b7 -> 0xe22310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::roof::inc::wooden_floor::impl$2::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder> @ 0xb782df -> 0xe22310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::roof::inc::wooden_floor::impl$2::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder> @ 0xb78302 -> 0xe22310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::roof::inc::wooden_floor::impl$2::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder> @ 0xb78325 -> 0xe22310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::roof::inc::wooden_floor::impl$2::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder> @ 0xb78348 -> 0xe22310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::roof::inc::wooden_floor::impl$2::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder> @ 0xb7836b -> 0xe22310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::roof::inc::wooden_floor::impl$2::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder> @ 0xb7838e -> 0xe22310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::roof::inc::wooden_floor::impl$2::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder> @ 0xb783b1 -> 0xe22310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::roof::inc::wooden_floor::impl$2::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder> @ 0xb783d4 -> 0xe22310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::roof::inc::wooden_floor::impl$2::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder> @ 0xb783f7 -> 0xe22310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::roof::inc::wooden_floor::impl$2::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder> @ 0xb7841a -> 0xe22310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::roof::inc::wooden_floor::impl$2::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder> @ 0xb7843d -> 0xe22310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::roof::inc::stone_floor::impl$2::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder> @ 0xc49528 -> 0xe22310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::roof::inc::stone_floor::impl$2::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder> @ 0xc49552 -> 0xe22310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::roof::inc::stone_floor::impl$2::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder> @ 0xc49574 -> 0xe22310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::roof::inc::stone_floor::impl$2::bind<rhapsody::raw::binding_set::RawProtocol,rhapsody::raw::binding_set::RawBinder> @ 0xc49597 -> 0xe22310 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::binding_set::impl$14::uniform_buf
// 信息 ID: f32949db24ef593d7beca1a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::inc::color_library::ColorLibrary<rhapsody::raw::binding_set::RawProtocol> > @ 0x1576ab0 -> 0xe22230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::binding_set::impl$2::deref
// 信息 ID: 29a315a321f189b962e14a85
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 72d967759317642b3735ebd5: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>,bevy_ecs::change_detection::Res<country_core::startup::load_shaders::ShaderLoadingOptions>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<country_core::startup::load_shaders::ShaderModDeps> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::ShadersChangedEvent>)
// 引用方: 原程序直接转移 505 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::shadow_receiver_frustum::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x234831 -> 0xe6ac10 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::shadow_receiver_frustum::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x2348b2 -> 0xe6ac10 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::egui::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x235cf1 -> 0xe6ac10 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::egui::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x235d72 -> 0xe6ac10 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::blit::blit_stretch::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x237171 -> 0xe6ac10 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::node_builder::draw_stream::RasterStreamRecorder::shader<country_core::shader::blit::blit_stretch::vs_main<rhapsody::rg::binding::RgProtocol> > @ 0x2371f2 -> 0xe6ac10 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::init_binding_set<rhapsody_core::shader::view::ViewConstants,rhapsody_core::shader::view::ViewConstants> @ 0x453b4b -> 0xe6ac10 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::shadow::SunViews<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::shadow::SunViews<rhapsody::rg::binding::RgProtocol> > @ 0x454e5a -> 0xe6ac10 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::roof::inc::dormer_hole::DormerHoles<shabriri_rt::api::EmptyProtocol>,country_core::shader::roof::inc::dormer_hole::DormerHoles<rhapsody::rg::binding::RgProtocol> > @ 0x45533a -> 0xe6ac10 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::wall_slot_identity::WallSlotIdentity<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::wall_slot_identity::WallSlotIdentity<rhapsody::rg::binding::RgProtocol> > @ 0x45582a -> 0xe6ac10 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::wall_color_customization::WallColorData<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::wall_color_customization::WallColorData<rhapsody::rg::binding::RgProtocol> > @ 0x455dca -> 0xe6ac10 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::rt_defs::RtScene<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::rt_defs::RtScene<rhapsody::rg::binding::RgProtocol> > @ 0x4562ea -> 0xe6ac10 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::window_color_customization::WindowColorCustomization<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::window_color_customization::WindowColorCustomization<rhapsody::rg::binding::RgProtocol> > @ 0x45684a -> 0xe6ac10 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<rhapsody_core::shader::frame::FrameCounter,rhapsody_core::shader::frame::FrameCounter> | rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::current_view::CurrentView,country_core::shader::inc::current_view::CurrentView> @ 0x456d2a -> 0xe6ac10 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::nani::inc::indirection::NaniInstanceIndirection<shabriri_rt::api::EmptyProtocol>,country_core::shader::nani::inc::indirection::NaniInstanceIndirection<rhapsody::rg::binding::RgProtocol> > @ 0x4571ca -> 0xe6ac10 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::halftimber_color_customization::HalftimberColors<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::halftimber_color_customization::HalftimberColors<rhapsody::rg::binding::RgProtocol> > | rhapsody::rg::api::GraphApi::push_instant_binding_set<country_core::shader::inc::wooden_wall_color_customization::WoodenWallColors<shabriri_rt::api::EmptyProtocol>,country_core::shader::inc::wooden_wall_color_customization::WoodenWallColors<rhapsody::rg::binding::RgProtocol> > @ 0x4576aa -> 0xe6ac10 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::binding_set::impl$4::destroy_resource
// 信息 ID: 97823fe49663a681b8cd9baf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::respool::Pool::take_out_waste<rhapsody::raw::binding_set::BindingSetLayoutSchema,ref_mut$<rhapsody::raw::disposal::DisposalBucket> > @ 0xe6adda -> 0xe75360 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::binding_set::impl$9::destroy_resource
// 信息 ID: 64a2c38f983e4761a15012e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::respool::Pool::take_out_waste<rhapsody::raw::binding_set::BindingSetSchema,ref_mut$<rhapsody::raw::disposal::DisposalBucket> > @ 0xe6c6ac -> 0xe750f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===

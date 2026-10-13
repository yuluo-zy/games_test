// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _unmapped/module-00245.rs
// Original source: 
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: module-00245.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: module-00245.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: module-00245.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// 候选参数类型: ["country_core::systems::collision::world::SubworldColliderHandle", "ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >"]

// ------------------------------------------------------------------
// 函数: rhapsody::raw::respool::Pool::alloc<rhapsody::raw::binding_set::BindingSetLayoutSchema>
// 信息 ID: 5f14ab9bec29cdf83d6bdcaa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::respool::Pool::alloc -> ["rhapsody::raw::binding_set::BindingSetLayoutSchema"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_binding_set_layout<shaderface::ShaderVar<country_core::shader::inc::shadow::SunViews<shabriri_rt::api::EmptyProtocol> > > @ 0x5c15ba -> 0x66fbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_binding_set_layout<shaderface::ShaderVar<country_core::shader::inc::flag::FlagTextures<shabriri_rt::api::EmptyProtocol> > > @ 0x5c282a -> 0x66fbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_binding_set_layout<shaderface::ShaderVar<rhapsody_core::shader::frame::FrameCounter> > | rhapsody::raw::device::GraphicsDevice::create_binding_set_layout<shaderface::ShaderVar<country_core::shader::inc::frame::FrameConstants> > | rhapsody::raw::device::GraphicsDevice::create_binding_set_layout<shaderface::ShaderVar<country_core::shader::inc::current_view::CurrentView> > | rhapsody::raw::device::GraphicsDevice::create_binding_set_layout<shaderface::ShaderVar<rhapsody_core::shader::view::ViewConstants> > @ 0x5c3a9a -> 0x66fbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_binding_set_layout<shaderface::ShaderVar<country_core::shader::inc::halftimber_color_customization::HalftimberColors<shabriri_rt::api::EmptyProtocol> > > | rhapsody::raw::device::GraphicsDevice::create_binding_set_layout<shaderface::ShaderVar<country_core::shader::inc::wooden_wall_color_customization::WoodenWallColors<shabriri_rt::api::EmptyProtocol> > > @ 0x5c4d0a -> 0x66fbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_binding_set_layout<shaderface::ShaderVar<country_core::shader::nani::inc::indirection::NaniInstanceIndirection<shabriri_rt::api::EmptyProtocol> > > @ 0x5c5f7a -> 0x66fbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_binding_set_layout<shaderface::ShaderVar<country_core::shader::inc::wall_color_customization::WallColorData<shabriri_rt::api::EmptyProtocol> > > @ 0x5c71ea -> 0x66fbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_binding_set_layout<shaderface::ShaderVar<country_core::shader::inc::window_color_customization::WindowColorCustomization<shabriri_rt::api::EmptyProtocol> > > @ 0x5c845a -> 0x66fbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_binding_set_layout<shaderface::ShaderVar<country_core::shader::inc::rt_defs::RtScene<shabriri_rt::api::EmptyProtocol> > > @ 0x5c96ca -> 0x66fbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_binding_set_layout<shaderface::ShaderVar<country_core::shader::roof::inc::dormer_hole::DormerHoles<shabriri_rt::api::EmptyProtocol> > > @ 0x5ca93a -> 0x66fbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_binding_set_layout<shaderface::ShaderVar<country_core::shader::inc::roof_color_data::RoofColors<shabriri_rt::api::EmptyProtocol> > > @ 0x5cbbaa -> 0x66fbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_binding_set_layout<shaderface::ShaderVar<country_core::shader::inc::wall_slot_identity::WallSlotIdentity<shabriri_rt::api::EmptyProtocol> > > @ 0x5cce1a -> 0x66fbc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_binding_set_layout<shaderface::ShaderVar<country_core::shader::inc::stone_foundation_color_customization::StoneFoundationColors<shabriri_rt::api::EmptyProtocol> > > @ 0x5ce08a -> 0x66fbc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::respool::Pool::alloc<rhapsody::raw::shader::RasterPipelineSchema>
// 信息 ID: c9f4aa0f9b8df00f12875042
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::respool::Pool::alloc -> ["rhapsody::raw::shader::RasterPipelineSchema"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<rhapsody::raw::shader::CompiledShaderPipeline,alloc::alloc::Global> > @ 0x5d0126 -> 0x66f720 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::respool::Pool::alloc<rhapsody::raw::texture::TextureSchema>
// 信息 ID: d98373a0cafb2eba4f6f0cc6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::respool::Pool::alloc -> ["rhapsody::raw::texture::TextureSchema"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0x5d4811 -> 0x66f2a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x5d5331 -> 0x66f2a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] unknown_source_preserved_in_module_bucket; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===

// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _references/sources/32d7ddb11ac6b9d7/buffer.rs
// Original source: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: buffer.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: buffer.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: buffer.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::BufferDesc::alignment
// 信息 ID: 4e578f03defb362c051e6c36
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 145 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::roofs::roof_buffers::WoodenGablePlankSsbo> @ 0x13fdbaa -> 0xe182c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::sparkle::SparkleSsbo> @ 0x13ff8b6 -> 0xe182c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo> @ 0x14015ca -> 0xe182c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::geometry::instanced_wall::BrickTransformSsbo> @ 0x1a3e5bb -> 0xe182c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shadowed_buffer::ShadowedBuffer::grow<country_core::render::nani::instance_data::PlantInstanceData> @ 0x1a467ae -> 0xe182c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shadowed_buffer::ShadowedBuffer::grow<country_core::render::nani::instance_data::PlasterInstanceData> @ 0x1a46be1 -> 0xe182c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shadowed_buffer::ShadowedBuffer::grow<country_core::render::nani::instance_data::WrenInstanceData> @ 0x1a46f92 -> 0xe182c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shadowed_buffer::ShadowedBuffer::grow<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData> @ 0x1a47402 -> 0xe182c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shadowed_buffer::ShadowedBuffer::grow<country_core::render::nani::instance_data::StoneFloorInstanceData> @ 0x1a4790a -> 0xe182c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shadowed_buffer::ShadowedBuffer::grow<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData> @ 0x1a47c8a -> 0xe182c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shadowed_buffer::ShadowedBuffer::grow<country_core::render::nani::instance_data::SmokeInstanceData> @ 0x1a480d2 -> 0xe182c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shadowed_buffer::ShadowedBuffer::grow<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> @ 0x1a485e1 -> 0xe182c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shadowed_buffer::ShadowedBuffer::grow<country_core::render::nani::instance_data::VelocityOnlyInstanceData> @ 0x1a48992 -> 0xe182c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shadowed_buffer::ShadowedBuffer::grow<country_core::render::nani::instance_data::GothicWindowBricksInstanceData> @ 0x1a48df8 -> 0xe182c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shadowed_buffer::ShadowedBuffer::grow<country_core::render::nani::instance_data::LanternEmissiveInstanceData> @ 0x1a4938a -> 0xe182c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shadowed_buffer::ShadowedBuffer::grow<country_core::render::nani::instance_data::StairsFloorInstanceData> @ 0x1a497d1 -> 0xe182c0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::BufferDesc::new
// 信息 ID: 9675dc5447c0ed29fa718a38
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 147 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::vermeer::vermeer_render_from_target @ 0x941ad7 -> 0xe18250 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::painterly::painterly_pass @ 0x9ba07e -> 0xe18250 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::roofs::roof_buffers::WoodenGablePlankSsbo> @ 0x13fdb9c -> 0xe18250 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::sparkle::SparkleSsbo> @ 0x13ff8a8 -> 0xe18250 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo> @ 0x14015bc -> 0xe18250 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::geometry::instanced_wall::BrickTransformSsbo> @ 0x1a3e5ad -> 0xe18250 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shadowed_buffer::ShadowedBuffer::grow<country_core::render::nani::instance_data::PlantInstanceData> @ 0x1a467a1 -> 0xe18250 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shadowed_buffer::ShadowedBuffer::grow<country_core::render::nani::instance_data::PlasterInstanceData> @ 0x1a46bd4 -> 0xe18250 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shadowed_buffer::ShadowedBuffer::grow<country_core::render::nani::instance_data::WrenInstanceData> @ 0x1a46f85 -> 0xe18250 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shadowed_buffer::ShadowedBuffer::grow<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData> @ 0x1a473f5 -> 0xe18250 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shadowed_buffer::ShadowedBuffer::grow<country_core::render::nani::instance_data::StoneFloorInstanceData> @ 0x1a478fd -> 0xe18250 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shadowed_buffer::ShadowedBuffer::grow<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData> @ 0x1a47c7d -> 0xe18250 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shadowed_buffer::ShadowedBuffer::grow<country_core::render::nani::instance_data::SmokeInstanceData> @ 0x1a480c5 -> 0xe18250 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shadowed_buffer::ShadowedBuffer::grow<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> @ 0x1a485d4 -> 0xe18250 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shadowed_buffer::ShadowedBuffer::grow<country_core::render::nani::instance_data::VelocityOnlyInstanceData> @ 0x1a48985 -> 0xe18250 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shadowed_buffer::ShadowedBuffer::grow<country_core::render::nani::instance_data::GothicWindowBricksInstanceData> @ 0x1a48deb -> 0xe18250 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu
// 信息 ID: ad3a31435f6ce4d5dab9bc2f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::panic_screenshot::create_graphics_resources @ 0x3f827f -> 0xe180d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu_low_prio
// 信息 ID: 780865470c5b9568533beff6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5ceb4f -> 0xe181d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::solid_vertex_color_wooden_gate::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x147d285 -> 0xe181d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x147f67f -> 0xe181d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x14ed3f5 -> 0xe181d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x14ed97f -> 0xe181d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x15e3d80 -> 0xe181d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x15e42ff -> 0xe181d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x1ac7b30 -> 0xe181d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x28317bf -> 0xe181d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x2b1db6f -> 0xe181d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::BufferDesc::new_gpu_only
// 信息 ID: b6d69a17792bbef6103c4163
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 39 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x37033c -> 0xe18050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x3798e2 -> 0xe18050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::maintain_wooden_freeform_wall_boards @ 0x817444 -> 0xe18050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wooden_arch_frame_systems::maintain_wooden_arch_frames @ 0x81976a -> 0xe18050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::stone_foundation_surface_systems::append_stone_foundation_surface_face @ 0x83f5ec -> 0xe18050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_assemble_wood_misc::upload_wooden_frames_to_gpu @ 0x8ac765 -> 0xe18050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::stone_foundation_cutouts::upload_stone_foundation_cutouts_to_gpu @ 0x8dbca6 -> 0xe18050 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::vao::build_vao @ 0xb91582 -> 0xe18050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> > @ 0x1464c4f -> 0xe18050 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::solid_vertex_color_wooden_gate::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x147cef6 -> 0xe18050 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_roof_cloth::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14e778a -> 0xe18050 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_wall_cloth::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14e973c -> 0xe18050 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_small::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14eb638 -> 0xe18050 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::trimesh::TrimeshBuilder::build @ 0x14faf9f -> 0xe18050 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::rocky_terrain::stairs::stairs_indirect::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x1556487 -> 0xe18050 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::loading_screen::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x155b52a -> 0xe18050 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::BufferDesc::new_gpu_to_cpu
// 信息 ID: 4c24c72570df5396e8965202
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x4bccc3 -> 0xe18150 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::tgsr_capture::start_recording @ 0x5af9ce -> 0xe18150 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::texture_export::export_packaged_textures @ 0xa93457 -> 0xe18150 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::image_baker::bake_grass2_world_albedo_tint_from_env @ 0xafd1f1 -> 0xe18150 (direct_call); 同地址别名未裁定。
// [静态直接引用] clutter_icon_pipe::render_icon @ 0xceac4d -> 0xe18150 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$16::copy_to_slice
// 信息 ID: 7267f232cbd24947b95ffd9f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::solid_vertex_color_wooden_gate::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x147cfd9 -> 0xe143a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::solid_vertex_color_wooden_gate::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x147d3f5 -> 0xe143a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x14ed3a0 -> 0xe143a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x14ed537 -> 0xe143a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x15e3d28 -> 0xe143a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x15e3ebb -> 0xe143a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x1ac7ad8 -> 0xe143a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x1ac7c6b -> 0xe143a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$17::byte_count
// 信息 ID: 920bf11e25c9285c849b97ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 30 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5cea04 -> 0xe124a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5ceaa3 -> 0xe124a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5ceab9 -> 0xe124a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5cead4 -> 0xe124a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5cec6b -> 0xe124a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x147f534 -> 0xe124a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x147f5d3 -> 0xe124a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x147f5e9 -> 0xe124a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x147f604 -> 0xe124a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x147f79b -> 0xe124a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x14ed834 -> 0xe124a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x14ed8d3 -> 0xe124a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x14ed8e9 -> 0xe124a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x14ed904 -> 0xe124a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x14eda9b -> 0xe124a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x15e41b4 -> 0xe124a0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$17::copy_to_slice
// 信息 ID: 5443bc2bdb7a6e73526504af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5ceaf0 -> 0xe124b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5cec8e -> 0xe124b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x147f620 -> 0xe124b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x147f7be -> 0xe124b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x14ed920 -> 0xe124b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x14edabe -> 0xe124b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x15e42a0 -> 0xe124b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x15e443e -> 0xe124b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x2831760 -> 0xe124b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x28318fe -> 0xe124b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x2b1db10 -> 0xe124b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x2b1dcae -> 0xe124b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$19::update_buffer_bytes::closure$0
// 信息 ID: 609597cadca413cb9da7efcb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::buffer::impl$19::update_buffer_bytes", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::buffer::impl$19::update_buffer_bytes::closure_env$0 <- rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$19::update_buffer_bytes::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$19::update_buffer_bytes::closure_env$0> @ 0xe52e55 -> 0xe51100 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$19::update_buffer_bytes::closure_env$0> @ 0xe52f28 -> 0xe51100 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$9::destroy_resource
// 信息 ID: f1e537c0f4cf04e9a0a5d34d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::respool::Pool::take_out_waste<rhapsody::raw::buffer::BufferSchema,ref_mut$<rhapsody::raw::disposal::DisposalBucket> > @ 0xe6c3fb -> 0xe75710 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===

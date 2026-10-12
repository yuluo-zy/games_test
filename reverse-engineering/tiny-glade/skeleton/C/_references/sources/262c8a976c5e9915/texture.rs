// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _references/sources/262c8a976c5e9915/texture.rs
// Original source: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: texture.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: texture.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: texture.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::update_texture_impl
// 信息 ID: b1a720c35f47630b2d37d4d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_texture @ 0xe5e4f2 -> 0xe5eb90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::texture::MipChainIter::new
// 信息 ID: d606524b1f96611414b44741
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::texture::TextureDesc::array_elements
// 信息 ID: 912a8b5bd5bc03d265615a6d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::load_texture::create_game_texture @ 0xb90b6a -> 0xe185a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_window_cut_sdf::glade_startup_window_cut_sdf @ 0xc6cdfe -> 0xe185a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] clutter_icon_pipe::create_texture_from_packaged_blob @ 0xcef83c -> 0xe185a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::texture::TextureHandleExt::view_impl<ref_mut$<rhapsody::raw::texture::Texture> >
// 信息 ID: 6f825ff3049e3c16e7e0ff95
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["rhapsody::raw::texture::Texture"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::texture::TextureHandleExt::view_impl -> ["ref_mut$<rhapsody::raw::texture::Texture>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::solid_vertex_color_wooden_gate::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x147dc14 -> 0x24eae40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_roof_cloth::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14e81d4 -> 0x250e3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_wall_cloth::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14ea0e4 -> 0x250e3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_small::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14ebfd4 -> 0x250e3c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::texture::TextureHandleExt::view_impl<rhapsody::raw::respool::HandleBorrow<rhapsody::raw::texture::TextureSchema> >
// 信息 ID: 0f8ae2b3831168553b27332b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::respool::HandleBorrow -> ["rhapsody::raw::texture::TextureSchema"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::texture::TextureHandleExt::view_impl -> ["rhapsody::raw::respool::HandleBorrow<rhapsody::raw::texture::TextureSchema>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_registry::ResourceRegistry::image_view @ 0xe14707 -> 0xe6ef60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::texture::TextureHandleExt::view_impl<rhapsody::raw::texture::Texture>
// 信息 ID: f90bc15df8e1c1d3c9aac61e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::texture::TextureHandleExt::view_impl -> ["rhapsody::raw::texture::Texture"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::solid_vertex_color_wooden_gate::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x147dae8 -> 0x24ea910 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_roof_cloth::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14e80a7 -> 0x250de90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_wall_cloth::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14e9fb7 -> 0x250de90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_binding_set<country_core::shader::flag_small::MaterialData<rhapsody::raw::binding_set::RawProtocol> > @ 0x14ebea7 -> 0x250de90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::texture::TextureSchema::create
// 信息 ID: 128102ea41eabe78ad62b5db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0x5d46bd -> 0xe18780 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x5d51dd -> 0xe18780 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::in_place_collect::from_iter_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::Image,alloc::alloc::Global>,rhapsody::raw::swapchain::impl$6::new_impl::closure_env$4>,rhapsody::raw::texture::Texture> @ 0xe43f9b -> 0xe18780 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0xe5f816 -> 0xe18780 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0x11a56ad -> 0xe18780 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x15e6ead -> 0xe18780 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<ref$<slice2$<u8> > > @ 0x15e79d8 -> 0xe18780 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x1acd686 -> 0xe18780 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x1cc4f16 -> 0xe18780 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x1cfce40 -> 0xe18780 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x1e4fce0 -> 0xe18780 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x21d94c0 -> 0xe18780 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::texture::create_vk_image_view
// 信息 ID: dffb9953786f23ac34e9e537
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::texture::TextureSchema::create @ 0xe187c8 -> 0xe18960 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::texture::TextureHandleExt::view_impl<rhapsody::raw::respool::HandleBorrow<rhapsody::raw::texture::TextureSchema> > @ 0xe6f229 -> 0xe18960 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::texture::TextureHandleExt::view_impl<ref_mut$<rhapsody::raw::texture::Texture> > @ 0x18062d2 -> 0xe18960 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::texture::TextureHandleExt::view_impl<rhapsody::raw::texture::Texture> @ 0x1806802 -> 0xe18960 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::texture::TextureHandleExt::view_impl<rhapsody::raw::texture::Texture> @ 0x24eabb2 -> 0xe18960 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::texture::TextureHandleExt::view_impl<ref_mut$<rhapsody::raw::texture::Texture> > @ 0x24eb0e2 -> 0xe18960 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::texture::TextureHandleExt::view_impl<rhapsody::raw::texture::Texture> @ 0x250e132 -> 0xe18960 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::texture::TextureHandleExt::view_impl<ref_mut$<rhapsody::raw::texture::Texture> > @ 0x250e662 -> 0xe18960 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::texture::get_image_create_info
// 信息 ID: b5dd085b63ecc4ed0cccf1dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 11 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0x5d42bd -> 0xe18f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x5d4dbf -> 0xe18f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0xe5f3d6 -> 0xe18f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0x11a52ad -> 0xe18f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x15e6a8f -> 0xe18f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<ref$<slice2$<u8> > > @ 0x15e75b3 -> 0xe18f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x1acd286 -> 0xe18f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x1cc4b16 -> 0xe18f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x1cfca24 -> 0xe18f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x1e4f8c4 -> 0xe18f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x21d90a4 -> 0xe18f30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::texture::impl$0::destroy_resource
// 信息 ID: a3fb2def3aceb4e9518c61ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::respool::Pool::take_out_waste<rhapsody::raw::texture::TextureSchema,ref_mut$<rhapsody::raw::disposal::DisposalBucket> > @ 0xe6be51 -> 0xe767a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::texture::impl$10::from
// 信息 ID: 436555fc549679fb34efca39
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0x5d428e -> 0xe124f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0x11a527e -> 0xe124f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture_with_mem_location<ref$<slice2$<u8> > > @ 0x15e7581 -> 0xe124f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::texture::impl$12::next
// 信息 ID: d8b76ea45f4f0e4cc2b01d43
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::texture::impl$14::next @ 0xe1228c -> 0xe195f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::texture::impl$14::next @ 0xe122d9 -> 0xe195f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter_nested::impl$0::from_iter<rhapsody::raw::texture::TextureUpdateData,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::take::Take<rhapsody::raw::texture::MipChainIter>,core::slice::iter::Iter<ref$<slice2$<u8> > > >,rhapsody::raw::texture::impl$15::init_texture_data::closure_env$0> > @ 0x1e514a4 -> 0xe195f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::texture::impl$14::next
// 信息 ID: 5963351359851e071d551c24
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter_nested::impl$0::from_iter<rhapsody::raw::texture::TextureUpdateData,rhapsody::raw::texture::LinearInitialDataIter> @ 0x1e5300d -> 0xe12260 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter_nested::impl$0::from_iter<rhapsody::raw::texture::TextureUpdateData,rhapsody::raw::texture::LinearInitialDataIter> @ 0x1e530ce -> 0xe12260 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::texture::impl$15::init_texture_data::closure$1
// 信息 ID: 8b883acb7d36eee1a7e678d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::texture::impl$15::init_texture_data", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::texture::impl$15::init_texture_data::closure_env$1 <- rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::texture::impl$15::init_texture_data::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::texture::impl$15::init_texture_data::closure_env$1> @ 0xe52315 -> 0xe5e8e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::texture::impl$15::init_texture_data::closure_env$1> @ 0xe523b5 -> 0xe5e8e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::texture::impl$19::into_tex_dim
// 信息 ID: d940505cef3166f191d2fc6f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_window_cut_sdf::glade_startup_window_cut_sdf @ 0xc6cde4 -> 0xe14270 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::texture::impl$21::into_tex_dim
// 信息 ID: 8ac552da4a3c4c9e788b62c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] clutter_icon_pipe::IconMapTextures::get @ 0xced179 -> 0xe145e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] clutter_icon_pipe::IconMapTextures::get @ 0xced4bf -> 0xe145e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::texture::impl$22::into_tex_dim
// 信息 ID: d6989e8db6c2294275befca1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_window_cut_sdf::glade_startup_window_cut_sdf @ 0xc6cdb1 -> 0xe19120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::texture::impl$55::fmt
// 信息 ID: d943e4aa1770b415ac5fbc08
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::texture::impl$63::fmt
// 信息 ID: 99b2d57eee52964c053d2d6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::texture::impl$71::fmt
// 信息 ID: 80ab19a5e2397d2b2df18ad4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::texture::impl$81::fmt
// 信息 ID: 5fdcd620b32bf77c0f1bf2d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::texture::impl$91::fmt
// 信息 ID: 87cfdf03c375c0e1527f9f77
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===

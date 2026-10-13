// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _references/sources/cc146eb03156d40a/binding.rs
// Original source: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/binding.rs
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: binding.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: binding.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: binding.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::rg::binding::BindDynamicBuilder::read_tex
// 信息 ID: 83e0f98e941cc4e1ca661597
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_recorded_node @ 0xe25718 -> 0xe20870 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::binding::BindDynamicBuilder::write_tex
// 信息 ID: 7c20a3fb2c321a74552e71a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::api::GraphApi::add_recorded_node @ 0xe25749 -> 0xe20a70 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::api::GraphApi::init_texture_as_debug_garbage @ 0xe2820d -> 0xe20a70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::binding::impl$7::bind_entry_point<rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder>
// 信息 ID: dbceae9a7590b9a608cc03df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::binding::impl$7::bind_entry_point -> ["rhapsody::rg::node_builder::compute_pass_builder::RgComputeEntryBinder"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point<rhapsody::rg::binding::BindDynamic> @ 0xe2423c -> 0xe1a460 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::binding::rg_binding_to_descriptor_binding_impl
// 信息 ID: da82493ff2bed005ab6a4286
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::into_iter::impl$6::try_fold<enum2$<core::option::Option<rhapsody::rg::recorder::RecordedBindingSet> >,alloc::alloc::Global,alloc::vec::in_place_drop::InPlaceDrop<enum2$<core::option::Option<rhapsody::raw::binding_set::BindingSet> > >,core::iter::adapters::map::map_try_fold::closure_env$0<enum2$<core::option::Option<rhapsody::rg::recorder::RecordedBindingSet> >,enum2$<core::option::Option<rhapsody::raw::binding_set::BindingSet> >,alloc::vec::in_place_drop::InPlaceDrop<enum2$<core::option::Option<rhapsody::raw::binding_set::BindingSet> > >,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<enum2$<core::option::Option<rhapsody::raw::binding_set::BindingSet> > >,never$> >,rhapsody::rg::resource_registry::impl$2::compile_binding_sets::closure_env$0,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<enum2$<core::option::Option<rhapsody::raw::binding_set::BindingSet> > > >,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<enum2$<core::option::Option<rhapsody::raw::binding_set::BindingSet> > >,never$> > > @ 0xe2e86d -> 0xe20c70 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::recorder::impl$8::new_in::render_impl<rhapsody::rg::node_builder::compute_pass_builder::impl$12::dispatch_impl::closure_env$0> @ 0xe741ca -> 0xe20c70 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::chain::impl$1::fold<core::iter::adapters::map::Map<core::iter::adapters::take::Take<ref_mut$<alloc::vec::into_iter::IntoIter<tuple$<shaderface::reflection::BindingIndex,shaderface::reflection::BindingArrayElement,enum2$<rhapsody::rg::render_api::NodeBinding>,enum2$<core::option::Option<rhapsody::rg::resource_info::ResourceAccessIndex> > >,alloc::alloc::Global> > >,rhapsody::rg::node_builder::draw_stream::impl$10::render::closure_env$0>,core::iter::adapters::take::Take<ref_mut$<alloc::vec::into_iter::IntoIter<tuple$<shaderface::reflection::BindingIndex,shaderface::reflection::BindingArrayElement,enum2$<rhapsody::raw::binding::DescriptorSetBinding> >,alloc::alloc::Global> > >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shaderface::reflection::BindingIndex,shaderface::reflection::BindingArrayElement,enum2$<rhapsody::raw::binding::DescriptorSetBinding> >,alloc::vec::impl$21::extend_trusted::closure_env$0<tuple$<shaderface::reflection::BindingIndex,shaderface::reflection::BindingArrayElement,enum2$<rhapsody::raw::binding::DescriptorSetBinding> >,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::take::Take<ref_mut$<alloc::vec::into_iter::IntoIter<tuple$<shaderface::reflection::BindingIndex,shaderface::reflection::BindingArrayElement,enum2$<rhapsody::rg::render_api::NodeBinding>,enum2$<core::option::Option<rhapsody::rg::resource_info::ResourceAccessIndex> > >,alloc::alloc::Global> > >,rhapsody::rg::node_builder::draw_stream::impl$10::render::closure_env$0>,core::iter::adapters::take::Take<ref_mut$<alloc::vec::into_iter::IntoIter<tuple$<shaderface::reflection::BindingIndex,shaderface::reflection::BindingArrayElement,enum2$<rhapsody::raw::binding::DescriptorSetBinding> >,alloc::alloc::Global> > > > > > > @ 0xe7d291 -> 0xe20c70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===

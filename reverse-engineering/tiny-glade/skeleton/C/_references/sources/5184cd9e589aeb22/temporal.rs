// Generated from PDB metadata by skeleton/_tools/enrich_skeleton.py.
// Version: C
// Skeleton path: _references/sources/5184cd9e589aeb22/temporal.rs
// Original source: /home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/temporal.rs
// 函数体、未记录参数名和布局 = unknown。

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: temporal.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: temporal.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: temporal.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 候选参数类型: ["rhapsody::raw::physical_device::PhysicalDevice"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 候选参数类型: ["rhapsody::raw::physical_device::PhysicalDevice"]

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::TemporalGraphState::clear
// 信息 ID: a922f68f61e881ff110e0dce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::rcache::RenderResourceCache::clear_resources @ 0xe714f9 -> 0xe355a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::TemporalGraphState::get_and_flip_parity
// 信息 ID: b4238cad187e40bf7d822137
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<ref$<str$> > @ 0x10e1277 -> 0xe35350 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<ref$<str$> > @ 0x17511b9 -> 0xe35350 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<ref$<str$> > @ 0x1cf5cb7 -> 0xe35350 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::impl$10::get_or_create_temporal<alloc::string::String>
// 信息 ID: 300d210be1ab29a5569e90db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::temporal::impl$10::get_or_create_temporal -> ["alloc::string::String"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<ref$<str$> > @ 0x10e11b4 -> 0x10e0780 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<ref$<str$> > @ 0x10e120c -> 0x10e0780 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<ref$<str$> > @ 0x1cf5bf4 -> 0x1cf51c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<ref$<str$> > @ 0x1cf5c4c -> 0x1cf51c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_source_attribution; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::impl$10::get_or_create_temporal<ref$<str$> >
// 信息 ID: 5bc6d51f7da765f56e50c0f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::temporal::impl$10::get_or_create_temporal -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_core::reprojection::render_reprojection_map @ 0xe04bea -> 0x1e4c980 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<ref$<str$> >
// 信息 ID: b6af1f3da13a7a52df54b7c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody_taa::render_tgsr_with_aux @ 0x6f1a1f -> 0x10e1130 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::create_temporal_history @ 0x6f22c7 -> 0x10e1130 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_taa::render_tgsr2_with_aux_and_weights @ 0x6f2469 -> 0x10e1130 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_gi::calculate_gtao @ 0xccc32b -> 0x1cf5b70 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_gi::eval_gi_screen_radiance @ 0xccc86d -> 0x1cf5b70 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody_gi::eval_gi_screen_radiance @ 0xccc936 -> 0x1cf5b70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。
// [文件映射] pdb_symbolizer_namespace_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// === PDB ENRICHMENT V1 END ===

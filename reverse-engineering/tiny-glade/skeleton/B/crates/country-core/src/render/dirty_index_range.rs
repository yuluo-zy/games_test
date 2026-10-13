// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/render/dirty_index_range.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d6f675689a80efbf155c0be3580faef3ad222d4863afb9464032c8875d41a63b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: dirty_index_range.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 518, 523, 1622, 1653
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::dirty_index_range::impl$2::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1653,"record_offset":118540,"rva":23673648,"va":5392382768}
// 编译实例 RVAs: 0x1693b30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field2_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9490177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392382768,"raw_hex":"e8ff306fff","size":5,"soft_fail":false,"target_va":5382892704,"va":5392382876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::dirty_index_range::DirtyIndexRange::add
// 记录形式: inline_instance, inlinee_declaration; instances=66
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x73E9
// 首条位置: {"module":523,"inlinee":29673}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::fetch_max
//     owner=rust_standard_library; count=63
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":523,"record_offset":9296,"parent_record_offset":9212,"ancestor_record_offset":9212,"inlinee":29643,"source_line":3344,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::fetch_min
//     owner=rust_standard_library; count=63
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":523,"record_offset":9240,"parent_record_offset":9212,"ancestor_record_offset":9212,"inlinee":29641,"source_line":3390,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umax
//     owner=rust_standard_library; count=63
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":523,"record_offset":9320,"parent_record_offset":9296,"ancestor_record_offset":9212,"inlinee":29644,"source_line":3962,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umin
//     owner=rust_standard_library; count=63
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":523,"record_offset":9264,"parent_record_offset":9240,"ancestor_record_offset":9212,"inlinee":29642,"source_line":3979,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::dirty_index_range::DirtyIndexRange::is_empty
// 记录形式: inline_instance, inlinee_declaration; instances=19
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x73CE
// 首条位置: {"module":518,"inlinee":29646}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1653,"record_offset":32064,"parent_record_offset":32040,"ancestor_record_offset":32040,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01653.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1653,"record_offset":32088,"parent_record_offset":32064,"ancestor_record_offset":32040,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01653.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::dirty_index_range::DirtyIndexRange::clear
// 记录形式: inline_instance, inlinee_declaration; instances=18
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x73EA
// 首条位置: {"module":1653,"inlinee":29674}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::dirty_index_range::DirtyIndexRange::to_range
// 记录形式: inline_instance, inlinee_declaration; instances=18
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x73EB
// 首条位置: {"module":1653,"inlinee":29675}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1653,"record_offset":32180,"parent_record_offset":32012,"ancestor_record_offset":32012,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01653.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::is_empty
//     owner=private_source_ownership_candidate; count=17
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1653,"record_offset":32040,"parent_record_offset":32012,"ancestor_record_offset":32012,"inlinee":29646,"source_line":18,"archive":"_pdb/B/modules/01653.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1653,"record_offset":32064,"parent_record_offset":32040,"ancestor_record_offset":32012,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01653.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=68
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1653,"record_offset":32088,"parent_record_offset":32064,"ancestor_record_offset":32012,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01653.jsonl.gz"}
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
// 完整结构及原始类型记录: dirty_index_range.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::dirty_index_range::DirtyIndexRange
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: dirty_index_range.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: dirty_index_range.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::render::dirty_index_range::DirtyIndexRange::add
// 信息 ID: caf1bcfa4542809896107bfd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 63 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_mesh; module=523, inline_record=9212; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::remove_instance; module=523, inline_record=9728; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::extend_no_data; module=523, inline_record=15900; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::create_instances<country_core::resources::placement_data::PlacementDataSsbo,country_core::systems::prefab::create_prefab_gpu_instances::closure_env$0>; module=1622, inline_record=76500; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::create_instances<country_core::resources::placement_data::PlacementDataSsbo,country_core::systems::prefab::create_prefab_gpu_instances::closure_env$0>; module=1622, inline_record=76844; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::UberInstanceData>; module=1622, inline_record=78908; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::UberInstanceData>; module=1622, inline_record=79204; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::SolidVertexColorInstanceData>; module=1622, inline_record=80856; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::dirty_index_range::DirtyIndexRange::clear
// 信息 ID: 7a12abf676259e350deae2d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 17 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>; module=1653, inline_record=35604; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::UberInstanceData>; module=1653, inline_record=40704; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidAlbedoMapInstanceData>; module=1653, inline_record=45812; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>; module=1653, inline_record=50932; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SmokeInstanceData>; module=1653, inline_record=56032; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::StoneFloorInstanceData>; module=1653, inline_record=61136; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::LanternEmissiveInstanceData>; module=1653, inline_record=66244; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::PlasterInstanceData>; module=1653, inline_record=71344; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::dirty_index_range::DirtyIndexRange::is_empty
// 信息 ID: 6c4a7cf7e919dadfa3356e54
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 17 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>; module=1653, inline_record=32040; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::UberInstanceData>; module=1653, inline_record=37140; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidAlbedoMapInstanceData>; module=1653, inline_record=42248; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>; module=1653, inline_record=47368; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SmokeInstanceData>; module=1653, inline_record=52468; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::StoneFloorInstanceData>; module=1653, inline_record=57572; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::LanternEmissiveInstanceData>; module=1653, inline_record=62680; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::PlasterInstanceData>; module=1653, inline_record=67780; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::dirty_index_range::DirtyIndexRange::to_range
// 信息 ID: d125d578479b12cbc77c1893
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 17 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>; module=1653, inline_record=32012; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::UberInstanceData>; module=1653, inline_record=37112; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidAlbedoMapInstanceData>; module=1653, inline_record=42220; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>; module=1653, inline_record=47340; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SmokeInstanceData>; module=1653, inline_record=52440; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::StoneFloorInstanceData>; module=1653, inline_record=57544; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::LanternEmissiveInstanceData>; module=1653, inline_record=62652; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::PlasterInstanceData>; module=1653, inline_record=67752; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::dirty_index_range::impl$2::fmt
// 信息 ID: 4c1abd8061f8606210ba0bc0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

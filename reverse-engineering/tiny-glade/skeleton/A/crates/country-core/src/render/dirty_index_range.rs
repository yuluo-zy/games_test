// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/render/dirty_index_range.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: d6f675689a80efbf155c0be3580faef3ad222d4863afb9464032c8875d41a63b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: dirty_index_range.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 455, 483, 1371, 1450
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::dirty_index_range::impl$2::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1450,"record_offset":41892,"rva":17891008,"va":5386600128}
// 编译实例 RVAs: 0x110fec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field2_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9016033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386600128,"raw_hex":"e81f6d76ff","size":5,"soft_fail":false,"target_va":5377584208,"va":5386600236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::dirty_index_range::DirtyIndexRange::add
// 记录形式: inline_instance, inlinee_declaration; instances=38
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x65F9
// 首条位置: {"module":455,"inlinee":26105}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::fetch_max
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":455,"record_offset":11816,"parent_record_offset":11732,"ancestor_record_offset":11732,"inlinee":26108,"source_line":2932,"archive":"_pdb/A/modules/00455.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::fetch_min
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":455,"record_offset":11760,"parent_record_offset":11732,"ancestor_record_offset":11732,"inlinee":26106,"source_line":2978,"archive":"_pdb/A/modules/00455.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umax
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":455,"record_offset":11840,"parent_record_offset":11816,"ancestor_record_offset":11732,"inlinee":26109,"source_line":3538,"archive":"_pdb/A/modules/00455.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umin
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":455,"record_offset":11784,"parent_record_offset":11760,"ancestor_record_offset":11732,"inlinee":26107,"source_line":3555,"archive":"_pdb/A/modules/00455.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::dirty_index_range::DirtyIndexRange::is_empty
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6A7F
// 首条位置: {"module":483,"inlinee":27263}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1450,"record_offset":19544,"parent_record_offset":19520,"ancestor_record_offset":19520,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1450,"record_offset":19568,"parent_record_offset":19544,"ancestor_record_offset":19520,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::dirty_index_range::DirtyIndexRange::clear
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6A87
// 首条位置: {"module":1450,"inlinee":27271}
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
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6A88
// 首条位置: {"module":1450,"inlinee":27272}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1450,"record_offset":19660,"parent_record_offset":19492,"ancestor_record_offset":19492,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::is_empty
//     owner=private_source_ownership_candidate; count=9
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":19520,"parent_record_offset":19492,"ancestor_record_offset":19492,"inlinee":27263,"source_line":18,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1450,"record_offset":19544,"parent_record_offset":19520,"ancestor_record_offset":19492,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1450,"record_offset":19568,"parent_record_offset":19544,"ancestor_record_offset":19492,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01450.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::render::dirty_index_range::DirtyIndexRange::add
// 信息 ID: ab1326ec4e60206440e9ad16
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 35 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_mesh; module=455, inline_record=11732; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::remove_instance; module=455, inline_record=12408; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::extend_no_data; module=455, inline_record=13996; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::create_instances<country_core::resources::placement_data::PlacementDataSsbo,country_core::systems::prefab::create_prefab_gpu_instances::closure_env$0>; module=1371, inline_record=11872; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::create_instances<country_core::resources::placement_data::PlacementDataSsbo,country_core::systems::prefab::create_prefab_gpu_instances::closure_env$0>; module=1371, inline_record=12200; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_data<country_core::render::nani::instance_data::SolidVertexColorInstanceData>; module=1371, inline_record=16768; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_data<country_core::render::nani::instance_data::SolidVertexColorInstanceData>; module=1371, inline_record=17476; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_data<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData>; module=1371, inline_record=19180; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::dirty_index_range::DirtyIndexRange::clear
// 信息 ID: 8b14bc6837913ba826b9a94e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>; module=1450, inline_record=21704; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::buffers::GpuInstance>; module=1450, inline_record=24200; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>; module=1450, inline_record=26728; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>; module=1450, inline_record=29252; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::resources::placement_data::PlacementDataSsbo>; module=1450, inline_record=31760; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::StoneFloorInstanceData>; module=1450, inline_record=34276; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData>; module=1450, inline_record=36800; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::WrenInstanceData>; module=1450, inline_record=39308; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::dirty_index_range::DirtyIndexRange::is_empty
// 信息 ID: d746faecb78628816b4fafd6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>; module=1450, inline_record=19520; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::buffers::GpuInstance>; module=1450, inline_record=22020; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>; module=1450, inline_record=24548; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>; module=1450, inline_record=27072; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::resources::placement_data::PlacementDataSsbo>; module=1450, inline_record=29576; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::StoneFloorInstanceData>; module=1450, inline_record=32092; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData>; module=1450, inline_record=34620; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::WrenInstanceData>; module=1450, inline_record=37128; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::dirty_index_range::DirtyIndexRange::to_range
// 信息 ID: fe506a6be5dfc6022e2bc4df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>; module=1450, inline_record=19492; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::buffers::GpuInstance>; module=1450, inline_record=21992; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>; module=1450, inline_record=24520; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>; module=1450, inline_record=27044; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::resources::placement_data::PlacementDataSsbo>; module=1450, inline_record=29548; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::StoneFloorInstanceData>; module=1450, inline_record=32064; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData>; module=1450, inline_record=34592; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::WrenInstanceData>; module=1450, inline_record=37100; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::dirty_index_range::impl$2::fmt
// 信息 ID: 419424c815c046ef1aa56cde
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

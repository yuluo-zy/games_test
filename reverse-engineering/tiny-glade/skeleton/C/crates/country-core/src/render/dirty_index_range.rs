// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/render/dirty_index_range.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: d6f675689a80efbf155c0be3580faef3ad222d4863afb9464032c8875d41a63b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: dirty_index_range.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 513, 522, 597, 1732
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
// 首条位置: {"module":1732,"record_offset":185268,"rva":27589552,"va":5396298672}
// 编译实例 RVAs: 0x1a4fbb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_struct_field2_finish
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11477425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396298672,"raw_hex":"e84fde50ff","size":5,"soft_fail":false,"target_va":5384821360,"va":5396298780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::dirty_index_range::DirtyIndexRange::add
// 记录形式: inline_instance, inlinee_declaration; instances=113
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7455
// 首条位置: {"module":513,"inlinee":29781}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::fetch_max
//     owner=rust_standard_library; count=110
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":513,"record_offset":2596,"parent_record_offset":2512,"ancestor_record_offset":2512,"inlinee":29784,"source_line":3500,"archive":"_pdb/C/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::fetch_min
//     owner=rust_standard_library; count=110
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":513,"record_offset":2540,"parent_record_offset":2512,"ancestor_record_offset":2512,"inlinee":29782,"source_line":3547,"archive":"_pdb/C/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umax
//     owner=rust_standard_library; count=110
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":513,"record_offset":2620,"parent_record_offset":2596,"ancestor_record_offset":2512,"inlinee":29785,"source_line":4184,"archive":"_pdb/C/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umin
//     owner=rust_standard_library; count=110
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":513,"record_offset":2564,"parent_record_offset":2540,"ancestor_record_offset":2512,"inlinee":29783,"source_line":4201,"archive":"_pdb/C/modules/00513.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::dirty_index_range::DirtyIndexRange::is_empty
// 记录形式: inline_instance, inlinee_declaration; instances=22
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x947F
// 首条位置: {"module":597,"inlinee":38015}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1732,"record_offset":87008,"parent_record_offset":86984,"ancestor_record_offset":86984,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01732.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1732,"record_offset":87032,"parent_record_offset":87008,"ancestor_record_offset":86984,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01732.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::dirty_index_range::DirtyIndexRange::clear
// 记录形式: inline_instance, inlinee_declaration; instances=21
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9491
// 首条位置: {"module":1732,"inlinee":38033}
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
// 记录形式: inline_instance, inlinee_declaration; instances=21
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9492
// 首条位置: {"module":1732,"inlinee":38034}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1732,"record_offset":87124,"parent_record_offset":86956,"ancestor_record_offset":86956,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01732.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::is_empty
//     owner=private_source_ownership_candidate; count=20
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1732,"record_offset":86984,"parent_record_offset":86956,"ancestor_record_offset":86956,"inlinee":38015,"source_line":18,"archive":"_pdb/C/modules/01732.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1732,"record_offset":87008,"parent_record_offset":86984,"ancestor_record_offset":86956,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01732.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=80
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1732,"record_offset":87032,"parent_record_offset":87008,"ancestor_record_offset":86956,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01732.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::render::dirty_index_range::DirtyIndexRange::add
// 信息 ID: a10be42685301e1e6c4b5ee6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 110 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData>; module=513, inline_record=2512; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData>; module=513, inline_record=3316; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::SmokeInstanceData>; module=513, inline_record=4924; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::SmokeInstanceData>; module=513, inline_record=5728; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::StoneFloorInstanceData>; module=513, inline_record=7340; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::StoneFloorInstanceData>; module=513, inline_record=8144; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>; module=513, inline_record=9760; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>; module=513, inline_record=10564; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::dirty_index_range::DirtyIndexRange::clear
// 信息 ID: d894fce8442c071efccb924f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 20 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>; module=1732, inline_record=90416; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SmokeInstanceData>; module=1732, inline_record=95188; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::shader::inc::stone_foundation_surface::StoneFoundationSurfaceInstanceData>; module=1732, inline_record=99988; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::StoneFloorInstanceData>; module=1732, inline_record=104764; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::BalconyDoorInstanceData>; module=1732, inline_record=109540; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::LanternEmissiveInstanceData>; module=1732, inline_record=114320; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::StairRopeInstanceData>; module=1732, inline_record=119096; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::UberInstanceData>; module=1732, inline_record=123868; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::dirty_index_range::DirtyIndexRange::is_empty
// 信息 ID: 095a3ae79c4da7adb0287b91
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 20 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>; module=1732, inline_record=86984; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SmokeInstanceData>; module=1732, inline_record=91756; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::shader::inc::stone_foundation_surface::StoneFoundationSurfaceInstanceData>; module=1732, inline_record=96556; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::StoneFloorInstanceData>; module=1732, inline_record=101332; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::BalconyDoorInstanceData>; module=1732, inline_record=106108; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::LanternEmissiveInstanceData>; module=1732, inline_record=110888; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::StairRopeInstanceData>; module=1732, inline_record=115664; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::UberInstanceData>; module=1732, inline_record=120436; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::dirty_index_range::DirtyIndexRange::to_range
// 信息 ID: 3b49b5cfa352e41267366443
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 20 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>; module=1732, inline_record=86956; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SmokeInstanceData>; module=1732, inline_record=91728; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::shader::inc::stone_foundation_surface::StoneFoundationSurfaceInstanceData>; module=1732, inline_record=96528; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::StoneFloorInstanceData>; module=1732, inline_record=101304; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::BalconyDoorInstanceData>; module=1732, inline_record=106080; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::LanternEmissiveInstanceData>; module=1732, inline_record=110860; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::StairRopeInstanceData>; module=1732, inline_record=115636; 不是独立调用边。
// [PDB 内联宿主] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::UberInstanceData>; module=1732, inline_record=120408; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::dirty_index_range::impl$2::fmt
// 信息 ID: cd1bd7c9584db0a182471d9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

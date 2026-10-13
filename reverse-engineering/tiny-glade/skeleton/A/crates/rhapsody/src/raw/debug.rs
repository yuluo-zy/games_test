// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/raw/debug.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 9c229283cf0a8120dfa4ae3b0bec3dec5fec546eb352e13c78cad99c666b591f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: debug.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 570
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::begin_label
// 记录形式: compiled_procedure; instances=1
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":24020,"rva":7582128,"va":5376291248}
// 编译实例 RVAs: 0x73b1b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3541725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376291248,"raw_hex":"e823f5c9ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376291432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140869090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1236643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376291248,"raw_hex":"e8a3de1200","size":5,"soft_fail":false,"target_va":5377527952,"va":5376291304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26687621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376291248,"raw_hex":"e885389701","size":5,"soft_fail":false,"target_va":5402979152,"va":5376291526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26688623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376291248,"raw_hex":"e86f3c9701","size":5,"soft_fail":false,"target_va":5402980128,"va":5376291500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*2152(%rsi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5376291248,"raw_hex":"ff9668080000","size":6,"soft_fail":false,"va":5376291404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::ffi::c_str::CString::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\ffi\c_str.rs
//     evidence={"module":570,"record_offset":24236,"parent_record_offset":24020,"ancestor_record_offset":24020,"inlinee":9051,"source_line":263,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::ext::debug_utils::Device::cmd_begin_debug_utils_label
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/ext/debug_utils.rs
//     evidence={"module":570,"record_offset":24576,"parent_record_offset":24020,"ancestor_record_offset":24020,"inlinee":30928,"source_line":29,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$1652::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":24512,"parent_record_offset":24020,"ancestor_record_offset":24020,"inlinee":30927,"source_line":19693,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":24604,"parent_record_offset":24020,"ancestor_record_offset":24020,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":24264,"parent_record_offset":24020,"ancestor_record_offset":24020,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::command_buffer::CommandBuffer::raw
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":570,"record_offset":24540,"parent_record_offset":24020,"ancestor_record_offset":24020,"inlinee":30906,"source_line":177,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::debug_utils
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":570,"record_offset":24180,"parent_record_offset":24020,"ancestor_record_offset":24020,"inlinee":30918,"source_line":604,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":24460,"parent_record_offset":24436,"ancestor_record_offset":24020,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":24436,"parent_record_offset":24380,"ancestor_record_offset":24020,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":570,"record_offset":24680,"parent_record_offset":24656,"ancestor_record_offset":24020,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\ffi\c_str.rs
//     evidence={"module":570,"record_offset":24628,"parent_record_offset":24604,"ancestor_record_offset":24020,"inlinee":8650,"source_line":703,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":24408,"parent_record_offset":24380,"ancestor_record_offset":24020,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":24380,"parent_record_offset":24352,"ancestor_record_offset":24020,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":24204,"parent_record_offset":24180,"ancestor_record_offset":24020,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":24304,"parent_record_offset":24264,"ancestor_record_offset":24020,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::end_label
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":24952,"rva":7582544,"va":5376291664}
// 编译实例 RVAs: 0x73b350
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26687421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376291664,"raw_hex":"e8bd379701","size":5,"soft_fail":false,"target_va":5402979152,"va":5376291726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::extensions_generated::ext::debug_utils::Device::cmd_end_debug_utils_label
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/ext/debug_utils.rs
//     evidence={"module":570,"record_offset":25172,"parent_record_offset":24952,"ancestor_record_offset":24952,"inlinee":30929,"source_line":39,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::command_buffer::CommandBuffer::raw
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":570,"record_offset":25140,"parent_record_offset":24952,"ancestor_record_offset":24952,"inlinee":30906,"source_line":177,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::debug_utils
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":570,"record_offset":25092,"parent_record_offset":24952,"ancestor_record_offset":24952,"inlinee":30918,"source_line":604,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":25112,"parent_record_offset":25092,"ancestor_record_offset":24952,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00570.jsonl.gz"}
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
// 完整结构及原始类型记录: debug.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: debug.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: debug.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::begin_label
// 信息 ID: 36956cd1efb7f681e2842ff9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources> @ 0x74aaf -> 0x73b1b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources> @ 0x74f6d -> 0x73b1b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x1002642 -> 0x73b1b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::end_label
// 信息 ID: d28c5d81b84d4902ddba3cfd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources> @ 0x74f2c -> 0x73b350 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources> @ 0x75254 -> 0x73b350 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x1003853 -> 0x73b350 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

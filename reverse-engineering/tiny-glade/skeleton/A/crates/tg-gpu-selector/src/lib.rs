// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tg-gpu-selector/src/lib.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 9ab17016059cabcc9662f7ef83133963fd4abe21875e97d78deca676fdfd2377
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 269
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use tg_gpu_selector; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::run_gpu_selection_gui
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":269,"record_offset":1576,"rva":4136432,"va":5372845552}
// 编译实例 RVAs: 0x3f1df0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f2cd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5246632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372845552,"raw_hex":"e8a80e5000","size":5,"soft_fail":false,"target_va":5378092240,"va":5372845603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f53e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5256490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372845552,"raw_hex":"e82a355000","size":5,"soft_fail":false,"target_va":5378102240,"va":5372845745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1193893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372845552,"raw_hex":"e85bc8edff","size":5,"soft_fail":false,"target_va":5371651952,"va":5372845840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$rhapsody..raw..init..UnavailableGraphicsDevice$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5242747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372845552,"raw_hex":"e87bff4f00","size":5,"soft_fail":false,"target_va":5378088608,"va":5372845856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<tg_gpu_selector::gpu_selection::GpuSelectionReason> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372845552,"raw_hex":"e833fdffff","size":5,"soft_fail":false,"target_va":5372845136,"va":5372845848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":269,"record_offset":1764,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00269.jsonl.gz"}
//   [pdb_inline_direct] tg_gpu_selector::gpu_selection::Controls::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-gpu-selector/src/gpu_selection.rs
//     evidence={"module":269,"record_offset":1704,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":14739,"source_line":71,"archive":"_pdb/A/modules/00269.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":269,"record_offset":1788,"parent_record_offset":1764,"ancestor_record_offset":1576,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00269.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":269,"record_offset":1732,"parent_record_offset":1704,"ancestor_record_offset":1576,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00269.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":269,"record_offset":1824,"parent_record_offset":1788,"ancestor_record_offset":1576,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00269.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":269,"record_offset":1848,"parent_record_offset":1824,"ancestor_record_offset":1576,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00269.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":269,"record_offset":1880,"parent_record_offset":1788,"ancestor_record_offset":1576,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00269.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::run_vulkan_configuration_gui
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":269,"record_offset":1920,"rva":4136752,"va":5372845872}
// 编译实例 RVAs: 0x3f1f30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f3620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5248673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372845872,"raw_hex":"e8a1165000","size":5,"soft_fail":false,"target_va":5378094624,"va":5372845946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1194127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372845872,"raw_hex":"e871c7edff","size":5,"soft_fail":false,"target_va":5371651952,"va":5372846074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_gpu_selector::vulkan_config::Controls::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5265400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372845872,"raw_hex":"e8f8575000","size":5,"soft_fail":false,"target_va":5378111328,"va":5372845923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":269,"record_offset":2120,"parent_record_offset":1920,"ancestor_record_offset":1920,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00269.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":269,"record_offset":2060,"parent_record_offset":1920,"ancestor_record_offset":1920,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00269.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":269,"record_offset":2092,"parent_record_offset":1920,"ancestor_record_offset":1920,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00269.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":269,"record_offset":2144,"parent_record_offset":2120,"ancestor_record_offset":1920,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00269.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":269,"record_offset":2180,"parent_record_offset":2144,"ancestor_record_offset":1920,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00269.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":269,"record_offset":2204,"parent_record_offset":2180,"ancestor_record_offset":1920,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00269.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":269,"record_offset":2236,"parent_record_offset":2144,"ancestor_record_offset":1920,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00269.jsonl.gz"}
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
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tg_gpu_selector::run_gpu_selection_gui
// 信息 ID: cbd2d0f105bc5ec5bbbd770e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c9027 -> 0x3f1df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c70a1 -> 0x3f1df0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::run_vulkan_configuration_gui
// 信息 ID: 888cfabbb910897c4e26b18a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::vulkan_config_main::run_vulkan_configuration_app @ 0x244a6a -> 0x3f1f30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

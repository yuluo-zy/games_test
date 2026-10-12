// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/utils/src/geometry/split_heightmap.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 149585c5b9a0e4fbce07c4e2d95bbed25475dc96487340f5eb73faf8c3e94fcc
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: split_heightmap.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3826
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::geometry::split_heightmap::split_heightmap_by_threshold<system_wall_constructor::utils::trim_rows::RowTrimmerSink>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3826,"record_offset":260,"rva":29571760,"va":5398280880}
// 编译实例 RVAs: 0x1c33ab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141782160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: split_heightmap.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4921998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398280880,"raw_hex":"e872e5b4ff","size":5,"soft_fail":false,"target_va":5393359200,"va":5398281193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141782210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: split_heightmap.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4921891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398280880,"raw_hex":"e8dde5b4ff","size":5,"soft_fail":false,"target_va":5393359376,"va":5398281262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-26975786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398280880,"raw_hex":"e8d66164fe","size":5,"soft_fail":false,"target_va":5371305232,"va":5398281013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4697451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398280880,"raw_hex":"e86bad4700","size":5,"soft_fail":false,"target_va":5402978880,"va":5398281424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4697927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398280880,"raw_hex":"e847af4700","size":5,"soft_fail":false,"target_va":5402979344,"va":5398281412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3826,"record_offset":820,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/03826.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::trim_rows::impl$0::above
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/trim_rows.rs
//     evidence={"module":3826,"record_offset":472,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":89677,"source_line":138,"archive":"_pdb/A/modules/03826.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3826,"record_offset":692,"parent_record_offset":660,"ancestor_record_offset":260,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03826.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3826,"record_offset":660,"parent_record_offset":624,"ancestor_record_offset":260,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03826.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3826,"record_offset":624,"parent_record_offset":472,"ancestor_record_offset":260,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03826.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":3826,"record_offset":856,"parent_record_offset":820,"ancestor_record_offset":260,"inlinee":8603,"source_line":47,"archive":"_pdb/A/modules/03826.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3826,"record_offset":588,"parent_record_offset":508,"ancestor_record_offset":260,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/03826.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3826,"record_offset":788,"parent_record_offset":472,"ancestor_record_offset":260,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03826.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3826,"record_offset":944,"parent_record_offset":888,"ancestor_record_offset":260,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03826.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3826,"record_offset":724,"parent_record_offset":624,"ancestor_record_offset":260,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03826.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3826,"record_offset":532,"parent_record_offset":508,"ancestor_record_offset":260,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03826.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3826,"record_offset":756,"parent_record_offset":472,"ancestor_record_offset":260,"inlinee":7667,"source_line":325,"archive":"_pdb/A/modules/03826.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3826,"record_offset":556,"parent_record_offset":532,"ancestor_record_offset":260,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03826.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3826,"record_offset":888,"parent_record_offset":856,"ancestor_record_offset":260,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03826.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::point_u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3826,"record_offset":508,"parent_record_offset":472,"ancestor_record_offset":260,"inlinee":27900,"source_line":133,"archive":"_pdb/A/modules/03826.jsonl.gz"}
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
// 完整结构及原始类型记录: split_heightmap.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: split_heightmap.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: split_heightmap.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::geometry::split_heightmap::split_heightmap_by_threshold<system_wall_constructor::utils::trim_rows::RowTrimmerSink>
// 信息 ID: f38b7d60597a75da77b70e1b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::geometry::split_heightmap::split_heightmap_by_threshold -> ["system_wall_constructor::utils::trim_rows::RowTrimmerSink"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::trim_rows::trim_rows @ 0x1781966 -> 0x1c33ab0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

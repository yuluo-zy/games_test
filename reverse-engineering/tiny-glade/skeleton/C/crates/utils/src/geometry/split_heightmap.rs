// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/utils/src/geometry/split_heightmap.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 149585c5b9a0e4fbce07c4e2d95bbed25475dc96487340f5eb73faf8c3e94fcc
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: split_heightmap.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1537
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::geometry::split_heightmap::split_heightmap_by_threshold<system_wall_constructor::utils::trim_rows::RowTrimmerSink>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1537,"record_offset":280,"rva":21593024,"va":5390302144}
// 编译实例 RVAs: 0x1497bc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408426a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: split_heightmap.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12932750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390302144,"raw_hex":"e872a93aff","size":5,"soft_fail":false,"target_va":5377369760,"va":5390302505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140842760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: split_heightmap.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12932627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390302144,"raw_hex":"e8eda93aff","size":5,"soft_fail":false,"target_va":5377369952,"va":5390302574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: split_heightmap.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26048098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390302144,"raw_hex":"e862768d01","size":5,"soft_fail":false,"target_va":5416350384,"va":5390302281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26318876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390302144,"raw_hex":"e81c989101","size":5,"soft_fail":false,"target_va":5416621600,"va":5390302719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26319010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390302144,"raw_hex":"e8a2989101","size":5,"soft_fail":false,"target_va":5416621762,"va":5390302747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1537,"record_offset":936,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::trim_rows::impl$0::above
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/trim_rows.rs
//     evidence={"module":1537,"record_offset":492,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26658,"source_line":138,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1537,"record_offset":748,"parent_record_offset":716,"ancestor_record_offset":280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1537,"record_offset":796,"parent_record_offset":772,"ancestor_record_offset":280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1537,"record_offset":772,"parent_record_offset":748,"ancestor_record_offset":280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1537,"record_offset":716,"parent_record_offset":680,"ancestor_record_offset":280,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1537,"record_offset":656,"parent_record_offset":492,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1537,"record_offset":680,"parent_record_offset":656,"ancestor_record_offset":280,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1537,"record_offset":972,"parent_record_offset":936,"ancestor_record_offset":280,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1537,"record_offset":620,"parent_record_offset":540,"ancestor_record_offset":280,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1537,"record_offset":900,"parent_record_offset":492,"ancestor_record_offset":280,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1537,"record_offset":1036,"parent_record_offset":1004,"ancestor_record_offset":280,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1537,"record_offset":836,"parent_record_offset":680,"ancestor_record_offset":280,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1537,"record_offset":564,"parent_record_offset":540,"ancestor_record_offset":280,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1537,"record_offset":872,"parent_record_offset":492,"ancestor_record_offset":280,"inlinee":5770,"source_line":304,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1537,"record_offset":588,"parent_record_offset":564,"ancestor_record_offset":280,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1537,"record_offset":1004,"parent_record_offset":972,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::point_u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1537,"record_offset":540,"parent_record_offset":492,"ancestor_record_offset":280,"inlinee":26651,"source_line":157,"archive":"_pdb/C/modules/01537.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::geometry::split_heightmap::split_heightmap_by_threshold<system_wall_constructor::utils::trim_rows::RowTrimmerSink>
// 信息 ID: 7a3bbb3ced8d79bd59a412c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::geometry::split_heightmap::split_heightmap_by_threshold -> ["system_wall_constructor::utils::trim_rows::RowTrimmerSink"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::trim_rows::trim_rows @ 0x842b16 -> 0x1497bc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

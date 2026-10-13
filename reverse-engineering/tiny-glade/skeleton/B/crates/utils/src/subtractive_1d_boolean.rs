// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/utils/src/subtractive_1d_boolean.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: b62edd6f31a1dcf3cf95d39f781064528b89662cafb972159949d8b0523771bb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: subtractive_1d_boolean.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3020
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::subtractive_1d_boolean::SubtractiveBoolean1D::subtract
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3020,"record_offset":636,"rva":33407824,"va":5402116944}
// 编译实例 RVAs: 0x1fdc350
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: subtractive_1d_boolean.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27036196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402116944,"raw_hex":"e8dc7563fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5402117695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-27760640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402116944,"raw_hex":"e8006858fe","size":5,"soft_fail":false,"target_va":5374356720,"va":5402117355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3020,"record_offset":804,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3020,"record_offset":1160,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3020,"record_offset":908,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::impl$62::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3020,"record_offset":1996,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":19712,"source_line":343,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3020,"record_offset":2500,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3020,"record_offset":832,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3020,"record_offset":2684,"parent_record_offset":2660,"ancestor_record_offset":636,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3020,"record_offset":2660,"parent_record_offset":2572,"ancestor_record_offset":636,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3020,"record_offset":1224,"parent_record_offset":1192,"ancestor_record_offset":636,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3020,"record_offset":2600,"parent_record_offset":2572,"ancestor_record_offset":636,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3020,"record_offset":2572,"parent_record_offset":2548,"ancestor_record_offset":636,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3020,"record_offset":1272,"parent_record_offset":1248,"ancestor_record_offset":636,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3020,"record_offset":1248,"parent_record_offset":1224,"ancestor_record_offset":636,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3020,"record_offset":2548,"parent_record_offset":2524,"ancestor_record_offset":636,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3020,"record_offset":1192,"parent_record_offset":1160,"ancestor_record_offset":636,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$18::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone.rs
//     evidence={"module":3020,"record_offset":2024,"parent_record_offset":1996,"ancestor_record_offset":636,"inlinee":11764,"source_line":336,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3020,"record_offset":984,"parent_record_offset":940,"ancestor_record_offset":636,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::lt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3020,"record_offset":1040,"parent_record_offset":940,"ancestor_record_offset":636,"inlinee":9733,"source_line":1939,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3020,"record_offset":1008,"parent_record_offset":984,"ancestor_record_offset":636,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::lt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3020,"record_offset":1064,"parent_record_offset":1040,"ancestor_record_offset":636,"inlinee":10039,"source_line":1786,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3020,"record_offset":2628,"parent_record_offset":2600,"ancestor_record_offset":636,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3020,"record_offset":940,"parent_record_offset":908,"ancestor_record_offset":636,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3020,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":636,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3020,"record_offset":872,"parent_record_offset":832,"ancestor_record_offset":636,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3020,"record_offset":1312,"parent_record_offset":1160,"ancestor_record_offset":636,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::subtractive_1d_boolean::SubtractiveBoolean1D::new_range
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3020,"record_offset":252,"rva":33407696,"va":5402116816}
// 编译实例 RVAs: 0x1fdc2d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: subtractive_1d_boolean.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27035374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402116816,"raw_hex":"e8127963fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5402116857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9424059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402116816,"raw_hex":"e8bbcc8f00","size":5,"soft_fail":false,"target_va":5411540992,"va":5402116928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3020,"record_offset":404,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3020,"record_offset":548,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3020,"record_offset":456,"parent_record_offset":432,"ancestor_record_offset":252,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3020,"record_offset":480,"parent_record_offset":456,"ancestor_record_offset":252,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3020,"record_offset":432,"parent_record_offset":404,"ancestor_record_offset":252,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3020,"record_offset":572,"parent_record_offset":548,"ancestor_record_offset":252,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3020,"record_offset":596,"parent_record_offset":572,"ancestor_record_offset":252,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3020,"record_offset":504,"parent_record_offset":480,"ancestor_record_offset":252,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03020.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::subtractive_1d_boolean::SubtractiveBoolean1D::result
// 记录形式: compiled_procedure; instances=1
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3020,"record_offset":2972,"rva":12810080,"va":5381519200}
// 编译实例 RVAs: 0xc37760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: subtractive_1d_boolean.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::subtractive_1d_boolean::SubtractiveBoolean1D
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 3
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: subtractive_1d_boolean.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: subtractive_1d_boolean.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::subtractive_1d_boolean::SubtractiveBoolean1D::new_range
// 信息 ID: 28bed90995df136b9d506e2a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121c505 -> 0x1fdc2d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121ea8b -> 0x1fdc2d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new @ 0x12245e9 -> 0x1fdc2d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::subtractive_1d_boolean::SubtractiveBoolean1D::result
// 信息 ID: 7c4c9e75182dfd0a6777897d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_reflect::reflect::PartialReflect::debug<bevy_ecs::hierarchy::Children> @ 0xcde625 -> 0xc37760 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1215b6d -> 0xc37760 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x1216094 -> 0xc37760 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0x121645e -> 0xc37760 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121cc6c -> 0xc37760 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121f119 -> 0xc37760 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new @ 0x122463c -> 0xc37760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::subtractive_1d_boolean::SubtractiveBoolean1D::subtract
// 信息 ID: bb6b86a49b614077d8937d86
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121ca56 -> 0x1fdc350 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121cb0f -> 0x1fdc350 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121cbc8 -> 0x1fdc350 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121cc50 -> 0x1fdc350 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121eca9 -> 0x1fdc350 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new @ 0x122462a -> 0x1fdc350 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

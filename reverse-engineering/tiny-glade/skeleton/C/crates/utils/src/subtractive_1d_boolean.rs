// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/utils/src/subtractive_1d_boolean.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: b62edd6f31a1dcf3cf95d39f781064528b89662cafb972159949d8b0523771bb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: subtractive_1d_boolean.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 3154
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
// 首条位置: {"module":3154,"record_offset":396,"rva":38678016,"va":5407387136}
// 编译实例 RVAs: 0x24e2e00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-31709153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407387136,"raw_hex":"e81f281cfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5407387852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f940
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: subtractive_1d_boolean.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8964520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407387136,"raw_hex":"e8a8c98800","size":5,"soft_fail":false,"target_va":5416352064,"va":5407387539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3154,"record_offset":564,"parent_record_offset":396,"ancestor_record_offset":396,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3154,"record_offset":656,"parent_record_offset":396,"ancestor_record_offset":396,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":3154,"record_offset":868,"parent_record_offset":396,"ancestor_record_offset":396,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::impl$64::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":3154,"record_offset":1764,"parent_record_offset":396,"ancestor_record_offset":396,"inlinee":12115,"source_line":354,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3154,"record_offset":2408,"parent_record_offset":396,"ancestor_record_offset":396,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3154,"record_offset":592,"parent_record_offset":396,"ancestor_record_offset":396,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3154,"record_offset":2592,"parent_record_offset":2568,"ancestor_record_offset":396,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3154,"record_offset":2616,"parent_record_offset":2592,"ancestor_record_offset":396,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3154,"record_offset":2640,"parent_record_offset":2616,"ancestor_record_offset":396,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3154,"record_offset":2568,"parent_record_offset":2480,"ancestor_record_offset":396,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3154,"record_offset":744,"parent_record_offset":712,"ancestor_record_offset":396,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3154,"record_offset":2508,"parent_record_offset":2480,"ancestor_record_offset":396,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3154,"record_offset":2480,"parent_record_offset":2456,"ancestor_record_offset":396,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3154,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":396,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3154,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":396,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3154,"record_offset":2456,"parent_record_offset":2432,"ancestor_record_offset":396,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3154,"record_offset":712,"parent_record_offset":680,"ancestor_record_offset":396,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3154,"record_offset":680,"parent_record_offset":656,"ancestor_record_offset":396,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$35::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\clone.rs
//     evidence={"module":3154,"record_offset":1792,"parent_record_offset":1764,"ancestor_record_offset":396,"inlinee":8861,"source_line":614,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3154,"record_offset":936,"parent_record_offset":896,"ancestor_record_offset":396,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::lt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3154,"record_offset":992,"parent_record_offset":896,"ancestor_record_offset":396,"inlinee":6830,"source_line":2141,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3154,"record_offset":960,"parent_record_offset":936,"ancestor_record_offset":396,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::lt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3154,"record_offset":1016,"parent_record_offset":992,"ancestor_record_offset":396,"inlinee":8911,"source_line":1916,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3154,"record_offset":2536,"parent_record_offset":2508,"ancestor_record_offset":396,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":3154,"record_offset":896,"parent_record_offset":868,"ancestor_record_offset":396,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3154,"record_offset":2432,"parent_record_offset":2408,"ancestor_record_offset":396,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3154,"record_offset":620,"parent_record_offset":592,"ancestor_record_offset":396,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3154,"record_offset":832,"parent_record_offset":680,"ancestor_record_offset":396,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03154.jsonl.gz"}
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
// 首条位置: {"module":3154,"record_offset":2992,"rva":38678928,"va":5407388048}
// 编译实例 RVAs: 0x24e3190
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-31709404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407388048,"raw_hex":"e824271cfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5407388087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: subtractive_1d_boolean.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38665885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407388048,"raw_hex":"e86301b2fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5407388072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9232606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407388048,"raw_hex":"e8dee08c00","size":5,"soft_fail":false,"target_va":5416620769,"va":5407388158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3154,"record_offset":3140,"parent_record_offset":2992,"ancestor_record_offset":2992,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3154,"record_offset":3220,"parent_record_offset":3196,"ancestor_record_offset":2992,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3154,"record_offset":3244,"parent_record_offset":3220,"ancestor_record_offset":2992,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3154,"record_offset":3268,"parent_record_offset":3244,"ancestor_record_offset":2992,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3154,"record_offset":3196,"parent_record_offset":3168,"ancestor_record_offset":2992,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/03154.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3154,"record_offset":3168,"parent_record_offset":3140,"ancestor_record_offset":2992,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/03154.jsonl.gz"}
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
// 首条位置: {"module":3154,"record_offset":260,"rva":6916944,"va":5375626064}
// 编译实例 RVAs: 0x698b50
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::subtractive_1d_boolean::SubtractiveBoolean1D::new_range
// 信息 ID: c08e786b558889ac4088df23
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143e555 -> 0x24e3190 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::place_rectangle_corner_beams @ 0x1443827 -> 0x24e3190 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1446f7e -> 0x24e3190 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::subtractive_1d_boolean::SubtractiveBoolean1D::result
// 信息 ID: c76068f14878de30fccdd62a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 26 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::wall_selection_tags @ 0x672211 -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::wall_selection_tags @ 0x67227a -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::wall_selection_tags @ 0x672314 -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::wall_selection_tags @ 0x672d0b -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mod_textures::packaged_textures @ 0x9e32dd -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::missing_mods::MissingModRef::from_recorded_entry @ 0xa0d439 -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::missing_mods::collect_missing_mods @ 0xa0dbb4 -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::texture_export::export_packaged_textures @ 0xa93011 -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_textures::read_texture_list @ 0xa94371 -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_textures::reload_packaged_textures_by_name @ 0xa97101 -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_publish::start_upload @ 0xaff2bc -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_subscriptions::lookup_mod_on_workshop @ 0xb37e25 -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons @ 0xb93812 -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::grass2::read_grass2_runtime_config @ 0xc84941 -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] clutter_icon_pipe::packaged_texture_blob_path @ 0xcef241 -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1436416 -> 0x698b50 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::subtractive_1d_boolean::SubtractiveBoolean1D::subtract
// 信息 ID: 48b45205ad19311b271ec7f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143ea47 -> 0x24e2e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143eb0a -> 0x24e2e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143ebc6 -> 0x24e2e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143ec49 -> 0x24e2e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::place_rectangle_corner_beams @ 0x1443a2c -> 0x24e2e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1446fc1 -> 0x24e2e00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

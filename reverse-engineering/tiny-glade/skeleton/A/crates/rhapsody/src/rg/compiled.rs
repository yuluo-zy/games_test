// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/rg/compiled.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 26f58052de8754497cfdd78d48021011c1fdd1c49c4413e84fc71d80accf75c8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: compiled.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 52, 73, 178, 563
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::BatchCompiler::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":563,"record_offset":244,"rva":7550896,"va":5376260016}
// 编译实例 RVAs: 0x7337b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::BatchCompiler::flush_batch
// 记录形式: compiled_procedure; instances=1
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":563,"record_offset":364,"rva":7550976,"va":5376260096}
// 编译实例 RVAs: 0x733800
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3510622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376260096,"raw_hex":"e8a26ecaff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376260329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4952553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376260096,"raw_hex":"e8176eb4ff","size":5,"soft_fail":false,"target_va":5371307664,"va":5376260212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":563,"record_offset":620,"parent_record_offset":364,"ancestor_record_offset":364,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00563.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":563,"record_offset":504,"parent_record_offset":364,"ancestor_record_offset":364,"inlinee":5326,"source_line":845,"archive":"_pdb/A/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":563,"record_offset":948,"parent_record_offset":924,"ancestor_record_offset":364,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":563,"record_offset":924,"parent_record_offset":832,"ancestor_record_offset":364,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":563,"record_offset":860,"parent_record_offset":832,"ancestor_record_offset":364,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":563,"record_offset":700,"parent_record_offset":668,"ancestor_record_offset":364,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":563,"record_offset":832,"parent_record_offset":808,"ancestor_record_offset":364,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":563,"record_offset":668,"parent_record_offset":620,"ancestor_record_offset":364,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":563,"record_offset":528,"parent_record_offset":504,"ancestor_record_offset":364,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":563,"record_offset":888,"parent_record_offset":860,"ancestor_record_offset":364,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":563,"record_offset":760,"parent_record_offset":620,"ancestor_record_offset":364,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":563,"record_offset":556,"parent_record_offset":528,"ancestor_record_offset":364,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00563.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":563,"record_offset":584,"parent_record_offset":528,"ancestor_record_offset":364,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00563.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::BatchCompiler::compile
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DF0
// 首条位置: {"module":52,"inlinee":7664}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":11200,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":11328,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":11100,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":52,"record_offset":10996,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":52,"record_offset":11424,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":4963,"source_line":2524,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":52,"record_offset":11752,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":7667,"source_line":325,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":52,"record_offset":11128,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":52,"record_offset":11264,"parent_record_offset":11240,"ancestor_record_offset":10920,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":11240,"parent_record_offset":11200,"ancestor_record_offset":10920,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":52,"record_offset":11060,"parent_record_offset":11020,"ancestor_record_offset":10920,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":52,"record_offset":11296,"parent_record_offset":11200,"ancestor_record_offset":10920,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":52,"record_offset":11448,"parent_record_offset":11424,"ancestor_record_offset":10920,"inlinee":4965,"source_line":215,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":52,"record_offset":11564,"parent_record_offset":11472,"ancestor_record_offset":10920,"inlinee":4968,"source_line":216,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":52,"record_offset":11356,"parent_record_offset":11328,"ancestor_record_offset":10920,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":52,"record_offset":11384,"parent_record_offset":11356,"ancestor_record_offset":10920,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":52,"record_offset":11472,"parent_record_offset":11448,"ancestor_record_offset":10920,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":52,"record_offset":11020,"parent_record_offset":10996,"ancestor_record_offset":10920,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":52,"record_offset":11588,"parent_record_offset":11564,"ancestor_record_offset":10920,"inlinee":7666,"source_line":196,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::CompiledRenderGraph::new<country_core::render::pass_builder::pass_resources::PassResources>
// 记录形式: compiled_procedure; instances=1
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":52,"record_offset":13288,"rva":482336,"va":5369191456}
// 编译实例 RVAs: 0x75c20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e7780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t465649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369191456,"raw_hex":"e8f11a0700","size":5,"soft_fail":false,"target_va":5369657216,"va":5369191562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140367730
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3086758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369191456,"raw_hex":"e8a6192f00","size":5,"soft_fail":false,"target_va":5372278576,"va":5369191813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140378770
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3156445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369191456,"raw_hex":"e8dd293000","size":5,"soft_fail":false,"target_va":5372348272,"va":5369191822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140386b70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3214834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369191456,"raw_hex":"e8f20d3100","size":5,"soft_fail":false,"target_va":5372406640,"va":5369191801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3557851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369191456,"raw_hex":"e8db493600","size":5,"soft_fail":false,"target_va":5372749712,"va":5369191856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140733480
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7067602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369191456,"raw_hex":"e8d2d76b00","size":5,"soft_fail":false,"target_va":5376259200,"va":5369191593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34401771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369191456,"raw_hex":"e8ebed0c02","size":5,"soft_fail":false,"target_va":5403593360,"va":5369191584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":52,"record_offset":13580,"parent_record_offset":13288,"ancestor_record_offset":13288,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":52,"record_offset":13664,"parent_record_offset":13288,"ancestor_record_offset":13288,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":52,"record_offset":13496,"parent_record_offset":13288,"ancestor_record_offset":13288,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":52,"record_offset":13840,"parent_record_offset":13816,"ancestor_record_offset":13288,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":52,"record_offset":13816,"parent_record_offset":13716,"ancestor_record_offset":13288,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":52,"record_offset":13748,"parent_record_offset":13716,"ancestor_record_offset":13288,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":52,"record_offset":13716,"parent_record_offset":13688,"ancestor_record_offset":13288,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":13604,"parent_record_offset":13580,"ancestor_record_offset":13288,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":52,"record_offset":13628,"parent_record_offset":13604,"ancestor_record_offset":13288,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":52,"record_offset":13780,"parent_record_offset":13748,"ancestor_record_offset":13288,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":52,"record_offset":13688,"parent_record_offset":13664,"ancestor_record_offset":13288,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":52,"record_offset":13544,"parent_record_offset":13520,"ancestor_record_offset":13288,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":52,"record_offset":13520,"parent_record_offset":13496,"ancestor_record_offset":13288,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::impl$1::new::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DD9
// 首条位置: {"module":52,"inlinee":7641}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":73,"record_offset":116748,"parent_record_offset":116600,"ancestor_record_offset":116600,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_info::ResourceAccessIndex::try_index
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":73,"record_offset":116636,"parent_record_offset":116600,"ancestor_record_offset":116600,"inlinee":7642,"source_line":194,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":73,"record_offset":116684,"parent_record_offset":116660,"ancestor_record_offset":116600,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":73,"record_offset":116776,"parent_record_offset":116748,"ancestor_record_offset":116600,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":73,"record_offset":116804,"parent_record_offset":116776,"ancestor_record_offset":116600,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_info::ResourceAccessIndex::valid
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":73,"record_offset":116660,"parent_record_offset":116636,"ancestor_record_offset":116600,"inlinee":5446,"source_line":203,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_info::impl$14::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":73,"record_offset":116708,"parent_record_offset":116684,"ancestor_record_offset":116600,"inlinee":5448,"source_line":227,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::CompiledRenderGraph::provide_lazy_texture
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 144
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DF6
// 首条位置: {"module":178,"inlinee":7670}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":178,"record_offset":5792,"parent_record_offset":5756,"ancestor_record_offset":5756,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":178,"record_offset":5860,"parent_record_offset":5836,"ancestor_record_offset":5756,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":178,"record_offset":5836,"parent_record_offset":5792,"ancestor_record_offset":5756,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":178,"record_offset":5892,"parent_record_offset":5792,"ancestor_record_offset":5756,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00178.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::CompiledRenderGraph::compute_batches<country_core::render::pass_builder::pass_resources::PassResources>
// 记录形式: compiled_procedure; instances=1
// 源码行: 149
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":52,"record_offset":10704,"rva":481600,"va":5369190720}
// 编译实例 RVAs: 0x75940
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3558527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369190720,"raw_hex":"e87f4c3600","size":5,"soft_fail":false,"target_va":5372749712,"va":5369191180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14049d3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4356515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369190720,"raw_hex":"e8a3794200","size":5,"soft_fail":false,"target_va":5373547456,"va":5369190936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2114017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369190720,"raw_hex":"e8e1412000","size":5,"soft_fail":false,"target_va":5371305056,"va":5369191034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33788090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369190720,"raw_hex":"e8ba900302","size":5,"soft_fail":false,"target_va":5402979344,"va":5369191249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::compiled::BatchCompiler::flush_batch
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t7069213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369190720,"raw_hex":"e81dde6b00","size":5,"soft_fail":false,"target_va":5376260096,"va":5369190878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::compiled::BatchCompiler::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7069242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369190720,"raw_hex":"e83ade6b00","size":5,"soft_fail":false,"target_va":5376260016,"va":5369190769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":52,"record_offset":11788,"parent_record_offset":10704,"ancestor_record_offset":10704,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::compiled::BatchCompiler::compile
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/compiled.rs
//     evidence={"module":52,"record_offset":10920,"parent_record_offset":10704,"ancestor_record_offset":10704,"inlinee":7664,"source_line":76,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":52,"record_offset":11988,"parent_record_offset":11964,"ancestor_record_offset":10704,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":52,"record_offset":11964,"parent_record_offset":11864,"ancestor_record_offset":10704,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":52,"record_offset":11896,"parent_record_offset":11864,"ancestor_record_offset":10704,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":52,"record_offset":11264,"parent_record_offset":11240,"ancestor_record_offset":10704,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":52,"record_offset":11864,"parent_record_offset":11836,"ancestor_record_offset":10704,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":11240,"parent_record_offset":11200,"ancestor_record_offset":10704,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":11200,"parent_record_offset":10920,"ancestor_record_offset":10704,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":11328,"parent_record_offset":10920,"ancestor_record_offset":10704,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":11100,"parent_record_offset":10920,"ancestor_record_offset":10704,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":12304,"parent_record_offset":12280,"ancestor_record_offset":10704,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":52,"record_offset":10996,"parent_record_offset":10920,"ancestor_record_offset":10704,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":52,"record_offset":11928,"parent_record_offset":11896,"ancestor_record_offset":10704,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":52,"record_offset":11812,"parent_record_offset":11788,"ancestor_record_offset":10704,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":52,"record_offset":11060,"parent_record_offset":11020,"ancestor_record_offset":10704,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":52,"record_offset":11296,"parent_record_offset":11200,"ancestor_record_offset":10704,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":52,"record_offset":11448,"parent_record_offset":11424,"ancestor_record_offset":10704,"inlinee":4965,"source_line":215,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":52,"record_offset":11564,"parent_record_offset":11472,"ancestor_record_offset":10704,"inlinee":4968,"source_line":216,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":52,"record_offset":11424,"parent_record_offset":10920,"ancestor_record_offset":10704,"inlinee":4963,"source_line":2524,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":52,"record_offset":11752,"parent_record_offset":10920,"ancestor_record_offset":10704,"inlinee":7667,"source_line":325,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":52,"record_offset":11356,"parent_record_offset":11328,"ancestor_record_offset":10704,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":52,"record_offset":11384,"parent_record_offset":11356,"ancestor_record_offset":10704,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":52,"record_offset":11472,"parent_record_offset":11448,"ancestor_record_offset":10704,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":52,"record_offset":11020,"parent_record_offset":10996,"ancestor_record_offset":10704,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":52,"record_offset":11588,"parent_record_offset":11564,"ancestor_record_offset":10704,"inlinee":7666,"source_line":196,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources>
// 记录形式: compiled_procedure; instances=1
// 源码行: 156
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":52,"record_offset":3928,"rva":476848,"va":5369185968}
// 编译实例 RVAs: 0x746b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140005a60
//     owner=unresolved_namespace; count=1
//     target_aliases=118 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-458296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e8c801f9ff","size":5,"soft_fail":false,"target_va":5368732256,"va":5369190547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14005b630
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-107253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e80b5dfeff","size":5,"soft_fail":false,"target_va":5369083440,"va":5369190688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140392740
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3264091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e85bce3100","size":5,"soft_fail":false,"target_va":5372454720,"va":5369190624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3560725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e815553600","size":5,"soft_fail":false,"target_va":5372749712,"va":5369188982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072f520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7054951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e867a66b00","size":5,"soft_fail":false,"target_va":5376242976,"va":5369188020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407334d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7072737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e8e1eb6b00","size":5,"soft_fail":false,"target_va":5376259280,"va":5369186538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140733630
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7073238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e8d6ed6b00","size":5,"soft_fail":false,"target_va":5376259632,"va":5369186389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140733760
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7072974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e8ceec6b00","size":5,"soft_fail":false,"target_va":5376259936,"va":5369186957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140733790
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7073064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e828ed6b00","size":5,"soft_fail":false,"target_va":5376259984,"va":5369186915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140737840
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7089809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e8912e6c00","size":5,"soft_fail":false,"target_va":5376276544,"va":5369186730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140738b70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7094506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e8ea406c00","size":5,"soft_fail":false,"target_va":5376281456,"va":5369186945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14073a8d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7101934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e8ee5d6c00","size":5,"soft_fail":false,"target_va":5376288976,"va":5369187037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14073b1b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7104252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e8fc666c00","size":5,"soft_fail":false,"target_va":5376291248,"va":5369186991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14073b350
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7103519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e81f646c00","size":5,"soft_fail":false,"target_va":5376291664,"va":5369188140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209cdc0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33716628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e894790202","size":5,"soft_fail":false,"target_va":5402906048,"va":5369189415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209cf00
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33717048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e8387b0202","size":5,"soft_fail":false,"target_va":5402906368,"va":5369189315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33789016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e858940302","size":5,"soft_fail":false,"target_va":5402978688,"va":5369189667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t33789393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e8d1950302","size":5,"soft_fail":false,"target_va":5402978880,"va":5369189482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t33789911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e8d7970302","size":5,"soft_fail":false,"target_va":5402979152,"va":5369189236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t33790318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e86e990302","size":5,"soft_fail":false,"target_va":5402979344,"va":5369189021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e8f2f3ffff","size":5,"soft_fail":false,"target_va":5369185040,"va":5369188121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33790490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e81a9a0302","size":5,"soft_fail":false,"target_va":5402980128,"va":5369189633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33790757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e8259b0302","size":5,"soft_fail":false,"target_va":5402980304,"va":5369189542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33790796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e84c9b0302","size":5,"soft_fail":false,"target_va":5402980320,"va":5369189519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33790505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e8299a0302","size":5,"soft_fail":false,"target_va":5402980656,"va":5369190146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7168978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e8d2636d00","size":5,"soft_fail":false,"target_va":5376356912,"va":5369187929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7172721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e871726d00","size":5,"soft_fail":false,"target_va":5376360560,"va":5369187834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7169097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e849646d00","size":5,"soft_fail":false,"target_va":5376356144,"va":5369187042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::barrier::BufferBarrier::new
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t7072754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e8f2eb6b00","size":5,"soft_fail":false,"target_va":5376259136,"va":5369186377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::barrier::ImageBarrier::new
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t7072557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e82deb6b00","size":5,"soft_fail":false,"target_va":5376259088,"va":5369186526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::barrier::image_aspect_mask_from_format
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t7072659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e893eb6b00","size":5,"soft_fail":false,"target_va":5376259168,"va":5369186504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::command_buffer::CommandBufferSeqTracker::track
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t7145897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e8a9096d00","size":5,"soft_fail":false,"target_va":5376332256,"va":5369186354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::compiled::pass_access_range_indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7073485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e8cdee6b00","size":5,"soft_fail":false,"target_va":5376260352,"va":5369186862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::recorder::PassResourceAccessType::can_be_concurrent_with
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7071319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e857e66b00","size":5,"soft_fail":false,"target_va":5376258736,"va":5369187412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource::RuntimeResource::borrow
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t7030805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e815486b00","size":5,"soft_fail":false,"target_va":5376217072,"va":5369186262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_registry::ResourceRegistry::resource
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t7148519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e8e7136d00","size":5,"soft_fail":false,"target_va":5376334768,"va":5369186244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h3d9350ee38791ca5 (.llvm.8305701337189838128)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3437307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369185968,"raw_hex":"e8fb723400","size":5,"soft_fail":false,"target_va":5372625168,"va":5369187856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5369185968,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5369188098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":5388,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":5496,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":52,"record_offset":9024,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":52,"record_offset":5304,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":52,"record_offset":8872,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":52,"record_offset":8988,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":52,"record_offset":10204,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":52,"record_offset":9052,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":52,"record_offset":5636,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":52,"record_offset":7824,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":52,"record_offset":6808,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::issue_and_reset_pipeline_barrier
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/barrier.rs
//     evidence={"module":52,"record_offset":6772,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":7657,"source_line":419,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$17::track
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":52,"record_offset":5608,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":7654,"source_line":250,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::compiled::CompiledRenderGraph::add_pre_pass_barriers
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/compiled.rs
//     evidence={"module":52,"record_offset":5728,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":7655,"source_line":333,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::compiled::CompiledRenderGraph::gather_batch_preamble_barriers
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/compiled.rs
//     evidence={"module":52,"record_offset":4196,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":7644,"source_line":232,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::compiled::CompiledRenderGraph::record_postamble
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/compiled.rs
//     evidence={"module":52,"record_offset":9092,"parent_record_offset":3928,"ancestor_record_offset":3928,"inlinee":7659,"source_line":268,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":52,"record_offset":10404,"parent_record_offset":10380,"ancestor_record_offset":3928,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":52,"record_offset":10380,"parent_record_offset":10280,"ancestor_record_offset":3928,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":52,"record_offset":10312,"parent_record_offset":10280,"ancestor_record_offset":3928,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":52,"record_offset":4428,"parent_record_offset":4404,"ancestor_record_offset":3928,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":52,"record_offset":10280,"parent_record_offset":10252,"ancestor_record_offset":3928,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":4404,"parent_record_offset":4344,"ancestor_record_offset":3928,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":4344,"parent_record_offset":4196,"ancestor_record_offset":3928,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":4316,"parent_record_offset":4196,"ancestor_record_offset":3928,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":4376,"parent_record_offset":4344,"ancestor_record_offset":3928,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":52,"record_offset":4548,"parent_record_offset":4196,"ancestor_record_offset":3928,"inlinee":6077,"source_line":156,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":52,"record_offset":4720,"parent_record_offset":4696,"ancestor_record_offset":3928,"inlinee":7344,"source_line":157,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":4784,"parent_record_offset":4196,"ancestor_record_offset":3928,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":9244,"parent_record_offset":9092,"ancestor_record_offset":3928,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":52,"record_offset":8224,"parent_record_offset":8196,"ancestor_record_offset":3928,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":52,"record_offset":8276,"parent_record_offset":8252,"ancestor_record_offset":3928,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":52,"record_offset":8252,"parent_record_offset":8224,"ancestor_record_offset":3928,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":52,"record_offset":8164,"parent_record_offset":8132,"ancestor_record_offset":3928,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":52,"record_offset":8196,"parent_record_offset":8164,"ancestor_record_offset":3928,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":52,"record_offset":8368,"parent_record_offset":8196,"ancestor_record_offset":3928,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":52,"record_offset":8480,"parent_record_offset":8456,"ancestor_record_offset":3928,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":52,"record_offset":5352,"parent_record_offset":5328,"ancestor_record_offset":3928,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":52,"record_offset":5328,"parent_record_offset":5304,"ancestor_record_offset":3928,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":52,"record_offset":8300,"parent_record_offset":8276,"ancestor_record_offset":3928,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":52,"record_offset":10344,"parent_record_offset":10312,"ancestor_record_offset":3928,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":52,"record_offset":7688,"parent_record_offset":7600,"ancestor_record_offset":3928,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":52,"record_offset":8060,"parent_record_offset":8036,"ancestor_record_offset":3928,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":52,"record_offset":4696,"parent_record_offset":4548,"ancestor_record_offset":3928,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":52,"record_offset":8104,"parent_record_offset":7944,"ancestor_record_offset":3928,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":52,"record_offset":5244,"parent_record_offset":4196,"ancestor_record_offset":3928,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":52,"record_offset":6116,"parent_record_offset":6092,"ancestor_record_offset":3928,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":52,"record_offset":8432,"parent_record_offset":8132,"ancestor_record_offset":3928,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":52,"record_offset":4508,"parent_record_offset":4484,"ancestor_record_offset":3928,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":52,"record_offset":4660,"parent_record_offset":4636,"ancestor_record_offset":3928,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":52,"record_offset":7184,"parent_record_offset":7160,"ancestor_record_offset":3928,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":52,"record_offset":7160,"parent_record_offset":7136,"ancestor_record_offset":3928,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":52,"record_offset":4608,"parent_record_offset":4572,"ancestor_record_offset":3928,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":52,"record_offset":4744,"parent_record_offset":4720,"ancestor_record_offset":3928,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":52,"record_offset":8324,"parent_record_offset":8300,"ancestor_record_offset":3928,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":52,"record_offset":8832,"parent_record_offset":7904,"ancestor_record_offset":3928,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":52,"record_offset":7632,"parent_record_offset":7600,"ancestor_record_offset":3928,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":52,"record_offset":4460,"parent_record_offset":4344,"ancestor_record_offset":3928,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":52,"record_offset":4812,"parent_record_offset":4784,"ancestor_record_offset":3928,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":52,"record_offset":5420,"parent_record_offset":5388,"ancestor_record_offset":3928,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":52,"record_offset":7656,"parent_record_offset":7632,"ancestor_record_offset":3928,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":52,"record_offset":4840,"parent_record_offset":4812,"ancestor_record_offset":3928,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":52,"record_offset":5452,"parent_record_offset":5420,"ancestor_record_offset":3928,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":52,"record_offset":6092,"parent_record_offset":6044,"ancestor_record_offset":3928,"inlinee":5475,"source_line":373,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":52,"record_offset":6044,"parent_record_offset":6004,"ancestor_record_offset":3928,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":52,"record_offset":4484,"parent_record_offset":4460,"ancestor_record_offset":3928,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":52,"record_offset":4636,"parent_record_offset":4572,"ancestor_record_offset":3928,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":52,"record_offset":7136,"parent_record_offset":7112,"ancestor_record_offset":3928,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":52,"record_offset":7464,"parent_record_offset":7440,"ancestor_record_offset":3928,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":52,"record_offset":4572,"parent_record_offset":4548,"ancestor_record_offset":3928,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":52,"record_offset":7112,"parent_record_offset":7004,"ancestor_record_offset":3928,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":52,"record_offset":7600,"parent_record_offset":7572,"ancestor_record_offset":3928,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":52,"record_offset":6844,"parent_record_offset":6808,"ancestor_record_offset":3928,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":52,"record_offset":6976,"parent_record_offset":6948,"ancestor_record_offset":3928,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":52,"record_offset":6948,"parent_record_offset":6916,"ancestor_record_offset":3928,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":52,"record_offset":6892,"parent_record_offset":6868,"ancestor_record_offset":3928,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":52,"record_offset":6916,"parent_record_offset":6892,"ancestor_record_offset":3928,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":52,"record_offset":6868,"parent_record_offset":6844,"ancestor_record_offset":3928,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":52,"record_offset":7440,"parent_record_offset":6916,"ancestor_record_offset":3928,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":52,"record_offset":7572,"parent_record_offset":7540,"ancestor_record_offset":3928,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":52,"record_offset":7720,"parent_record_offset":7572,"ancestor_record_offset":3928,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":52,"record_offset":7540,"parent_record_offset":7512,"ancestor_record_offset":3928,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":52,"record_offset":7512,"parent_record_offset":6808,"ancestor_record_offset":3928,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":52,"record_offset":7744,"parent_record_offset":7720,"ancestor_record_offset":3928,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":52,"record_offset":7004,"parent_record_offset":6976,"ancestor_record_offset":3928,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":52,"record_offset":7252,"parent_record_offset":7004,"ancestor_record_offset":3928,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":52,"record_offset":7224,"parent_record_offset":7004,"ancestor_record_offset":3928,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":52,"record_offset":7056,"parent_record_offset":7004,"ancestor_record_offset":3928,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":52,"record_offset":7864,"parent_record_offset":7824,"ancestor_record_offset":3928,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":52,"record_offset":7984,"parent_record_offset":7944,"ancestor_record_offset":3928,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":52,"record_offset":8132,"parent_record_offset":7944,"ancestor_record_offset":3928,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":52,"record_offset":8404,"parent_record_offset":8132,"ancestor_record_offset":3928,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::device::GraphicsDevice::issue_and_reset_pipeline_barrier
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/barrier.rs
//     evidence={"module":52,"record_offset":10108,"parent_record_offset":9092,"ancestor_record_offset":3928,"inlinee":7657,"source_line":419,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":52,"record_offset":5020,"parent_record_offset":4976,"ancestor_record_offset":3928,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":52,"record_offset":5076,"parent_record_offset":4976,"ancestor_record_offset":3928,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":52,"record_offset":5048,"parent_record_offset":4976,"ancestor_record_offset":3928,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":52,"record_offset":6336,"parent_record_offset":5728,"ancestor_record_offset":3928,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$4::get_tracked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":52,"record_offset":4976,"parent_record_offset":4196,"ancestor_record_offset":3928,"inlinee":7646,"source_line":328,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::graph::ExportableResource::index
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":52,"record_offset":9348,"parent_record_offset":9092,"ancestor_record_offset":3928,"inlinee":7661,"source_line":37,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::graph::ExportableResource::raw
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":52,"record_offset":9372,"parent_record_offset":9348,"ancestor_record_offset":3928,"inlinee":7662,"source_line":29,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":52,"record_offset":8008,"parent_record_offset":7984,"ancestor_record_offset":3928,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":52,"record_offset":8036,"parent_record_offset":8008,"ancestor_record_offset":3928,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":52,"record_offset":7944,"parent_record_offset":7904,"ancestor_record_offset":3928,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":52,"record_offset":7904,"parent_record_offset":7864,"ancestor_record_offset":3928,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::CompiledRenderGraph::gather_batch_preamble_barriers
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 232
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DDC
// 首条位置: {"module":52,"inlinee":7644}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":4344,"parent_record_offset":4196,"ancestor_record_offset":4196,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":4316,"parent_record_offset":4196,"ancestor_record_offset":4196,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":52,"record_offset":4548,"parent_record_offset":4196,"ancestor_record_offset":4196,"inlinee":6077,"source_line":156,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":4784,"parent_record_offset":4196,"ancestor_record_offset":4196,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":52,"record_offset":5244,"parent_record_offset":4196,"ancestor_record_offset":4196,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get_tracked
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":52,"record_offset":4976,"parent_record_offset":4196,"ancestor_record_offset":4196,"inlinee":7646,"source_line":328,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":52,"record_offset":4428,"parent_record_offset":4404,"ancestor_record_offset":4196,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":4404,"parent_record_offset":4344,"ancestor_record_offset":4196,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":4376,"parent_record_offset":4344,"ancestor_record_offset":4196,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":52,"record_offset":4720,"parent_record_offset":4696,"ancestor_record_offset":4196,"inlinee":7344,"source_line":157,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":52,"record_offset":4696,"parent_record_offset":4548,"ancestor_record_offset":4196,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":52,"record_offset":4508,"parent_record_offset":4484,"ancestor_record_offset":4196,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":52,"record_offset":4660,"parent_record_offset":4636,"ancestor_record_offset":4196,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":52,"record_offset":4608,"parent_record_offset":4572,"ancestor_record_offset":4196,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":52,"record_offset":4744,"parent_record_offset":4720,"ancestor_record_offset":4196,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":52,"record_offset":4460,"parent_record_offset":4344,"ancestor_record_offset":4196,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":52,"record_offset":4812,"parent_record_offset":4784,"ancestor_record_offset":4196,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":52,"record_offset":4840,"parent_record_offset":4812,"ancestor_record_offset":4196,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":52,"record_offset":4484,"parent_record_offset":4460,"ancestor_record_offset":4196,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":52,"record_offset":4636,"parent_record_offset":4572,"ancestor_record_offset":4196,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":52,"record_offset":4572,"parent_record_offset":4548,"ancestor_record_offset":4196,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":52,"record_offset":5020,"parent_record_offset":4976,"ancestor_record_offset":4196,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":52,"record_offset":5076,"parent_record_offset":4976,"ancestor_record_offset":4196,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":52,"record_offset":5048,"parent_record_offset":4976,"ancestor_record_offset":4196,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::CompiledRenderGraph::record_postamble
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 268
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DEB
// 首条位置: {"module":52,"inlinee":7659}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":9404,"parent_record_offset":9092,"ancestor_record_offset":9092,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":9244,"parent_record_offset":9092,"ancestor_record_offset":9092,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":52,"record_offset":10144,"parent_record_offset":9092,"ancestor_record_offset":9092,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":52,"record_offset":9272,"parent_record_offset":9092,"ancestor_record_offset":9092,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::issue_and_reset_pipeline_barrier
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/barrier.rs
//     evidence={"module":52,"record_offset":10108,"parent_record_offset":9092,"ancestor_record_offset":9092,"inlinee":7657,"source_line":419,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get_tracked
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":52,"record_offset":9844,"parent_record_offset":9092,"ancestor_record_offset":9092,"inlinee":7646,"source_line":328,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::graph::ExportableResource::index
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":52,"record_offset":9348,"parent_record_offset":9092,"ancestor_record_offset":9092,"inlinee":7661,"source_line":37,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":52,"record_offset":9312,"parent_record_offset":9272,"ancestor_record_offset":9092,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":52,"record_offset":9432,"parent_record_offset":9404,"ancestor_record_offset":9092,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":52,"record_offset":9460,"parent_record_offset":9432,"ancestor_record_offset":9092,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":52,"record_offset":9888,"parent_record_offset":9844,"ancestor_record_offset":9092,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":52,"record_offset":9944,"parent_record_offset":9844,"ancestor_record_offset":9092,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":52,"record_offset":9916,"parent_record_offset":9844,"ancestor_record_offset":9092,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::graph::ExportableResource::raw
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":52,"record_offset":9372,"parent_record_offset":9348,"ancestor_record_offset":9092,"inlinee":7662,"source_line":29,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::CompiledRenderGraph::add_pre_pass_barriers
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 333
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DE7
// 首条位置: {"module":52,"inlinee":7655}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":52,"record_offset":5868,"parent_record_offset":5728,"ancestor_record_offset":5728,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":52,"record_offset":6468,"parent_record_offset":5728,"ancestor_record_offset":5728,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":52,"record_offset":6160,"parent_record_offset":5728,"ancestor_record_offset":5728,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":52,"record_offset":6336,"parent_record_offset":5728,"ancestor_record_offset":5728,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":52,"record_offset":6116,"parent_record_offset":6092,"ancestor_record_offset":5728,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":52,"record_offset":6204,"parent_record_offset":6160,"ancestor_record_offset":5728,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":52,"record_offset":5896,"parent_record_offset":5868,"ancestor_record_offset":5728,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":52,"record_offset":5924,"parent_record_offset":5896,"ancestor_record_offset":5728,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":52,"record_offset":6092,"parent_record_offset":6044,"ancestor_record_offset":5728,"inlinee":5475,"source_line":373,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":52,"record_offset":6044,"parent_record_offset":6004,"ancestor_record_offset":5728,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":52,"record_offset":6380,"parent_record_offset":6336,"ancestor_record_offset":5728,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":52,"record_offset":6436,"parent_record_offset":6336,"ancestor_record_offset":5728,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":52,"record_offset":6408,"parent_record_offset":6336,"ancestor_record_offset":5728,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::CompiledRenderGraph::retire<country_core::render::pass_builder::pass_resources::PassResources>
// 记录形式: compiled_procedure; instances=1
// 源码行: 381
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":52,"record_offset":13888,"rva":482848,"va":5369191968}
// 编译实例 RVAs: 0x75e20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140367730
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3086485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369191968,"raw_hex":"e895182f00","size":5,"soft_fail":false,"target_va":5372278576,"va":5369192086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140378770
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3155982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369191968,"raw_hex":"e80e283000","size":5,"soft_fail":false,"target_va":5372348272,"va":5369192285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140383da0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3202598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369191968,"raw_hex":"e826de3000","size":5,"soft_fail":false,"target_va":5372394912,"va":5369192309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3557579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369191968,"raw_hex":"e8cb483600","size":5,"soft_fail":false,"target_va":5372749712,"va":5369192128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34401325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369191968,"raw_hex":"e82dec0c02","size":5,"soft_fail":false,"target_va":5403593360,"va":5369192030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::on_retire
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7026998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369191968,"raw_hex":"e836396b00","size":5,"soft_fail":false,"target_va":5376219056,"va":5369192053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_registry::ResourceRegistry::release
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7146431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369191968,"raw_hex":"e8bf0b6d00","size":5,"soft_fail":false,"target_va":5376338480,"va":5369192044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":52,"record_offset":14092,"parent_record_offset":13888,"ancestor_record_offset":13888,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":52,"record_offset":14292,"parent_record_offset":14268,"ancestor_record_offset":13888,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":52,"record_offset":14268,"parent_record_offset":14168,"ancestor_record_offset":13888,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":52,"record_offset":14200,"parent_record_offset":14168,"ancestor_record_offset":13888,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":52,"record_offset":14168,"parent_record_offset":14140,"ancestor_record_offset":13888,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":52,"record_offset":14232,"parent_record_offset":14200,"ancestor_record_offset":13888,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":52,"record_offset":14112,"parent_record_offset":14092,"ancestor_record_offset":13888,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00052.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::pass_access_range_indices
// 记录形式: compiled_procedure; instances=1
// 源码行: 394
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":563,"record_offset":1004,"rva":7551232,"va":5376260352}
// 编译实例 RVAs: 0x733900
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::resource_info::ResourceAccessIndex::index
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":563,"record_offset":1140,"parent_record_offset":1004,"ancestor_record_offset":1004,"inlinee":30755,"source_line":189,"archive":"_pdb/A/modules/00563.jsonl.gz"}
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
// 完整结构及原始类型记录: compiled.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::compiled::Batch
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<rhapsody::rg::compiled::Batch,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["rhapsody::rg::compiled::Batch", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<rhapsody::rg::compiled::Batch,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<rhapsody::rg::compiled::Batch,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<rhapsody::rg::compiled::Batch,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["rhapsody::rg::compiled::Batch", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::compiled::BatchCompiler
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 2
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::compiled::CompiledRenderGraph
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 9; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::compiled::CompiledRenderGraph::record_batch", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::compiled::CompiledRenderGraph::compute_batches<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::compiled::CompiledRenderGraph::compute_batches", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::compiled::CompiledRenderGraph::new<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::compiled::CompiledRenderGraph::new", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::compiled::CompiledRenderGraph::retire<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::compiled::CompiledRenderGraph::retire", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::compiled::CompiledRenderGraph<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::compiled::CompiledRenderGraph", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::compiled::CompiledRenderGraph<country_core::render::pass_builder::pass_resources::PassResources> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::compiled::CompiledRenderGraph<country_core::render::pass_builder::pass_resources::PassResources>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: compiled.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: compiled.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 候选参数类型: ["rhapsody::raw::physical_device::PhysicalDevice"]
// [文件级接口候选，具体函数归属 unknown] country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] std::ffi::os_str::OsString (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(ref$<glam::f32::vec2::Vec2>)
// 候选参数类型: ["ref$<glam::f32::vec2::Vec2>"]

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::BatchCompiler::compile
// 信息 ID: 35a2942767203d0a65877c4c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::compiled::CompiledRenderGraph::compute_batches<country_core::render::pass_builder::pass_resources::PassResources>; module=52, inline_record=10920; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::BatchCompiler::flush_batch
// 信息 ID: a38565b068a31466dc9b5553
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::compute_batches<country_core::render::pass_builder::pass_resources::PassResources> @ 0x759de -> 0x733800 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::compute_batches<country_core::render::pass_builder::pass_resources::PassResources> @ 0x75ac3 -> 0x733800 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::compute_batches<country_core::render::pass_builder::pass_resources::PassResources> @ 0x75a96 -> 0x733800 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::BatchCompiler::new
// 信息 ID: 323c10c0eefa5df7d9092647
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::compute_batches<country_core::render::pass_builder::pass_resources::PassResources> @ 0x75971 -> 0x7337b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::CompiledRenderGraph::add_pre_pass_barriers
// 信息 ID: dd1b9b2dce651275e01cc299
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources>; module=52, inline_record=5728; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::CompiledRenderGraph::compute_batches<country_core::render::pass_builder::pass_resources::PassResources>
// 信息 ID: d8a2c5d8c999ba2729753d65
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::compiled::CompiledRenderGraph::compute_batches -> ["country_core::render::pass_builder::pass_resources::PassResources"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2debc6 -> 0x75940 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::CompiledRenderGraph::gather_batch_preamble_barriers
// 信息 ID: bb0b714172f0c1d0d7b7871f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources>; module=52, inline_record=4196; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::CompiledRenderGraph::new<country_core::render::pass_builder::pass_resources::PassResources>
// 信息 ID: ac428ea7ad70d1982b74bbd3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::compiled::CompiledRenderGraph::new -> ["country_core::render::pass_builder::pass_resources::PassResources"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::graph::RenderGraph::compile<country_core::render::pass_builder::pass_resources::PassResources> @ 0x66ac6 -> 0x75c20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::CompiledRenderGraph::provide_lazy_texture
// 信息 ID: e8fa04e3ac1db608447d0dfb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources>; module=178, inline_record=5756; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources>; module=178, inline_record=8148; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources>
// 信息 ID: b30e71d953e939b949f9605b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::compiled::CompiledRenderGraph::record_batch -> ["country_core::render::pass_builder::pass_resources::PassResources"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2dfcd2 -> 0x746b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::CompiledRenderGraph::record_postamble
// 信息 ID: cfe1553d447eab8e07f8036d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources>; module=52, inline_record=9092; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::CompiledRenderGraph::retire<country_core::render::pass_builder::pass_resources::PassResources>
// 信息 ID: f754551c36a7e7205ec58c67
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::compiled::CompiledRenderGraph::retire -> ["country_core::render::pass_builder::pass_resources::PassResources"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2e03fd -> 0x75e20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::impl$1::new::closure$0
// 信息 ID: ddbc7bc22ad2446678445dc0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::rg::compiled::impl$1::new", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::rg::compiled::impl$1::new::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::rg::resource::ResourceHandle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::rg::resource::ResourceHandle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > >; module=73, inline_record=116600; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::rg::resource::ResourceHandle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > >; module=73, inline_record=117584; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::pass_access_range_indices
// 信息 ID: 270d56e2db040c70d62b902a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources> @ 0x74a2e -> 0x733900 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

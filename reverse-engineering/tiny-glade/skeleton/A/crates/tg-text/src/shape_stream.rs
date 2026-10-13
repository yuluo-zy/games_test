// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tg-text/src/shape_stream.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: b940e44fb0623989fdba29fed3016ec86a076fddc80faa41eec50e5ac98a11cb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: shape_stream.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1022, 1023, 1025, 1026, 2447, 3678
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use ron; // namespace candidate
//   package identities: []
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use tg_text; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 7, 51, 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC73D
// 首条位置: {"module":2447,"inlinee":51005}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2447,"record_offset":80168,"parent_record_offset":80088,"ancestor_record_offset":80088,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":2447,"record_offset":16380,"parent_record_offset":16356,"ancestor_record_offset":16356,"inlinee":8726,"source_line":233,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":2447,"record_offset":80112,"parent_record_offset":80088,"ancestor_record_offset":80088,"inlinee":6873,"source_line":1150,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_seq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":2447,"record_offset":80136,"parent_record_offset":80112,"ancestor_record_offset":80088,"inlinee":8558,"source_line":188,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$8::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC747
// 首条位置: {"module":1025,"inlinee":51015}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::ShapeStream::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1022,"record_offset":244,"rva":5686128,"va":5374395248}
// 编译实例 RVAs: 0x56c370
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::ShapeStream::fill_color
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1022,"record_offset":360,"rva":10910144,"va":5379619264}
// 编译实例 RVAs: 0xa679c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a624b0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379619264,"raw_hex":"e857aaffff","size":5,"soft_fail":false,"target_va":5379597488,"va":5379619412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8314082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379619264,"raw_hex":"e81e2381ff","size":5,"soft_fail":false,"target_va":5371305232,"va":5379619309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1022,"record_offset":492,"parent_record_offset":360,"ancestor_record_offset":360,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1022,"record_offset":556,"parent_record_offset":524,"ancestor_record_offset":360,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1022,"record_offset":524,"parent_record_offset":492,"ancestor_record_offset":360,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1022,"record_offset":588,"parent_record_offset":492,"ancestor_record_offset":360,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::ShapeStream::image<ref$<str$> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1023,"record_offset":1300,"rva":10911840,"va":5379620960}
// 编译实例 RVAs: 0xa68060
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6871515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379620960,"raw_hex":"e8252697ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5379621222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a624b0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379620960,"raw_hex":"e816a3ffff","size":5,"soft_fail":false,"target_va":5379597488,"va":5379621269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a67b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379620960,"raw_hex":"e8c5faffff","size":5,"soft_fail":false,"target_va":5379619664,"va":5379620998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8315812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379620960,"raw_hex":"e85c1c81ff","size":5,"soft_fail":false,"target_va":5371305232,"va":5379621039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1023,"record_offset":1472,"parent_record_offset":1300,"ancestor_record_offset":1300,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1023,"record_offset":1444,"parent_record_offset":1300,"ancestor_record_offset":1300,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1023,"record_offset":1800,"parent_record_offset":1776,"ancestor_record_offset":1300,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1023,"record_offset":1776,"parent_record_offset":1752,"ancestor_record_offset":1300,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1023,"record_offset":1548,"parent_record_offset":1516,"ancestor_record_offset":1300,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1023,"record_offset":1752,"parent_record_offset":1728,"ancestor_record_offset":1300,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1023,"record_offset":1516,"parent_record_offset":1472,"ancestor_record_offset":1300,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1023,"record_offset":1608,"parent_record_offset":1472,"ancestor_record_offset":1300,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1023,"record_offset":1580,"parent_record_offset":1472,"ancestor_record_offset":1300,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::ShapeStream::image<u32>
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1023,"record_offset":1868,"rva":10912176,"va":5379621296}
// 编译实例 RVAs: 0xa681b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6871853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379621296,"raw_hex":"e8d32497ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5379621560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a624b0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379621296,"raw_hex":"e8c4a1ffff","size":5,"soft_fail":false,"target_va":5379597488,"va":5379621607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a67b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379621296,"raw_hex":"e853f9ffff","size":5,"soft_fail":false,"target_va":5379619632,"va":5379621336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8316150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379621296,"raw_hex":"e80a1b81ff","size":5,"soft_fail":false,"target_va":5371305232,"va":5379621377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1023,"record_offset":2032,"parent_record_offset":1868,"ancestor_record_offset":1868,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1023,"record_offset":2004,"parent_record_offset":1868,"ancestor_record_offset":1868,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1023,"record_offset":2360,"parent_record_offset":2336,"ancestor_record_offset":1868,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1023,"record_offset":2336,"parent_record_offset":2312,"ancestor_record_offset":1868,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1023,"record_offset":2108,"parent_record_offset":2076,"ancestor_record_offset":1868,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1023,"record_offset":2312,"parent_record_offset":2288,"ancestor_record_offset":1868,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1023,"record_offset":2076,"parent_record_offset":2032,"ancestor_record_offset":1868,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1023,"record_offset":2168,"parent_record_offset":2032,"ancestor_record_offset":1868,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1023,"record_offset":2140,"parent_record_offset":2032,"ancestor_record_offset":1868,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::ShapeStream::corner_radius
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1022,"record_offset":624,"rva":10910320,"va":5379619440}
// 编译实例 RVAs: 0xa67a70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a624b0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379619440,"raw_hex":"e8a3a9ffff","size":5,"soft_fail":false,"target_va":5379597488,"va":5379619592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8314261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379619440,"raw_hex":"e86b2281ff","size":5,"soft_fail":false,"target_va":5371305232,"va":5379619488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1022,"record_offset":760,"parent_record_offset":624,"ancestor_record_offset":624,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1022,"record_offset":824,"parent_record_offset":792,"ancestor_record_offset":624,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1022,"record_offset":792,"parent_record_offset":760,"ancestor_record_offset":624,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1022,"record_offset":856,"parent_record_offset":760,"ancestor_record_offset":624,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::ShapeStream::is_empty
// 记录形式: compiled_procedure; instances=1
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1022,"record_offset":892,"rva":10910496,"va":5379619616}
// 编译实例 RVAs: 0xa67b20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1022,"record_offset":1020,"parent_record_offset":892,"ancestor_record_offset":892,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1023,"record_offset":640,"rva":10911584,"va":5379620704}
// 编译实例 RVAs: 0xa67f60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t23340954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379620704,"raw_hex":"e89a276401","size":5,"soft_fail":false,"target_va":5402961856,"va":5379620897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1023,"record_offset":816,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1023,"record_offset":840,"parent_record_offset":816,"ancestor_record_offset":640,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1023,"record_offset":864,"parent_record_offset":840,"ancestor_record_offset":640,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1023,"record_offset":888,"parent_record_offset":864,"ancestor_record_offset":640,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 51, 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x145E1
// 首条位置: {"module":2447,"inlinee":83425}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":2447,"record_offset":8948,"parent_record_offset":8920,"ancestor_record_offset":8920,"inlinee":10204,"source_line":747,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2447,"record_offset":11096,"parent_record_offset":11072,"ancestor_record_offset":10888,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2447,"record_offset":8984,"parent_record_offset":8948,"ancestor_record_offset":8920,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2447,"record_offset":9048,"parent_record_offset":8948,"ancestor_record_offset":8920,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2447,"record_offset":11120,"parent_record_offset":11096,"ancestor_record_offset":10888,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2447,"record_offset":11144,"parent_record_offset":11120,"ancestor_record_offset":10888,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2447,"record_offset":11072,"parent_record_offset":11040,"ancestor_record_offset":10888,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2447,"record_offset":9020,"parent_record_offset":8948,"ancestor_record_offset":8920,"inlinee":9500,"source_line":1504,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::shape_stream::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":2447,"record_offset":11040,"parent_record_offset":11008,"ancestor_record_offset":10888,"inlinee":51001,"source_line":85,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC738
// 首条位置: {"module":2447,"inlinee":51000}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2447,"record_offset":62928,"parent_record_offset":62876,"ancestor_record_offset":62876,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2447,"record_offset":62900,"parent_record_offset":62876,"ancestor_record_offset":62876,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize<ref_mut$<ron::de::tag::Deserializer> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1023,"record_offset":2428,"rva":10912512,"va":5379621632}
// 编译实例 RVAs: 0xa68300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a67b70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379621632,"raw_hex":"e82bf8ffff","size":5,"soft_fail":false,"target_va":5379619696,"va":5379621696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141608c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12192120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379621632,"raw_hex":"e87809ba00","size":5,"soft_fail":false,"target_va":5391813776,"va":5379621651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1023,"record_offset":2684,"parent_record_offset":2428,"ancestor_record_offset":2428,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1023,"record_offset":2600,"parent_record_offset":2428,"ancestor_record_offset":2428,"inlinee":6890,"source_line":700,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1023,"record_offset":2712,"parent_record_offset":2684,"ancestor_record_offset":2428,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_string
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":1023,"record_offset":2648,"parent_record_offset":2624,"ancestor_record_offset":2428,"inlinee":7915,"source_line":455,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_string
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":1023,"record_offset":2624,"parent_record_offset":2600,"ancestor_record_offset":2428,"inlinee":7913,"source_line":139,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$19::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC761
// 首条位置: {"module":1026,"inlinee":51041}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::borrow::impl$2::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\borrow.rs
//     evidence={"module":1026,"record_offset":5608,"parent_record_offset":5568,"ancestor_record_offset":5568,"inlinee":4409,"source_line":195,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1026,"record_offset":5932,"parent_record_offset":5908,"ancestor_record_offset":5568,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1026,"record_offset":5956,"parent_record_offset":5932,"ancestor_record_offset":5568,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1026,"record_offset":5908,"parent_record_offset":5872,"ancestor_record_offset":5568,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1026,"record_offset":5872,"parent_record_offset":5844,"ancestor_record_offset":5568,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1026,"record_offset":5844,"parent_record_offset":5816,"ancestor_record_offset":5568,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1026,"record_offset":6028,"parent_record_offset":5844,"ancestor_record_offset":5568,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1026,"record_offset":5780,"parent_record_offset":5752,"ancestor_record_offset":5568,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1026,"record_offset":5752,"parent_record_offset":5724,"ancestor_record_offset":5568,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1026,"record_offset":5696,"parent_record_offset":5668,"ancestor_record_offset":5568,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1026,"record_offset":5724,"parent_record_offset":5696,"ancestor_record_offset":5568,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1026,"record_offset":5668,"parent_record_offset":5640,"ancestor_record_offset":5568,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1026,"record_offset":5640,"parent_record_offset":5608,"ancestor_record_offset":5568,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1026,"record_offset":5816,"parent_record_offset":5780,"ancestor_record_offset":5568,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1026,"record_offset":6092,"parent_record_offset":6068,"ancestor_record_offset":5568,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1026,"record_offset":6068,"parent_record_offset":5780,"ancestor_record_offset":5568,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1026,"record_offset":5984,"parent_record_offset":5956,"ancestor_record_offset":5568,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$1::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1022,"record_offset":1048,"rva":10910512,"va":5379619632}
// 编译实例 RVAs: 0xa67b30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$2::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1022,"record_offset":1160,"rva":10910544,"va":5379619664}
// 编译实例 RVAs: 0xa67b50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$3::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1022,"record_offset":1272,"rva":10910576,"va":5379619696}
// 编译实例 RVAs: 0xa67b70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC739
// 首条位置: {"module":2447,"inlinee":51001}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2447,"record_offset":11072,"parent_record_offset":11040,"ancestor_record_offset":11040,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2447,"record_offset":11096,"parent_record_offset":11072,"ancestor_record_offset":11040,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2447,"record_offset":11120,"parent_record_offset":11096,"ancestor_record_offset":11040,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2447,"record_offset":11144,"parent_record_offset":11120,"ancestor_record_offset":11040,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC73A
// 首条位置: {"module":2447,"inlinee":51002}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2447,"record_offset":15656,"parent_record_offset":15628,"ancestor_record_offset":15628,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2447,"record_offset":15680,"parent_record_offset":15656,"ancestor_record_offset":15628,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2447,"record_offset":15704,"parent_record_offset":15680,"ancestor_record_offset":15628,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2447,"record_offset":15728,"parent_record_offset":15704,"ancestor_record_offset":15628,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x145E9
// 首条位置: {"module":2447,"inlinee":83433}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":2447,"record_offset":15520,"parent_record_offset":15488,"ancestor_record_offset":15488,"inlinee":10204,"source_line":747,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2447,"record_offset":15680,"parent_record_offset":15656,"ancestor_record_offset":15488,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2447,"record_offset":15564,"parent_record_offset":15520,"ancestor_record_offset":15488,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2447,"record_offset":15704,"parent_record_offset":15680,"ancestor_record_offset":15488,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2447,"record_offset":15728,"parent_record_offset":15704,"ancestor_record_offset":15488,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2447,"record_offset":15656,"parent_record_offset":15628,"ancestor_record_offset":15488,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2447,"record_offset":15600,"parent_record_offset":15520,"ancestor_record_offset":15488,"inlinee":9500,"source_line":1504,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":2447,"record_offset":15628,"parent_record_offset":15600,"ancestor_record_offset":15488,"inlinee":51002,"source_line":85,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x145E7
// 首条位置: {"module":2447,"inlinee":83431}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2447,"record_offset":16044,"parent_record_offset":14716,"ancestor_record_offset":14716,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2447,"record_offset":17448,"parent_record_offset":14716,"ancestor_record_offset":14716,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2447,"record_offset":15944,"parent_record_offset":14716,"ancestor_record_offset":14716,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2447,"record_offset":17900,"parent_record_offset":14716,"ancestor_record_offset":14716,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2447,"record_offset":14768,"parent_record_offset":14716,"ancestor_record_offset":14716,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2447,"record_offset":15980,"parent_record_offset":14716,"ancestor_record_offset":14716,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2447,"record_offset":17160,"parent_record_offset":17136,"ancestor_record_offset":14716,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2447,"record_offset":17136,"parent_record_offset":17112,"ancestor_record_offset":14716,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2447,"record_offset":17112,"parent_record_offset":17088,"ancestor_record_offset":14716,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2447,"record_offset":15680,"parent_record_offset":15656,"ancestor_record_offset":14716,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2447,"record_offset":14980,"parent_record_offset":14808,"ancestor_record_offset":14716,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2447,"record_offset":15020,"parent_record_offset":14980,"ancestor_record_offset":14716,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2447,"record_offset":15156,"parent_record_offset":14808,"ancestor_record_offset":14716,"inlinee":4980,"source_line":1499,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2447,"record_offset":16988,"parent_record_offset":16628,"ancestor_record_offset":14716,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2447,"record_offset":14940,"parent_record_offset":14808,"ancestor_record_offset":14716,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2447,"record_offset":15704,"parent_record_offset":15680,"ancestor_record_offset":14716,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2447,"record_offset":15728,"parent_record_offset":15704,"ancestor_record_offset":14716,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2447,"record_offset":15656,"parent_record_offset":15628,"ancestor_record_offset":14716,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/id.rs
//     evidence={"module":2447,"record_offset":15060,"parent_record_offset":14808,"ancestor_record_offset":14716,"inlinee":10226,"source_line":11,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":2447,"record_offset":15520,"parent_record_offset":15488,"ancestor_record_offset":14716,"inlinee":10204,"source_line":747,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":2447,"record_offset":14808,"parent_record_offset":14768,"ancestor_record_offset":14716,"inlinee":10224,"source_line":838,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_value_seed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":2447,"record_offset":16108,"parent_record_offset":16072,"ancestor_record_offset":14716,"inlinee":10245,"source_line":857,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::Deserializer::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":2447,"record_offset":16852,"parent_record_offset":16628,"ancestor_record_offset":14716,"inlinee":83435,"source_line":10,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":2447,"record_offset":16380,"parent_record_offset":16356,"ancestor_record_offset":14716,"inlinee":8726,"source_line":233,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2447,"record_offset":15600,"parent_record_offset":15520,"ancestor_record_offset":14716,"inlinee":9500,"source_line":1504,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2447,"record_offset":15100,"parent_record_offset":14808,"ancestor_record_offset":14716,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::shape_stream::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":2447,"record_offset":16356,"parent_record_offset":16332,"ancestor_record_offset":14716,"inlinee":51005,"source_line":7,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":2447,"record_offset":15628,"parent_record_offset":15600,"ancestor_record_offset":14716,"inlinee":51002,"source_line":85,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":2447,"record_offset":15124,"parent_record_offset":15100,"ancestor_record_offset":14716,"inlinee":83433,"source_line":85,"archive":"_pdb/A/modules/02447.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum<ron::de::Enum>
// 记录形式: compiled_procedure; instances=1
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1023,"record_offset":244,"rva":10911184,"va":5379620304}
// 编译实例 RVAs: 0xa67dd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1415fe650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12150891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379620304,"raw_hex":"e86b68b900","size":5,"soft_fail":false,"target_va":5391771216,"va":5379620320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415ff410
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12154354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379620304,"raw_hex":"e8f275b900","size":5,"soft_fail":false,"target_va":5391774736,"va":5379620377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141600900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12159495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379620304,"raw_hex":"e8078ab900","size":5,"soft_fail":false,"target_va":5391780096,"va":5379620596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141601380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12162226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379620304,"raw_hex":"e8b294b900","size":5,"soft_fail":false,"target_va":5391782784,"va":5379620553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1023,"record_offset":544,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1023,"record_offset":452,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1023,"record_offset":488,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1023,"record_offset":424,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   [pdb_inline_direct] serde::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1023,"record_offset":516,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":10107,"source_line":2117,"archive":"_pdb/A/modules/01023.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$21::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC75F
// 首条位置: {"module":1026,"inlinee":51039}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tg_text::shape_stream::impl$19::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":1026,"record_offset":5568,"parent_record_offset":5512,"ancestor_record_offset":5512,"inlinee":51041,"source_line":60,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1026,"record_offset":5932,"parent_record_offset":5908,"ancestor_record_offset":5512,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1026,"record_offset":5956,"parent_record_offset":5932,"ancestor_record_offset":5512,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1026,"record_offset":5908,"parent_record_offset":5872,"ancestor_record_offset":5512,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$2::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\borrow.rs
//     evidence={"module":1026,"record_offset":5608,"parent_record_offset":5568,"ancestor_record_offset":5512,"inlinee":4409,"source_line":195,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1026,"record_offset":5872,"parent_record_offset":5844,"ancestor_record_offset":5512,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1026,"record_offset":5844,"parent_record_offset":5816,"ancestor_record_offset":5512,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1026,"record_offset":6028,"parent_record_offset":5844,"ancestor_record_offset":5512,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1026,"record_offset":5780,"parent_record_offset":5752,"ancestor_record_offset":5512,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1026,"record_offset":5752,"parent_record_offset":5724,"ancestor_record_offset":5512,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1026,"record_offset":5696,"parent_record_offset":5668,"ancestor_record_offset":5512,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1026,"record_offset":5724,"parent_record_offset":5696,"ancestor_record_offset":5512,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1026,"record_offset":5668,"parent_record_offset":5640,"ancestor_record_offset":5512,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1026,"record_offset":5640,"parent_record_offset":5608,"ancestor_record_offset":5512,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1026,"record_offset":5816,"parent_record_offset":5780,"ancestor_record_offset":5512,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1026,"record_offset":6092,"parent_record_offset":6068,"ancestor_record_offset":5512,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1026,"record_offset":6068,"parent_record_offset":5780,"ancestor_record_offset":5512,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1026,"record_offset":5984,"parent_record_offset":5956,"ancestor_record_offset":5512,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01026.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$22::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC71A
// 首条位置: {"module":1022,"inlinee":50970}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$4::image_name_to_index
// 记录形式: compiled_procedure; instances=1
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1022,"record_offset":1384,"rva":10910608,"va":5379619728}
// 编译实例 RVAs: 0xa67b90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23359396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379619728,"raw_hex":"e8a46f6401","size":5,"soft_fail":false,"target_va":5402979152,"va":5379619751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::RawShapeStream::new
// 记录形式: compiled_procedure, inlinee_declaration; instances=2
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC718
// 首条位置: {"module":1022,"inlinee":50968}
// 编译实例 RVAs: 0xa67bb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tg_text::shape_stream::impl$22::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":1022,"record_offset":1640,"parent_record_offset":1512,"ancestor_record_offset":1512,"inlinee":50970,"source_line":95,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::RawShapeStream::append
// 记录形式: compiled_procedure; instances=1
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1022,"record_offset":1668,"rva":10910672,"va":5379619792}
// 编译实例 RVAs: 0xa67bd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23298489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379619792,"raw_hex":"e8b9816301","size":5,"soft_fail":false,"target_va":5402918528,"va":5379620034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rdi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379619792,"raw_hex":"ff5718","size":3,"soft_fail":false,"va":5379619893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1022,"record_offset":1816,"parent_record_offset":1668,"ancestor_record_offset":1668,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1022,"record_offset":2228,"parent_record_offset":1668,"ancestor_record_offset":1668,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1022,"record_offset":1844,"parent_record_offset":1668,"ancestor_record_offset":1668,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_direct] tg_text::shape_stream::Image::with_scaling
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":1022,"record_offset":1944,"parent_record_offset":1668,"ancestor_record_offset":1668,"inlinee":50973,"source_line":190,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_direct] tg_text::shape_stream::RawShapeStream::encode
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":1022,"record_offset":1972,"parent_record_offset":1668,"ancestor_record_offset":1668,"inlinee":50974,"source_line":203,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1022,"record_offset":2724,"parent_record_offset":2696,"ancestor_record_offset":1668,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1022,"record_offset":2144,"parent_record_offset":2116,"ancestor_record_offset":1668,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1022,"record_offset":2808,"parent_record_offset":2776,"ancestor_record_offset":1668,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1022,"record_offset":2116,"parent_record_offset":2088,"ancestor_record_offset":1668,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1022,"record_offset":2056,"parent_record_offset":2028,"ancestor_record_offset":1668,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1022,"record_offset":2776,"parent_record_offset":2596,"ancestor_record_offset":1668,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1022,"record_offset":2184,"parent_record_offset":2056,"ancestor_record_offset":1668,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1022,"record_offset":2088,"parent_record_offset":2056,"ancestor_record_offset":1668,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1022,"record_offset":2000,"parent_record_offset":1972,"ancestor_record_offset":1668,"inlinee":14886,"source_line":3157,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1022,"record_offset":2028,"parent_record_offset":2000,"ancestor_record_offset":1668,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1022,"record_offset":1904,"parent_record_offset":1844,"ancestor_record_offset":1668,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::RawShapeStream::clear
// 记录形式: compiled_procedure, inlinee_declaration; instances=2
// 源码行: 162, 163
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC71B
// 首条位置: {"module":1022,"inlinee":50971}
// 编译实例 RVAs: 0x7bf410
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1022,"record_offset":3016,"parent_record_offset":2888,"ancestor_record_offset":2888,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::Image::with_scaling
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 190
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC71D
// 首条位置: {"module":1022,"inlinee":50973}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::RawShapeStream::encode
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 203
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC71E
// 首条位置: {"module":1022,"inlinee":50974}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1022,"record_offset":2000,"parent_record_offset":1972,"ancestor_record_offset":1972,"inlinee":14886,"source_line":3157,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1022,"record_offset":2724,"parent_record_offset":2696,"ancestor_record_offset":2512,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1022,"record_offset":2144,"parent_record_offset":2116,"ancestor_record_offset":1972,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1022,"record_offset":2808,"parent_record_offset":2776,"ancestor_record_offset":2512,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1022,"record_offset":2116,"parent_record_offset":2088,"ancestor_record_offset":1972,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1022,"record_offset":2056,"parent_record_offset":2028,"ancestor_record_offset":1972,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1022,"record_offset":2776,"parent_record_offset":2596,"ancestor_record_offset":2512,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1022,"record_offset":2184,"parent_record_offset":2056,"ancestor_record_offset":1972,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1022,"record_offset":2088,"parent_record_offset":2056,"ancestor_record_offset":1972,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01022.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1022,"record_offset":2028,"parent_record_offset":2000,"ancestor_record_offset":1972,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/01022.jsonl.gz"}
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
// 完整结构及原始类型记录: shape_stream.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_text::shape_stream::Image
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_text::shape_stream::NoImageNameMapping
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tg_text::shape_stream::NoImageNameMapping>", "constructor": "core::ptr::drop_in_place", "arguments": ["tg_text::shape_stream::NoImageNameMapping"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_text::shape_stream::RawShapeStream
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 3
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_text::shape_stream::ShapeStream
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tg_text::shape_stream::ShapeStream>", "constructor": "core::ptr::drop_in_place", "arguments": ["tg_text::shape_stream::ShapeStream"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tg_text::shape_stream::ShapeStream::image<ref$<str$> >", "constructor": "tg_text::shape_stream::ShapeStream::image", "arguments": ["ref$<str$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tg_text::shape_stream::ShapeStream::image<u32>", "constructor": "tg_text::shape_stream::ShapeStream::image", "arguments": ["u32"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_text::shape_stream::ShapeStreamItem
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tg_text::shape_stream::ShapeStreamItem>", "constructor": "enum2$", "arguments": ["tg_text::shape_stream::ShapeStreamItem"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<enum2$<tg_text::shape_stream::ShapeStreamItem>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["enum2$<tg_text::shape_stream::ShapeStreamItem>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<enum2$<tg_text::shape_stream::ShapeStreamItem>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<enum2$<tg_text::shape_stream::ShapeStreamItem>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$10::clone<enum2$<tg_text::shape_stream::ShapeStreamItem>,alloc::alloc::Global>", "constructor": "alloc::vec::impl$10::clone", "arguments": ["enum2$<tg_text::shape_stream::ShapeStreamItem>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<enum2$<tg_text::shape_stream::ShapeStreamItem> >", "constructor": "core::marker::PhantomData", "arguments": ["enum2$<tg_text::shape_stream::ShapeStreamItem>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$5::next_element_seed<core::marker::PhantomData<enum2$<tg_text::shape_stream::ShapeStreamItem> > >", "constructor": "ron::de::impl$5::next_element_seed", "arguments": ["core::marker::PhantomData<enum2$<tg_text::shape_stream::ShapeStreamItem> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<enum2$<tg_text::shape_stream::ShapeStreamItem> >", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["enum2$<tg_text::shape_stream::ShapeStreamItem>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_seq<serde::de::impls::impl$19::deserialize::VecVisitor<enum2$<tg_text::shape_stream::ShapeStreamItem> > >", "constructor": "ron::de::impl$2::deserialize_seq", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<enum2$<tg_text::shape_stream::ShapeStreamItem> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<enum2$<tg_text::shape_stream::ShapeStreamItem>,ron::de::CommaSeparated>", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["enum2$<tg_text::shape_stream::ShapeStreamItem>", "ron::de::CommaSeparated"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: shape_stream.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: shape_stream.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::Image::with_scaling
// 信息 ID: 271f9ff521b51d94d2b91cb3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_text::shape_stream::RawShapeStream::append; module=1022, inline_record=1944; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::RawShapeStream::append
// 信息 ID: 15cb44cb912fb5f5f6f2c3ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$11::prepare @ 0x43b81f -> 0xa67bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$14::prepare @ 0x43bb4f -> 0xa67bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$21::prepare @ 0x43bf5f -> 0xa67bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$21::prepare @ 0x43c0b9 -> 0xa67bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::credits::impl$3::render @ 0x15eab07 -> 0xa67bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::impl$10::prepare @ 0x15ecded -> 0xa67bd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::RawShapeStream::clear
// 信息 ID: 6eae7913e0d4b9f89e613214
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 44 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$11::prepare @ 0x43b7cd -> 0x7bf410 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$14::prepare @ 0x43bb39 -> 0x7bf410 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$21::prepare @ 0x43bf05 -> 0x7bf410 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$21::prepare @ 0x43bfe9 -> 0x7bf410 (direct_call); 同地址别名未裁定。
// [静态直接引用] minidumper::ipc::server::Server::run @ 0x4cf696 -> 0x7bf410 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::impl$10::prepare @ 0x15ecda1 -> 0x7bf410 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::geometry::BoundsBuilder>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x18a8f37 -> 0x7bf410 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::geometry::BoundsBuilder>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x18a8e72 -> 0x7bf410 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::path_builder::TransformSink<zeno::geometry::BoundsBuilder>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x18a96f2 -> 0x7bf410 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x18a99f9 -> 0x7bf410 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x18a98fc -> 0x7bf410 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::scratch::impl$1::apply::closure_env$0<ref$<ref$<slice2$<enum2$<zeno::command::Command> > > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x18ab124 -> 0x7bf410 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x18ab471 -> 0x7bf410 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x18ab37b -> 0x7bf410 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::raster::Rasterizer<zeno::raster::HeapStorage>,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x18acb24 -> 0x7bf410 (direct_call); 同地址别名未裁定。
// [静态直接引用] zeno::stroke::stroke_with_storage<core::iter::adapters::copied::Copied<core::slice::iter::Iter<enum2$<zeno::command::Command> > >,zeno::geometry::BoundsBuilder,alloc::vec::Vec<enum2$<zeno::segment::Segment>,alloc::alloc::Global> > @ 0x18acdf7 -> 0x7bf410 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::RawShapeStream::encode
// 信息 ID: 9ca1cacf4524821f494f5c9d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_text::shape_stream::RawShapeStream::append; module=1022, inline_record=1972; 不是独立调用边。
// [PDB 内联宿主] tg_text::shape_stream::RawShapeStream::append; module=1022, inline_record=2256; 不是独立调用边。
// [PDB 内联宿主] tg_text::shape_stream::RawShapeStream::append; module=1022, inline_record=2512; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::RawShapeStream::new
// 信息 ID: d8005c40385118298894ed94
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::credits::impl$3::render @ 0x15eaad0 -> 0xa67bb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::ShapeStream::corner_radius
// 信息 ID: 39de106b3197d303c5c3b794
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$21::prepare @ 0x43bf45 -> 0xa67a70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::ShapeStream::fill_color
// 信息 ID: 43b3d51bf9c3505bc95397cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$11::prepare @ 0x43b806 -> 0xa679c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$21::prepare @ 0x43bf32 -> 0xa679c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$21::prepare @ 0x43c08e -> 0xa679c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::credits::impl$3::render @ 0x15eaae9 -> 0xa679c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::impl$10::prepare @ 0x15ecdd7 -> 0xa679c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::ShapeStream::image<ref$<str$> >
// 信息 ID: dc65843a74d162c5435aea26
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tg_text::shape_stream::ShapeStream::image -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$21::prepare @ 0x43c00e -> 0xa68060 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::impl$10::prepare @ 0x15ecdc9 -> 0xa68060 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::ShapeStream::image<u32>
// 信息 ID: 6c7a4213fe9c54553056c619
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_text::shape_stream::ShapeStream::image -> ["u32"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$11::prepare @ 0x43b7f3 -> 0xa681b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::ShapeStream::is_empty
// 信息 ID: 032dbeae9052e81aac80fdae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$14::render @ 0x43bb7c -> 0xa67b20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::ShapeStream::new
// 信息 ID: b4eb977892a8713d8a3d39ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::app_systems::impl$0::add_wall_events @ 0x252b1b -> 0x56c370 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$11::prepare @ 0x43b7d9 -> 0x56c370 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$21::prepare @ 0x43bf11 -> 0x56c370 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$21::prepare @ 0x43bff5 -> 0x56c370 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::credits::impl$3::render @ 0x15eaad9 -> 0x56c370 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::impl$10::prepare @ 0x15ecdad -> 0x56c370 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize
// 信息 ID: 43ebad8051f51fe7d0cf5dc4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$9::struct_variant<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=2447, inline_record=16356; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$5::next_element_seed<core::marker::PhantomData<enum2$<tg_text::shape_stream::ShapeStreamItem> > >; module=2447, inline_record=34336; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<toolbar::mei::config::_::impl$0::deserialize::__Visitor>; module=2447, inline_record=80088; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: aa7900653c389cec99eeaa04
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<tg_text::shape_stream::_::impl$0::deserialize::__Field> >; module=2447, inline_record=11040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 信息 ID: 292e0a8c5733866750f30bda
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] tg_text::shape_stream::_::impl$0::deserialize::impl$0::visit_str -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$8::variant_seed<core::marker::PhantomData<tg_text::shape_stream::_::impl$0::deserialize::__Field> > @ 0x15fe007 -> 0xa67f60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 40bd9dc53a863ec7739a4e9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<tg_text::shape_stream::_::impl$0::deserialize::__Field> >; module=2447, inline_record=8920; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<tg_text::shape_stream::_::impl$0::deserialize::__Field> >; module=2447, inline_record=10888; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: ac30c7923e700f27b8bfe0d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<tg_text::shape_stream::_::impl$0::deserialize::__Visitor>; module=2447, inline_record=62876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 信息 ID: b3d3b21d93167b5558303fc4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$9::struct_variant<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=2447, inline_record=15628; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__FieldVisitor>; module=3678, inline_record=4356; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__FieldVisitor>; module=3678, inline_record=4776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 信息 ID: 918a43dd8acd21ad8e3255c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$9::struct_variant<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=2447, inline_record=15124; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$9::struct_variant<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=2447, inline_record=15332; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$9::struct_variant<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=2447, inline_record=15488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 信息 ID: f4b86f4d749b04b4f7777378
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$9::struct_variant<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=2447, inline_record=14716; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum<ron::de::Enum>
// 信息 ID: 4c1ce898b251b6d090f2ba45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum -> ["ron::de::Enum"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_enum<tg_text::shape_stream::_::impl$0::deserialize::__Visitor> @ 0x1609404 -> 0xa67dd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize<ref_mut$<ron::de::tag::Deserializer> >
// 信息 ID: 85ceb9d9846b7f8ccaf363cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["ron::de::tag::Deserializer"]; 字段布局 = unknown。
// [类型关系] tg_text::shape_stream::_::impl$0::deserialize -> ["ref_mut$<ron::de::tag::Deserializer>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$9::struct_variant<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x15ffc69 -> 0xa68300 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$19::clone
// 信息 ID: 92b5e60ffc890ad0f6c6237c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$10::clone<enum2$<tg_text::shape_stream::ShapeStreamItem>,alloc::alloc::Global>; module=1026, inline_record=5568; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$1::from
// 信息 ID: 945f9e07ba9b33d047b063b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_text::shape_stream::ShapeStream::image<u32> @ 0xa681d8 -> 0xa67b30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$21::clone
// 信息 ID: 0b2960383681dbf2ae183609
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$10::clone<enum2$<tg_text::shape_stream::ShapeStreamItem>,alloc::alloc::Global>; module=1026, inline_record=5512; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$22::default
// 信息 ID: 5620afccce2d2ea3ff15c5d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_text::shape_stream::RawShapeStream::new; module=1022, inline_record=1640; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$2::from
// 信息 ID: 91ab5cb3eaf393da00ceed5d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_text::shape_stream::ShapeStream::image<ref$<str$> > @ 0xa68086 -> 0xa67b50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$3::from
// 信息 ID: 703263ae1fd66f139f814cf0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_text::shape_stream::_::impl$0::deserialize<ref_mut$<ron::de::tag::Deserializer> > @ 0xa68340 -> 0xa67b70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$4::image_name_to_index
// 信息 ID: ca8f4d8b63b0eb4023c36274
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$8::clone
// 信息 ID: 9d9630ef7da72fa2cafc4687
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::config::instantiate_ui_template_impl; module=1025, inline_record=1968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

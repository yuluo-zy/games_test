// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 5251f4e9315389468e34802d10803b0e29a7a669ca6b2f95e5935be2ea9e6857
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: reflection_ty.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1610, 3127, 4115
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use shader_pipeline; // namespace candidate
//   package identities: [{"name":"shader-pipeline","source":"local_or_path","version":"0.1.0"}]
// use speedy; // namespace candidate
//   package identities: [{"name":"speedy","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$1::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3127,"record_offset":2252,"rva":34329504,"va":5403038624}
// 编译实例 RVAs: 0x20bd3a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20150711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403038624,"raw_hex":"e84986ccfe","size":5,"soft_fail":false,"target_va":5382888144,"va":5403038850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-20146457","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5403038624,"raw_hex":"e9e796ccfe","size":5,"soft_fail":false,"target_va":5382892432,"va":5403038884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3127,"record_offset":2440,"parent_record_offset":2252,"ancestor_record_offset":2252,"inlinee":7062,"source_line":1917,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_direct] core::ops::bit::impl$312::shl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":3127,"record_offset":2384,"parent_record_offset":2252,"ancestor_record_offset":2252,"inlinee":53863,"source_line":35,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::bit::impl$311::shl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\bit.rs
//     evidence={"module":3127,"record_offset":2408,"parent_record_offset":2384,"ancestor_record_offset":2252,"inlinee":53865,"source_line":469,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$9::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3127,"record_offset":4220,"rva":34330448,"va":5403039568}
// 编译实例 RVAs: 0x20bd750
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20147190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403039568,"raw_hex":"e80a94ccfe","size":5,"soft_fail":false,"target_va":5382892432,"va":5403039617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-20151589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403039568,"raw_hex":"e8db82ccfe","size":5,"soft_fail":false,"target_va":5382888144,"va":5403039728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3127,"record_offset":4352,"parent_record_offset":4220,"ancestor_record_offset":4220,"inlinee":7062,"source_line":1917,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3127,"record_offset":4380,"parent_record_offset":4220,"ancestor_record_offset":4220,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3127,"record_offset":4472,"parent_record_offset":4420,"ancestor_record_offset":4220,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3127,"record_offset":4420,"parent_record_offset":4380,"ancestor_record_offset":4220,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$19::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120B2
// 首条位置: {"module":1610,"inlinee":73906}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":130824,"parent_record_offset":130804,"ancestor_record_offset":130804,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":130976,"parent_record_offset":130804,"ancestor_record_offset":130804,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":130908,"parent_record_offset":130880,"ancestor_record_offset":130804,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1610,"record_offset":130940,"parent_record_offset":130880,"ancestor_record_offset":130804,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":130852,"parent_record_offset":130824,"ancestor_record_offset":130804,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":130880,"parent_record_offset":130824,"ancestor_record_offset":130804,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":131000,"parent_record_offset":130976,"ancestor_record_offset":130804,"inlinee":30530,"source_line":116,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":131024,"parent_record_offset":131000,"ancestor_record_offset":130804,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::ScalarType::nbyte
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 64, 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A3C1
// 首条位置: {"module":3127,"inlinee":107457}
// 编译实例 RVAs: 0x20bd380
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$25::clone
// 记录形式: inlinee_declaration; instances=2
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A3D7
// 首条位置: {"module":3127,"inlinee":107479}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$27::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120BA
// 首条位置: {"module":1610,"inlinee":73914}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":132964,"parent_record_offset":132928,"ancestor_record_offset":132928,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":133112,"parent_record_offset":133084,"ancestor_record_offset":132928,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1610,"record_offset":133144,"parent_record_offset":133084,"ancestor_record_offset":132928,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$19::read_from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":132996,"parent_record_offset":132964,"ancestor_record_offset":132928,"inlinee":73906,"source_line":17,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":133056,"parent_record_offset":133028,"ancestor_record_offset":132928,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":133084,"parent_record_offset":133028,"ancestor_record_offset":132928,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":133208,"parent_record_offset":133180,"ancestor_record_offset":132928,"inlinee":30530,"source_line":116,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":133028,"parent_record_offset":132996,"ancestor_record_offset":132928,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":133180,"parent_record_offset":132996,"ancestor_record_offset":132928,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::VectorType::nbyte
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A3C3
// 首条位置: {"module":3127,"inlinee":107459}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] shader_pipeline::reflection_ty::ScalarType::nbyte
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":3127,"record_offset":4860,"parent_record_offset":4836,"ancestor_record_offset":4836,"inlinee":107457,"source_line":64,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$3::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A3CF
// 首条位置: {"module":3127,"inlinee":107471}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3127,"record_offset":5424,"parent_record_offset":5400,"ancestor_record_offset":5400,"inlinee":7062,"source_line":1917,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$36::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120CD
// 首条位置: {"module":1610,"inlinee":73933}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":134128,"parent_record_offset":134104,"ancestor_record_offset":134104,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":134212,"parent_record_offset":134184,"ancestor_record_offset":134104,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1610,"record_offset":134244,"parent_record_offset":134184,"ancestor_record_offset":134104,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":134156,"parent_record_offset":134128,"ancestor_record_offset":134104,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":134184,"parent_record_offset":134128,"ancestor_record_offset":134104,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$43::clone
// 记录形式: inlinee_declaration; instances=2
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A3D5
// 首条位置: {"module":3127,"inlinee":107477}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$45::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120B8
// 首条位置: {"module":1610,"inlinee":73912}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":134024,"parent_record_offset":132856,"ancestor_record_offset":132856,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":133900,"parent_record_offset":132856,"ancestor_record_offset":132856,"inlinee":30524,"source_line":108,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":132892,"parent_record_offset":132856,"ancestor_record_offset":132856,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":133112,"parent_record_offset":133084,"ancestor_record_offset":132856,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1610,"record_offset":133144,"parent_record_offset":133084,"ancestor_record_offset":132856,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$19::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":132996,"parent_record_offset":132964,"ancestor_record_offset":132856,"inlinee":73906,"source_line":17,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$27::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":132928,"parent_record_offset":132892,"ancestor_record_offset":132856,"inlinee":73914,"source_line":116,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$36::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":134104,"parent_record_offset":134080,"ancestor_record_offset":132856,"inlinee":73933,"source_line":140,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$45::read_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":134052,"parent_record_offset":134024,"ancestor_record_offset":132856,"inlinee":73931,"source_line":147,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$45::read_from::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":134448,"parent_record_offset":134420,"ancestor_record_offset":132856,"inlinee":73934,"source_line":147,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":133056,"parent_record_offset":133028,"ancestor_record_offset":132856,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":133084,"parent_record_offset":133028,"ancestor_record_offset":132856,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":133208,"parent_record_offset":133180,"ancestor_record_offset":132856,"inlinee":30530,"source_line":116,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$35::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":134524,"parent_record_offset":134500,"ancestor_record_offset":132856,"inlinee":73903,"source_line":116,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$6::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":134500,"parent_record_offset":134476,"ancestor_record_offset":132856,"inlinee":73925,"source_line":171,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":133028,"parent_record_offset":132996,"ancestor_record_offset":132856,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":134548,"parent_record_offset":134524,"ancestor_record_offset":132856,"inlinee":73904,"source_line":288,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":132964,"parent_record_offset":132928,"ancestor_record_offset":132856,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$45::read_from::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120CB
// 首条位置: {"module":1610,"inlinee":73931}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":134080,"parent_record_offset":134052,"ancestor_record_offset":134052,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":134212,"parent_record_offset":134184,"ancestor_record_offset":134052,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1610,"record_offset":134244,"parent_record_offset":134184,"ancestor_record_offset":134052,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$36::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":134104,"parent_record_offset":134080,"ancestor_record_offset":134052,"inlinee":73933,"source_line":140,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":134156,"parent_record_offset":134128,"ancestor_record_offset":134052,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":134184,"parent_record_offset":134128,"ancestor_record_offset":134052,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":134128,"parent_record_offset":134104,"ancestor_record_offset":134052,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$45::read_from::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120CE
// 首条位置: {"module":1610,"inlinee":73934}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":134476,"parent_record_offset":134448,"ancestor_record_offset":134448,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":134632,"parent_record_offset":134604,"ancestor_record_offset":134448,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1610,"record_offset":134660,"parent_record_offset":134604,"ancestor_record_offset":134448,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":134576,"parent_record_offset":134548,"ancestor_record_offset":134448,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":134604,"parent_record_offset":134548,"ancestor_record_offset":134448,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$35::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":134524,"parent_record_offset":134500,"ancestor_record_offset":134448,"inlinee":73903,"source_line":116,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$6::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":134500,"parent_record_offset":134476,"ancestor_record_offset":134448,"inlinee":73925,"source_line":171,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":134548,"parent_record_offset":134524,"ancestor_record_offset":134448,"inlinee":73904,"source_line":288,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::MatrixType::nbyte
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 169
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A3C9
// 首条位置: {"module":3127,"inlinee":107465}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] shader_pipeline::reflection_ty::VectorType::nbyte
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":3127,"record_offset":4836,"parent_record_offset":4812,"ancestor_record_offset":4812,"inlinee":107459,"source_line":130,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::ScalarType::nbyte
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":3127,"record_offset":4860,"parent_record_offset":4836,"ancestor_record_offset":4812,"inlinee":107457,"source_line":64,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$5::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3127,"record_offset":2696,"rva":34329776,"va":5403038896}
// 编译实例 RVAs: 0x20bd4b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27957669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403038896,"raw_hex":"e85b6655fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5403039152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27957859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403038896,"raw_hex":"e89d6555fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5403039358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d8d6c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20119237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403038896,"raw_hex":"e83b01cdfe","size":5,"soft_fail":false,"target_va":5382919872,"va":5403039104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8501565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403038896,"raw_hex":"e83db98100","size":5,"soft_fail":false,"target_va":5411540963,"va":5403039393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20151193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403038896,"raw_hex":"e86784ccfe","size":5,"soft_fail":false,"target_va":5382888144,"va":5403039332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8503442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403038896,"raw_hex":"e892c08100","size":5,"soft_fail":false,"target_va":5411542880,"va":5403039433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":3127,"record_offset":3244,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3127,"record_offset":2836,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3127,"record_offset":3732,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":7062,"source_line":1917,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3127,"record_offset":3760,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3127,"record_offset":3576,"parent_record_offset":3552,"ancestor_record_offset":2696,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3127,"record_offset":3600,"parent_record_offset":3576,"ancestor_record_offset":2696,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3127,"record_offset":3184,"parent_record_offset":3160,"ancestor_record_offset":2696,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3127,"record_offset":3552,"parent_record_offset":3524,"ancestor_record_offset":2696,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3127,"record_offset":3160,"parent_record_offset":3104,"ancestor_record_offset":2696,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3127,"record_offset":3464,"parent_record_offset":3436,"ancestor_record_offset":2696,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3127,"record_offset":3132,"parent_record_offset":3104,"ancestor_record_offset":2696,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3127,"record_offset":3104,"parent_record_offset":3080,"ancestor_record_offset":2696,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3127,"record_offset":3524,"parent_record_offset":3492,"ancestor_record_offset":2696,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3127,"record_offset":3492,"parent_record_offset":3464,"ancestor_record_offset":2696,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3127,"record_offset":3080,"parent_record_offset":3056,"ancestor_record_offset":2696,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3127,"record_offset":3400,"parent_record_offset":3372,"ancestor_record_offset":2696,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3127,"record_offset":3372,"parent_record_offset":3344,"ancestor_record_offset":2696,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3127,"record_offset":3316,"parent_record_offset":3272,"ancestor_record_offset":2696,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3127,"record_offset":3344,"parent_record_offset":3316,"ancestor_record_offset":2696,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3127,"record_offset":3272,"parent_record_offset":3244,"ancestor_record_offset":2696,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3127,"record_offset":2916,"parent_record_offset":2864,"ancestor_record_offset":2696,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3127,"record_offset":2864,"parent_record_offset":2836,"ancestor_record_offset":2696,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3127,"record_offset":3436,"parent_record_offset":3400,"ancestor_record_offset":2696,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3127,"record_offset":2944,"parent_record_offset":2864,"ancestor_record_offset":2696,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3127,"record_offset":3676,"parent_record_offset":3652,"ancestor_record_offset":2696,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3127,"record_offset":3652,"parent_record_offset":3400,"ancestor_record_offset":2696,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3127,"record_offset":3008,"parent_record_offset":2864,"ancestor_record_offset":2696,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3127,"record_offset":2972,"parent_record_offset":2864,"ancestor_record_offset":2696,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$51::clone
// 记录形式: inlinee_declaration; instances=2
// 源码行: 191
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A3D3
// 首条位置: {"module":3127,"inlinee":107475}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$53::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 191
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120B4
// 首条位置: {"module":1610,"inlinee":73908}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1610,"record_offset":132488,"parent_record_offset":131244,"ancestor_record_offset":131244,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":131800,"parent_record_offset":131244,"ancestor_record_offset":131244,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":131620,"parent_record_offset":131244,"ancestor_record_offset":131244,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":131648,"parent_record_offset":131244,"ancestor_record_offset":131244,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":131676,"parent_record_offset":131244,"ancestor_record_offset":131244,"inlinee":30524,"source_line":108,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":131300,"parent_record_offset":131244,"ancestor_record_offset":131244,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":131512,"parent_record_offset":131488,"ancestor_record_offset":131244,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":131536,"parent_record_offset":131512,"ancestor_record_offset":131244,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":132656,"parent_record_offset":132632,"ancestor_record_offset":131244,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":131456,"parent_record_offset":131416,"ancestor_record_offset":131244,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":131488,"parent_record_offset":131456,"ancestor_record_offset":131244,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":132632,"parent_record_offset":132608,"ancestor_record_offset":131244,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1610,"record_offset":131416,"parent_record_offset":131340,"ancestor_record_offset":131244,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1610,"record_offset":132608,"parent_record_offset":132488,"ancestor_record_offset":131244,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":131988,"parent_record_offset":131960,"ancestor_record_offset":131244,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1610,"record_offset":131764,"parent_record_offset":131736,"ancestor_record_offset":131244,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1610,"record_offset":132520,"parent_record_offset":132488,"ancestor_record_offset":131244,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1610,"record_offset":131564,"parent_record_offset":131536,"ancestor_record_offset":131244,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":131380,"parent_record_offset":131340,"ancestor_record_offset":131244,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$53::read_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":131828,"parent_record_offset":131800,"ancestor_record_offset":131244,"inlinee":73922,"source_line":191,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$53::read_from::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":132220,"parent_record_offset":132192,"ancestor_record_offset":131244,"inlinee":73923,"source_line":191,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":131708,"parent_record_offset":131676,"ancestor_record_offset":131244,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":131736,"parent_record_offset":131676,"ancestor_record_offset":131244,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$24::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":131340,"parent_record_offset":131300,"ancestor_record_offset":131244,"inlinee":73910,"source_line":573,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":131880,"parent_record_offset":131856,"ancestor_record_offset":131244,"inlinee":30530,"source_line":116,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$35::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":132296,"parent_record_offset":132272,"ancestor_record_offset":131244,"inlinee":73903,"source_line":116,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$6::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":132272,"parent_record_offset":132248,"ancestor_record_offset":131244,"inlinee":73925,"source_line":171,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":131904,"parent_record_offset":131880,"ancestor_record_offset":131244,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":132320,"parent_record_offset":132296,"ancestor_record_offset":131244,"inlinee":73904,"source_line":288,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":131856,"parent_record_offset":131828,"ancestor_record_offset":131244,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$53::read_from::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 191
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120C2
// 首条位置: {"module":1610,"inlinee":73922}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":131856,"parent_record_offset":131828,"ancestor_record_offset":131828,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":131988,"parent_record_offset":131960,"ancestor_record_offset":131828,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1610,"record_offset":132016,"parent_record_offset":131960,"ancestor_record_offset":131828,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":131932,"parent_record_offset":131904,"ancestor_record_offset":131828,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":131960,"parent_record_offset":131904,"ancestor_record_offset":131828,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":131880,"parent_record_offset":131856,"ancestor_record_offset":131828,"inlinee":30530,"source_line":116,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":131904,"parent_record_offset":131880,"ancestor_record_offset":131828,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$53::read_from::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 191
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120C3
// 首条位置: {"module":1610,"inlinee":73923}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":132248,"parent_record_offset":132220,"ancestor_record_offset":132220,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":132404,"parent_record_offset":132376,"ancestor_record_offset":132220,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1610,"record_offset":132432,"parent_record_offset":132376,"ancestor_record_offset":132220,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":132348,"parent_record_offset":132320,"ancestor_record_offset":132220,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":132376,"parent_record_offset":132320,"ancestor_record_offset":132220,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$35::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":132296,"parent_record_offset":132272,"ancestor_record_offset":132220,"inlinee":73903,"source_line":116,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$6::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":132272,"parent_record_offset":132248,"ancestor_record_offset":132220,"inlinee":73925,"source_line":171,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":132320,"parent_record_offset":132296,"ancestor_record_offset":132220,"inlinee":73904,"source_line":288,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::ArrayType::nbyte
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 215
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A3CB
// 首条位置: {"module":3127,"inlinee":107467}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3127,"record_offset":5020,"parent_record_offset":4996,"ancestor_record_offset":4996,"inlinee":5059,"source_line":1048,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$7::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 220
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A3CD
// 首条位置: {"module":3127,"inlinee":107469}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3127,"record_offset":5340,"parent_record_offset":5304,"ancestor_record_offset":5304,"inlinee":7062,"source_line":1917,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$58::clone
// 记录形式: inlinee_declaration; instances=1
// 源码行: 229
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E99C
// 首条位置: {"module":4115,"inlinee":125340}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$61::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 229
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120C7
// 首条位置: {"module":1610,"inlinee":73927}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1610,"record_offset":140276,"parent_record_offset":138396,"ancestor_record_offset":138396,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":138564,"parent_record_offset":138396,"ancestor_record_offset":138396,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":140244,"parent_record_offset":138396,"ancestor_record_offset":138396,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":140504,"parent_record_offset":138396,"ancestor_record_offset":138396,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":138440,"parent_record_offset":138396,"ancestor_record_offset":138396,"inlinee":30524,"source_line":108,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":139984,"parent_record_offset":138396,"ancestor_record_offset":138396,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":139140,"parent_record_offset":139116,"ancestor_record_offset":138396,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":139164,"parent_record_offset":139140,"ancestor_record_offset":138396,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":139600,"parent_record_offset":139576,"ancestor_record_offset":138396,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":139116,"parent_record_offset":139072,"ancestor_record_offset":138396,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":139576,"parent_record_offset":139520,"ancestor_record_offset":138396,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":138988,"parent_record_offset":138948,"ancestor_record_offset":138396,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":139548,"parent_record_offset":139520,"ancestor_record_offset":138396,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":139520,"parent_record_offset":139496,"ancestor_record_offset":138396,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":139072,"parent_record_offset":139028,"ancestor_record_offset":138396,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":139028,"parent_record_offset":138988,"ancestor_record_offset":138396,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":139496,"parent_record_offset":139472,"ancestor_record_offset":138396,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1610,"record_offset":139416,"parent_record_offset":139380,"ancestor_record_offset":138396,"inlinee":14307,"source_line":576,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1610,"record_offset":138908,"parent_record_offset":138828,"ancestor_record_offset":138396,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1610,"record_offset":138948,"parent_record_offset":138908,"ancestor_record_offset":138396,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":138760,"parent_record_offset":138732,"ancestor_record_offset":138396,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1610,"record_offset":139348,"parent_record_offset":139312,"ancestor_record_offset":138396,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1610,"record_offset":138528,"parent_record_offset":138500,"ancestor_record_offset":138396,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1610,"record_offset":139448,"parent_record_offset":139416,"ancestor_record_offset":138396,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":139312,"parent_record_offset":138608,"ancestor_record_offset":138396,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":139652,"parent_record_offset":139380,"ancestor_record_offset":138396,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$61::read_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":138608,"parent_record_offset":138564,"ancestor_record_offset":138396,"inlinee":73929,"source_line":229,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::error::error_invalid_string_utf8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/error.rs
//     evidence={"module":1610,"record_offset":139700,"parent_record_offset":139676,"ancestor_record_offset":138396,"inlinee":27487,"source_line":137,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":1610,"record_offset":138652,"parent_record_offset":138608,"ancestor_record_offset":138396,"inlinee":30496,"source_line":159,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::vec_to_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":1610,"record_offset":139380,"parent_record_offset":139348,"ancestor_record_offset":138396,"inlinee":30508,"source_line":39,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":138472,"parent_record_offset":138440,"ancestor_record_offset":138396,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":138500,"parent_record_offset":138440,"ancestor_record_offset":138396,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$35::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":140032,"parent_record_offset":140008,"ancestor_record_offset":138396,"inlinee":73903,"source_line":116,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$6::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":140008,"parent_record_offset":139984,"ancestor_record_offset":138396,"inlinee":73925,"source_line":171,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":138676,"parent_record_offset":138652,"ancestor_record_offset":138396,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":140056,"parent_record_offset":140032,"ancestor_record_offset":138396,"inlinee":73904,"source_line":288,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":138828,"parent_record_offset":138608,"ancestor_record_offset":138396,"inlinee":30501,"source_line":514,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$61::read_from::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 229
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120C9
// 首条位置: {"module":1610,"inlinee":73929}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":139312,"parent_record_offset":138608,"ancestor_record_offset":138608,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":1610,"record_offset":138652,"parent_record_offset":138608,"ancestor_record_offset":138608,"inlinee":30496,"source_line":159,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":138828,"parent_record_offset":138608,"ancestor_record_offset":138608,"inlinee":30501,"source_line":514,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":139140,"parent_record_offset":139116,"ancestor_record_offset":138608,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":139164,"parent_record_offset":139140,"ancestor_record_offset":138608,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":139600,"parent_record_offset":139576,"ancestor_record_offset":138608,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":139116,"parent_record_offset":139072,"ancestor_record_offset":138608,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":139576,"parent_record_offset":139520,"ancestor_record_offset":138608,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":138988,"parent_record_offset":138948,"ancestor_record_offset":138608,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":139548,"parent_record_offset":139520,"ancestor_record_offset":138608,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":139520,"parent_record_offset":139496,"ancestor_record_offset":138608,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":139072,"parent_record_offset":139028,"ancestor_record_offset":138608,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":139028,"parent_record_offset":138988,"ancestor_record_offset":138608,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":139496,"parent_record_offset":139472,"ancestor_record_offset":138608,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1610,"record_offset":139416,"parent_record_offset":139380,"ancestor_record_offset":138608,"inlinee":14307,"source_line":576,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1610,"record_offset":138908,"parent_record_offset":138828,"ancestor_record_offset":138608,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1610,"record_offset":138948,"parent_record_offset":138908,"ancestor_record_offset":138608,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":138760,"parent_record_offset":138732,"ancestor_record_offset":138608,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1610,"record_offset":139348,"parent_record_offset":139312,"ancestor_record_offset":138608,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1610,"record_offset":138788,"parent_record_offset":138732,"ancestor_record_offset":138608,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1610,"record_offset":139448,"parent_record_offset":139416,"ancestor_record_offset":138608,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":139652,"parent_record_offset":139380,"ancestor_record_offset":138608,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::error::error_invalid_string_utf8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/error.rs
//     evidence={"module":1610,"record_offset":139700,"parent_record_offset":139676,"ancestor_record_offset":138608,"inlinee":27487,"source_line":137,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::vec_to_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":1610,"record_offset":139380,"parent_record_offset":139348,"ancestor_record_offset":138608,"inlinee":30508,"source_line":39,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":138704,"parent_record_offset":138676,"ancestor_record_offset":138608,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":138732,"parent_record_offset":138676,"ancestor_record_offset":138608,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":138676,"parent_record_offset":138652,"ancestor_record_offset":138608,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$67::clone
// 记录形式: inlinee_declaration; instances=2
// 源码行: 235
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A3DB
// 首条位置: {"module":3127,"inlinee":107483}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$70::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 235
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120BE
// 首条位置: {"module":1610,"inlinee":73918}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1610,"record_offset":140880,"parent_record_offset":136096,"ancestor_record_offset":136096,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":136300,"parent_record_offset":136096,"ancestor_record_offset":136096,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":141108,"parent_record_offset":136096,"ancestor_record_offset":136096,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":1610,"record_offset":137724,"parent_record_offset":136096,"ancestor_record_offset":136096,"inlinee":30496,"source_line":159,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":136168,"parent_record_offset":136096,"ancestor_record_offset":136096,"inlinee":30524,"source_line":108,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":137900,"parent_record_offset":136096,"ancestor_record_offset":136096,"inlinee":30501,"source_line":514,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":136884,"parent_record_offset":136860,"ancestor_record_offset":136096,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":136908,"parent_record_offset":136884,"ancestor_record_offset":136096,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":137340,"parent_record_offset":137316,"ancestor_record_offset":136096,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":136860,"parent_record_offset":136816,"ancestor_record_offset":136096,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":137316,"parent_record_offset":137260,"ancestor_record_offset":136096,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":136732,"parent_record_offset":136692,"ancestor_record_offset":136096,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":137288,"parent_record_offset":137260,"ancestor_record_offset":136096,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":137260,"parent_record_offset":137236,"ancestor_record_offset":136096,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":136816,"parent_record_offset":136772,"ancestor_record_offset":136096,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":136772,"parent_record_offset":136732,"ancestor_record_offset":136096,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":137236,"parent_record_offset":137212,"ancestor_record_offset":136096,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1610,"record_offset":137152,"parent_record_offset":137120,"ancestor_record_offset":136096,"inlinee":14307,"source_line":576,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1610,"record_offset":136652,"parent_record_offset":136572,"ancestor_record_offset":136096,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1610,"record_offset":136692,"parent_record_offset":136652,"ancestor_record_offset":136096,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":136504,"parent_record_offset":136476,"ancestor_record_offset":136096,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1610,"record_offset":138012,"parent_record_offset":137900,"ancestor_record_offset":136096,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1610,"record_offset":137088,"parent_record_offset":137056,"ancestor_record_offset":136096,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1610,"record_offset":136264,"parent_record_offset":136236,"ancestor_record_offset":136096,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1610,"record_offset":137188,"parent_record_offset":137152,"ancestor_record_offset":136096,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1610,"record_offset":138320,"parent_record_offset":137900,"ancestor_record_offset":136096,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1610,"record_offset":140764,"parent_record_offset":137900,"ancestor_record_offset":136096,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::and_then
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":137056,"parent_record_offset":136348,"ancestor_record_offset":136096,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":137392,"parent_record_offset":137120,"ancestor_record_offset":136096,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":140244,"parent_record_offset":138396,"ancestor_record_offset":136096,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":140504,"parent_record_offset":138396,"ancestor_record_offset":136096,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$61::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":138396,"parent_record_offset":138348,"ancestor_record_offset":136096,"inlinee":73927,"source_line":229,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$61::read_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":138608,"parent_record_offset":138564,"ancestor_record_offset":136096,"inlinee":73929,"source_line":229,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$70::read_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":136348,"parent_record_offset":136300,"ancestor_record_offset":136096,"inlinee":73920,"source_line":235,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::error::error_invalid_string_utf8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/error.rs
//     evidence={"module":1610,"record_offset":137440,"parent_record_offset":137416,"ancestor_record_offset":136096,"inlinee":27487,"source_line":137,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":1610,"record_offset":136396,"parent_record_offset":136348,"ancestor_record_offset":136096,"inlinee":30496,"source_line":159,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::vec_to_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":1610,"record_offset":137120,"parent_record_offset":137088,"ancestor_record_offset":136096,"inlinee":30508,"source_line":39,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":136208,"parent_record_offset":136168,"ancestor_record_offset":136096,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":136236,"parent_record_offset":136168,"ancestor_record_offset":136096,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$35::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":140032,"parent_record_offset":140008,"ancestor_record_offset":136096,"inlinee":73903,"source_line":116,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$6::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":140008,"parent_record_offset":139984,"ancestor_record_offset":136096,"inlinee":73925,"source_line":171,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":136420,"parent_record_offset":136396,"ancestor_record_offset":136096,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":140056,"parent_record_offset":140032,"ancestor_record_offset":136096,"inlinee":73904,"source_line":288,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":138440,"parent_record_offset":138396,"ancestor_record_offset":136096,"inlinee":30524,"source_line":108,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":138348,"parent_record_offset":137900,"ancestor_record_offset":136096,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":136572,"parent_record_offset":136348,"ancestor_record_offset":136096,"inlinee":30501,"source_line":514,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$70::read_from::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 235
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120C0
// 首条位置: {"module":1610,"inlinee":73920}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":137056,"parent_record_offset":136348,"ancestor_record_offset":136348,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":1610,"record_offset":136396,"parent_record_offset":136348,"ancestor_record_offset":136348,"inlinee":30496,"source_line":159,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":136572,"parent_record_offset":136348,"ancestor_record_offset":136348,"inlinee":30501,"source_line":514,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":136884,"parent_record_offset":136860,"ancestor_record_offset":136348,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":136908,"parent_record_offset":136884,"ancestor_record_offset":136348,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":137340,"parent_record_offset":137316,"ancestor_record_offset":136348,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":136860,"parent_record_offset":136816,"ancestor_record_offset":136348,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":137316,"parent_record_offset":137260,"ancestor_record_offset":136348,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":136732,"parent_record_offset":136692,"ancestor_record_offset":136348,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":137288,"parent_record_offset":137260,"ancestor_record_offset":136348,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":137260,"parent_record_offset":137236,"ancestor_record_offset":136348,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":136816,"parent_record_offset":136772,"ancestor_record_offset":136348,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":136772,"parent_record_offset":136732,"ancestor_record_offset":136348,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":137236,"parent_record_offset":137212,"ancestor_record_offset":136348,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1610,"record_offset":137152,"parent_record_offset":137120,"ancestor_record_offset":136348,"inlinee":14307,"source_line":576,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1610,"record_offset":136652,"parent_record_offset":136572,"ancestor_record_offset":136348,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1610,"record_offset":136692,"parent_record_offset":136652,"ancestor_record_offset":136348,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":136504,"parent_record_offset":136476,"ancestor_record_offset":136348,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1610,"record_offset":137088,"parent_record_offset":137056,"ancestor_record_offset":136348,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1610,"record_offset":136532,"parent_record_offset":136476,"ancestor_record_offset":136348,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1610,"record_offset":137188,"parent_record_offset":137152,"ancestor_record_offset":136348,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":137392,"parent_record_offset":137120,"ancestor_record_offset":136348,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::error::error_invalid_string_utf8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/error.rs
//     evidence={"module":1610,"record_offset":137440,"parent_record_offset":137416,"ancestor_record_offset":136348,"inlinee":27487,"source_line":137,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::vec_to_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":1610,"record_offset":137120,"parent_record_offset":137088,"ancestor_record_offset":136348,"inlinee":30508,"source_line":39,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":136448,"parent_record_offset":136420,"ancestor_record_offset":136348,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":136476,"parent_record_offset":136420,"ancestor_record_offset":136348,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":136420,"parent_record_offset":136396,"ancestor_record_offset":136348,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::StructType::nbyte
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 244
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A3C7
// 首条位置: {"module":3127,"inlinee":107463}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3127,"record_offset":4752,"parent_record_offset":4700,"ancestor_record_offset":4700,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3127,"record_offset":4724,"parent_record_offset":4700,"ancestor_record_offset":4700,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$8::nbyte::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":3127,"record_offset":4780,"parent_record_offset":4752,"ancestor_record_offset":4700,"inlinee":107461,"source_line":247,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$8::nbyte::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 247
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A3C5
// 首条位置: {"module":3127,"inlinee":107461}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$75::clone
// 记录形式: inlinee_declaration; instances=2
// 源码行: 272
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A3D9
// 首条位置: {"module":3127,"inlinee":107481}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$78::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 272
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120BC
// 首条位置: {"module":1610,"inlinee":73916}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":135956,"parent_record_offset":135600,"ancestor_record_offset":135600,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":135636,"parent_record_offset":135600,"ancestor_record_offset":135600,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":135848,"parent_record_offset":135824,"ancestor_record_offset":135600,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":135872,"parent_record_offset":135848,"ancestor_record_offset":135600,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":135792,"parent_record_offset":135752,"ancestor_record_offset":135600,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":135824,"parent_record_offset":135792,"ancestor_record_offset":135600,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1610,"record_offset":135752,"parent_record_offset":135676,"ancestor_record_offset":135600,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1610,"record_offset":135900,"parent_record_offset":135872,"ancestor_record_offset":135600,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":135716,"parent_record_offset":135676,"ancestor_record_offset":135600,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$24::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":135676,"parent_record_offset":135636,"ancestor_record_offset":135600,"inlinee":73910,"source_line":573,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$10::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 277
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A3D1
// 首条位置: {"module":3127,"inlinee":107473}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3127,"record_offset":5488,"parent_record_offset":5456,"ancestor_record_offset":5456,"inlinee":7062,"source_line":1917,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 284
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1610,"record_offset":130388,"rva":21473248,"va":5390182368}
// 编译实例 RVAs: 0x147a7e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15284604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390182368,"raw_hex":"e884c616ff","size":5,"soft_fail":false,"target_va":5374900736,"va":5390185335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15101597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390182368,"raw_hex":"e8639119ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5390183080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15102104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390182368,"raw_hex":"e8688f19ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5390183603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088b180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12517629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390182368,"raw_hex":"e803ff40ff","size":5,"soft_fail":false,"target_va":5377667456,"va":5390185080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140892f20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12485458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390182368,"raw_hex":"e8ae7c41ff","size":5,"soft_fail":false,"target_va":5377699616,"va":5390185069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140896a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12471019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390182368,"raw_hex":"e815b541ff","size":5,"soft_fail":false,"target_va":5377714784,"va":5390185798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428fc010
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21499604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390182368,"raw_hex":"e8d40e4801","size":5,"soft_fail":false,"target_va":5411684368,"va":5390184759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21663627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390182368,"raw_hex":"e88b8f4a01","size":5,"soft_fail":false,"target_va":5411847168,"va":5390183536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t21355846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390182368,"raw_hex":"e846dd4501","size":5,"soft_fail":false,"target_va":5411540992,"va":5390185141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t21355834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390182368,"raw_hex":"e83add4501","size":5,"soft_fail":false,"target_va":5411540963,"va":5390185124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<shader_pipeline::reflection_ty::Type> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390182368,"raw_hex":"e85bf0ffff","size":5,"soft_fail":false,"target_va":5390181408,"va":5390185408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7281970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390182368,"raw_hex":"e8cee290ff","size":5,"soft_fail":false,"target_va":5382901600,"va":5390183565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390182368,"raw_hex":"e80bffffff","size":5,"soft_fail":false,"target_va":5390182368,"va":5390182608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":141368,"parent_record_offset":130388,"ancestor_record_offset":130388,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":132788,"parent_record_offset":130388,"ancestor_record_offset":130388,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":130628,"parent_record_offset":130388,"ancestor_record_offset":130388,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":130780,"parent_record_offset":130388,"ancestor_record_offset":130388,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":131512,"parent_record_offset":131488,"ancestor_record_offset":130388,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":131536,"parent_record_offset":131512,"ancestor_record_offset":130388,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":132656,"parent_record_offset":132632,"ancestor_record_offset":130388,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":131456,"parent_record_offset":131416,"ancestor_record_offset":130388,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":131488,"parent_record_offset":131456,"ancestor_record_offset":130388,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":132632,"parent_record_offset":132608,"ancestor_record_offset":130388,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1610,"record_offset":131416,"parent_record_offset":131340,"ancestor_record_offset":130388,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1610,"record_offset":132608,"parent_record_offset":132488,"ancestor_record_offset":130388,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":136732,"parent_record_offset":136692,"ancestor_record_offset":130388,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":137288,"parent_record_offset":137260,"ancestor_record_offset":130388,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":137260,"parent_record_offset":137236,"ancestor_record_offset":130388,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":136816,"parent_record_offset":136772,"ancestor_record_offset":130388,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":136772,"parent_record_offset":136732,"ancestor_record_offset":130388,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":137236,"parent_record_offset":137212,"ancestor_record_offset":130388,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1610,"record_offset":137152,"parent_record_offset":137120,"ancestor_record_offset":130388,"inlinee":14307,"source_line":576,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1610,"record_offset":136652,"parent_record_offset":136572,"ancestor_record_offset":130388,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1610,"record_offset":136692,"parent_record_offset":136652,"ancestor_record_offset":130388,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":130712,"parent_record_offset":130684,"ancestor_record_offset":130388,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1610,"record_offset":138012,"parent_record_offset":137900,"ancestor_record_offset":130388,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1610,"record_offset":137088,"parent_record_offset":137056,"ancestor_record_offset":130388,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1610,"record_offset":130744,"parent_record_offset":130684,"ancestor_record_offset":130388,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1610,"record_offset":132488,"parent_record_offset":131244,"ancestor_record_offset":130388,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1610,"record_offset":138320,"parent_record_offset":137900,"ancestor_record_offset":130388,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1610,"record_offset":131564,"parent_record_offset":131536,"ancestor_record_offset":130388,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1610,"record_offset":140764,"parent_record_offset":137900,"ancestor_record_offset":130388,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::and_then
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":131800,"parent_record_offset":131244,"ancestor_record_offset":130388,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":137392,"parent_record_offset":137120,"ancestor_record_offset":130388,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":131380,"parent_record_offset":131340,"ancestor_record_offset":130388,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":131648,"parent_record_offset":131244,"ancestor_record_offset":130388,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$19::read_from
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":130804,"parent_record_offset":130780,"ancestor_record_offset":130388,"inlinee":73906,"source_line":17,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$27::read_from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":132928,"parent_record_offset":132892,"ancestor_record_offset":130388,"inlinee":73914,"source_line":116,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$36::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":134104,"parent_record_offset":134080,"ancestor_record_offset":130388,"inlinee":73933,"source_line":140,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$45::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":132856,"parent_record_offset":132816,"ancestor_record_offset":130388,"inlinee":73912,"source_line":147,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$45::read_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":134052,"parent_record_offset":134024,"ancestor_record_offset":130388,"inlinee":73931,"source_line":147,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$45::read_from::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":134448,"parent_record_offset":134420,"ancestor_record_offset":130388,"inlinee":73934,"source_line":147,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$53::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":131244,"parent_record_offset":131188,"ancestor_record_offset":130388,"inlinee":73908,"source_line":191,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$53::read_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":131828,"parent_record_offset":131800,"ancestor_record_offset":130388,"inlinee":73922,"source_line":191,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$53::read_from::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":132220,"parent_record_offset":132192,"ancestor_record_offset":130388,"inlinee":73923,"source_line":191,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$61::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":138396,"parent_record_offset":138348,"ancestor_record_offset":130388,"inlinee":73927,"source_line":229,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$61::read_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":138608,"parent_record_offset":138564,"ancestor_record_offset":130388,"inlinee":73929,"source_line":229,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$70::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":136096,"parent_record_offset":136020,"ancestor_record_offset":130388,"inlinee":73918,"source_line":235,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$70::read_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":136348,"parent_record_offset":136300,"ancestor_record_offset":130388,"inlinee":73920,"source_line":235,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$78::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1610,"record_offset":135600,"parent_record_offset":135560,"ancestor_record_offset":130388,"inlinee":73916,"source_line":272,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::error::error_invalid_string_utf8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/error.rs
//     evidence={"module":1610,"record_offset":137440,"parent_record_offset":137416,"ancestor_record_offset":130388,"inlinee":27487,"source_line":137,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":1610,"record_offset":136396,"parent_record_offset":136348,"ancestor_record_offset":130388,"inlinee":30496,"source_line":159,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::vec_to_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":1610,"record_offset":137120,"parent_record_offset":137088,"ancestor_record_offset":130388,"inlinee":30508,"source_line":39,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":130656,"parent_record_offset":130628,"ancestor_record_offset":130388,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=26
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":130684,"parent_record_offset":130628,"ancestor_record_offset":130388,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$24::read_from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":131340,"parent_record_offset":131300,"ancestor_record_offset":130388,"inlinee":73910,"source_line":573,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":131000,"parent_record_offset":130976,"ancestor_record_offset":130388,"inlinee":30530,"source_line":116,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$35::read_from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":132296,"parent_record_offset":132272,"ancestor_record_offset":130388,"inlinee":73903,"source_line":116,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$6::read_from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":132272,"parent_record_offset":132248,"ancestor_record_offset":130388,"inlinee":73925,"source_line":171,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":130824,"parent_record_offset":130804,"ancestor_record_offset":130388,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u64
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":132320,"parent_record_offset":132296,"ancestor_record_offset":130388,"inlinee":73904,"source_line":288,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u8
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":131676,"parent_record_offset":131244,"ancestor_record_offset":130388,"inlinee":30524,"source_line":108,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":130976,"parent_record_offset":130804,"ancestor_record_offset":130388,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":136572,"parent_record_offset":136348,"ancestor_record_offset":130388,"inlinee":30501,"source_line":514,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::Type::nbyte
// 记录形式: compiled_procedure; instances=1
// 源码行: 325
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3127,"record_offset":4556,"rva":34331136,"va":5403040256}
// 编译实例 RVAs: 0x20bda00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] shader_pipeline::reflection_ty::Type::nbyte
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403040256,"raw_hex":"e87dffffff","size":5,"soft_fail":false,"target_va":5403040256,"va":5403040382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] shader_pipeline::reflection_ty::ArrayType::nbyte
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":3127,"record_offset":4996,"parent_record_offset":4556,"ancestor_record_offset":4556,"inlinee":107467,"source_line":215,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::reflection_ty::MatrixType::nbyte
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":3127,"record_offset":4812,"parent_record_offset":4556,"ancestor_record_offset":4556,"inlinee":107465,"source_line":169,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::reflection_ty::ScalarType::nbyte
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":3127,"record_offset":4900,"parent_record_offset":4556,"ancestor_record_offset":4556,"inlinee":107457,"source_line":64,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::reflection_ty::StructType::nbyte
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":3127,"record_offset":4700,"parent_record_offset":4556,"ancestor_record_offset":4556,"inlinee":107463,"source_line":244,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::reflection_ty::VectorType::nbyte
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":3127,"record_offset":4932,"parent_record_offset":4556,"ancestor_record_offset":4556,"inlinee":107459,"source_line":130,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3127,"record_offset":4752,"parent_record_offset":4700,"ancestor_record_offset":4556,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3127,"record_offset":5020,"parent_record_offset":4996,"ancestor_record_offset":4556,"inlinee":5059,"source_line":1048,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3127,"record_offset":4724,"parent_record_offset":4700,"ancestor_record_offset":4556,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::ScalarType::nbyte
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":3127,"record_offset":4860,"parent_record_offset":4836,"ancestor_record_offset":4556,"inlinee":107457,"source_line":64,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::VectorType::nbyte
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":3127,"record_offset":4836,"parent_record_offset":4812,"ancestor_record_offset":4556,"inlinee":107459,"source_line":130,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$8::nbyte::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":3127,"record_offset":4780,"parent_record_offset":4752,"ancestor_record_offset":4556,"inlinee":107461,"source_line":247,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$12::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 347
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3127,"record_offset":5168,"rva":34331456,"va":5403040576}
// 编译实例 RVAs: 0x20bdb40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20148522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403040576,"raw_hex":"e8d68eccfe","size":5,"soft_fail":false,"target_va":5382892432,"va":5403040949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-20152910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403040576,"raw_hex":"e8b27dccfe","size":5,"soft_fail":false,"target_va":5382888144,"va":5403041049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-20148716","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5403040576,"raw_hex":"e9148eccfe","size":5,"soft_fail":false,"target_va":5382892432,"va":5403041143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1420bd3a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-2017","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5403040576,"raw_hex":"e91ff8ffff","size":5,"soft_fail":false,"target_va":5403038624,"va":5403040636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1420bd4b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-1885","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5403040576,"raw_hex":"e9a3f8ffff","size":5,"soft_fail":false,"target_va":5403038896,"va":5403040776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1420bd750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-1550","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5403040576,"raw_hex":"e9f2f9ffff","size":5,"soft_fail":false,"target_va":5403039568,"va":5403041113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] shader_pipeline::reflection_ty::impl$10::fmt
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":3127,"record_offset":5456,"parent_record_offset":5168,"ancestor_record_offset":5168,"inlinee":107473,"source_line":277,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::reflection_ty::impl$3::fmt
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":3127,"record_offset":5400,"parent_record_offset":5168,"ancestor_record_offset":5168,"inlinee":107471,"source_line":135,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::reflection_ty::impl$7::fmt
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":3127,"record_offset":5304,"parent_record_offset":5168,"ancestor_record_offset":5168,"inlinee":107469,"source_line":220,"archive":"_pdb/B/modules/03127.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3127,"record_offset":5340,"parent_record_offset":5304,"ancestor_record_offset":5168,"inlinee":7062,"source_line":1917,"archive":"_pdb/B/modules/03127.jsonl.gz"}
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
// 完整结构及原始类型记录: reflection_ty.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: reflection_ty.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: reflection_ty.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::ArrayType::nbyte
// 信息 ID: 21b1488fdcdb1f09eef0f22b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::Type::nbyte; module=3127, inline_record=4996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::MatrixType::nbyte
// 信息 ID: dd8ddbdff066128b149b1a62
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::Type::nbyte; module=3127, inline_record=4812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::ScalarType::nbyte
// 信息 ID: 8cc543dddd9fa3825f2e925c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > @ 0x16a3b40 -> 0x20bd380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > @ 0x16a3b7f -> 0x20bd380 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] shader_pipeline::reflection_ty::Type::nbyte; module=3127, inline_record=4860; 不是独立调用边。
// [PDB 内联宿主] shader_pipeline::reflection_ty::Type::nbyte; module=3127, inline_record=4900; 不是独立调用边。
// [PDB 内联宿主] shader_pipeline::reflection_ty::Type::nbyte; module=3127, inline_record=4956; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::StructType::nbyte
// 信息 ID: f031277fafe0c03ade0e6f23
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::Type::nbyte; module=3127, inline_record=4700; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::Type::nbyte
// 信息 ID: d90f98e6581439cdda76ae49
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > @ 0x16a3ba2 -> 0x20bda00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::VectorType::nbyte
// 信息 ID: c16c24bcecfc0e482c099dbf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::Type::nbyte; module=3127, inline_record=4836; 不是独立调用边。
// [PDB 内联宿主] shader_pipeline::reflection_ty::Type::nbyte; module=3127, inline_record=4932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$10::fmt
// 信息 ID: 1f87a8951bf711f956241dd4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$12::fmt; module=3127, inline_record=5456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$12::fmt
// 信息 ID: eff749330e8d413c08af756c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$21::fmt<enum2$<shader_pipeline::reflection_ty::Type>,alloc::alloc::Global> @ 0x243d423 -> 0x20bdb40 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$19::read_from
// 信息 ID: 7a29af92e4d5b121490f0979
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1610, inline_record=130804; 不是独立调用边。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1610, inline_record=132996; 不是独立调用边。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1610, inline_record=134828; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$1::fmt
// 信息 ID: 454bb3ca09d28487ac350468
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::fmt::impl$76::fmt<enum2$<shader_pipeline::reflection_ty::ScalarType> > | core::fmt::impl$76::fmt<enum2$<shader_pipeline::reflection_ty::ScalarType> > @ 0x18c2cb3 -> 0x20bd3a0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] shader_pipeline::reflection_ty::impl$12::fmt @ 0x20bdb7c -> 0x20bd3a0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$25::clone
// 信息 ID: 907b9ef86be104a01c1671a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$27::read_from
// 信息 ID: f9cc316f84a7aa554606167b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1610, inline_record=132928; 不是独立调用边。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1610, inline_record=134760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$36::read_from
// 信息 ID: 6937b16ad26fbfda00ef59ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1610, inline_record=134104; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$3::fmt
// 信息 ID: 87bbb18c74caea6bbaf2b7a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$12::fmt; module=3127, inline_record=5400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$43::clone
// 信息 ID: 4b80e304ef0df47cf32e7035
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$45::read_from
// 信息 ID: 90dfa20a056b6e5e6a9437e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1610, inline_record=132856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$45::read_from::closure$0
// 信息 ID: 93172d646b16902adc57840d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "shader_pipeline::reflection_ty::impl$45::read_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1610, inline_record=134052; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$45::read_from::closure$1
// 信息 ID: fe289a1f2b934fb9ef382df5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "shader_pipeline::reflection_ty::impl$45::read_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1610, inline_record=134448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$51::clone
// 信息 ID: ad884e58b82b3f41e4fff220
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$53::read_from
// 信息 ID: 34aabd045769b7dcc4595918
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1610, inline_record=131244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$53::read_from::closure$0
// 信息 ID: 5a8e5ed316e692eb94e90c8f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "shader_pipeline::reflection_ty::impl$53::read_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1610, inline_record=131828; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$53::read_from::closure$1
// 信息 ID: fb157b4c8ff4ae731b76e00c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "shader_pipeline::reflection_ty::impl$53::read_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1610, inline_record=132220; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$58::clone
// 信息 ID: 7c1ea88bd9ca7b2b4391e1b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$5::fmt
// 信息 ID: 357afe29edb758d9825fcb80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$61::read_from
// 信息 ID: b66dc6fdb455914b632283d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1610, inline_record=138396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$61::read_from::closure$0
// 信息 ID: 61adb61c2705da0d54006116
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "shader_pipeline::reflection_ty::impl$61::read_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1610, inline_record=138608; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$67::clone
// 信息 ID: 840292a5adf4ab039786ba04
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$70::read_from
// 信息 ID: aafd8fa722b1c98c46feff55
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1610, inline_record=136096; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$70::read_from::closure$0
// 信息 ID: 52589de0f5fc2ae35e774694
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "shader_pipeline::reflection_ty::impl$70::read_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1610, inline_record=136348; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$75::clone
// 信息 ID: 630b74732fc1d9f8bd72a3cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$78::read_from
// 信息 ID: bc3370abdc0671e6faa3db92
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1610, inline_record=135600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$7::fmt
// 信息 ID: adc3d679af481ac3da5fa169
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$12::fmt; module=3127, inline_record=5304; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >
// 信息 ID: 1f0882c6b41520c1de05521b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] speedy::readable::BufferReader -> ["speedy::context::LittleEndian"]; 字段布局 = unknown。
// [类型关系] shader_pipeline::reflection_ty::impl$86::read_from -> ["speedy::context::LittleEndian", "speedy::readable::BufferReader<speedy::context::LittleEndian>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,speedy::error::Error> >,core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,std::collections::hash::map::HashMap<alloc::string::String,shader_pipeline::reflection::ShaderInput,std::hash::random::RandomState> >,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,std::collections::hash::map::HashMap<alloc::string::String,shader_pipeline::reflection::ShaderInput,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,shader_pipeline::reflection::ShaderInput,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,std::collections::hash::map::HashMap<alloc::string::String,shader_pipeline::reflection::ShaderInput,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > > @ 0x1479e53 -> 0x147a7e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$8::nbyte::closure$0
// 信息 ID: 52a7aa1fc99a34e7a830f561
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "shader_pipeline::reflection_ty::impl$8::nbyte", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::Type::nbyte; module=3127, inline_record=4780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$9::fmt
// 信息 ID: 2b5d83425640738897493bc6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

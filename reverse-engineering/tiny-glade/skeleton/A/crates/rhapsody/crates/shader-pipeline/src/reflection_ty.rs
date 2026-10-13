// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 5251f4e9315389468e34802d10803b0e29a7a669ca6b2f95e5935be2ea9e6857
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: reflection_ty.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1321, 2835, 3948
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use shader_pipeline; // namespace candidate
//   package identities: []
// use speedy; // namespace candidate
//   package identities: [{"name":"speedy","source":"PDB_registry_path_only","version":"0.8.6"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$1::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2835,"record_offset":1960,"rva":25309504,"va":5394018624}
// 编译实例 RVAs: 0x1823140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16438327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394018624,"raw_hex":"e8c92b05ff","size":5,"soft_fail":false,"target_va":5377580528,"va":5394018850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-16434969","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5394018624,"raw_hex":"e9e73805ff","size":5,"soft_fail":false,"target_va":5377583920,"va":5394018884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2835,"record_offset":2148,"parent_record_offset":1960,"ancestor_record_offset":1960,"inlinee":5667,"source_line":1619,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_direct] core::ops::bit::impl$312::shl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":2835,"record_offset":2092,"parent_record_offset":1960,"ancestor_record_offset":1960,"inlinee":43780,"source_line":35,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::bit::impl$311::shl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\bit.rs
//     evidence={"module":2835,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":1960,"inlinee":43782,"source_line":469,"archive":"_pdb/A/modules/02835.jsonl.gz"}
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
// 首条位置: {"module":2835,"record_offset":3860,"rva":25310448,"va":5394019568}
// 编译实例 RVAs: 0x18234f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16435710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394019568,"raw_hex":"e8023605ff","size":5,"soft_fail":false,"target_va":5377583920,"va":5394019625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-16439213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394019568,"raw_hex":"e8532805ff","size":5,"soft_fail":false,"target_va":5377580528,"va":5394019736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2835,"record_offset":3992,"parent_record_offset":3860,"ancestor_record_offset":3860,"inlinee":5667,"source_line":1619,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2835,"record_offset":4020,"parent_record_offset":3860,"ancestor_record_offset":3860,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2835,"record_offset":4116,"parent_record_offset":4060,"ancestor_record_offset":3860,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2835,"record_offset":4060,"parent_record_offset":4020,"ancestor_record_offset":3860,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$19::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF188
// 首条位置: {"module":1321,"inlinee":61832}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":111356,"parent_record_offset":111328,"ancestor_record_offset":111328,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":111512,"parent_record_offset":111328,"ancestor_record_offset":111328,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":111444,"parent_record_offset":111416,"ancestor_record_offset":111328,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":111476,"parent_record_offset":111416,"ancestor_record_offset":111328,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":111388,"parent_record_offset":111356,"ancestor_record_offset":111328,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":111416,"parent_record_offset":111356,"ancestor_record_offset":111328,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":111536,"parent_record_offset":111512,"ancestor_record_offset":111328,"inlinee":61803,"source_line":116,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":111560,"parent_record_offset":111536,"ancestor_record_offset":111328,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::ScalarType::nbyte
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16151
// 首条位置: {"module":2835,"inlinee":90449}
// 编译实例 RVAs: 0x1823110
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
// IPI IDs: 0x16167
// 首条位置: {"module":2835,"inlinee":90471}
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
// IPI IDs: 0xF190
// 首条位置: {"module":1321,"inlinee":61840}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":113564,"parent_record_offset":113528,"ancestor_record_offset":113528,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":113716,"parent_record_offset":113688,"ancestor_record_offset":113528,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":113748,"parent_record_offset":113688,"ancestor_record_offset":113528,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$19::read_from
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":113596,"parent_record_offset":113564,"ancestor_record_offset":113528,"inlinee":61832,"source_line":17,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":113660,"parent_record_offset":113628,"ancestor_record_offset":113528,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":113688,"parent_record_offset":113628,"ancestor_record_offset":113528,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":113812,"parent_record_offset":113784,"ancestor_record_offset":113528,"inlinee":61803,"source_line":116,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":113628,"parent_record_offset":113596,"ancestor_record_offset":113528,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":113784,"parent_record_offset":113596,"ancestor_record_offset":113528,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::VectorType::nbyte
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16153
// 首条位置: {"module":2835,"inlinee":90451}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] shader_pipeline::reflection_ty::ScalarType::nbyte
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":2835,"record_offset":4544,"parent_record_offset":4516,"ancestor_record_offset":4516,"inlinee":90449,"source_line":64,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$3::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1615F
// 首条位置: {"module":2835,"inlinee":90463}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2835,"record_offset":5180,"parent_record_offset":5156,"ancestor_record_offset":5156,"inlinee":5667,"source_line":1619,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$36::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF19D
// 首条位置: {"module":1321,"inlinee":61853}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":114620,"parent_record_offset":114596,"ancestor_record_offset":114596,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":114704,"parent_record_offset":114676,"ancestor_record_offset":114596,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":114736,"parent_record_offset":114676,"ancestor_record_offset":114596,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":114648,"parent_record_offset":114620,"ancestor_record_offset":114596,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":114676,"parent_record_offset":114620,"ancestor_record_offset":114596,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$43::clone
// 记录形式: inlinee_declaration; instances=2
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16165
// 首条位置: {"module":2835,"inlinee":90469}
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
// IPI IDs: 0xF18E
// 首条位置: {"module":1321,"inlinee":61838}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":114516,"parent_record_offset":113448,"ancestor_record_offset":113448,"inlinee":4730,"source_line":1319,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":114392,"parent_record_offset":113448,"ancestor_record_offset":113448,"inlinee":61845,"source_line":108,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":113492,"parent_record_offset":113448,"ancestor_record_offset":113448,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":113716,"parent_record_offset":113688,"ancestor_record_offset":113448,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":113748,"parent_record_offset":113688,"ancestor_record_offset":113448,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$19::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":113596,"parent_record_offset":113564,"ancestor_record_offset":113448,"inlinee":61832,"source_line":17,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$27::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":113528,"parent_record_offset":113492,"ancestor_record_offset":113448,"inlinee":61840,"source_line":116,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$36::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":114596,"parent_record_offset":114572,"ancestor_record_offset":113448,"inlinee":61853,"source_line":140,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$45::read_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":114544,"parent_record_offset":114516,"ancestor_record_offset":113448,"inlinee":61851,"source_line":147,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$45::read_from::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":114940,"parent_record_offset":114912,"ancestor_record_offset":113448,"inlinee":61861,"source_line":147,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":113660,"parent_record_offset":113628,"ancestor_record_offset":113448,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":113688,"parent_record_offset":113628,"ancestor_record_offset":113448,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":113812,"parent_record_offset":113784,"ancestor_record_offset":113448,"inlinee":61803,"source_line":116,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$35::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":115016,"parent_record_offset":114992,"ancestor_record_offset":113448,"inlinee":61797,"source_line":116,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$6::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":114992,"parent_record_offset":114968,"ancestor_record_offset":113448,"inlinee":61856,"source_line":171,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":113628,"parent_record_offset":113596,"ancestor_record_offset":113448,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":115040,"parent_record_offset":115016,"ancestor_record_offset":113448,"inlinee":61798,"source_line":288,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":113564,"parent_record_offset":113528,"ancestor_record_offset":113448,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$45::read_from::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF19B
// 首条位置: {"module":1321,"inlinee":61851}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":114572,"parent_record_offset":114544,"ancestor_record_offset":114544,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":114704,"parent_record_offset":114676,"ancestor_record_offset":114544,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":114736,"parent_record_offset":114676,"ancestor_record_offset":114544,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$36::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":114596,"parent_record_offset":114572,"ancestor_record_offset":114544,"inlinee":61853,"source_line":140,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":114648,"parent_record_offset":114620,"ancestor_record_offset":114544,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":114676,"parent_record_offset":114620,"ancestor_record_offset":114544,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":114620,"parent_record_offset":114596,"ancestor_record_offset":114544,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$45::read_from::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1A5
// 首条位置: {"module":1321,"inlinee":61861}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":114968,"parent_record_offset":114940,"ancestor_record_offset":114940,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":115124,"parent_record_offset":115096,"ancestor_record_offset":114940,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":115152,"parent_record_offset":115096,"ancestor_record_offset":114940,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":115068,"parent_record_offset":115040,"ancestor_record_offset":114940,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":115096,"parent_record_offset":115040,"ancestor_record_offset":114940,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$35::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":115016,"parent_record_offset":114992,"ancestor_record_offset":114940,"inlinee":61797,"source_line":116,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$6::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":114992,"parent_record_offset":114968,"ancestor_record_offset":114940,"inlinee":61856,"source_line":171,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":115040,"parent_record_offset":115016,"ancestor_record_offset":114940,"inlinee":61798,"source_line":288,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::MatrixType::nbyte
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 169
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16159
// 首条位置: {"module":2835,"inlinee":90457}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2835,"record_offset":4584,"parent_record_offset":4488,"ancestor_record_offset":4488,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::reflection_ty::VectorType::nbyte
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":2835,"record_offset":4516,"parent_record_offset":4488,"ancestor_record_offset":4488,"inlinee":90451,"source_line":130,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::ScalarType::nbyte
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":2835,"record_offset":4544,"parent_record_offset":4516,"ancestor_record_offset":4488,"inlinee":90449,"source_line":64,"archive":"_pdb/A/modules/02835.jsonl.gz"}
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
// 首条位置: {"module":2835,"record_offset":2404,"rva":25309776,"va":5394018896}
// 编译实例 RVAs: 0x1823250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21269463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394018896,"raw_hex":"e82974bbfe","size":5,"soft_fail":false,"target_va":5372749696,"va":5394019154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21269653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394018896,"raw_hex":"e86b73bbfe","size":5,"soft_fail":false,"target_va":5372749712,"va":5394019360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087d5d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16407895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394018896,"raw_hex":"e8a9a205ff","size":5,"soft_fail":false,"target_va":5377611216,"va":5394019106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8958709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394018896,"raw_hex":"e8f5b28800","size":5,"soft_fail":false,"target_va":5402978144,"va":5394019430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16438811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394018896,"raw_hex":"e8e52905ff","size":5,"soft_fail":false,"target_va":5377580528,"va":5394019334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8960710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394018896,"raw_hex":"e8c6ba8800","size":5,"soft_fail":false,"target_va":5402980128,"va":5394019413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":2835,"record_offset":2900,"parent_record_offset":2404,"ancestor_record_offset":2404,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":2835,"record_offset":2544,"parent_record_offset":2404,"ancestor_record_offset":2404,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2835,"record_offset":3412,"parent_record_offset":2404,"ancestor_record_offset":2404,"inlinee":5667,"source_line":1619,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2835,"record_offset":3440,"parent_record_offset":2404,"ancestor_record_offset":2404,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2835,"record_offset":3200,"parent_record_offset":3176,"ancestor_record_offset":2404,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2835,"record_offset":3224,"parent_record_offset":3200,"ancestor_record_offset":2404,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2835,"record_offset":2848,"parent_record_offset":2824,"ancestor_record_offset":2404,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2835,"record_offset":3176,"parent_record_offset":3148,"ancestor_record_offset":2404,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2835,"record_offset":2824,"parent_record_offset":2768,"ancestor_record_offset":2404,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2835,"record_offset":2796,"parent_record_offset":2768,"ancestor_record_offset":2404,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2835,"record_offset":3148,"parent_record_offset":3120,"ancestor_record_offset":2404,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2835,"record_offset":3120,"parent_record_offset":3092,"ancestor_record_offset":2404,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2835,"record_offset":3296,"parent_record_offset":3120,"ancestor_record_offset":2404,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2835,"record_offset":2768,"parent_record_offset":2740,"ancestor_record_offset":2404,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2835,"record_offset":3056,"parent_record_offset":3028,"ancestor_record_offset":2404,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2835,"record_offset":3028,"parent_record_offset":3000,"ancestor_record_offset":2404,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2835,"record_offset":2972,"parent_record_offset":2928,"ancestor_record_offset":2404,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2835,"record_offset":3000,"parent_record_offset":2972,"ancestor_record_offset":2404,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2835,"record_offset":2928,"parent_record_offset":2900,"ancestor_record_offset":2404,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":2835,"record_offset":2596,"parent_record_offset":2544,"ancestor_record_offset":2404,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2835,"record_offset":3092,"parent_record_offset":3056,"ancestor_record_offset":2404,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2835,"record_offset":2624,"parent_record_offset":2544,"ancestor_record_offset":2404,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2835,"record_offset":3356,"parent_record_offset":3332,"ancestor_record_offset":2404,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2835,"record_offset":3332,"parent_record_offset":3056,"ancestor_record_offset":2404,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2835,"record_offset":2688,"parent_record_offset":2544,"ancestor_record_offset":2404,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2835,"record_offset":3252,"parent_record_offset":3224,"ancestor_record_offset":2404,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2835,"record_offset":2652,"parent_record_offset":2544,"ancestor_record_offset":2404,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$51::clone
// 记录形式: inlinee_declaration; instances=2
// 源码行: 191
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16163
// 首条位置: {"module":2835,"inlinee":90467}
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
// IPI IDs: 0xF18A
// 首条位置: {"module":1321,"inlinee":61834}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":113092,"parent_record_offset":111820,"ancestor_record_offset":111820,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":112380,"parent_record_offset":111820,"ancestor_record_offset":111820,"inlinee":4730,"source_line":1319,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":112200,"parent_record_offset":111820,"ancestor_record_offset":111820,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":112228,"parent_record_offset":111820,"ancestor_record_offset":111820,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":112256,"parent_record_offset":111820,"ancestor_record_offset":111820,"inlinee":61845,"source_line":108,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":111880,"parent_record_offset":111820,"ancestor_record_offset":111820,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":112092,"parent_record_offset":112068,"ancestor_record_offset":111820,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":112116,"parent_record_offset":112092,"ancestor_record_offset":111820,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":113268,"parent_record_offset":113244,"ancestor_record_offset":111820,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":112036,"parent_record_offset":111996,"ancestor_record_offset":111820,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":112068,"parent_record_offset":112036,"ancestor_record_offset":111820,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":113244,"parent_record_offset":113220,"ancestor_record_offset":111820,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1321,"record_offset":111996,"parent_record_offset":111920,"ancestor_record_offset":111820,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1321,"record_offset":113220,"parent_record_offset":113092,"ancestor_record_offset":111820,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":112568,"parent_record_offset":112540,"ancestor_record_offset":111820,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":112344,"parent_record_offset":112316,"ancestor_record_offset":111820,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":113128,"parent_record_offset":113092,"ancestor_record_offset":111820,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":112144,"parent_record_offset":112116,"ancestor_record_offset":111820,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":111960,"parent_record_offset":111920,"ancestor_record_offset":111820,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$53::read_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":112408,"parent_record_offset":112380,"ancestor_record_offset":111820,"inlinee":61849,"source_line":191,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$53::read_from::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":112804,"parent_record_offset":112772,"ancestor_record_offset":111820,"inlinee":61854,"source_line":191,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":112288,"parent_record_offset":112256,"ancestor_record_offset":111820,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":112316,"parent_record_offset":112256,"ancestor_record_offset":111820,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$24::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":111920,"parent_record_offset":111880,"ancestor_record_offset":111820,"inlinee":61836,"source_line":573,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":112460,"parent_record_offset":112436,"ancestor_record_offset":111820,"inlinee":61803,"source_line":116,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$35::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":112892,"parent_record_offset":112864,"ancestor_record_offset":111820,"inlinee":61797,"source_line":116,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$6::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":112864,"parent_record_offset":112836,"ancestor_record_offset":111820,"inlinee":61856,"source_line":171,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":112484,"parent_record_offset":112460,"ancestor_record_offset":111820,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":112920,"parent_record_offset":112892,"ancestor_record_offset":111820,"inlinee":61798,"source_line":288,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":112436,"parent_record_offset":112408,"ancestor_record_offset":111820,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$53::read_from::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 191
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF199
// 首条位置: {"module":1321,"inlinee":61849}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":112436,"parent_record_offset":112408,"ancestor_record_offset":112408,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":112568,"parent_record_offset":112540,"ancestor_record_offset":112408,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":112596,"parent_record_offset":112540,"ancestor_record_offset":112408,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":112512,"parent_record_offset":112484,"ancestor_record_offset":112408,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":112540,"parent_record_offset":112484,"ancestor_record_offset":112408,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":112460,"parent_record_offset":112436,"ancestor_record_offset":112408,"inlinee":61803,"source_line":116,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":112484,"parent_record_offset":112460,"ancestor_record_offset":112408,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$53::read_from::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 191
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF19E
// 首条位置: {"module":1321,"inlinee":61854}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":112836,"parent_record_offset":112804,"ancestor_record_offset":112804,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":113008,"parent_record_offset":112980,"ancestor_record_offset":112804,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":113036,"parent_record_offset":112980,"ancestor_record_offset":112804,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":112952,"parent_record_offset":112920,"ancestor_record_offset":112804,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":112980,"parent_record_offset":112920,"ancestor_record_offset":112804,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$35::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":112892,"parent_record_offset":112864,"ancestor_record_offset":112804,"inlinee":61797,"source_line":116,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$6::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":112864,"parent_record_offset":112836,"ancestor_record_offset":112804,"inlinee":61856,"source_line":171,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":112920,"parent_record_offset":112892,"ancestor_record_offset":112804,"inlinee":61798,"source_line":288,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::ArrayType::nbyte
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 215
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1615B
// 首条位置: {"module":2835,"inlinee":90459}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2835,"record_offset":4748,"parent_record_offset":4724,"ancestor_record_offset":4724,"inlinee":4156,"source_line":1003,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$7::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 220
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1615D
// 首条位置: {"module":2835,"inlinee":90461}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2835,"record_offset":5096,"parent_record_offset":5060,"ancestor_record_offset":5060,"inlinee":5667,"source_line":1619,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$58::clone
// 记录形式: inlinee_declaration; instances=1
// 源码行: 229
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B6EC
// 首条位置: {"module":3948,"inlinee":112364}
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
// IPI IDs: 0xF1A2
// 首条位置: {"module":1321,"inlinee":61858}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":120760,"parent_record_offset":118900,"ancestor_record_offset":118900,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":119068,"parent_record_offset":118900,"ancestor_record_offset":118900,"inlinee":4730,"source_line":1319,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":120700,"parent_record_offset":118900,"ancestor_record_offset":118900,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":120732,"parent_record_offset":118900,"ancestor_record_offset":118900,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":118944,"parent_record_offset":118900,"ancestor_record_offset":118900,"inlinee":61845,"source_line":108,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":120440,"parent_record_offset":118900,"ancestor_record_offset":118900,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":119596,"parent_record_offset":119572,"ancestor_record_offset":118900,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":119620,"parent_record_offset":119596,"ancestor_record_offset":118900,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":120088,"parent_record_offset":120064,"ancestor_record_offset":118900,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":119572,"parent_record_offset":119528,"ancestor_record_offset":118900,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":120064,"parent_record_offset":120008,"ancestor_record_offset":118900,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":120036,"parent_record_offset":120008,"ancestor_record_offset":118900,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":119528,"parent_record_offset":119488,"ancestor_record_offset":118900,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":119488,"parent_record_offset":119448,"ancestor_record_offset":118900,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":119692,"parent_record_offset":119488,"ancestor_record_offset":118900,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":120008,"parent_record_offset":119984,"ancestor_record_offset":118900,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1321,"record_offset":119928,"parent_record_offset":119892,"ancestor_record_offset":118900,"inlinee":13019,"source_line":576,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1321,"record_offset":119408,"parent_record_offset":119332,"ancestor_record_offset":118900,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1321,"record_offset":119448,"parent_record_offset":119408,"ancestor_record_offset":118900,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":119264,"parent_record_offset":119236,"ancestor_record_offset":118900,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1321,"record_offset":119860,"parent_record_offset":119824,"ancestor_record_offset":118900,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":119032,"parent_record_offset":119004,"ancestor_record_offset":118900,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":119960,"parent_record_offset":119928,"ancestor_record_offset":118900,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":119648,"parent_record_offset":119620,"ancestor_record_offset":118900,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":119824,"parent_record_offset":119112,"ancestor_record_offset":118900,"inlinee":4730,"source_line":1319,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":120136,"parent_record_offset":119892,"ancestor_record_offset":118900,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$61::read_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":119112,"parent_record_offset":119068,"ancestor_record_offset":118900,"inlinee":61860,"source_line":229,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::error::error_invalid_string_utf8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/error.rs
//     evidence={"module":1321,"record_offset":120184,"parent_record_offset":120160,"ancestor_record_offset":118900,"inlinee":27867,"source_line":137,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1321,"record_offset":119156,"parent_record_offset":119112,"ancestor_record_offset":118900,"inlinee":61820,"source_line":159,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::vec_to_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1321,"record_offset":119892,"parent_record_offset":119860,"ancestor_record_offset":118900,"inlinee":61822,"source_line":39,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":118976,"parent_record_offset":118944,"ancestor_record_offset":118900,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":119004,"parent_record_offset":118944,"ancestor_record_offset":118900,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$35::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":120488,"parent_record_offset":120464,"ancestor_record_offset":118900,"inlinee":61797,"source_line":116,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$6::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":120464,"parent_record_offset":120440,"ancestor_record_offset":118900,"inlinee":61856,"source_line":171,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":119180,"parent_record_offset":119156,"ancestor_record_offset":118900,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":120512,"parent_record_offset":120488,"ancestor_record_offset":118900,"inlinee":61798,"source_line":288,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":119332,"parent_record_offset":119112,"ancestor_record_offset":118900,"inlinee":61821,"source_line":514,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$61::read_from::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 229
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1A4
// 首条位置: {"module":1321,"inlinee":61860}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":119824,"parent_record_offset":119112,"ancestor_record_offset":119112,"inlinee":4730,"source_line":1319,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1321,"record_offset":119156,"parent_record_offset":119112,"ancestor_record_offset":119112,"inlinee":61820,"source_line":159,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":119332,"parent_record_offset":119112,"ancestor_record_offset":119112,"inlinee":61821,"source_line":514,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":119596,"parent_record_offset":119572,"ancestor_record_offset":119112,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":119620,"parent_record_offset":119596,"ancestor_record_offset":119112,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":120088,"parent_record_offset":120064,"ancestor_record_offset":119112,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":119572,"parent_record_offset":119528,"ancestor_record_offset":119112,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":120064,"parent_record_offset":120008,"ancestor_record_offset":119112,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":120036,"parent_record_offset":120008,"ancestor_record_offset":119112,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":119528,"parent_record_offset":119488,"ancestor_record_offset":119112,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":119488,"parent_record_offset":119448,"ancestor_record_offset":119112,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":119692,"parent_record_offset":119488,"ancestor_record_offset":119112,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":120008,"parent_record_offset":119984,"ancestor_record_offset":119112,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1321,"record_offset":119928,"parent_record_offset":119892,"ancestor_record_offset":119112,"inlinee":13019,"source_line":576,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1321,"record_offset":119408,"parent_record_offset":119332,"ancestor_record_offset":119112,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1321,"record_offset":119448,"parent_record_offset":119408,"ancestor_record_offset":119112,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":119264,"parent_record_offset":119236,"ancestor_record_offset":119112,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1321,"record_offset":119860,"parent_record_offset":119824,"ancestor_record_offset":119112,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":119292,"parent_record_offset":119236,"ancestor_record_offset":119112,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":119960,"parent_record_offset":119928,"ancestor_record_offset":119112,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":119648,"parent_record_offset":119620,"ancestor_record_offset":119112,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":120136,"parent_record_offset":119892,"ancestor_record_offset":119112,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::error::error_invalid_string_utf8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/error.rs
//     evidence={"module":1321,"record_offset":120184,"parent_record_offset":120160,"ancestor_record_offset":119112,"inlinee":27867,"source_line":137,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::vec_to_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1321,"record_offset":119892,"parent_record_offset":119860,"ancestor_record_offset":119112,"inlinee":61822,"source_line":39,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":119208,"parent_record_offset":119180,"ancestor_record_offset":119112,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":119236,"parent_record_offset":119180,"ancestor_record_offset":119112,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":119180,"parent_record_offset":119156,"ancestor_record_offset":119112,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$67::clone
// 记录形式: inlinee_declaration; instances=2
// 源码行: 235
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1616B
// 首条位置: {"module":2835,"inlinee":90475}
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
// IPI IDs: 0xF194
// 首条位置: {"module":1321,"inlinee":61844}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":121248,"parent_record_offset":116544,"ancestor_record_offset":116544,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":116728,"parent_record_offset":116544,"ancestor_record_offset":116544,"inlinee":4730,"source_line":1319,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":118304,"parent_record_offset":116544,"ancestor_record_offset":116544,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1321,"record_offset":118116,"parent_record_offset":116544,"ancestor_record_offset":116544,"inlinee":61820,"source_line":159,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":116604,"parent_record_offset":116544,"ancestor_record_offset":116544,"inlinee":61845,"source_line":108,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":118332,"parent_record_offset":116544,"ancestor_record_offset":116544,"inlinee":61821,"source_line":514,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":117276,"parent_record_offset":117252,"ancestor_record_offset":116544,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":117300,"parent_record_offset":117276,"ancestor_record_offset":116544,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":117764,"parent_record_offset":117740,"ancestor_record_offset":116544,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":117252,"parent_record_offset":117208,"ancestor_record_offset":116544,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":117740,"parent_record_offset":117684,"ancestor_record_offset":116544,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":117712,"parent_record_offset":117684,"ancestor_record_offset":116544,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":117208,"parent_record_offset":117168,"ancestor_record_offset":116544,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":117168,"parent_record_offset":117128,"ancestor_record_offset":116544,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":117372,"parent_record_offset":117168,"ancestor_record_offset":116544,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":117684,"parent_record_offset":117660,"ancestor_record_offset":116544,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1321,"record_offset":117600,"parent_record_offset":117568,"ancestor_record_offset":116544,"inlinee":13019,"source_line":576,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1321,"record_offset":117088,"parent_record_offset":117008,"ancestor_record_offset":116544,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1321,"record_offset":117128,"parent_record_offset":117088,"ancestor_record_offset":116544,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1321,"record_offset":118612,"parent_record_offset":118576,"ancestor_record_offset":116544,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1321,"record_offset":118636,"parent_record_offset":118612,"ancestor_record_offset":116544,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":116940,"parent_record_offset":116912,"ancestor_record_offset":116544,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1321,"record_offset":118432,"parent_record_offset":118332,"ancestor_record_offset":116544,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1321,"record_offset":117536,"parent_record_offset":117504,"ancestor_record_offset":116544,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":116692,"parent_record_offset":116664,"ancestor_record_offset":116544,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":117636,"parent_record_offset":117600,"ancestor_record_offset":116544,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1321,"record_offset":118828,"parent_record_offset":118332,"ancestor_record_offset":116544,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":117328,"parent_record_offset":117300,"ancestor_record_offset":116544,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":121160,"parent_record_offset":118332,"ancestor_record_offset":116544,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::and_then
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":117504,"parent_record_offset":116776,"ancestor_record_offset":116544,"inlinee":4730,"source_line":1319,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":117812,"parent_record_offset":117568,"ancestor_record_offset":116544,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":120700,"parent_record_offset":118900,"ancestor_record_offset":116544,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":120732,"parent_record_offset":118900,"ancestor_record_offset":116544,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$61::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":118900,"parent_record_offset":118856,"ancestor_record_offset":116544,"inlinee":61858,"source_line":229,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$61::read_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":119112,"parent_record_offset":119068,"ancestor_record_offset":116544,"inlinee":61860,"source_line":229,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$70::read_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":116776,"parent_record_offset":116728,"ancestor_record_offset":116544,"inlinee":61847,"source_line":235,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::error::error_invalid_string_utf8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/error.rs
//     evidence={"module":1321,"record_offset":117860,"parent_record_offset":117836,"ancestor_record_offset":116544,"inlinee":27867,"source_line":137,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1321,"record_offset":116824,"parent_record_offset":116776,"ancestor_record_offset":116544,"inlinee":61820,"source_line":159,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::vec_to_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1321,"record_offset":117568,"parent_record_offset":117536,"ancestor_record_offset":116544,"inlinee":61822,"source_line":39,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":116636,"parent_record_offset":116604,"ancestor_record_offset":116544,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":116664,"parent_record_offset":116604,"ancestor_record_offset":116544,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$35::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":120488,"parent_record_offset":120464,"ancestor_record_offset":116544,"inlinee":61797,"source_line":116,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$6::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":120464,"parent_record_offset":120440,"ancestor_record_offset":116544,"inlinee":61856,"source_line":171,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":116852,"parent_record_offset":116824,"ancestor_record_offset":116544,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":120512,"parent_record_offset":120488,"ancestor_record_offset":116544,"inlinee":61798,"source_line":288,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":118944,"parent_record_offset":118900,"ancestor_record_offset":116544,"inlinee":61845,"source_line":108,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":118856,"parent_record_offset":118332,"ancestor_record_offset":116544,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":117008,"parent_record_offset":116776,"ancestor_record_offset":116544,"inlinee":61821,"source_line":514,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$70::read_from::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 235
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF197
// 首条位置: {"module":1321,"inlinee":61847}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":117504,"parent_record_offset":116776,"ancestor_record_offset":116776,"inlinee":4730,"source_line":1319,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1321,"record_offset":116824,"parent_record_offset":116776,"ancestor_record_offset":116776,"inlinee":61820,"source_line":159,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":117008,"parent_record_offset":116776,"ancestor_record_offset":116776,"inlinee":61821,"source_line":514,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":117276,"parent_record_offset":117252,"ancestor_record_offset":116776,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":117300,"parent_record_offset":117276,"ancestor_record_offset":116776,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":117764,"parent_record_offset":117740,"ancestor_record_offset":116776,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":117252,"parent_record_offset":117208,"ancestor_record_offset":116776,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":117740,"parent_record_offset":117684,"ancestor_record_offset":116776,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":117712,"parent_record_offset":117684,"ancestor_record_offset":116776,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":117208,"parent_record_offset":117168,"ancestor_record_offset":116776,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":117168,"parent_record_offset":117128,"ancestor_record_offset":116776,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":117372,"parent_record_offset":117168,"ancestor_record_offset":116776,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":117684,"parent_record_offset":117660,"ancestor_record_offset":116776,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1321,"record_offset":117600,"parent_record_offset":117568,"ancestor_record_offset":116776,"inlinee":13019,"source_line":576,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1321,"record_offset":117088,"parent_record_offset":117008,"ancestor_record_offset":116776,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1321,"record_offset":117128,"parent_record_offset":117088,"ancestor_record_offset":116776,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":116940,"parent_record_offset":116912,"ancestor_record_offset":116776,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1321,"record_offset":117536,"parent_record_offset":117504,"ancestor_record_offset":116776,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":116968,"parent_record_offset":116912,"ancestor_record_offset":116776,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":117636,"parent_record_offset":117600,"ancestor_record_offset":116776,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":117328,"parent_record_offset":117300,"ancestor_record_offset":116776,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":117812,"parent_record_offset":117568,"ancestor_record_offset":116776,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::error::error_invalid_string_utf8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/error.rs
//     evidence={"module":1321,"record_offset":117860,"parent_record_offset":117836,"ancestor_record_offset":116776,"inlinee":27867,"source_line":137,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::vec_to_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1321,"record_offset":117568,"parent_record_offset":117536,"ancestor_record_offset":116776,"inlinee":61822,"source_line":39,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":116884,"parent_record_offset":116852,"ancestor_record_offset":116776,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":116912,"parent_record_offset":116852,"ancestor_record_offset":116776,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":116852,"parent_record_offset":116824,"ancestor_record_offset":116776,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::StructType::nbyte
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 244
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16157
// 首条位置: {"module":2835,"inlinee":90455}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2835,"record_offset":4400,"parent_record_offset":4344,"ancestor_record_offset":4344,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2835,"record_offset":4368,"parent_record_offset":4344,"ancestor_record_offset":4344,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2835,"record_offset":4448,"parent_record_offset":4428,"ancestor_record_offset":4344,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$8::nbyte::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":2835,"record_offset":4428,"parent_record_offset":4400,"ancestor_record_offset":4344,"inlinee":90453,"source_line":247,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$8::nbyte::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 247
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16155
// 首条位置: {"module":2835,"inlinee":90453}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2835,"record_offset":4448,"parent_record_offset":4428,"ancestor_record_offset":4428,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$75::clone
// 记录形式: inlinee_declaration; instances=2
// 源码行: 272
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16169
// 首条位置: {"module":2835,"inlinee":90473}
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
// IPI IDs: 0xF192
// 首条位置: {"module":1321,"inlinee":61842}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":116416,"parent_record_offset":116060,"ancestor_record_offset":116060,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":116096,"parent_record_offset":116060,"ancestor_record_offset":116060,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":116308,"parent_record_offset":116284,"ancestor_record_offset":116060,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":116332,"parent_record_offset":116308,"ancestor_record_offset":116060,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":116252,"parent_record_offset":116212,"ancestor_record_offset":116060,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":116284,"parent_record_offset":116252,"ancestor_record_offset":116060,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1321,"record_offset":116212,"parent_record_offset":116136,"ancestor_record_offset":116060,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":116360,"parent_record_offset":116332,"ancestor_record_offset":116060,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":116176,"parent_record_offset":116136,"ancestor_record_offset":116060,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$24::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":116136,"parent_record_offset":116096,"ancestor_record_offset":116060,"inlinee":61836,"source_line":573,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection_ty::impl$10::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 277
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16161
// 首条位置: {"module":2835,"inlinee":90465}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2835,"record_offset":5244,"parent_record_offset":5212,"ancestor_record_offset":5212,"inlinee":5667,"source_line":1619,"archive":"_pdb/A/modules/02835.jsonl.gz"}
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
// 首条位置: {"module":1321,"record_offset":110880,"rva":14585056,"va":5383294176}
// 编译实例 RVAs: 0xde8ce0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10982796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383294176,"raw_hex":"e8746a58ff","size":5,"soft_fail":false,"target_va":5372314208,"va":5383296999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038d1e0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10862837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383294176,"raw_hex":"e80b3f5aff","size":5,"soft_fail":false,"target_va":5372432864,"va":5383295696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403902a0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10852075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383294176,"raw_hex":"e815695aff","size":5,"soft_fail":false,"target_va":5372445344,"va":5383297414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10545099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383294176,"raw_hex":"e835185fff","size":5,"soft_fail":false,"target_va":5372749696,"va":5383294790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10545864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383294176,"raw_hex":"e838155fff","size":5,"soft_fail":false,"target_va":5372749712,"va":5383295571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e64e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7351907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383294176,"raw_hex":"e89dd18fff","size":5,"soft_fail":false,"target_va":5375943904,"va":5383295806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19662534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383294176,"raw_hex":"e8c6062c01","size":5,"soft_fail":false,"target_va":5402959328,"va":5383296789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20297510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383294176,"raw_hex":"e826b73501","size":5,"soft_fail":false,"target_va":5403593360,"va":5383295845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t19681259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383294176,"raw_hex":"e8eb4f2c01","size":5,"soft_fail":false,"target_va":5402978144,"va":5383296880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<shader_pipeline::reflection_ty::Type> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383294176,"raw_hex":"e8dbedffff","size":5,"soft_fail":false,"target_va":5383292832,"va":5383297472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5704455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383294176,"raw_hex":"e8f9f4a8ff","size":5,"soft_fail":false,"target_va":5377591424,"va":5383295874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383294176,"raw_hex":"e82affffff","size":5,"soft_fail":false,"target_va":5383294176,"va":5383294385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":115216,"parent_record_offset":110880,"ancestor_record_offset":110880,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":111272,"parent_record_offset":110880,"ancestor_record_offset":110880,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":111120,"parent_record_offset":110880,"ancestor_record_offset":110880,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":111300,"parent_record_offset":110880,"ancestor_record_offset":110880,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":112092,"parent_record_offset":112068,"ancestor_record_offset":110880,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":112116,"parent_record_offset":112092,"ancestor_record_offset":110880,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":113268,"parent_record_offset":113244,"ancestor_record_offset":110880,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":112036,"parent_record_offset":111996,"ancestor_record_offset":110880,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":112068,"parent_record_offset":112036,"ancestor_record_offset":110880,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":113244,"parent_record_offset":113220,"ancestor_record_offset":110880,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1321,"record_offset":111996,"parent_record_offset":111920,"ancestor_record_offset":110880,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1321,"record_offset":113220,"parent_record_offset":113092,"ancestor_record_offset":110880,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":117712,"parent_record_offset":117684,"ancestor_record_offset":110880,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":117208,"parent_record_offset":117168,"ancestor_record_offset":110880,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":117168,"parent_record_offset":117128,"ancestor_record_offset":110880,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":117372,"parent_record_offset":117168,"ancestor_record_offset":110880,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":117684,"parent_record_offset":117660,"ancestor_record_offset":110880,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1321,"record_offset":117600,"parent_record_offset":117568,"ancestor_record_offset":110880,"inlinee":13019,"source_line":576,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1321,"record_offset":117088,"parent_record_offset":117008,"ancestor_record_offset":110880,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1321,"record_offset":117128,"parent_record_offset":117088,"ancestor_record_offset":110880,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1321,"record_offset":118612,"parent_record_offset":118576,"ancestor_record_offset":110880,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1321,"record_offset":118636,"parent_record_offset":118612,"ancestor_record_offset":110880,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":111204,"parent_record_offset":111176,"ancestor_record_offset":110880,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1321,"record_offset":118432,"parent_record_offset":118332,"ancestor_record_offset":110880,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1321,"record_offset":117536,"parent_record_offset":117504,"ancestor_record_offset":110880,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":111236,"parent_record_offset":111176,"ancestor_record_offset":110880,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":113092,"parent_record_offset":111820,"ancestor_record_offset":110880,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1321,"record_offset":118828,"parent_record_offset":118332,"ancestor_record_offset":110880,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":112144,"parent_record_offset":112116,"ancestor_record_offset":110880,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":121160,"parent_record_offset":118332,"ancestor_record_offset":110880,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::and_then
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":112380,"parent_record_offset":111820,"ancestor_record_offset":110880,"inlinee":4730,"source_line":1319,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":117812,"parent_record_offset":117568,"ancestor_record_offset":110880,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":111960,"parent_record_offset":111920,"ancestor_record_offset":110880,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":112228,"parent_record_offset":111820,"ancestor_record_offset":110880,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$19::read_from
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":111328,"parent_record_offset":111300,"ancestor_record_offset":110880,"inlinee":61832,"source_line":17,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$27::read_from
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":113528,"parent_record_offset":113492,"ancestor_record_offset":110880,"inlinee":61840,"source_line":116,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$36::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":114596,"parent_record_offset":114572,"ancestor_record_offset":110880,"inlinee":61853,"source_line":140,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$45::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":113448,"parent_record_offset":113400,"ancestor_record_offset":110880,"inlinee":61838,"source_line":147,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$45::read_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":114544,"parent_record_offset":114516,"ancestor_record_offset":110880,"inlinee":61851,"source_line":147,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$45::read_from::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":114940,"parent_record_offset":114912,"ancestor_record_offset":110880,"inlinee":61861,"source_line":147,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$53::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":111820,"parent_record_offset":111756,"ancestor_record_offset":110880,"inlinee":61834,"source_line":191,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$53::read_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":112408,"parent_record_offset":112380,"ancestor_record_offset":110880,"inlinee":61849,"source_line":191,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$53::read_from::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":112804,"parent_record_offset":112772,"ancestor_record_offset":110880,"inlinee":61854,"source_line":191,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$61::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":118900,"parent_record_offset":118856,"ancestor_record_offset":110880,"inlinee":61858,"source_line":229,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$61::read_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":119112,"parent_record_offset":119068,"ancestor_record_offset":110880,"inlinee":61860,"source_line":229,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$70::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":116544,"parent_record_offset":116480,"ancestor_record_offset":110880,"inlinee":61844,"source_line":235,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$70::read_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":116776,"parent_record_offset":116728,"ancestor_record_offset":110880,"inlinee":61847,"source_line":235,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$78::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":1321,"record_offset":116060,"parent_record_offset":116020,"ancestor_record_offset":110880,"inlinee":61842,"source_line":272,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::error::error_invalid_string_utf8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/error.rs
//     evidence={"module":1321,"record_offset":117860,"parent_record_offset":117836,"ancestor_record_offset":110880,"inlinee":27867,"source_line":137,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1321,"record_offset":116824,"parent_record_offset":116776,"ancestor_record_offset":110880,"inlinee":61820,"source_line":159,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::vec_to_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1321,"record_offset":117568,"parent_record_offset":117536,"ancestor_record_offset":110880,"inlinee":61822,"source_line":39,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":111148,"parent_record_offset":111120,"ancestor_record_offset":110880,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=26
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":111176,"parent_record_offset":111120,"ancestor_record_offset":110880,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$24::read_from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":111920,"parent_record_offset":111880,"ancestor_record_offset":110880,"inlinee":61836,"source_line":573,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":111536,"parent_record_offset":111512,"ancestor_record_offset":110880,"inlinee":61803,"source_line":116,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$35::read_from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":112892,"parent_record_offset":112864,"ancestor_record_offset":110880,"inlinee":61797,"source_line":116,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$6::read_from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":112864,"parent_record_offset":112836,"ancestor_record_offset":110880,"inlinee":61856,"source_line":171,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":111356,"parent_record_offset":111328,"ancestor_record_offset":110880,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u64
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":112920,"parent_record_offset":112892,"ancestor_record_offset":110880,"inlinee":61798,"source_line":288,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u8
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":112256,"parent_record_offset":111820,"ancestor_record_offset":110880,"inlinee":61845,"source_line":108,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":111512,"parent_record_offset":111328,"ancestor_record_offset":110880,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":117008,"parent_record_offset":116776,"ancestor_record_offset":110880,"inlinee":61821,"source_line":514,"archive":"_pdb/A/modules/01321.jsonl.gz"}
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
// 首条位置: {"module":2835,"record_offset":4200,"rva":25311120,"va":5394020240}
// 编译实例 RVAs: 0x1823790
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] shader_pipeline::reflection_ty::Type::nbyte
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394020240,"raw_hex":"e88affffff","size":5,"soft_fail":false,"target_va":5394020240,"va":5394020353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] shader_pipeline::reflection_ty::ArrayType::nbyte
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":2835,"record_offset":4724,"parent_record_offset":4200,"ancestor_record_offset":4200,"inlinee":90459,"source_line":215,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::reflection_ty::MatrixType::nbyte
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":2835,"record_offset":4488,"parent_record_offset":4200,"ancestor_record_offset":4200,"inlinee":90457,"source_line":169,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::reflection_ty::ScalarType::nbyte
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":2835,"record_offset":4616,"parent_record_offset":4200,"ancestor_record_offset":4200,"inlinee":90449,"source_line":64,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::reflection_ty::StructType::nbyte
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":2835,"record_offset":4344,"parent_record_offset":4200,"ancestor_record_offset":4200,"inlinee":90455,"source_line":244,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::reflection_ty::VectorType::nbyte
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":2835,"record_offset":4652,"parent_record_offset":4200,"ancestor_record_offset":4200,"inlinee":90451,"source_line":130,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2835,"record_offset":4400,"parent_record_offset":4344,"ancestor_record_offset":4200,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2835,"record_offset":4448,"parent_record_offset":4428,"ancestor_record_offset":4200,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2835,"record_offset":4748,"parent_record_offset":4724,"ancestor_record_offset":4200,"inlinee":4156,"source_line":1003,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2835,"record_offset":4368,"parent_record_offset":4344,"ancestor_record_offset":4200,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::ScalarType::nbyte
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":2835,"record_offset":4544,"parent_record_offset":4516,"ancestor_record_offset":4200,"inlinee":90449,"source_line":64,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::VectorType::nbyte
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":2835,"record_offset":4516,"parent_record_offset":4488,"ancestor_record_offset":4200,"inlinee":90451,"source_line":130,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection_ty::impl$8::nbyte::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":2835,"record_offset":4428,"parent_record_offset":4400,"ancestor_record_offset":4200,"inlinee":90453,"source_line":247,"archive":"_pdb/A/modules/02835.jsonl.gz"}
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
// 首条位置: {"module":2835,"record_offset":4924,"rva":25311456,"va":5394020576}
// 编译实例 RVAs: 0x18238e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16437056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394020576,"raw_hex":"e8c03005ff","size":5,"soft_fail":false,"target_va":5377583920,"va":5394020971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-16440398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394020576,"raw_hex":"e8b22305ff","size":5,"soft_fail":false,"target_va":5377580528,"va":5394020921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-16437246","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5394020576,"raw_hex":"e9023005ff","size":5,"soft_fail":false,"target_va":5377583920,"va":5394021161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x141823140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-2017","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5394020576,"raw_hex":"e91ff8ffff","size":5,"soft_fail":false,"target_va":5394018624,"va":5394020636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x141823250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-1885","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5394020576,"raw_hex":"e9a3f8ffff","size":5,"soft_fail":false,"target_va":5394018896,"va":5394020776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1418234f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reflection_ty.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-1568","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5394020576,"raw_hex":"e9e0f9ffff","size":5,"soft_fail":false,"target_va":5394019568,"va":5394021131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] shader_pipeline::reflection_ty::impl$10::fmt
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":2835,"record_offset":5212,"parent_record_offset":4924,"ancestor_record_offset":4924,"inlinee":90465,"source_line":277,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::reflection_ty::impl$3::fmt
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":2835,"record_offset":5156,"parent_record_offset":4924,"ancestor_record_offset":4924,"inlinee":90463,"source_line":135,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::reflection_ty::impl$7::fmt
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs
//     evidence={"module":2835,"record_offset":5060,"parent_record_offset":4924,"ancestor_record_offset":4924,"inlinee":90461,"source_line":220,"archive":"_pdb/A/modules/02835.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2835,"record_offset":5096,"parent_record_offset":5060,"ancestor_record_offset":4924,"inlinee":5667,"source_line":1619,"archive":"_pdb/A/modules/02835.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::ArrayType::nbyte
// 信息 ID: 88c3e28e82d438fd4938ec25
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::Type::nbyte; module=2835, inline_record=4724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::MatrixType::nbyte
// 信息 ID: b364113420ca3658e44939f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::Type::nbyte; module=2835, inline_record=4488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::ScalarType::nbyte
// 信息 ID: d5fe517b20a3ec02621b8a66
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > @ 0xe9ae0c -> 0x1823110 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > @ 0xe9ae3e -> 0x1823110 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] shader_pipeline::reflection_ty::Type::nbyte; module=2835, inline_record=4544; 不是独立调用边。
// [PDB 内联宿主] shader_pipeline::reflection_ty::Type::nbyte; module=2835, inline_record=4616; 不是独立调用边。
// [PDB 内联宿主] shader_pipeline::reflection_ty::Type::nbyte; module=2835, inline_record=4680; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::StructType::nbyte
// 信息 ID: d7340e7bc3ea64eca874f738
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::Type::nbyte; module=2835, inline_record=4344; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::Type::nbyte
// 信息 ID: a695c342480376d64f543c3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > @ 0xe9ae61 -> 0x1823790 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::VectorType::nbyte
// 信息 ID: 1ddece3bd96870c63922fab0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::Type::nbyte; module=2835, inline_record=4516; 不是独立调用边。
// [PDB 内联宿主] shader_pipeline::reflection_ty::Type::nbyte; module=2835, inline_record=4652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$10::fmt
// 信息 ID: 4b735daa9ea99a9308ac95b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$12::fmt; module=2835, inline_record=5212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$12::fmt
// 信息 ID: 511c541ffe67634f08caaddb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$35::fmt<enum2$<shader_pipeline::reflection_ty::Type>,alloc::alloc::Global> @ 0x1c95e43 -> 0x18238e0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$19::read_from
// 信息 ID: 0ad8a154c9f49c2a2544cb02
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1321, inline_record=111328; 不是独立调用边。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1321, inline_record=113596; 不是独立调用边。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1321, inline_record=115384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$1::fmt
// 信息 ID: 3c9a8c8088c2773950202854
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::fmt::impl$53::fmt<enum2$<shader_pipeline::reflection_ty::ScalarType> > | core::fmt::impl$53::fmt<enum2$<shader_pipeline::reflection_ty::ScalarType> > @ 0x117dd43 -> 0x1823140 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] shader_pipeline::reflection_ty::impl$12::fmt @ 0x182391c -> 0x1823140 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$25::clone
// 信息 ID: c41c4d3912cd44b68c5009b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$27::read_from
// 信息 ID: 753e8253febaec585428f8fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1321, inline_record=113528; 不是独立调用边。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1321, inline_record=115324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$36::read_from
// 信息 ID: 258c61456ef8aa210f5e6615
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1321, inline_record=114596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$3::fmt
// 信息 ID: aed4752bba8e9d257a7199b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$12::fmt; module=2835, inline_record=5156; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$43::clone
// 信息 ID: f1b77792b3af67d104d9db3f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$45::read_from
// 信息 ID: 5c3c48a621befa77ad239e2f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1321, inline_record=113448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$45::read_from::closure$0
// 信息 ID: 2602cf3293f791586a126e77
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "shader_pipeline::reflection_ty::impl$45::read_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1321, inline_record=114544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$45::read_from::closure$1
// 信息 ID: 27c4dc6171f9a0039a9ddfc2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "shader_pipeline::reflection_ty::impl$45::read_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1321, inline_record=114940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$51::clone
// 信息 ID: 2558643c882094bd1a0fc1c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$53::read_from
// 信息 ID: 04d4ab3187263c3516443568
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1321, inline_record=111820; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$53::read_from::closure$0
// 信息 ID: 9c8d003f29f8623462cce573
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "shader_pipeline::reflection_ty::impl$53::read_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1321, inline_record=112408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$53::read_from::closure$1
// 信息 ID: 18ea13ff755a546b46aaf95b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "shader_pipeline::reflection_ty::impl$53::read_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1321, inline_record=112804; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$58::clone
// 信息 ID: 353a815d61c27d4ef8af1658
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$5::fmt
// 信息 ID: 8590b2c6a7df2969d118bff8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$61::read_from
// 信息 ID: 0b9b3e3cbbcfcd2163a273f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1321, inline_record=118900; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$61::read_from::closure$0
// 信息 ID: 7c27fefb5a2fd1bc9e024c46
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "shader_pipeline::reflection_ty::impl$61::read_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1321, inline_record=119112; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$67::clone
// 信息 ID: 918316018ffd91860dc519ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$70::read_from
// 信息 ID: ea38953a7d58c73d6b864ac1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1321, inline_record=116544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$70::read_from::closure$0
// 信息 ID: 975c6c05d0b8767f3fac30b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "shader_pipeline::reflection_ty::impl$70::read_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1321, inline_record=116776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$75::clone
// 信息 ID: 903ea216dd3749e7030cdce3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$78::read_from
// 信息 ID: 64b6fde24f7bfc1d7548d2e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >; module=1321, inline_record=116060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$7::fmt
// 信息 ID: 1e1b489cbac2668667b1ca8f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::impl$12::fmt; module=2835, inline_record=5060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$86::read_from<speedy::context::LittleEndian,speedy::readable::BufferReader<speedy::context::LittleEndian> >
// 信息 ID: 3762f7aecdc5fcc91bc0d3de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] speedy::readable::BufferReader -> ["speedy::context::LittleEndian"]; 字段布局 = unknown。
// [类型关系] shader_pipeline::reflection_ty::impl$86::read_from -> ["speedy::context::LittleEndian", "speedy::readable::BufferReader<speedy::context::LittleEndian>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,speedy::error::Error> >,core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,std::collections::hash::map::HashMap<alloc::string::String,shader_pipeline::reflection::ShaderInput,std::hash::random::RandomState> >,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,std::collections::hash::map::HashMap<alloc::string::String,shader_pipeline::reflection::ShaderInput,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,hashbrown::map::impl$94::extend::closure_env$0<alloc::string::String,shader_pipeline::reflection::ShaderInput,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,std::collections::hash::map::HashMap<alloc::string::String,shader_pipeline::reflection::ShaderInput,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > > @ 0xde7aaa -> 0xde8ce0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$8::nbyte::closure$0
// 信息 ID: 6c73c84dfc52c5ecfbd2484a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "shader_pipeline::reflection_ty::impl$8::nbyte", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection_ty::Type::nbyte; module=2835, inline_record=4428; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection_ty::impl$9::fmt
// 信息 ID: f2a7a110e0d815f96119a7a1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

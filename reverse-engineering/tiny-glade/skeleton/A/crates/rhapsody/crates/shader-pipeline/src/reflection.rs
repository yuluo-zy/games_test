// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/crates/shader-pipeline/src/reflection.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: c365ecfa1d1f95792c934448aba50fcbdef73d0f1c1dbe0065f2a526b879bcd6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: reflection.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 41, 67, 115, 120, 163, 200, 219, 228, 398, 424, 464, 468, 490, 608, 1318, 1319, 1321, 1323, 1324, 1329, 1331, 1332, 1370, 1611, 1612, 1697, 3010, 3083
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
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$16::eq
// 记录形式: inline_instance, inlinee_declaration; instances=215
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CE2
// 首条位置: {"module":41,"inlinee":7394}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$18::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF163
// 首条位置: {"module":1321,"inlinee":61795}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":85844,"parent_record_offset":85816,"ancestor_record_offset":85816,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":86008,"parent_record_offset":85972,"ancestor_record_offset":85816,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":86040,"parent_record_offset":85972,"ancestor_record_offset":85816,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":85940,"parent_record_offset":85900,"ancestor_record_offset":85816,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":85972,"parent_record_offset":85900,"ancestor_record_offset":85816,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$35::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":85872,"parent_record_offset":85844,"ancestor_record_offset":85816,"inlinee":61797,"source_line":116,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":85900,"parent_record_offset":85872,"ancestor_record_offset":85816,"inlinee":61798,"source_line":288,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$0::hash
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x682F
// 首条位置: {"module":1323,"inlinee":26671}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::BindingNameHash::with_suffix_dims
// 记录形式: compiled_procedure; instances=1
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":608,"record_offset":252,"rva":8194112,"va":5376903232}
// 编译实例 RVAs: 0x7d0840
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$25::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF2AB
// 首条位置: {"module":1331,"inlinee":62123}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1331,"record_offset":15384,"parent_record_offset":13584,"ancestor_record_offset":13584,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":14216,"parent_record_offset":13584,"ancestor_record_offset":13584,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":14044,"parent_record_offset":13584,"ancestor_record_offset":13584,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1331,"record_offset":13860,"parent_record_offset":13584,"ancestor_record_offset":13584,"inlinee":61820,"source_line":159,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_key_value_collection
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":14072,"parent_record_offset":13584,"ancestor_record_offset":13584,"inlinee":62118,"source_line":652,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":13644,"parent_record_offset":13584,"ancestor_record_offset":13584,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1331,"record_offset":13788,"parent_record_offset":13760,"ancestor_record_offset":13584,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1331,"record_offset":14148,"parent_record_offset":14120,"ancestor_record_offset":13584,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1331,"record_offset":14120,"parent_record_offset":14096,"ancestor_record_offset":13584,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1331,"record_offset":14668,"parent_record_offset":14532,"ancestor_record_offset":13584,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1331,"record_offset":14692,"parent_record_offset":14668,"ancestor_record_offset":13584,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1331,"record_offset":13816,"parent_record_offset":13760,"ancestor_record_offset":13584,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1331,"record_offset":15408,"parent_record_offset":15384,"ancestor_record_offset":13584,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$25::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":14176,"parent_record_offset":14120,"ancestor_record_offset":13584,"inlinee":24116,"source_line":1934,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1331,"record_offset":15456,"parent_record_offset":15432,"ancestor_record_offset":13584,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$61::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":15104,"parent_record_offset":15076,"ancestor_record_offset":13584,"inlinee":61810,"source_line":69,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$79::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":14840,"parent_record_offset":14816,"ancestor_record_offset":13584,"inlinee":61808,"source_line":75,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1331,"record_offset":14316,"parent_record_offset":14280,"ancestor_record_offset":13584,"inlinee":61830,"source_line":195,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1331,"record_offset":14344,"parent_record_offset":14316,"ancestor_record_offset":13584,"inlinee":61820,"source_line":159,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":13732,"parent_record_offset":13700,"ancestor_record_offset":13584,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_borrowed_from_reader
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":14572,"parent_record_offset":14532,"ancestor_record_offset":13584,"inlinee":62126,"source_line":123,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":13760,"parent_record_offset":13700,"ancestor_record_offset":13584,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$2::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1331,"record_offset":14280,"parent_record_offset":14248,"ancestor_record_offset":13584,"inlinee":62125,"source_line":54,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1331,"record_offset":13672,"parent_record_offset":13644,"ancestor_record_offset":13584,"inlinee":61803,"source_line":116,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_collection
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":14096,"parent_record_offset":14072,"ancestor_record_offset":13584,"inlinee":62119,"source_line":630,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_key_value_collection
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":14532,"parent_record_offset":14280,"ancestor_record_offset":13584,"inlinee":62118,"source_line":652,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":13700,"parent_record_offset":13672,"ancestor_record_offset":13584,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":14864,"parent_record_offset":14840,"ancestor_record_offset":13584,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$29::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF174
// 首条位置: {"module":1321,"inlinee":61812}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":89944,"parent_record_offset":89684,"ancestor_record_offset":89684,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":89712,"parent_record_offset":89684,"ancestor_record_offset":89684,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":89844,"parent_record_offset":89816,"ancestor_record_offset":89684,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":89872,"parent_record_offset":89816,"ancestor_record_offset":89684,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":89788,"parent_record_offset":89760,"ancestor_record_offset":89684,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":89816,"parent_record_offset":89760,"ancestor_record_offset":89684,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":89736,"parent_record_offset":89712,"ancestor_record_offset":89684,"inlinee":61803,"source_line":116,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":89760,"parent_record_offset":89736,"ancestor_record_offset":89684,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$33::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3010,"record_offset":856,"rva":26313968,"va":5395023088}
// 编译实例 RVAs: 0x19184f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reflection.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-17439207","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5395023088,"raw_hex":"e919e6f5fe","size":5,"soft_fail":false,"target_va":5377583920,"va":5395023122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$37::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF178
// 首条位置: {"module":1321,"inlinee":61816}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":97676,"parent_record_offset":97652,"ancestor_record_offset":97652,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":97760,"parent_record_offset":97732,"ancestor_record_offset":97652,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":97792,"parent_record_offset":97732,"ancestor_record_offset":97652,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":97704,"parent_record_offset":97676,"ancestor_record_offset":97652,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":97732,"parent_record_offset":97676,"ancestor_record_offset":97652,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$41::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3010,"record_offset":976,"rva":26314016,"va":5395023136}
// 编译实例 RVAs: 0x1918520
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_tuple_field1_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17437332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395023136,"raw_hex":"e86cedf5fe","size":5,"soft_fail":false,"target_va":5377585888,"va":5395023215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reflection.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-17439253","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5395023136,"raw_hex":"e9ebe5f5fe","size":5,"soft_fail":false,"target_va":5377583920,"va":5395023168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$45::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF17A
// 首条位置: {"module":1321,"inlinee":61818}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":97884,"parent_record_offset":97860,"ancestor_record_offset":97860,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":98036,"parent_record_offset":97860,"ancestor_record_offset":97860,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":97968,"parent_record_offset":97940,"ancestor_record_offset":97860,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":98000,"parent_record_offset":97940,"ancestor_record_offset":97860,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":97912,"parent_record_offset":97884,"ancestor_record_offset":97860,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":97940,"parent_record_offset":97884,"ancestor_record_offset":97860,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":98060,"parent_record_offset":98036,"ancestor_record_offset":97860,"inlinee":61803,"source_line":116,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":98084,"parent_record_offset":98060,"ancestor_record_offset":97860,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$48::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x181A3
// 首条位置: {"module":3010,"inlinee":98723}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$52::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF176
// 首条位置: {"module":1321,"inlinee":61814}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":98900,"parent_record_offset":97592,"ancestor_record_offset":97592,"inlinee":4730,"source_line":1319,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1321,"record_offset":98248,"parent_record_offset":97592,"ancestor_record_offset":97592,"inlinee":61820,"source_line":159,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":97628,"parent_record_offset":97592,"ancestor_record_offset":97592,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":98436,"parent_record_offset":97592,"ancestor_record_offset":97592,"inlinee":61821,"source_line":514,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":98672,"parent_record_offset":98648,"ancestor_record_offset":97592,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":98696,"parent_record_offset":98672,"ancestor_record_offset":97592,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":99160,"parent_record_offset":99136,"ancestor_record_offset":97592,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":98648,"parent_record_offset":98604,"ancestor_record_offset":97592,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":99136,"parent_record_offset":99080,"ancestor_record_offset":97592,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":99108,"parent_record_offset":99080,"ancestor_record_offset":97592,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":98604,"parent_record_offset":98572,"ancestor_record_offset":97592,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":98572,"parent_record_offset":98540,"ancestor_record_offset":97592,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":98768,"parent_record_offset":98572,"ancestor_record_offset":97592,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":99080,"parent_record_offset":99056,"ancestor_record_offset":97592,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1321,"record_offset":98996,"parent_record_offset":98964,"ancestor_record_offset":97592,"inlinee":13019,"source_line":576,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1321,"record_offset":98508,"parent_record_offset":98436,"ancestor_record_offset":97592,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1321,"record_offset":98540,"parent_record_offset":98508,"ancestor_record_offset":97592,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":97760,"parent_record_offset":97732,"ancestor_record_offset":97592,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1321,"record_offset":98932,"parent_record_offset":98900,"ancestor_record_offset":97592,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":97792,"parent_record_offset":97732,"ancestor_record_offset":97592,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":99032,"parent_record_offset":98996,"ancestor_record_offset":97592,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":98724,"parent_record_offset":98696,"ancestor_record_offset":97592,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":99208,"parent_record_offset":98964,"ancestor_record_offset":97592,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$37::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1321,"record_offset":97652,"parent_record_offset":97628,"ancestor_record_offset":97592,"inlinee":61816,"source_line":46,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$45::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1321,"record_offset":97860,"parent_record_offset":97836,"ancestor_record_offset":97592,"inlinee":61818,"source_line":56,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::error::error_invalid_string_utf8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/error.rs
//     evidence={"module":1321,"record_offset":99256,"parent_record_offset":99232,"ancestor_record_offset":97592,"inlinee":27867,"source_line":137,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::vec_to_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1321,"record_offset":98964,"parent_record_offset":98932,"ancestor_record_offset":97592,"inlinee":61822,"source_line":39,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":97704,"parent_record_offset":97676,"ancestor_record_offset":97592,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":97732,"parent_record_offset":97676,"ancestor_record_offset":97592,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":98060,"parent_record_offset":98036,"ancestor_record_offset":97592,"inlinee":61803,"source_line":116,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":97676,"parent_record_offset":97652,"ancestor_record_offset":97592,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":98036,"parent_record_offset":97860,"ancestor_record_offset":97592,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$56::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6821
// 首条位置: {"module":468,"inlinee":26657}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$58::eq
// 记录形式: inline_instance, inlinee_declaration; instances=29
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2E50
// 首条位置: {"module":163,"inlinee":11856}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$60::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11247
// 首条位置: {"module":1611,"inlinee":70215}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1611,"record_offset":2184,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":8445,"source_line":806,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1611,"record_offset":2208,"parent_record_offset":2184,"ancestor_record_offset":2160,"inlinee":8446,"source_line":377,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1611,"record_offset":2288,"parent_record_offset":2264,"ancestor_record_offset":2160,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1611,"record_offset":2232,"parent_record_offset":2208,"ancestor_record_offset":2160,"inlinee":8447,"source_line":2859,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1611,"record_offset":2264,"parent_record_offset":2208,"ancestor_record_offset":2160,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$61::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF172
// 首条位置: {"module":1321,"inlinee":61810}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":81360,"parent_record_offset":81336,"ancestor_record_offset":81336,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":81492,"parent_record_offset":81464,"ancestor_record_offset":81336,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":81520,"parent_record_offset":81464,"ancestor_record_offset":81336,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":81436,"parent_record_offset":81408,"ancestor_record_offset":81336,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":81464,"parent_record_offset":81408,"ancestor_record_offset":81336,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":81384,"parent_record_offset":81360,"ancestor_record_offset":81336,"inlinee":61803,"source_line":116,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":81408,"parent_record_offset":81384,"ancestor_record_offset":81336,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$74::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x31A4
// 首条位置: {"module":200,"inlinee":12708}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$76::eq
// 记录形式: inline_instance, inlinee_declaration; instances=149
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CE4
// 首条位置: {"module":41,"inlinee":7396}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$79::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF170
// 首条位置: {"module":1321,"inlinee":61808}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":81096,"parent_record_offset":81072,"ancestor_record_offset":81072,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":81232,"parent_record_offset":81204,"ancestor_record_offset":81072,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":81260,"parent_record_offset":81204,"ancestor_record_offset":81072,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":81172,"parent_record_offset":81144,"ancestor_record_offset":81072,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1321,"record_offset":81204,"parent_record_offset":81144,"ancestor_record_offset":81072,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1321,"record_offset":81120,"parent_record_offset":81096,"ancestor_record_offset":81072,"inlinee":61803,"source_line":116,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1321,"record_offset":81144,"parent_record_offset":81120,"ancestor_record_offset":81072,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$84::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF2A5
// 首条位置: {"module":1331,"inlinee":62117}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":13100,"parent_record_offset":12756,"ancestor_record_offset":12756,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1331,"record_offset":12780,"parent_record_offset":12756,"ancestor_record_offset":12756,"inlinee":61820,"source_line":159,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_key_value_collection
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":12956,"parent_record_offset":12756,"ancestor_record_offset":12756,"inlinee":62118,"source_line":652,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1331,"record_offset":12888,"parent_record_offset":12860,"ancestor_record_offset":12756,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1331,"record_offset":13032,"parent_record_offset":13004,"ancestor_record_offset":12756,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1331,"record_offset":13004,"parent_record_offset":12980,"ancestor_record_offset":12756,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1331,"record_offset":12916,"parent_record_offset":12860,"ancestor_record_offset":12756,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$25::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":13060,"parent_record_offset":13004,"ancestor_record_offset":12756,"inlinee":24116,"source_line":1934,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":12832,"parent_record_offset":12804,"ancestor_record_offset":12756,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":12860,"parent_record_offset":12804,"ancestor_record_offset":12756,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_collection
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":12980,"parent_record_offset":12956,"ancestor_record_offset":12756,"inlinee":62119,"source_line":630,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":12804,"parent_record_offset":12780,"ancestor_record_offset":12756,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$89::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF2A3
// 首条位置: {"module":1331,"inlinee":62115}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1331,"record_offset":11988,"parent_record_offset":11932,"ancestor_record_offset":11932,"inlinee":61820,"source_line":159,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":12192,"parent_record_offset":11932,"ancestor_record_offset":11932,"inlinee":61821,"source_line":514,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1331,"record_offset":12568,"parent_record_offset":12544,"ancestor_record_offset":11932,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1331,"record_offset":12592,"parent_record_offset":12568,"ancestor_record_offset":11932,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1331,"record_offset":12544,"parent_record_offset":12452,"ancestor_record_offset":11932,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1331,"record_offset":12452,"parent_record_offset":12424,"ancestor_record_offset":11932,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1331,"record_offset":12424,"parent_record_offset":12388,"ancestor_record_offset":11932,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1331,"record_offset":12664,"parent_record_offset":12424,"ancestor_record_offset":11932,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1331,"record_offset":12352,"parent_record_offset":12192,"ancestor_record_offset":11932,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1331,"record_offset":12388,"parent_record_offset":12352,"ancestor_record_offset":11932,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1331,"record_offset":12488,"parent_record_offset":12452,"ancestor_record_offset":11932,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1331,"record_offset":12512,"parent_record_offset":12488,"ancestor_record_offset":11932,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1331,"record_offset":12124,"parent_record_offset":12096,"ancestor_record_offset":11932,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1331,"record_offset":13032,"parent_record_offset":13004,"ancestor_record_offset":11932,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1331,"record_offset":13004,"parent_record_offset":12980,"ancestor_record_offset":11932,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1331,"record_offset":12320,"parent_record_offset":12192,"ancestor_record_offset":11932,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1331,"record_offset":12152,"parent_record_offset":12096,"ancestor_record_offset":11932,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1331,"record_offset":13196,"parent_record_offset":12192,"ancestor_record_offset":11932,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1331,"record_offset":12704,"parent_record_offset":12192,"ancestor_record_offset":11932,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1331,"record_offset":12620,"parent_record_offset":12592,"ancestor_record_offset":11932,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1331,"record_offset":13140,"parent_record_offset":12192,"ancestor_record_offset":11932,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$25::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":13060,"parent_record_offset":13004,"ancestor_record_offset":11932,"inlinee":24116,"source_line":1934,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":13100,"parent_record_offset":12756,"ancestor_record_offset":11932,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$84::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":12756,"parent_record_offset":12732,"ancestor_record_offset":11932,"inlinee":62117,"source_line":78,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1331,"record_offset":12780,"parent_record_offset":12756,"ancestor_record_offset":11932,"inlinee":61820,"source_line":159,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":12064,"parent_record_offset":12024,"ancestor_record_offset":11932,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":12096,"parent_record_offset":12024,"ancestor_record_offset":11932,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_collection
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":12980,"parent_record_offset":12956,"ancestor_record_offset":11932,"inlinee":62119,"source_line":630,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_key_value_collection
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":12956,"parent_record_offset":12756,"ancestor_record_offset":11932,"inlinee":62118,"source_line":652,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":12024,"parent_record_offset":11988,"ancestor_record_offset":11932,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":12732,"parent_record_offset":12192,"ancestor_record_offset":11932,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$94::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF2A1
// 首条位置: {"module":1331,"inlinee":62113}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1331,"record_offset":16876,"parent_record_offset":11792,"ancestor_record_offset":11792,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":13416,"parent_record_offset":11792,"ancestor_record_offset":11792,"inlinee":4730,"source_line":1319,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":15696,"parent_record_offset":11792,"ancestor_record_offset":11792,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":13264,"parent_record_offset":11792,"ancestor_record_offset":11792,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1331,"record_offset":15724,"parent_record_offset":11792,"ancestor_record_offset":11792,"inlinee":61820,"source_line":159,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_key_value_collection
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":15936,"parent_record_offset":11792,"ancestor_record_offset":11792,"inlinee":62118,"source_line":652,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":13292,"parent_record_offset":11792,"ancestor_record_offset":11792,"inlinee":61845,"source_line":108,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":11876,"parent_record_offset":11792,"ancestor_record_offset":11792,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1331,"record_offset":12568,"parent_record_offset":12544,"ancestor_record_offset":11792,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1331,"record_offset":12592,"parent_record_offset":12568,"ancestor_record_offset":11792,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1331,"record_offset":12544,"parent_record_offset":12452,"ancestor_record_offset":11792,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1331,"record_offset":12452,"parent_record_offset":12424,"ancestor_record_offset":11792,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1331,"record_offset":12424,"parent_record_offset":12388,"ancestor_record_offset":11792,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1331,"record_offset":12664,"parent_record_offset":12424,"ancestor_record_offset":11792,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1331,"record_offset":12352,"parent_record_offset":12192,"ancestor_record_offset":11792,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1331,"record_offset":12388,"parent_record_offset":12352,"ancestor_record_offset":11792,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1331,"record_offset":12488,"parent_record_offset":12452,"ancestor_record_offset":11792,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1331,"record_offset":12512,"parent_record_offset":12488,"ancestor_record_offset":11792,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1331,"record_offset":12124,"parent_record_offset":12096,"ancestor_record_offset":11792,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1331,"record_offset":13032,"parent_record_offset":13004,"ancestor_record_offset":11792,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1331,"record_offset":13004,"parent_record_offset":12980,"ancestor_record_offset":11792,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1331,"record_offset":14668,"parent_record_offset":14532,"ancestor_record_offset":11792,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1331,"record_offset":12320,"parent_record_offset":12192,"ancestor_record_offset":11792,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1331,"record_offset":12152,"parent_record_offset":12096,"ancestor_record_offset":11792,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1331,"record_offset":13196,"parent_record_offset":12192,"ancestor_record_offset":11792,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1331,"record_offset":12704,"parent_record_offset":12192,"ancestor_record_offset":11792,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1331,"record_offset":12620,"parent_record_offset":12592,"ancestor_record_offset":11792,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1331,"record_offset":13140,"parent_record_offset":12192,"ancestor_record_offset":11792,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$25::from_iter
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":13060,"parent_record_offset":13004,"ancestor_record_offset":11792,"inlinee":24116,"source_line":1934,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":13100,"parent_record_offset":12756,"ancestor_record_offset":11792,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":14044,"parent_record_offset":13584,"ancestor_record_offset":11792,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1331,"record_offset":15456,"parent_record_offset":15432,"ancestor_record_offset":11792,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$25::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":13584,"parent_record_offset":13528,"ancestor_record_offset":11792,"inlinee":62123,"source_line":31,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$61::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":15104,"parent_record_offset":15076,"ancestor_record_offset":11792,"inlinee":61810,"source_line":69,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$79::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":14840,"parent_record_offset":14816,"ancestor_record_offset":11792,"inlinee":61808,"source_line":75,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$84::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":12756,"parent_record_offset":12732,"ancestor_record_offset":11792,"inlinee":62117,"source_line":78,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$89::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":11932,"parent_record_offset":11876,"ancestor_record_offset":11792,"inlinee":62115,"source_line":83,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$94::read_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":13472,"parent_record_offset":13416,"ancestor_record_offset":11792,"inlinee":62121,"source_line":88,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$94::read_from::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":17232,"parent_record_offset":17204,"ancestor_record_offset":11792,"inlinee":62127,"source_line":88,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$94::read_from::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":17260,"parent_record_offset":17232,"ancestor_record_offset":11792,"inlinee":62129,"source_line":88,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1331,"record_offset":14316,"parent_record_offset":14280,"ancestor_record_offset":11792,"inlinee":61830,"source_line":195,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1331,"record_offset":11988,"parent_record_offset":11932,"ancestor_record_offset":11792,"inlinee":61820,"source_line":159,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":12064,"parent_record_offset":12024,"ancestor_record_offset":11792,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_borrowed_from_reader
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":14572,"parent_record_offset":14532,"ancestor_record_offset":11792,"inlinee":62126,"source_line":123,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":12096,"parent_record_offset":12024,"ancestor_record_offset":11792,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$2::read_from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1331,"record_offset":14280,"parent_record_offset":14248,"ancestor_record_offset":11792,"inlinee":62125,"source_line":54,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1331,"record_offset":13672,"parent_record_offset":13644,"ancestor_record_offset":11792,"inlinee":61803,"source_line":116,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_collection
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":12980,"parent_record_offset":12956,"ancestor_record_offset":11792,"inlinee":62119,"source_line":630,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_key_value_collection
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":12956,"parent_record_offset":12756,"ancestor_record_offset":11792,"inlinee":62118,"source_line":652,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":12024,"parent_record_offset":11988,"ancestor_record_offset":11792,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":12732,"parent_record_offset":12192,"ancestor_record_offset":11792,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":12192,"parent_record_offset":11932,"ancestor_record_offset":11792,"inlinee":61821,"source_line":514,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$94::read_from::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF2A9
// 首条位置: {"module":1331,"inlinee":62121}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":15616,"parent_record_offset":13472,"ancestor_record_offset":13472,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":13528,"parent_record_offset":13472,"ancestor_record_offset":13472,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1331,"record_offset":13788,"parent_record_offset":13760,"ancestor_record_offset":13472,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1331,"record_offset":14148,"parent_record_offset":14120,"ancestor_record_offset":13472,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1331,"record_offset":14120,"parent_record_offset":14096,"ancestor_record_offset":13472,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1331,"record_offset":14668,"parent_record_offset":14532,"ancestor_record_offset":13472,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1331,"record_offset":14692,"parent_record_offset":14668,"ancestor_record_offset":13472,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1331,"record_offset":13816,"parent_record_offset":13760,"ancestor_record_offset":13472,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1331,"record_offset":15384,"parent_record_offset":13584,"ancestor_record_offset":13472,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$25::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":14176,"parent_record_offset":14120,"ancestor_record_offset":13472,"inlinee":24116,"source_line":1934,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":14216,"parent_record_offset":13584,"ancestor_record_offset":13472,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":14044,"parent_record_offset":13584,"ancestor_record_offset":13472,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1331,"record_offset":15456,"parent_record_offset":15432,"ancestor_record_offset":13472,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$25::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":13584,"parent_record_offset":13528,"ancestor_record_offset":13472,"inlinee":62123,"source_line":31,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$61::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":15104,"parent_record_offset":15076,"ancestor_record_offset":13472,"inlinee":61810,"source_line":69,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$79::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":14840,"parent_record_offset":14816,"ancestor_record_offset":13472,"inlinee":61808,"source_line":75,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1331,"record_offset":14316,"parent_record_offset":14280,"ancestor_record_offset":13472,"inlinee":61830,"source_line":195,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1331,"record_offset":13860,"parent_record_offset":13584,"ancestor_record_offset":13472,"inlinee":61820,"source_line":159,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":13732,"parent_record_offset":13700,"ancestor_record_offset":13472,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_borrowed_from_reader
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":14572,"parent_record_offset":14532,"ancestor_record_offset":13472,"inlinee":62126,"source_line":123,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":13760,"parent_record_offset":13700,"ancestor_record_offset":13472,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$2::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1331,"record_offset":14280,"parent_record_offset":14248,"ancestor_record_offset":13472,"inlinee":62125,"source_line":54,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1331,"record_offset":13672,"parent_record_offset":13644,"ancestor_record_offset":13472,"inlinee":61803,"source_line":116,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_collection
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":14096,"parent_record_offset":14072,"ancestor_record_offset":13472,"inlinee":62119,"source_line":630,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_key_value_collection
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":14072,"parent_record_offset":13584,"ancestor_record_offset":13472,"inlinee":62118,"source_line":652,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":13700,"parent_record_offset":13672,"ancestor_record_offset":13472,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":13644,"parent_record_offset":13584,"ancestor_record_offset":13472,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$94::read_from::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF2AF
// 首条位置: {"module":1331,"inlinee":62127}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] shader_pipeline::reflection::impl$94::read_from::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":17260,"parent_record_offset":17232,"ancestor_record_offset":17232,"inlinee":62129,"source_line":88,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1331,"record_offset":17420,"parent_record_offset":17392,"ancestor_record_offset":17232,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1331,"record_offset":17448,"parent_record_offset":17392,"ancestor_record_offset":17232,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":17364,"parent_record_offset":17336,"ancestor_record_offset":17232,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":17392,"parent_record_offset":17336,"ancestor_record_offset":17232,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1331,"record_offset":17312,"parent_record_offset":17288,"ancestor_record_offset":17232,"inlinee":61803,"source_line":116,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":17336,"parent_record_offset":17312,"ancestor_record_offset":17232,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":17288,"parent_record_offset":17260,"ancestor_record_offset":17232,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$94::read_from::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF2B1
// 首条位置: {"module":1331,"inlinee":62129}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":17288,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1331,"record_offset":17420,"parent_record_offset":17392,"ancestor_record_offset":17260,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1331,"record_offset":17448,"parent_record_offset":17392,"ancestor_record_offset":17260,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":17364,"parent_record_offset":17336,"ancestor_record_offset":17260,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":17392,"parent_record_offset":17336,"ancestor_record_offset":17260,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1331,"record_offset":17312,"parent_record_offset":17288,"ancestor_record_offset":17260,"inlinee":61803,"source_line":116,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":17336,"parent_record_offset":17312,"ancestor_record_offset":17260,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$3::calculate_name_hashes::closure$0
// 记录形式: inlinee_declaration; instances=1
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11B5E
// 首条位置: {"module":1697,"inlinee":72542}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$3::calculate_name_hashes::closure$1
// 记录形式: inlinee_declaration; instances=1
// 源码行: 112
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11B5F
// 首条位置: {"module":1697,"inlinee":72543}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::len_without_suffix
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x896E
// 首条位置: {"module":608,"inlinee":35182}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::BindingNameHash::new
// 记录形式: compiled_procedure, inlinee_declaration; instances=2
// 源码行: 139
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8973
// 首条位置: {"module":608,"record_offset":392,"rva":8194128,"va":5376903248}
// 编译实例 RVAs: 0x7d0850
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] const_fnv1a_hash::fnv1a_hash_64
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10555364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376903248,"raw_hex":"e8e40fa100","size":5,"soft_fail":false,"target_va":5387458688,"va":5376903319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] shader_pipeline::reflection::len_without_suffix
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":608,"record_offset":528,"parent_record_offset":392,"ancestor_record_offset":392,"inlinee":35182,"source_line":117,"archive":"_pdb/A/modules/00608.jsonl.gz"}
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
// 完整结构及原始类型记录: reflection.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: reflection.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: reflection.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::decorator::DecoratorId (*)(ref$<country_core::resources::walls::decorator::DecoratorId>)
// 候选参数类型: ["ref$<country_core::resources::walls::decorator::DecoratorId>"]
// [文件级接口候选，具体函数归属 unknown] core::char::EscapeDebug (*)(char)
// 候选参数类型: ["char"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>,bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>", "bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::BindingNameHash::new
// 信息 ID: ae151a9cf5ddbc01db444cae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::GardenRaster> @ 0x2661fc -> 0x7d0850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::WaterRaster> @ 0x26678c -> 0x7d0850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PathRaster> @ 0x266d1c -> 0x7d0850 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::BindingNameHash::with_suffix_dims
// 信息 ID: e9cabeb113a72bf3f06c328b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 38 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64e3e -> 0x7d0840 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64af0 -> 0x7d0840 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1ccd6e -> 0x7d0840 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1cca20 -> 0x7d0840 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1cd6ce -> 0x7d0840 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x1cd380 -> 0x7d0840 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset> @ 0x338f1e -> 0x7d0840 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset> @ 0x338bd0 -> 0x7d0840 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> > @ 0x3582d9 -> 0x7d0840 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> > @ 0x35815b -> 0x7d0840 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> > @ 0x35823e -> 0x7d0840 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> > @ 0x358523 -> 0x7d0840 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> > @ 0x357e53 -> 0x7d0840 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TextureFromDisk> @ 0x540316 -> 0x7d0840 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TextureFromDisk> @ 0x5403dc -> 0x7d0840 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset> @ 0x540904 -> 0x7d0840 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$0::hash
// 信息 ID: 9b28cbb172d25d5b60a4f614
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<shader_pipeline::reflection::BindingNameHash,u32>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<shader_pipeline::reflection::BindingNameHash,u32,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > >; module=1323, inline_record=272876; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<shader_pipeline::reflection::BindingNameHash,u32>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<shader_pipeline::reflection::BindingNameHash,u32,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > >; module=1323, inline_record=274624; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > >; module=1323, inline_record=485944; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > >; module=1323, inline_record=487692; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$16::eq
// 信息 ID: 574f0e6b285f3e49578dcb8a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 198 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>; module=41, inline_record=1240; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>; module=41, inline_record=2648; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>; module=41, inline_record=4864; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform<country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder::closure_env$4<ref$<rhapsody::raw::texture::Texture> >,array$<u32,2> >; module=67, inline_record=1352; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo>; module=115, inline_record=7308; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo>; module=115, inline_record=8760; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo>; module=115, inline_record=10212; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform<array$<u32,2> >; module=115, inline_record=11744; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$18::read_from
// 信息 ID: 2a10a2cdad4d33ece783d99a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,speedy::error::Error> >,core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > >,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> >,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; module=1321, inline_record=85816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$25::read_from
// 信息 ID: 1f343e4bc92ed79d7c56fdc7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=1331, inline_record=13584; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$29::read_from
// 信息 ID: 6ad194832102a5432abb0556
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,speedy::error::Error> >,core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,std::collections::hash::map::HashMap<alloc::string::String,shader_pipeline::reflection::ShaderInput,std::hash::random::RandomState> >,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,std::collections::hash::map::HashMap<alloc::string::String,shader_pipeline::reflection::ShaderInput,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,hashbrown::map::impl$94::extend::closure_env$0<alloc::string::String,shader_pipeline::reflection::ShaderInput,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,std::collections::hash::map::HashMap<alloc::string::String,shader_pipeline::reflection::ShaderInput,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; module=1321, inline_record=89684; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$33::fmt
// 信息 ID: e9e2fdaf15637f88b298d66f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$37::read_from
// 信息 ID: 010cb9d1396cf94bfd1b9ea2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,speedy::error::Error> >,core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> >,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; module=1321, inline_record=97652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$3::calculate_name_hashes::closure$0
// 信息 ID: 810444e4c9e4b464ff271aec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "shader_pipeline::reflection::impl$3::calculate_name_hashes", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$3::calculate_name_hashes::closure$1
// 信息 ID: 21654c3ba83390967a768f30
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "shader_pipeline::reflection::impl$3::calculate_name_hashes", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$41::fmt
// 信息 ID: fbcc86de7953b5a5afe0b359
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$45::read_from
// 信息 ID: ee580179c7ce5a0f4ec005fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,speedy::error::Error> >,core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> >,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; module=1321, inline_record=97860; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$48::fmt
// 信息 ID: 162997d41640e51026263e21
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<shader_pipeline::reflection::Binding>; module=3010, inline_record=396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$52::read_from
// 信息 ID: 5ee4f12562b6433ef1c0fe80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,speedy::error::Error> >,core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> >,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; module=1321, inline_record=97592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$56::fmt
// 信息 ID: 1eb949ee49ca43aab9aecfd8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<shader_pipeline::reflection::BindingIndex>; module=468, inline_record=3936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$58::eq
// 信息 ID: 26083759660ba715c8562aec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 16 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> >; module=228, inline_record=9236; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> >; module=228, inline_record=11676; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>; module=398, inline_record=27752; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> >; module=464, inline_record=33272; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> >; module=464, inline_record=35800; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_external_storage_buffer; module=490, inline_record=35288; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >; module=1321, inline_record=72764; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >; module=1321, inline_record=94168; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$60::hash
// 信息 ID: b8047929e61307e5d67b79fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<shader_pipeline::reflection::BindingIndex> >; module=1611, inline_record=2160; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$61::read_from
// 信息 ID: 1cc2105a39eb6ca09cef03ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,speedy::error::Error> >,core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,std::hash::random::RandomState> >,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,hashbrown::map::impl$94::extend::closure_env$0<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; module=1321, inline_record=81336; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,speedy::error::Error> >,core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > >,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> >,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; module=1321, inline_record=86484; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,speedy::error::Error> >,core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> >,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; module=1321, inline_record=97284; 不是独立调用边。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=1331, inline_record=15104; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$74::fmt
// 信息 ID: c790b2c3402eea29f8f4cc30
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<shader_pipeline::reflection::SetIndex>; module=200, inline_record=700; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$51::fmt<shader_pipeline::reflection::SetIndex>; module=468, inline_record=4116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$76::eq
// 信息 ID: b7e32e4a7328b2e6295b20b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 138 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>; module=41, inline_record=1572; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo>; module=115, inline_record=7640; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo>; module=115, inline_record=9092; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo>; module=115, inline_record=10544; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::bind_resources::impl$0::bind_color_library<country_core::render::pass_builder::compute_pass_builder::ComputePass>; module=115, inline_record=15828; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=120, inline_record=1568; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=120, inline_record=6960; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>; module=219, inline_record=1548; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$79::read_from
// 信息 ID: 9f118aa6853bcb3ce9fa4f5a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,speedy::error::Error> >,core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,std::hash::random::RandomState> >,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,hashbrown::map::impl$94::extend::closure_env$0<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; module=1321, inline_record=81072; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,speedy::error::Error> >,core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > >,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> >,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; module=1321, inline_record=86192; 不是独立调用边。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=1331, inline_record=14840; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$84::read_from
// 信息 ID: 64e2659f368c146d3f2199d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=1331, inline_record=12756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$89::read_from
// 信息 ID: b1dd378180cdf87f30442275
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=1331, inline_record=11932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$94::read_from
// 信息 ID: 643024fbc0a0d9b400e1d151
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=1331, inline_record=11792; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$94::read_from::closure$0
// 信息 ID: 5bbb88a2462f8c3e2d2b8fed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "shader_pipeline::reflection::impl$94::read_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=1331, inline_record=13472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$94::read_from::closure$1
// 信息 ID: 3a8b581d6291d73167e1817d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "shader_pipeline::reflection::impl$94::read_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=1331, inline_record=17232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$94::read_from::closure$1::closure$0
// 信息 ID: ca905f83b8882639c1b0c39d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "shader_pipeline::reflection::impl$94::read_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=1331, inline_record=17260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::len_without_suffix
// 信息 ID: a8033daddcd938281994c772
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection::BindingNameHash::new; module=608, inline_record=528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

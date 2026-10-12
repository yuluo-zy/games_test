// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/asset/src/foliage.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 94b1d920f8684ed508d99ee011ae69bc58268426bf8315013450929e8183edb1
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: foliage.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 502, 728, 1659, 1666, 1670, 2142, 2150, 3610, 3623, 3625, 4596, 4603
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use bytecheck; // namespace candidate
//   package identities: [{"name":"bytecheck","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.2"}]
// use core; // namespace candidate
//   package identities: []
// use rkyv; // namespace candidate
//   package identities: [{"name":"rkyv","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.16"}]
// use std; // namespace candidate
//   package identities: []
// use turbosloth; // namespace candidate
//   package identities: [{"name":"turbosloth","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset::foliage::grass2_package_dir
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":728,"record_offset":400,"rva":14442192,"va":5383151312}
// 编译实例 RVAs: 0xdc5ed0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1504894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383151312,"raw_hex":"e87ef61600","size":5,"soft_fail":false,"target_va":5384656240,"va":5383151341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":728,"record_offset":520,"parent_record_offset":400,"ancestor_record_offset":400,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00728.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::foliage::grass2_runtime_config_name
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":728,"record_offset":652,"rva":14442336,"va":5383151456}
// 编译实例 RVAs: 0xdc5f60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1643351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383151456,"raw_hex":"e857131900","size":5,"soft_fail":false,"target_va":5384794864,"va":5383151508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":728,"record_offset":780,"parent_record_offset":652,"ancestor_record_offset":652,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00728.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":728,"record_offset":828,"parent_record_offset":804,"ancestor_record_offset":652,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00728.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":728,"record_offset":804,"parent_record_offset":780,"ancestor_record_offset":652,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00728.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::foliage::impl$7::check_bytes
// 记录形式: inlinee_declaration; instances=2
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F8A
// 首条位置: {"module":1670,"inlinee":28554}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::foliage::impl$7::check_bytes<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":502,"record_offset":1108,"rva":9434992,"va":5378144112}
// 编译实例 RVAs: 0x8ff770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1409023a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378144112,"raw_hex":"e8a12b0000","size":5,"soft_fail":false,"target_va":5378155424,"va":5378144250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a0a5f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1093173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378144112,"raw_hex":"e835ae1000","size":5,"soft_fail":false,"target_va":5379237360,"va":5378144182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rkyv::vec::impl$19::check_bytes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":502,"record_offset":1400,"parent_record_offset":1108,"ancestor_record_offset":1108,"inlinee":28271,"source_line":28,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] bytecheck::impl$14::check_bytes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bytecheck","version":"0.8.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytecheck-0.8.2/src/lib.rs
//     evidence={"module":502,"record_offset":2024,"parent_record_offset":2000,"ancestor_record_offset":1108,"inlinee":28505,"source_line":704,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":502,"record_offset":1700,"parent_record_offset":1676,"ancestor_record_offset":1108,"inlinee":21027,"source_line":559,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":502,"record_offset":1724,"parent_record_offset":1700,"ancestor_record_offset":1108,"inlinee":5306,"source_line":564,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":502,"record_offset":2112,"parent_record_offset":2084,"ancestor_record_offset":1108,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":502,"record_offset":2084,"parent_record_offset":2056,"ancestor_record_offset":1108,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":502,"record_offset":2056,"parent_record_offset":2024,"ancestor_record_offset":1108,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":502,"record_offset":2292,"parent_record_offset":2268,"ancestor_record_offset":1108,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::checked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":502,"record_offset":2268,"parent_record_offset":2232,"ancestor_record_offset":1108,"inlinee":28509,"source_line":1645,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":502,"record_offset":1548,"parent_record_offset":1524,"ancestor_record_offset":1108,"inlinee":4729,"source_line":1258,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":502,"record_offset":1608,"parent_record_offset":1500,"ancestor_record_offset":1108,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":502,"record_offset":1572,"parent_record_offset":1548,"ancestor_record_offset":1108,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":502,"record_offset":1852,"parent_record_offset":1760,"ancestor_record_offset":1108,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::core::impl$1::layout_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/core/mod.rs
//     evidence={"module":502,"record_offset":1676,"parent_record_offset":1648,"ancestor_record_offset":1108,"inlinee":28492,"source_line":39,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_wrapping
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":502,"record_offset":1476,"parent_record_offset":1452,"ancestor_record_offset":1108,"inlinee":28277,"source_line":325,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_wrapping_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":502,"record_offset":1500,"parent_record_offset":1476,"ancestor_record_offset":1108,"inlinee":28278,"source_line":236,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::offset_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":502,"record_offset":1524,"parent_record_offset":1500,"ancestor_record_offset":1108,"inlinee":28279,"source_line":205,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::as_ptr_wrapping
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":502,"record_offset":1452,"parent_record_offset":1424,"ancestor_record_offset":1108,"inlinee":28275,"source_line":606,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::impl$0::check_subtree_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/mod.rs
//     evidence={"module":502,"record_offset":1796,"parent_record_offset":1760,"ancestor_record_offset":1108,"inlinee":28495,"source_line":62,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::impl$0::pop_subtree_range
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/mod.rs
//     evidence={"module":502,"record_offset":2184,"parent_record_offset":1760,"ancestor_record_offset":1108,"inlinee":28506,"source_line":80,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::impl$0::push_subtree_range
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/mod.rs
//     evidence={"module":502,"record_offset":1880,"parent_record_offset":1760,"ancestor_record_offset":1108,"inlinee":28498,"source_line":70,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::impl$1::in_subtree
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/mod.rs
//     evidence={"module":502,"record_offset":1648,"parent_record_offset":1424,"ancestor_record_offset":1108,"inlinee":28490,"source_line":138,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::impl$1::in_subtree_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/mod.rs
//     evidence={"module":502,"record_offset":1760,"parent_record_offset":1648,"ancestor_record_offset":1108,"inlinee":28493,"source_line":114,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::validator::impl$12::pop_subtree_range
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/validator.rs
//     evidence={"module":502,"record_offset":2232,"parent_record_offset":2208,"ancestor_record_offset":1108,"inlinee":28508,"source_line":176,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::validator::impl$12::push_subtree_range
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/validator.rs
//     evidence={"module":502,"record_offset":1928,"parent_record_offset":1904,"ancestor_record_offset":1108,"inlinee":28501,"source_line":158,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::impl$1::check_subtree_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/mod.rs
//     evidence={"module":502,"record_offset":1820,"parent_record_offset":1796,"ancestor_record_offset":1108,"inlinee":28497,"source_line":32,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::impl$1::pop_subtree_range
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/mod.rs
//     evidence={"module":502,"record_offset":2208,"parent_record_offset":2184,"ancestor_record_offset":1108,"inlinee":28507,"source_line":50,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::impl$1::push_subtree_range
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/mod.rs
//     evidence={"module":502,"record_offset":1904,"parent_record_offset":1880,"ancestor_record_offset":1108,"inlinee":28499,"source_line":40,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::verify::impl$0::verify
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":502,"record_offset":1424,"parent_record_offset":1400,"ancestor_record_offset":1108,"inlinee":28273,"source_line":304,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::verify::impl$0::verify::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":502,"record_offset":2000,"parent_record_offset":1760,"ancestor_record_offset":1108,"inlinee":28503,"source_line":310,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::foliage::impl$9::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6FAD
// 首条位置: {"module":1666,"inlinee":28589}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::foliage::impl$47::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/foliage.rs
//     evidence={"module":1666,"record_offset":100412,"parent_record_offset":100008,"ancestor_record_offset":100008,"inlinee":28525,"source_line":173,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_direct] rkyv::impls::alloc::vec::impl$2::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/alloc/vec.rs
//     evidence={"module":1666,"record_offset":100036,"parent_record_offset":100008,"ancestor_record_offset":100008,"inlinee":28517,"source_line":45,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1666,"record_offset":100132,"parent_record_offset":100036,"ancestor_record_offset":100008,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1666,"record_offset":100328,"parent_record_offset":100296,"ancestor_record_offset":100008,"inlinee":4729,"source_line":1258,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1666,"record_offset":100352,"parent_record_offset":100328,"ancestor_record_offset":100008,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1666,"record_offset":100248,"parent_record_offset":100224,"ancestor_record_offset":100008,"inlinee":28522,"source_line":290,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1666,"record_offset":100272,"parent_record_offset":100248,"ancestor_record_offset":100008,"inlinee":28523,"source_line":219,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::offset_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1666,"record_offset":100296,"parent_record_offset":100272,"ancestor_record_offset":100008,"inlinee":28279,"source_line":205,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1666,"record_offset":100224,"parent_record_offset":100200,"ancestor_record_offset":100008,"inlinee":28521,"source_line":575,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1666,"record_offset":100200,"parent_record_offset":100164,"ancestor_record_offset":100008,"inlinee":28520,"source_line":40,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1666,"record_offset":100076,"parent_record_offset":100036,"ancestor_record_offset":100008,"inlinee":28518,"source_line":55,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1666,"record_offset":100100,"parent_record_offset":100076,"ancestor_record_offset":100008,"inlinee":28519,"source_line":45,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::asset::impl$79::deserialize
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":1666,"record_offset":100436,"parent_record_offset":100412,"ancestor_record_offset":100008,"inlinee":28527,"source_line":213,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::impl$26::deserialize
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":1666,"record_offset":100460,"parent_record_offset":100436,"ancestor_record_offset":100008,"inlinee":28529,"source_line":55,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::foliage::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=13
// 源码行: 26, 154, 174, 193, 245
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18046
// 首条位置: {"module":2150,"inlinee":98374}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::foliage::impl$14::check_bytes<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":502,"record_offset":39580,"rva":9446304,"va":5378155424}
// 编译实例 RVAs: 0x9023a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ff860
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378155424,"raw_hex":"e821d4ffff","size":5,"soft_fail":false,"target_va":5378144352,"va":5378155578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a0a5f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1081847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378155424,"raw_hex":"e8f7811000","size":5,"soft_fail":false,"target_va":5379237360,"va":5378155508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c5a920
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3507558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378155424,"raw_hex":"e866853500","size":5,"soft_fail":false,"target_va":5381663008,"va":5378155445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":502,"record_offset":39876,"parent_record_offset":39580,"ancestor_record_offset":39580,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_direct] rkyv::vec::impl$19::check_bytes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":502,"record_offset":39904,"parent_record_offset":39580,"ancestor_record_offset":39580,"inlinee":28271,"source_line":28,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] bytecheck::impl$14::check_bytes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bytecheck","version":"0.8.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytecheck-0.8.2/src/lib.rs
//     evidence={"module":502,"record_offset":40528,"parent_record_offset":40504,"ancestor_record_offset":39580,"inlinee":28505,"source_line":704,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":502,"record_offset":40204,"parent_record_offset":40180,"ancestor_record_offset":39580,"inlinee":21027,"source_line":559,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":502,"record_offset":40228,"parent_record_offset":40204,"ancestor_record_offset":39580,"inlinee":5306,"source_line":564,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":502,"record_offset":40616,"parent_record_offset":40588,"ancestor_record_offset":39580,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":502,"record_offset":40588,"parent_record_offset":40560,"ancestor_record_offset":39580,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":502,"record_offset":40560,"parent_record_offset":40528,"ancestor_record_offset":39580,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":502,"record_offset":40796,"parent_record_offset":40772,"ancestor_record_offset":39580,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::checked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":502,"record_offset":40772,"parent_record_offset":40736,"ancestor_record_offset":39580,"inlinee":28509,"source_line":1645,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":502,"record_offset":40052,"parent_record_offset":40028,"ancestor_record_offset":39580,"inlinee":4729,"source_line":1258,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":502,"record_offset":40112,"parent_record_offset":40004,"ancestor_record_offset":39580,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":502,"record_offset":40076,"parent_record_offset":40052,"ancestor_record_offset":39580,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":502,"record_offset":40356,"parent_record_offset":40264,"ancestor_record_offset":39580,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::core::impl$1::layout_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/core/mod.rs
//     evidence={"module":502,"record_offset":40180,"parent_record_offset":40152,"ancestor_record_offset":39580,"inlinee":28492,"source_line":39,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_wrapping
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":502,"record_offset":39980,"parent_record_offset":39956,"ancestor_record_offset":39580,"inlinee":28277,"source_line":325,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_wrapping_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":502,"record_offset":40004,"parent_record_offset":39980,"ancestor_record_offset":39580,"inlinee":28278,"source_line":236,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::offset_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":502,"record_offset":40028,"parent_record_offset":40004,"ancestor_record_offset":39580,"inlinee":28279,"source_line":205,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::as_ptr_wrapping
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":502,"record_offset":39956,"parent_record_offset":39928,"ancestor_record_offset":39580,"inlinee":28275,"source_line":606,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::impl$0::check_subtree_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/mod.rs
//     evidence={"module":502,"record_offset":40300,"parent_record_offset":40264,"ancestor_record_offset":39580,"inlinee":28495,"source_line":62,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::impl$0::pop_subtree_range
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/mod.rs
//     evidence={"module":502,"record_offset":40688,"parent_record_offset":40264,"ancestor_record_offset":39580,"inlinee":28506,"source_line":80,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::impl$0::push_subtree_range
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/mod.rs
//     evidence={"module":502,"record_offset":40384,"parent_record_offset":40264,"ancestor_record_offset":39580,"inlinee":28498,"source_line":70,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::impl$1::in_subtree
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/mod.rs
//     evidence={"module":502,"record_offset":40152,"parent_record_offset":39928,"ancestor_record_offset":39580,"inlinee":28490,"source_line":138,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::impl$1::in_subtree_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/mod.rs
//     evidence={"module":502,"record_offset":40264,"parent_record_offset":40152,"ancestor_record_offset":39580,"inlinee":28493,"source_line":114,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::validator::impl$12::pop_subtree_range
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/validator.rs
//     evidence={"module":502,"record_offset":40736,"parent_record_offset":40712,"ancestor_record_offset":39580,"inlinee":28508,"source_line":176,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::validator::impl$12::push_subtree_range
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/validator.rs
//     evidence={"module":502,"record_offset":40432,"parent_record_offset":40408,"ancestor_record_offset":39580,"inlinee":28501,"source_line":158,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::impl$1::check_subtree_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/mod.rs
//     evidence={"module":502,"record_offset":40324,"parent_record_offset":40300,"ancestor_record_offset":39580,"inlinee":28497,"source_line":32,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::impl$1::pop_subtree_range
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/mod.rs
//     evidence={"module":502,"record_offset":40712,"parent_record_offset":40688,"ancestor_record_offset":39580,"inlinee":28507,"source_line":50,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::impl$1::push_subtree_range
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/mod.rs
//     evidence={"module":502,"record_offset":40408,"parent_record_offset":40384,"ancestor_record_offset":39580,"inlinee":28499,"source_line":40,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::verify::impl$0::verify
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":502,"record_offset":39928,"parent_record_offset":39904,"ancestor_record_offset":39580,"inlinee":28273,"source_line":304,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::verify::impl$0::verify::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":502,"record_offset":40504,"parent_record_offset":40264,"ancestor_record_offset":39580,"inlinee":28503,"source_line":310,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::foliage::impl$16::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F7C
// 首条位置: {"module":502,"inlinee":28540}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":502,"record_offset":24416,"parent_record_offset":23884,"ancestor_record_offset":23884,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":502,"record_offset":23916,"parent_record_offset":23884,"ancestor_record_offset":23884,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_direct] rkyv::impls::alloc::vec::impl$2::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/alloc/vec.rs
//     evidence={"module":502,"record_offset":23948,"parent_record_offset":23884,"ancestor_record_offset":23884,"inlinee":28517,"source_line":45,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":502,"record_offset":24592,"parent_record_offset":24568,"ancestor_record_offset":23884,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":502,"record_offset":24616,"parent_record_offset":24592,"ancestor_record_offset":23884,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":502,"record_offset":24136,"parent_record_offset":23948,"ancestor_record_offset":23884,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":502,"record_offset":24640,"parent_record_offset":24616,"ancestor_record_offset":23884,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":502,"record_offset":24568,"parent_record_offset":24512,"ancestor_record_offset":23884,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":502,"record_offset":24540,"parent_record_offset":24512,"ancestor_record_offset":23884,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":502,"record_offset":24512,"parent_record_offset":24488,"ancestor_record_offset":23884,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":502,"record_offset":24488,"parent_record_offset":24464,"ancestor_record_offset":23884,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":502,"record_offset":24076,"parent_record_offset":24052,"ancestor_record_offset":23884,"inlinee":21027,"source_line":559,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":502,"record_offset":24100,"parent_record_offset":24076,"ancestor_record_offset":23884,"inlinee":5306,"source_line":564,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":502,"record_offset":24440,"parent_record_offset":24416,"ancestor_record_offset":23884,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":502,"record_offset":24332,"parent_record_offset":24300,"ancestor_record_offset":23884,"inlinee":4729,"source_line":1258,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":502,"record_offset":24356,"parent_record_offset":24332,"ancestor_record_offset":23884,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::core::impl$1::layout_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/core/mod.rs
//     evidence={"module":502,"record_offset":24052,"parent_record_offset":23948,"ancestor_record_offset":23884,"inlinee":28492,"source_line":39,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":502,"record_offset":24252,"parent_record_offset":24228,"ancestor_record_offset":23884,"inlinee":28522,"source_line":290,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":502,"record_offset":24276,"parent_record_offset":24252,"ancestor_record_offset":23884,"inlinee":28523,"source_line":219,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::offset_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":502,"record_offset":24300,"parent_record_offset":24276,"ancestor_record_offset":23884,"inlinee":28279,"source_line":205,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":502,"record_offset":24228,"parent_record_offset":24204,"ancestor_record_offset":23884,"inlinee":28521,"source_line":575,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":502,"record_offset":24204,"parent_record_offset":24168,"ancestor_record_offset":23884,"inlinee":28520,"source_line":40,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":502,"record_offset":23996,"parent_record_offset":23948,"ancestor_record_offset":23884,"inlinee":28518,"source_line":55,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":502,"record_offset":24020,"parent_record_offset":23996,"ancestor_record_offset":23884,"inlinee":28519,"source_line":45,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::foliage::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=6
// 源码行: 44, 81, 122, 154, 174, 226
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3610,"record_offset":18192,"rva":41012480,"va":5409721600}
// 编译实例 RVAs: 0x271cd00, 0x271ce30, 0x271d000, 0x271d130, 0x271d260, 0x271d390
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$::end
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-26494050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409721600,"raw_hex":"e89ebb6bfe","size":5,"soft_fail":false,"target_va":5383227792,"va":5409721837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14271c990
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409721600,"raw_hex":"e8d4fbffff","size":5,"soft_fail":false,"target_va":5409720720,"va":5409721783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272bb40
//     owner=unresolved_namespace; count=11
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t60557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409721904,"raw_hex":"e88dec0000","size":5,"soft_fail":false,"target_va":5409782592,"va":5409722030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272be60
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t60933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409722368,"raw_hex":"e805ee0000","size":5,"soft_fail":false,"target_va":5409783392,"va":5409722454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272c0d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t62325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409721600,"raw_hex":"e875f30000","size":5,"soft_fail":false,"target_va":5409784016,"va":5409721686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272c360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t61098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409723280,"raw_hex":"e8aaee0000","size":5,"soft_fail":false,"target_va":5409784672,"va":5409723569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272c5c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t61666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409723280,"raw_hex":"e8e2f00000","size":5,"soft_fail":false,"target_va":5409785280,"va":5409723609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272c820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t63662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409721904,"raw_hex":"e8aef80000","size":5,"soft_fail":false,"target_va":5409785888,"va":5409722221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272cce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t65338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409721600,"raw_hex":"e83aff0000","size":5,"soft_fail":false,"target_va":5409787104,"va":5409721761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272cf40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t64218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409723280,"raw_hex":"e8dafa0000","size":5,"soft_fail":false,"target_va":5409787712,"va":5409723489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272d1a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t64909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409723280,"raw_hex":"e88dfd0000","size":5,"soft_fail":false,"target_va":5409788320,"va":5409723406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272d410
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t65410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409723280,"raw_hex":"e882ff0000","size":5,"soft_fail":false,"target_va":5409788944,"va":5409723529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272d8d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t66464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409723280,"raw_hex":"e8a0030100","size":5,"soft_fail":false,"target_va":5409790160,"va":5409723691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272dda0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t67735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409723280,"raw_hex":"e897080100","size":5,"soft_fail":false,"target_va":5409791392,"va":5409723652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272e010
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t68239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409723280,"raw_hex":"e88f0a0100","size":5,"soft_fail":false,"target_va":5409792016,"va":5409723772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272e520
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t71317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409721904,"raw_hex":"e895160100","size":5,"soft_fail":false,"target_va":5409793312,"va":5409721990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3610,"record_offset":18348,"parent_record_offset":18192,"ancestor_record_offset":18192,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/03610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::foliage::impl$21::check_bytes<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":502,"record_offset":2496,"rva":9435232,"va":5378144352}
// 编译实例 RVAs: 0x8ff860
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a0a5f0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1092914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378144352,"raw_hex":"e832ad1000","size":5,"soft_fail":false,"target_va":5379237360,"va":5378144441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c5a920
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3518634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378144352,"raw_hex":"e8aab03500","size":5,"soft_fail":false,"target_va":5381663008,"va":5378144369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417b29c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15413245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378144352,"raw_hex":"e8fd2feb00","size":5,"soft_fail":false,"target_va":5393557952,"va":5378144702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1417b2970
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: foliage.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t15413131","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378144352,"raw_hex":"e98b2feb00","size":5,"soft_fail":false,"target_va":5393557872,"va":5378144736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":502,"record_offset":2792,"parent_record_offset":2496,"ancestor_record_offset":2496,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_direct] rkyv::vec::impl$19::check_bytes
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":502,"record_offset":2820,"parent_record_offset":2496,"ancestor_record_offset":2496,"inlinee":28271,"source_line":28,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":502,"record_offset":3116,"parent_record_offset":3092,"ancestor_record_offset":2496,"inlinee":21027,"source_line":559,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":502,"record_offset":3140,"parent_record_offset":3116,"ancestor_record_offset":2496,"inlinee":5306,"source_line":564,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":502,"record_offset":3292,"parent_record_offset":3176,"ancestor_record_offset":2496,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":502,"record_offset":2964,"parent_record_offset":2940,"ancestor_record_offset":2496,"inlinee":4729,"source_line":1258,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":502,"record_offset":3024,"parent_record_offset":2916,"ancestor_record_offset":2496,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":502,"record_offset":2988,"parent_record_offset":2964,"ancestor_record_offset":2496,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":502,"record_offset":3264,"parent_record_offset":3176,"ancestor_record_offset":2496,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::core::impl$1::layout_raw
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/core/mod.rs
//     evidence={"module":502,"record_offset":3092,"parent_record_offset":3064,"ancestor_record_offset":2496,"inlinee":28492,"source_line":39,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_wrapping
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":502,"record_offset":2892,"parent_record_offset":2868,"ancestor_record_offset":2496,"inlinee":28277,"source_line":325,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_wrapping_raw
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":502,"record_offset":2916,"parent_record_offset":2892,"ancestor_record_offset":2496,"inlinee":28278,"source_line":236,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::offset_raw
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":502,"record_offset":2940,"parent_record_offset":2916,"ancestor_record_offset":2496,"inlinee":28279,"source_line":205,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::as_ptr_wrapping
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":502,"record_offset":2868,"parent_record_offset":2840,"ancestor_record_offset":2496,"inlinee":28275,"source_line":606,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::impl$0::check_subtree_ptr
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/mod.rs
//     evidence={"module":502,"record_offset":3208,"parent_record_offset":3176,"ancestor_record_offset":2496,"inlinee":28495,"source_line":62,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::impl$0::pop_subtree_range
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/mod.rs
//     evidence={"module":502,"record_offset":3320,"parent_record_offset":3176,"ancestor_record_offset":2496,"inlinee":28506,"source_line":80,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::impl$1::in_subtree
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/mod.rs
//     evidence={"module":502,"record_offset":3064,"parent_record_offset":2840,"ancestor_record_offset":2496,"inlinee":28490,"source_line":138,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::impl$1::in_subtree_raw
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/mod.rs
//     evidence={"module":502,"record_offset":3176,"parent_record_offset":3064,"ancestor_record_offset":2496,"inlinee":28493,"source_line":114,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::archive::validator::impl$12::pop_subtree_range
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/archive/validator.rs
//     evidence={"module":502,"record_offset":3368,"parent_record_offset":3344,"ancestor_record_offset":2496,"inlinee":28508,"source_line":176,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::impl$1::check_subtree_ptr
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/mod.rs
//     evidence={"module":502,"record_offset":3232,"parent_record_offset":3208,"ancestor_record_offset":2496,"inlinee":28497,"source_line":32,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::validation::impl$1::pop_subtree_range
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/validation/mod.rs
//     evidence={"module":502,"record_offset":3344,"parent_record_offset":3320,"ancestor_record_offset":2496,"inlinee":28507,"source_line":50,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::verify::impl$0::verify
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":502,"record_offset":2840,"parent_record_offset":2820,"ancestor_record_offset":2496,"inlinee":28273,"source_line":304,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::foliage::impl$23::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F7E
// 首条位置: {"module":502,"inlinee":28542}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::foliage::impl$39::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/foliage.rs
//     evidence={"module":502,"record_offset":28300,"parent_record_offset":25568,"ancestor_record_offset":25568,"inlinee":28546,"source_line":153,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":502,"record_offset":29656,"parent_record_offset":25568,"ancestor_record_offset":25568,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":502,"record_offset":25596,"parent_record_offset":25568,"ancestor_record_offset":25568,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_direct] rkyv::impls::alloc::vec::impl$2::deserialize
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/alloc/vec.rs
//     evidence={"module":502,"record_offset":25684,"parent_record_offset":25568,"ancestor_record_offset":25568,"inlinee":28517,"source_line":45,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::asset::impl$79::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":502,"record_offset":25628,"parent_record_offset":25568,"ancestor_record_offset":25568,"inlinee":28527,"source_line":213,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":502,"record_offset":29808,"parent_record_offset":29784,"ancestor_record_offset":25568,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":502,"record_offset":29832,"parent_record_offset":29808,"ancestor_record_offset":25568,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":502,"record_offset":25912,"parent_record_offset":25684,"ancestor_record_offset":25568,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":502,"record_offset":29856,"parent_record_offset":29832,"ancestor_record_offset":25568,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":502,"record_offset":29784,"parent_record_offset":29728,"ancestor_record_offset":25568,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":502,"record_offset":29756,"parent_record_offset":29728,"ancestor_record_offset":25568,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":502,"record_offset":29728,"parent_record_offset":29704,"ancestor_record_offset":25568,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":502,"record_offset":29704,"parent_record_offset":29680,"ancestor_record_offset":25568,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] asset::foliage::impl$30::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/foliage.rs
//     evidence={"module":502,"record_offset":28916,"parent_record_offset":28796,"ancestor_record_offset":25568,"inlinee":28548,"source_line":121,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] asset::foliage::impl$77::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/foliage.rs
//     evidence={"module":502,"record_offset":27412,"parent_record_offset":27308,"ancestor_record_offset":25568,"inlinee":28544,"source_line":225,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":502,"record_offset":25852,"parent_record_offset":25828,"ancestor_record_offset":25568,"inlinee":21027,"source_line":559,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":502,"record_offset":25876,"parent_record_offset":25852,"ancestor_record_offset":25568,"inlinee":5306,"source_line":564,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":502,"record_offset":27348,"parent_record_offset":27308,"ancestor_record_offset":25568,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=54
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":502,"record_offset":29680,"parent_record_offset":29656,"ancestor_record_offset":25568,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":502,"record_offset":27520,"parent_record_offset":27308,"ancestor_record_offset":25568,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":502,"record_offset":26096,"parent_record_offset":26064,"ancestor_record_offset":25568,"inlinee":4729,"source_line":1258,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":502,"record_offset":26200,"parent_record_offset":26176,"ancestor_record_offset":25568,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":502,"record_offset":28164,"parent_record_offset":28120,"ancestor_record_offset":25568,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":502,"record_offset":28192,"parent_record_offset":28120,"ancestor_record_offset":25568,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":502,"record_offset":26120,"parent_record_offset":26096,"ancestor_record_offset":25568,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":502,"record_offset":26224,"parent_record_offset":26200,"ancestor_record_offset":25568,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":502,"record_offset":27376,"parent_record_offset":27348,"ancestor_record_offset":25568,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::core::impl$15::deserialize_unsized
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/core/mod.rs
//     evidence={"module":502,"record_offset":26176,"parent_record_offset":25684,"ancestor_record_offset":25568,"inlinee":28442,"source_line":335,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::core::impl$1::layout_raw
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/core/mod.rs
//     evidence={"module":502,"record_offset":25828,"parent_record_offset":25684,"ancestor_record_offset":25568,"inlinee":28492,"source_line":39,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":502,"record_offset":26016,"parent_record_offset":25992,"ancestor_record_offset":25568,"inlinee":28522,"source_line":290,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_raw
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":502,"record_offset":26040,"parent_record_offset":26016,"ancestor_record_offset":25568,"inlinee":28523,"source_line":219,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::offset_raw
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":502,"record_offset":26064,"parent_record_offset":26040,"ancestor_record_offset":25568,"inlinee":28279,"source_line":205,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::as_ptr
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":502,"record_offset":25992,"parent_record_offset":25968,"ancestor_record_offset":25568,"inlinee":28521,"source_line":575,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_ptr
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":502,"record_offset":25968,"parent_record_offset":25944,"ancestor_record_offset":25568,"inlinee":28520,"source_line":40,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_slice
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":502,"record_offset":25764,"parent_record_offset":25684,"ancestor_record_offset":25568,"inlinee":28518,"source_line":55,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::len
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":502,"record_offset":25792,"parent_record_offset":25764,"ancestor_record_offset":25568,"inlinee":28519,"source_line":45,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::asset::impl$79::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":502,"record_offset":28948,"parent_record_offset":28916,"ancestor_record_offset":25568,"inlinee":28527,"source_line":213,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::impl$26::deserialize
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":502,"record_offset":25652,"parent_record_offset":25628,"ancestor_record_offset":25568,"inlinee":28529,"source_line":55,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::foliage::impl$30::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F84
// 首条位置: {"module":502,"inlinee":28548}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] turbosloth::asset::impl$79::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":502,"record_offset":28948,"parent_record_offset":28916,"ancestor_record_offset":28916,"inlinee":28527,"source_line":213,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::impl$26::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":502,"record_offset":28980,"parent_record_offset":28948,"ancestor_record_offset":28916,"inlinee":28529,"source_line":55,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::foliage::impl$31::visit_asset_dependencies
// 记录形式: inlinee_declaration; instances=1
// 源码行: 123
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBA99
// 首条位置: {"module":728,"inlinee":47769}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::foliage::impl$35::resolve
// 记录形式: inlinee_declaration; instances=1
// 源码行: 151
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBA9D
// 首条位置: {"module":728,"inlinee":47773}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::foliage::impl$39::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 153
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F82
// 首条位置: {"module":502,"inlinee":28546}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::foliage::impl$40::visit_asset_dependencies
// 记录形式: inlinee_declaration; instances=1
// 源码行: 155
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBA97
// 首条位置: {"module":728,"inlinee":47767}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::foliage::impl$43::resolve
// 记录形式: inlinee_declaration; instances=1
// 源码行: 171
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBA9B
// 首条位置: {"module":728,"inlinee":47771}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::foliage::impl$47::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 173
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F6D
// 首条位置: {"module":502,"inlinee":28525}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] turbosloth::asset::impl$79::deserialize
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":1666,"record_offset":100436,"parent_record_offset":100412,"ancestor_record_offset":100412,"inlinee":28527,"source_line":213,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::impl$26::deserialize
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":1666,"record_offset":100460,"parent_record_offset":100436,"ancestor_record_offset":100412,"inlinee":28529,"source_line":55,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::foliage::impl$48::visit_asset_dependencies
// 记录形式: inlinee_declaration; instances=1
// 源码行: 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBA8F
// 首条位置: {"module":728,"inlinee":47759}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::foliage::impl$58::visit_asset_dependencies
// 记录形式: inlinee_declaration; instances=1
// 源码行: 194
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBA91
// 首条位置: {"module":728,"inlinee":47761}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::foliage::impl$77::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 225
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F80
// 首条位置: {"module":502,"inlinee":28544}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::foliage::impl$78::visit_asset_dependencies
// 记录形式: inlinee_declaration; instances=1
// 源码行: 227
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBA93
// 首条位置: {"module":728,"inlinee":47763}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::foliage::impl$88::visit_asset_dependencies
// 记录形式: inlinee_declaration; instances=1
// 源码行: 246
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBA95
// 首条位置: {"module":728,"inlinee":47765}
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
// 完整结构及原始类型记录: foliage.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::foliage::ArchivedGrass2Lod0PartAsset
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "rkyv::vec::impl$19::check_bytes<asset::foliage::ArchivedGrass2Lod0PartAsset,rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> >", "constructor": "rkyv::vec::impl$19::check_bytes", "arguments": ["asset::foliage::ArchivedGrass2Lod0PartAsset", "rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::foliage::ArchivedGrass2RuntimeExemplarAsset
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "rkyv::impls::core::impl$15::deserialize_unsized<asset::foliage::ArchivedGrass2RuntimeExemplarAsset,asset::foliage::Grass2RuntimeExemplarAsset,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >", "constructor": "rkyv::impls::core::impl$15::deserialize_unsized", "arguments": ["asset::foliage::ArchivedGrass2RuntimeExemplarAsset", "asset::foliage::Grass2RuntimeExemplarAsset", "rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::foliage::ArchivedGrass2SlotAsset
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "rkyv::impls::core::impl$15::deserialize_unsized<asset::foliage::ArchivedGrass2SlotAsset,asset::foliage::Grass2SlotAsset,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >", "constructor": "rkyv::impls::core::impl$15::deserialize_unsized", "arguments": ["asset::foliage::ArchivedGrass2SlotAsset", "asset::foliage::Grass2SlotAsset", "rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::foliage::Grass2CardPointPieceGroup
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<asset::foliage::Grass2CardPointPieceGroup,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["asset::foliage::Grass2CardPointPieceGroup", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::vec::Vec<asset::foliage::Grass2CardPointPieceGroup,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::vec::Vec<asset::foliage::Grass2CardPointPieceGroup,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::Grass2CardPointPieceGroup,alloc::alloc::Global> > >", "constructor": "serde_core::ser::Serializer::collect_seq", "arguments": ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<asset::foliage::Grass2CardPointPieceGroup,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<alloc::vec::Vec<asset::foliage::Grass2CardPointPieceGroup,alloc::alloc::Global> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["alloc::vec::Vec<asset::foliage::Grass2CardPointPieceGroup,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::foliage::Grass2CardPointPositionRange
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<asset::foliage::Grass2CardPointPositionRange,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["asset::foliage::Grass2CardPointPositionRange", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::vec::Vec<asset::foliage::Grass2CardPointPositionRange,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::vec::Vec<asset::foliage::Grass2CardPointPositionRange,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::Grass2CardPointPositionRange,alloc::alloc::Global> > >", "constructor": "serde_core::ser::Serializer::collect_seq", "arguments": ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<asset::foliage::Grass2CardPointPositionRange,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<alloc::vec::Vec<asset::foliage::Grass2CardPointPositionRange,alloc::alloc::Global> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["alloc::vec::Vec<asset::foliage::Grass2CardPointPositionRange,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::foliage::Grass2CardTextureAssets
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<asset::foliage::Grass2CardTextureAssets>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["asset::foliage::Grass2CardTextureAssets"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::foliage::Grass2ExemplarCullBounds
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<asset::foliage::Grass2ExemplarCullBounds>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["asset::foliage::Grass2ExemplarCullBounds"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::foliage::Grass2Lod0PartAsset
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<asset::foliage::Grass2Lod0PartAsset,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["asset::foliage::Grass2Lod0PartAsset", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::vec::Vec<asset::foliage::Grass2Lod0PartAsset,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::vec::Vec<asset::foliage::Grass2Lod0PartAsset,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::Grass2Lod0PartAsset,alloc::alloc::Global> > >", "constructor": "serde_core::ser::Serializer::collect_seq", "arguments": ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<asset::foliage::Grass2Lod0PartAsset,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<alloc::vec::Vec<asset::foliage::Grass2Lod0PartAsset,alloc::alloc::Global> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["alloc::vec::Vec<asset::foliage::Grass2Lod0PartAsset,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::foliage::Grass2RuntimeConfigAsset
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<asset::foliage::Grass2RuntimeConfigAsset>", "constructor": "core::ptr::drop_in_place", "arguments": ["asset::foliage::Grass2RuntimeConfigAsset"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::type_name<asset::foliage::Grass2RuntimeConfigAsset>", "constructor": "core::any::type_name", "arguments": ["asset::foliage::Grass2RuntimeConfigAsset"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rkyv::api::high::checked::from_bytes<asset::foliage::Grass2RuntimeConfigAsset,rancor::Error>", "constructor": "rkyv::api::high::checked::from_bytes", "arguments": ["asset::foliage::Grass2RuntimeConfigAsset", "rancor::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::store::read_packaged_asset<asset::foliage::Grass2RuntimeConfigAsset,ref$<std::path::PathBuf> >", "constructor": "turbosloth::store::read_packaged_asset", "arguments": ["asset::foliage::Grass2RuntimeConfigAsset", "ref$<std::path::PathBuf>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::export::to_json_value<asset::foliage::Grass2RuntimeConfigAsset>", "constructor": "turbosloth::export::to_json_value", "arguments": ["asset::foliage::Grass2RuntimeConfigAsset"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::foliage::Grass2RuntimeExemplarAsset
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 14
// [PDB 泛型关系，非字段关系] {"type": "rkyv::impls::core::impl$15::deserialize_unsized<asset::foliage::ArchivedGrass2RuntimeExemplarAsset,asset::foliage::Grass2RuntimeExemplarAsset,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >", "constructor": "rkyv::impls::core::impl$15::deserialize_unsized", "arguments": ["asset::foliage::ArchivedGrass2RuntimeExemplarAsset", "asset::foliage::Grass2RuntimeExemplarAsset", "rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<asset::foliage::Grass2RuntimeExemplarAsset>", "constructor": "core::ptr::drop_in_place", "arguments": ["asset::foliage::Grass2RuntimeExemplarAsset"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>", "constructor": "core::slice::iter::Iter", "arguments": ["asset::foliage::Grass2RuntimeExemplarAsset"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>", "country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>", "core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>", "country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>", "country_core::systems::grass2::impl$1::active_exemplars::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::next<alloc::vec::Vec<country_core::shader::grass2::inc::card_splat_common::Grass2CardPointPieceGroup,alloc::alloc::Global>,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_piece_groups::closure_env$0>", "constructor": "core::iter::adapters::map::impl$2::next", "arguments": ["alloc::vec::Vec<country_core::shader::grass2::inc::card_splat_common::Grass2CardPointPieceGroup,alloc::alloc::Global>", "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>", "country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_piece_groups::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u16,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_attrib_indices::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>", "ref$<alloc::vec::Vec<u16,alloc::alloc::Global> >", "country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_attrib_indices::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::Copied<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u16,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_attrib_indices::closure_env$0> >", "constructor": "core::iter::adapters::copied::Copied", "arguments": ["core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u16,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_attrib_indices::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<u16,core::iter::adapters::copied::Copied<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u16,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_attrib_indices::closure_env$0> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["u16", "core::iter::adapters::copied::Copied<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u16,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_attrib_indices::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u32,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_plant_offsets::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>", "ref$<alloc::vec::Vec<u32,alloc::alloc::Global> >", "country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_plant_offsets::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::Copied<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u32,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_plant_offsets::closure_env$0> >", "constructor": "core::iter::adapters::copied::Copied", "arguments": ["core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u32,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_plant_offsets::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::copied::Copied<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u32,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_plant_offsets::closure_env$0> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["u32", "core::iter::adapters::copied::Copied<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u32,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_plant_offsets::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>", "ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> >", "country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$0>,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$0>", "country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::shader::grass2::inc::card_splat_common::Grass2CardPointPosition,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$0>,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::shader::grass2::inc::card_splat_common::Grass2CardPointPosition", "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$0>,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::render_loop::compute_passes::grass2::load_grass2_exemplar_params::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>", "country_core::systems::render_loop::compute_passes::grass2::load_grass2_exemplar_params::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::shader::grass2::inc::card_splat_common::Grass2ExemplarParams,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::render_loop::compute_passes::grass2::load_grass2_exemplar_params::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::shader::grass2::inc::card_splat_common::Grass2ExemplarParams", "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::render_loop::compute_passes::grass2::load_grass2_exemplar_params::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_piece_groups::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>", "country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_piece_groups::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<country_core::shader::grass2::inc::card_splat_common::Grass2CardPointPieceGroup,alloc::alloc::Global>,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_piece_groups::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["alloc::vec::Vec<country_core::shader::grass2::inc::card_splat_common::Grass2CardPointPieceGroup,alloc::alloc::Global>", "core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_piece_groups::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::impl$1::next<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u16,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_attrib_indices::closure_env$0>,u16>", "constructor": "core::iter::adapters::copied::impl$1::next", "arguments": ["core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u16,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_attrib_indices::closure_env$0>", "u16"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::impl$1::next<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u32,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_plant_offsets::closure_env$0>,u32>", "constructor": "core::iter::adapters::copied::impl$1::next", "arguments": ["core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u32,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_plant_offsets::closure_env$0>", "u32"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<asset::foliage::Grass2RuntimeExemplarAsset,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["asset::foliage::Grass2RuntimeExemplarAsset", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::vec::Vec<asset::foliage::Grass2RuntimeExemplarAsset,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::vec::Vec<asset::foliage::Grass2RuntimeExemplarAsset,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::Grass2RuntimeExemplarAsset,alloc::alloc::Global> > >", "constructor": "serde_core::ser::Serializer::collect_seq", "arguments": ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<asset::foliage::Grass2RuntimeExemplarAsset,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<alloc::vec::Vec<asset::foliage::Grass2RuntimeExemplarAsset,alloc::alloc::Global> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["alloc::vec::Vec<asset::foliage::Grass2RuntimeExemplarAsset,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::foliage::Grass2SlotAsset
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 14
// [PDB 泛型关系，非字段关系] {"type": "rkyv::impls::core::impl$15::deserialize_unsized<asset::foliage::ArchivedGrass2SlotAsset,asset::foliage::Grass2SlotAsset,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >", "constructor": "rkyv::impls::core::impl$15::deserialize_unsized", "arguments": ["asset::foliage::ArchivedGrass2SlotAsset", "asset::foliage::Grass2SlotAsset", "rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<asset::foliage::Grass2SlotAsset>", "constructor": "core::ptr::drop_in_place", "arguments": ["asset::foliage::Grass2SlotAsset"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>", "constructor": "core::slice::iter::Iter", "arguments": ["asset::foliage::Grass2SlotAsset"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>", "core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>", "country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>", "country_core::systems::grass2::impl$1::active_exemplars::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::next<alloc::vec::Vec<country_core::shader::grass2::inc::card_splat_common::Grass2CardPointPieceGroup,alloc::alloc::Global>,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_piece_groups::closure_env$0>", "constructor": "core::iter::adapters::map::impl$2::next", "arguments": ["alloc::vec::Vec<country_core::shader::grass2::inc::card_splat_common::Grass2CardPointPieceGroup,alloc::alloc::Global>", "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>", "country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_piece_groups::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u16,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_attrib_indices::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>", "ref$<alloc::vec::Vec<u16,alloc::alloc::Global> >", "country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_attrib_indices::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::Copied<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u16,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_attrib_indices::closure_env$0> >", "constructor": "core::iter::adapters::copied::Copied", "arguments": ["core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u16,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_attrib_indices::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<u16,core::iter::adapters::copied::Copied<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u16,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_attrib_indices::closure_env$0> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["u16", "core::iter::adapters::copied::Copied<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u16,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_attrib_indices::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u32,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_plant_offsets::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>", "ref$<alloc::vec::Vec<u32,alloc::alloc::Global> >", "country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_plant_offsets::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::Copied<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u32,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_plant_offsets::closure_env$0> >", "constructor": "core::iter::adapters::copied::Copied", "arguments": ["core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u32,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_plant_offsets::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::copied::Copied<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u32,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_plant_offsets::closure_env$0> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["u32", "core::iter::adapters::copied::Copied<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u32,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_plant_offsets::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>", "ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> >", "country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$0>,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$0>", "country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::shader::grass2::inc::card_splat_common::Grass2CardPointPosition,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$0>,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::shader::grass2::inc::card_splat_common::Grass2CardPointPosition", "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$0>,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::render_loop::compute_passes::grass2::load_grass2_exemplar_params::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>", "country_core::systems::render_loop::compute_passes::grass2::load_grass2_exemplar_params::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::shader::grass2::inc::card_splat_common::Grass2ExemplarParams,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::render_loop::compute_passes::grass2::load_grass2_exemplar_params::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::shader::grass2::inc::card_splat_common::Grass2ExemplarParams", "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::render_loop::compute_passes::grass2::load_grass2_exemplar_params::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_piece_groups::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>", "country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_piece_groups::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<country_core::shader::grass2::inc::card_splat_common::Grass2CardPointPieceGroup,alloc::alloc::Global>,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_piece_groups::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["alloc::vec::Vec<country_core::shader::grass2::inc::card_splat_common::Grass2CardPointPieceGroup,alloc::alloc::Global>", "core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_piece_groups::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::impl$1::next<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u16,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_attrib_indices::closure_env$0>,u16>", "constructor": "core::iter::adapters::copied::impl$1::next", "arguments": ["core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u16,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_attrib_indices::closure_env$0>", "u16"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::impl$1::next<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u32,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_plant_offsets::closure_env$0>,u32>", "constructor": "core::iter::adapters::copied::impl$1::next", "arguments": ["core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<u32,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_plant_offsets::closure_env$0>", "u32"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<asset::foliage::Grass2SlotAsset,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["asset::foliage::Grass2SlotAsset", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::vec::Vec<asset::foliage::Grass2SlotAsset,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::vec::Vec<asset::foliage::Grass2SlotAsset,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::Grass2SlotAsset,alloc::alloc::Global> > >", "constructor": "serde_core::ser::Serializer::collect_seq", "arguments": ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<asset::foliage::Grass2SlotAsset,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<alloc::vec::Vec<asset::foliage::Grass2SlotAsset,alloc::alloc::Global> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["alloc::vec::Vec<asset::foliage::Grass2SlotAsset,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::foliage::PackedGrass2CardPointPosition
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["asset::foliage::PackedGrass2CardPointPosition", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>", "ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> >", "country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$0>,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$0>", "country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::shader::grass2::inc::card_splat_common::Grass2CardPointPosition,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$0>,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::shader::grass2::inc::card_splat_common::Grass2CardPointPosition", "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<asset::foliage::Grass2SlotAsset>,core::iter::adapters::map::Map<core::slice::iter::Iter<asset::foliage::Grass2RuntimeExemplarAsset>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure$0::closure_env$0>,country_core::systems::grass2::impl$1::active_slot_exemplars::closure_env$0>,country_core::systems::grass2::impl$1::active_exemplars::closure_env$0>,ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> >,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$0>,country_core::systems::render_loop::compute_passes::grass2::load_grass2_card_point_positions::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> > >", "constructor": "serde_core::ser::Serializer::collect_seq", "arguments": ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: foliage.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: foliage.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustClosed>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustClosed>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustClosed>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::FileDroppedOnWindow>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::UninstallModClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_publish::PublishModCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::FileDroppedOnWindow>", "bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::UninstallModClutterCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_publish::PublishModCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::FileDroppedOnWindow>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::UninstallModClutterCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_publish::PublishModCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::DeleteSessionCmd>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::DeleteSessionCmd>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::DeleteSessionCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState> > >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState> > >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>
// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>

// ------------------------------------------------------------------
// 函数: asset::foliage::_::impl$0::serialize
// 信息 ID: 222e2544da71ef5d8800dc44
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::export::to_json_value<asset::foliage::Grass2RuntimeConfigAsset>; module=2150, inline_record=1252; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::Grass2CardPointPieceGroup,alloc::alloc::Global> > >; module=3625, inline_record=2808; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::PackedGrass2CardPointPosition,alloc::alloc::Global> > >; module=3625, inline_record=7016; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<asset::foliage::Grass2CardTextureAssets>; module=3625, inline_record=69000; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<asset::foliage::Grass2ExemplarCullBounds>; module=3625, inline_record=71744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::foliage::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: a02edc7e53eec06c7c0fcc70
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] asset::foliage::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::Grass2CardPointPositionRange,alloc::alloc::Global> > > @ 0x272aa22 -> 0x271cd00 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::Grass2SlotAsset,alloc::alloc::Global> > > @ 0x272a302 -> 0x271ce30 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::Grass2Lod0PartAsset,alloc::alloc::Global> > > @ 0x2728b92 -> 0x271d000 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<asset::foliage::Grass2CardTextureAssets> @ 0x272dc8c -> 0x271d130 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<asset::foliage::Grass2ExemplarCullBounds> @ 0x272defc -> 0x271d260 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<asset::foliage::Grass2RuntimeExemplarAsset,alloc::alloc::Global> > > @ 0x272a765 -> 0x271d390 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::foliage::grass2_package_dir
// 信息 ID: cc2417242c6a522be1613419
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_grass2_lod0_meshes @ 0xa87970 -> 0xdc5ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::grass2::load_grass2_textures @ 0xc8341b -> 0xdc5ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::grass2::read_grass2_runtime_config @ 0xc848ae -> 0xdc5ed0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::foliage::grass2_runtime_config_name
// 信息 ID: c02b667e848637b821bb6b89
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::grass2::read_grass2_runtime_config @ 0xc848e7 -> 0xdc5f60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::foliage::impl$14::check_bytes<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> >
// 信息 ID: ac42555793d8ebedffb3216b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rkyv::validation::Validator -> ["rkyv::validation::archive::validator::ArchiveValidator", "rkyv::validation::shared::validator::SharedValidator"]; 字段布局 = unknown。
// [类型关系] rancor::Strategy -> ["rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>", "rancor::Error"]; 字段布局 = unknown。
// [类型关系] asset::foliage::impl$14::check_bytes -> ["rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::foliage::impl$7::check_bytes<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> > | asset::foliage::impl$7::check_bytes<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> > @ 0x8ff7fa -> 0x9023a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::foliage::impl$16::deserialize
// 信息 ID: 8dc244d0b94f951e2da2a746
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rkyv::impls::core::impl$15::deserialize_unsized<asset::foliage::ArchivedGrass2SlotAsset,asset::foliage::Grass2SlotAsset,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >; module=502, inline_record=23884; 不是独立调用边。
// [PDB 内联宿主] rkyv::impls::core::impl$15::deserialize_unsized<asset::foliage::ArchivedGrass2SlotAsset,asset::foliage::Grass2SlotAsset,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >; module=3610, inline_record=17000; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::foliage::impl$21::check_bytes<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> >
// 信息 ID: ec48b2657a851bb694cf30ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rkyv::validation::Validator -> ["rkyv::validation::archive::validator::ArchiveValidator", "rkyv::validation::shared::validator::SharedValidator"]; 字段布局 = unknown。
// [类型关系] rancor::Strategy -> ["rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>", "rancor::Error"]; 字段布局 = unknown。
// [类型关系] asset::foliage::impl$21::check_bytes -> ["rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::foliage::impl$14::check_bytes<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> > | asset::foliage::impl$14::check_bytes<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> > @ 0x90243a -> 0x8ff860 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::foliage::impl$23::deserialize
// 信息 ID: 902ee61e04ad10a6f6610e93
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rkyv::impls::core::impl$15::deserialize_unsized<asset::foliage::ArchivedGrass2RuntimeExemplarAsset,asset::foliage::Grass2RuntimeExemplarAsset,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >; module=502, inline_record=25568; 不是独立调用边。
// [PDB 内联宿主] rkyv::impls::core::impl$15::deserialize_unsized<asset::foliage::ArchivedGrass2RuntimeExemplarAsset,asset::foliage::Grass2RuntimeExemplarAsset,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >; module=3610, inline_record=6984; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::foliage::impl$30::deserialize
// 信息 ID: d3ada640dbd9d7b858452a1e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rkyv::impls::core::impl$15::deserialize_unsized<asset::foliage::ArchivedGrass2RuntimeExemplarAsset,asset::foliage::Grass2RuntimeExemplarAsset,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >; module=502, inline_record=28916; 不是独立调用边。
// [PDB 内联宿主] rkyv::impls::core::impl$15::deserialize_unsized<asset::foliage::ArchivedGrass2RuntimeExemplarAsset,asset::foliage::Grass2RuntimeExemplarAsset,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >; module=3610, inline_record=10332; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::foliage::impl$31::visit_asset_dependencies
// 信息 ID: 6515cf686d4e728792a32d6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::foliage::impl$35::resolve
// 信息 ID: 5616933acefbdf0ac02ed880
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::foliage::impl$39::deserialize
// 信息 ID: 98072ce9b1a4fb69b88440a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rkyv::impls::core::impl$15::deserialize_unsized<asset::foliage::ArchivedGrass2RuntimeExemplarAsset,asset::foliage::Grass2RuntimeExemplarAsset,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >; module=502, inline_record=28300; 不是独立调用边。
// [PDB 内联宿主] rkyv::impls::core::impl$15::deserialize_unsized<asset::foliage::ArchivedGrass2RuntimeExemplarAsset,asset::foliage::Grass2RuntimeExemplarAsset,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >; module=3610, inline_record=9716; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::foliage::impl$40::visit_asset_dependencies
// 信息 ID: 0ba243aba4c410082a33e646
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::foliage::impl$43::resolve
// 信息 ID: 22baa2263d87b063190396b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::foliage::impl$47::deserialize
// 信息 ID: 408c04ec96e45900883310de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rkyv::api::high::checked::from_bytes<asset::foliage::Grass2RuntimeConfigAsset,rancor::Error>; module=1666, inline_record=100412; 不是独立调用边。
// [PDB 内联宿主] rkyv::api::high::checked::from_bytes<asset::foliage::Grass2RuntimeConfigAsset,rancor::Error>; module=3623, inline_record=4832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::foliage::impl$48::visit_asset_dependencies
// 信息 ID: 05fcf5b068840cb775530b32
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::foliage::impl$58::visit_asset_dependencies
// 信息 ID: 85ca5d0addf83f2574062e8e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::foliage::impl$77::deserialize
// 信息 ID: 71e69fa9569988ac6dd520bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rkyv::impls::core::impl$15::deserialize_unsized<asset::foliage::ArchivedGrass2RuntimeExemplarAsset,asset::foliage::Grass2RuntimeExemplarAsset,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >; module=502, inline_record=27412; 不是独立调用边。
// [PDB 内联宿主] rkyv::impls::core::impl$15::deserialize_unsized<asset::foliage::ArchivedGrass2RuntimeExemplarAsset,asset::foliage::Grass2RuntimeExemplarAsset,rancor::Strategy<rkyv::de::pooling::alloc::Pool,rancor::Error> >; module=3610, inline_record=8828; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::foliage::impl$78::visit_asset_dependencies
// 信息 ID: 6a8cc13eb9cced64443db3a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::foliage::impl$7::check_bytes
// 信息 ID: c0f1baf0f0286daa68f141a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::foliage::impl$7::check_bytes<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> >
// 信息 ID: 5508b44487473230cbc9184e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rkyv::validation::Validator -> ["rkyv::validation::archive::validator::ArchiveValidator", "rkyv::validation::shared::validator::SharedValidator"]; 字段布局 = unknown。
// [类型关系] rancor::Strategy -> ["rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>", "rancor::Error"]; 字段布局 = unknown。
// [类型关系] asset::foliage::impl$7::check_bytes -> ["rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rkyv::api::high::checked::from_bytes<asset::foliage::Grass2RuntimeConfigAsset,rancor::Error> @ 0x17d09bb -> 0x8ff770 (direct_call); 同地址别名未裁定。
// [静态直接引用] rkyv::api::high::checked::from_bytes<asset::foliage::Grass2RuntimeConfigAsset,rancor::Error> @ 0x272878b -> 0x8ff770 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::foliage::impl$88::visit_asset_dependencies
// 信息 ID: f212b0da8b76e756a401519d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::foliage::impl$9::deserialize
// 信息 ID: 2f421e3a7f6830e03f168584
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rkyv::api::high::checked::from_bytes<asset::foliage::Grass2RuntimeConfigAsset,rancor::Error>; module=1666, inline_record=100008; 不是独立调用边。
// [PDB 内联宿主] rkyv::api::high::checked::from_bytes<asset::foliage::Grass2RuntimeConfigAsset,rancor::Error>; module=3623, inline_record=4428; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

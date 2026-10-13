// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/crates/shader-pipeline/src/reflection.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: c365ecfa1d1f95792c934448aba50fcbdef73d0f1c1dbe0065f2a526b879bcd6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: reflection.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 61, 102, 107, 117, 145, 149, 185, 214, 238, 241, 500, 502, 503, 517, 518, 534, 576, 752, 1610, 1613, 1624, 1632, 1638, 1657, 1714, 1729, 1917, 3153, 3311
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use const_fnv1a_hash; // namespace candidate
//   package identities: [{"name":"const-fnv1a-hash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.1.0"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use shader_pipeline; // namespace candidate
//   package identities: [{"name":"shader-pipeline","source":"local_or_path","version":"0.1.0"}]
// use speedy; // namespace candidate
//   package identities: [{"name":"speedy","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$16::eq
// 记录形式: inline_instance, inlinee_declaration; instances=220
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2582
// 首条位置: {"module":61,"inlinee":9602}
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
// IPI IDs: 0x120AD
// 首条位置: {"module":1610,"inlinee":73901}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":126712,"parent_record_offset":126688,"ancestor_record_offset":126688,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":126868,"parent_record_offset":126836,"ancestor_record_offset":126688,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1610,"record_offset":126900,"parent_record_offset":126836,"ancestor_record_offset":126688,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":126804,"parent_record_offset":126768,"ancestor_record_offset":126688,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":126836,"parent_record_offset":126768,"ancestor_record_offset":126688,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$35::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":126740,"parent_record_offset":126712,"ancestor_record_offset":126688,"inlinee":73903,"source_line":116,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":126768,"parent_record_offset":126740,"ancestor_record_offset":126688,"inlinee":73904,"source_line":288,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$0::hash
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2FB8
// 首条位置: {"module":117,"inlinee":12216}
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
// 首条位置: {"module":752,"record_offset":264,"rva":13137488,"va":5381846608}
// 编译实例 RVAs: 0xc87650
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
// IPI IDs: 0x7740
// 首条位置: {"module":534,"inlinee":30528}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":21840,"parent_record_offset":19648,"ancestor_record_offset":19648,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":20240,"parent_record_offset":19648,"ancestor_record_offset":19648,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":534,"record_offset":19920,"parent_record_offset":19648,"ancestor_record_offset":19648,"inlinee":30496,"source_line":159,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_key_value_collection
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":20096,"parent_record_offset":19648,"ancestor_record_offset":19648,"inlinee":30520,"source_line":652,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":19700,"parent_record_offset":19648,"ancestor_record_offset":19648,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":534,"record_offset":19848,"parent_record_offset":19820,"ancestor_record_offset":19648,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":534,"record_offset":20964,"parent_record_offset":20932,"ancestor_record_offset":19648,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":534,"record_offset":20172,"parent_record_offset":20144,"ancestor_record_offset":19648,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":534,"record_offset":21024,"parent_record_offset":21000,"ancestor_record_offset":19648,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":534,"record_offset":21196,"parent_record_offset":21048,"ancestor_record_offset":19648,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":20732,"parent_record_offset":20524,"ancestor_record_offset":19648,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":21048,"parent_record_offset":21024,"ancestor_record_offset":19648,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":21000,"parent_record_offset":20868,"ancestor_record_offset":19648,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":21220,"parent_record_offset":21196,"ancestor_record_offset":19648,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":20144,"parent_record_offset":20120,"ancestor_record_offset":19648,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":534,"record_offset":20676,"parent_record_offset":20524,"ancestor_record_offset":19648,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":534,"record_offset":20700,"parent_record_offset":20676,"ancestor_record_offset":19648,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":534,"record_offset":19876,"parent_record_offset":19820,"ancestor_record_offset":19648,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":21864,"parent_record_offset":21840,"ancestor_record_offset":19648,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":534,"record_offset":21164,"parent_record_offset":21076,"ancestor_record_offset":19648,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_unaligned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":21108,"parent_record_offset":21076,"ancestor_record_offset":19648,"inlinee":6431,"source_line":1461,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$25::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":20200,"parent_record_offset":20144,"ancestor_record_offset":19648,"inlinee":30523,"source_line":1984,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":534,"record_offset":20900,"parent_record_offset":20868,"ancestor_record_offset":19648,"inlinee":6916,"source_line":1103,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":534,"record_offset":20868,"parent_record_offset":20840,"ancestor_record_offset":19648,"inlinee":6915,"source_line":4480,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":534,"record_offset":21244,"parent_record_offset":21220,"ancestor_record_offset":19648,"inlinee":6918,"source_line":4492,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":534,"record_offset":20932,"parent_record_offset":20900,"ancestor_record_offset":19648,"inlinee":5655,"source_line":896,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":534,"record_offset":21912,"parent_record_offset":21888,"ancestor_record_offset":19648,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$61::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":21604,"parent_record_offset":21580,"ancestor_record_offset":19648,"inlinee":30537,"source_line":69,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$79::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":21344,"parent_record_offset":21320,"ancestor_record_offset":19648,"inlinee":30535,"source_line":75,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":534,"record_offset":20348,"parent_record_offset":20308,"ancestor_record_offset":19648,"inlinee":30513,"source_line":195,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":534,"record_offset":20372,"parent_record_offset":20348,"ancestor_record_offset":19648,"inlinee":30496,"source_line":159,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":19792,"parent_record_offset":19756,"ancestor_record_offset":19648,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_borrowed_from_reader
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":20572,"parent_record_offset":20524,"ancestor_record_offset":19648,"inlinee":30533,"source_line":123,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":19820,"parent_record_offset":19756,"ancestor_record_offset":19648,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$2::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":534,"record_offset":20308,"parent_record_offset":20272,"ancestor_record_offset":19648,"inlinee":30532,"source_line":54,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":534,"record_offset":19728,"parent_record_offset":19700,"ancestor_record_offset":19648,"inlinee":30530,"source_line":116,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_collection
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":20120,"parent_record_offset":20096,"ancestor_record_offset":19648,"inlinee":30521,"source_line":630,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_key_value_collection
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":20524,"parent_record_offset":20308,"ancestor_record_offset":19648,"inlinee":30520,"source_line":652,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":19756,"parent_record_offset":19728,"ancestor_record_offset":19648,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":21368,"parent_record_offset":21344,"ancestor_record_offset":19648,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":21076,"parent_record_offset":21048,"ancestor_record_offset":19648,"inlinee":30542,"source_line":22,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":534,"record_offset":20812,"parent_record_offset":20768,"ancestor_record_offset":19648,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$87::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":534,"record_offset":20768,"parent_record_offset":20732,"ancestor_record_offset":19648,"inlinee":11275,"source_line":3230,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$88::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":534,"record_offset":20840,"parent_record_offset":20768,"ancestor_record_offset":19648,"inlinee":11277,"source_line":3246,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$29::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120AB
// 首条位置: {"module":1610,"inlinee":73899}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":118368,"parent_record_offset":118108,"ancestor_record_offset":118108,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":118136,"parent_record_offset":118108,"ancestor_record_offset":118108,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":118268,"parent_record_offset":118240,"ancestor_record_offset":118108,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1610,"record_offset":118296,"parent_record_offset":118240,"ancestor_record_offset":118108,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":118212,"parent_record_offset":118184,"ancestor_record_offset":118108,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":118240,"parent_record_offset":118184,"ancestor_record_offset":118108,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":118160,"parent_record_offset":118136,"ancestor_record_offset":118108,"inlinee":30530,"source_line":116,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":118184,"parent_record_offset":118160,"ancestor_record_offset":118108,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/01610.jsonl.gz"}
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
// 首条位置: {"module":3311,"record_offset":440,"rva":34857232,"va":5403566352}
// 编译实例 RVAs: 0x213e110
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reflection.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-20673959","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5403566352,"raw_hex":"e9598ac4fe","size":5,"soft_fail":false,"target_va":5382892432,"va":5403566386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$37::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120A2
// 首条位置: {"module":1610,"inlinee":73890}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":90780,"parent_record_offset":90752,"ancestor_record_offset":90752,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":90872,"parent_record_offset":90844,"ancestor_record_offset":90752,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1610,"record_offset":90904,"parent_record_offset":90844,"ancestor_record_offset":90752,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":90816,"parent_record_offset":90780,"ancestor_record_offset":90752,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":90844,"parent_record_offset":90780,"ancestor_record_offset":90752,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/01610.jsonl.gz"}
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
// 首条位置: {"module":3311,"record_offset":560,"rva":34857280,"va":5403566400}
// 编译实例 RVAs: 0x213e140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_tuple_field1_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20672196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403566400,"raw_hex":"e83c91c4fe","size":5,"soft_fail":false,"target_va":5382894288,"va":5403566479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reflection.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-20674005","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5403566400,"raw_hex":"e92b8ac4fe","size":5,"soft_fail":false,"target_va":5382892432,"va":5403566432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$45::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120A4
// 首条位置: {"module":1610,"inlinee":73892}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":91004,"parent_record_offset":90976,"ancestor_record_offset":90976,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":91156,"parent_record_offset":90976,"ancestor_record_offset":90976,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":91088,"parent_record_offset":91060,"ancestor_record_offset":90976,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1610,"record_offset":91120,"parent_record_offset":91060,"ancestor_record_offset":90976,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":91032,"parent_record_offset":91004,"ancestor_record_offset":90976,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":91060,"parent_record_offset":91004,"ancestor_record_offset":90976,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":91184,"parent_record_offset":91156,"ancestor_record_offset":90976,"inlinee":30530,"source_line":116,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":91212,"parent_record_offset":91184,"ancestor_record_offset":90976,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$48::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C035
// 首条位置: {"module":3311,"inlinee":114741}
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
// IPI IDs: 0x120A0
// 首条位置: {"module":1610,"inlinee":73888}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":92060,"parent_record_offset":90688,"ancestor_record_offset":90688,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":91384,"parent_record_offset":90688,"ancestor_record_offset":90688,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":1610,"record_offset":91412,"parent_record_offset":90688,"ancestor_record_offset":90688,"inlinee":30496,"source_line":159,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":90724,"parent_record_offset":90688,"ancestor_record_offset":90688,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":91588,"parent_record_offset":90688,"ancestor_record_offset":90688,"inlinee":30501,"source_line":514,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":91888,"parent_record_offset":91864,"ancestor_record_offset":90688,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":91912,"parent_record_offset":91888,"ancestor_record_offset":90688,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":92344,"parent_record_offset":92320,"ancestor_record_offset":90688,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":91864,"parent_record_offset":91820,"ancestor_record_offset":90688,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":92320,"parent_record_offset":92264,"ancestor_record_offset":90688,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":91744,"parent_record_offset":91708,"ancestor_record_offset":90688,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":92292,"parent_record_offset":92264,"ancestor_record_offset":90688,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":92264,"parent_record_offset":92240,"ancestor_record_offset":90688,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":91820,"parent_record_offset":91780,"ancestor_record_offset":90688,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":91780,"parent_record_offset":91744,"ancestor_record_offset":90688,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":92240,"parent_record_offset":92216,"ancestor_record_offset":90688,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1610,"record_offset":92156,"parent_record_offset":92124,"ancestor_record_offset":90688,"inlinee":14307,"source_line":576,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1610,"record_offset":91672,"parent_record_offset":91588,"ancestor_record_offset":90688,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1610,"record_offset":91708,"parent_record_offset":91672,"ancestor_record_offset":90688,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":90872,"parent_record_offset":90844,"ancestor_record_offset":90688,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1610,"record_offset":92092,"parent_record_offset":92060,"ancestor_record_offset":90688,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1610,"record_offset":90904,"parent_record_offset":90844,"ancestor_record_offset":90688,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1610,"record_offset":92192,"parent_record_offset":92156,"ancestor_record_offset":90688,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1610,"record_offset":92396,"parent_record_offset":92124,"ancestor_record_offset":90688,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$37::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1610,"record_offset":90752,"parent_record_offset":90724,"ancestor_record_offset":90688,"inlinee":73890,"source_line":46,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$45::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1610,"record_offset":90976,"parent_record_offset":90948,"ancestor_record_offset":90688,"inlinee":73892,"source_line":56,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::error::error_invalid_string_utf8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/error.rs
//     evidence={"module":1610,"record_offset":92444,"parent_record_offset":92420,"ancestor_record_offset":90688,"inlinee":27487,"source_line":137,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::vec_to_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":1610,"record_offset":92124,"parent_record_offset":92092,"ancestor_record_offset":90688,"inlinee":30508,"source_line":39,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":90816,"parent_record_offset":90780,"ancestor_record_offset":90688,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1610,"record_offset":90844,"parent_record_offset":90780,"ancestor_record_offset":90688,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1610,"record_offset":91184,"parent_record_offset":91156,"ancestor_record_offset":90688,"inlinee":30530,"source_line":116,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":90780,"parent_record_offset":90752,"ancestor_record_offset":90688,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1610,"record_offset":91156,"parent_record_offset":90976,"ancestor_record_offset":90688,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$56::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13006
// 首条位置: {"module":1729,"inlinee":77830}
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
// 记录形式: inline_instance, inlinee_declaration; instances=28
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2ECE
// 首条位置: {"module":107,"inlinee":11982}
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
// IPI IDs: 0x3D84
// 首条位置: {"module":241,"inlinee":15748}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":241,"record_offset":63624,"parent_record_offset":63600,"ancestor_record_offset":63600,"inlinee":8973,"source_line":811,"archive":"_pdb/B/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":241,"record_offset":63648,"parent_record_offset":63624,"ancestor_record_offset":63600,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":241,"record_offset":63724,"parent_record_offset":63700,"ancestor_record_offset":63600,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":241,"record_offset":63672,"parent_record_offset":63648,"ancestor_record_offset":63600,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/00241.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":241,"record_offset":63700,"parent_record_offset":63648,"ancestor_record_offset":63600,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/00241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$61::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7749
// 首条位置: {"module":534,"inlinee":30537}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":21628,"parent_record_offset":21604,"ancestor_record_offset":21604,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":534,"record_offset":21760,"parent_record_offset":21732,"ancestor_record_offset":21604,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":534,"record_offset":21788,"parent_record_offset":21732,"ancestor_record_offset":21604,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":21704,"parent_record_offset":21676,"ancestor_record_offset":21604,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":21732,"parent_record_offset":21676,"ancestor_record_offset":21604,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":534,"record_offset":21652,"parent_record_offset":21628,"ancestor_record_offset":21604,"inlinee":30530,"source_line":116,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":21676,"parent_record_offset":21652,"ancestor_record_offset":21604,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$74::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13002
// 首条位置: {"module":1729,"inlinee":77826}
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
// 记录形式: inline_instance, inlinee_declaration; instances=160
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x258A
// 首条位置: {"module":61,"inlinee":9610}
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
// IPI IDs: 0x7747
// 首条位置: {"module":534,"inlinee":30535}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":21368,"parent_record_offset":21344,"ancestor_record_offset":21344,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":534,"record_offset":21500,"parent_record_offset":21472,"ancestor_record_offset":21344,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":534,"record_offset":21528,"parent_record_offset":21472,"ancestor_record_offset":21344,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":21444,"parent_record_offset":21416,"ancestor_record_offset":21344,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":21472,"parent_record_offset":21416,"ancestor_record_offset":21344,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":534,"record_offset":21392,"parent_record_offset":21368,"ancestor_record_offset":21344,"inlinee":30530,"source_line":116,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":21416,"parent_record_offset":21392,"ancestor_record_offset":21344,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$84::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7737
// 首条位置: {"module":534,"inlinee":30519}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":19212,"parent_record_offset":18868,"ancestor_record_offset":18868,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":534,"record_offset":18892,"parent_record_offset":18868,"ancestor_record_offset":18868,"inlinee":30496,"source_line":159,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_key_value_collection
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":19068,"parent_record_offset":18868,"ancestor_record_offset":18868,"inlinee":30520,"source_line":652,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":534,"record_offset":19000,"parent_record_offset":18972,"ancestor_record_offset":18868,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":534,"record_offset":19144,"parent_record_offset":19116,"ancestor_record_offset":18868,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":19116,"parent_record_offset":19092,"ancestor_record_offset":18868,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":534,"record_offset":19028,"parent_record_offset":18972,"ancestor_record_offset":18868,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$25::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":19172,"parent_record_offset":19116,"ancestor_record_offset":18868,"inlinee":30523,"source_line":1984,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":18944,"parent_record_offset":18916,"ancestor_record_offset":18868,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":18972,"parent_record_offset":18916,"ancestor_record_offset":18868,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_collection
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":19092,"parent_record_offset":19068,"ancestor_record_offset":18868,"inlinee":30521,"source_line":630,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":18916,"parent_record_offset":18892,"ancestor_record_offset":18868,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$89::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7735
// 首条位置: {"module":534,"inlinee":30517}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":534,"record_offset":18228,"parent_record_offset":18176,"ancestor_record_offset":18176,"inlinee":30496,"source_line":159,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":18416,"parent_record_offset":18176,"ancestor_record_offset":18176,"inlinee":30501,"source_line":514,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":18736,"parent_record_offset":18712,"ancestor_record_offset":18176,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":18760,"parent_record_offset":18736,"ancestor_record_offset":18176,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":18712,"parent_record_offset":18680,"ancestor_record_offset":18176,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":18628,"parent_record_offset":18600,"ancestor_record_offset":18176,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":18680,"parent_record_offset":18652,"ancestor_record_offset":18176,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":18652,"parent_record_offset":18628,"ancestor_record_offset":18176,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":18572,"parent_record_offset":18416,"ancestor_record_offset":18176,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":18600,"parent_record_offset":18572,"ancestor_record_offset":18176,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":534,"record_offset":18348,"parent_record_offset":18320,"ancestor_record_offset":18176,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":534,"record_offset":19144,"parent_record_offset":19116,"ancestor_record_offset":18176,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":19116,"parent_record_offset":19092,"ancestor_record_offset":18176,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":534,"record_offset":18544,"parent_record_offset":18416,"ancestor_record_offset":18176,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":534,"record_offset":18376,"parent_record_offset":18320,"ancestor_record_offset":18176,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":19308,"parent_record_offset":18416,"ancestor_record_offset":18176,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":534,"record_offset":18816,"parent_record_offset":18416,"ancestor_record_offset":18176,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":19252,"parent_record_offset":18416,"ancestor_record_offset":18176,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$25::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":19172,"parent_record_offset":19116,"ancestor_record_offset":18176,"inlinee":30523,"source_line":1984,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":19212,"parent_record_offset":18868,"ancestor_record_offset":18176,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$84::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":18868,"parent_record_offset":18844,"ancestor_record_offset":18176,"inlinee":30519,"source_line":78,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":534,"record_offset":18892,"parent_record_offset":18868,"ancestor_record_offset":18176,"inlinee":30496,"source_line":159,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":18288,"parent_record_offset":18256,"ancestor_record_offset":18176,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":18320,"parent_record_offset":18256,"ancestor_record_offset":18176,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_collection
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":19092,"parent_record_offset":19068,"ancestor_record_offset":18176,"inlinee":30521,"source_line":630,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_key_value_collection
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":19068,"parent_record_offset":18868,"ancestor_record_offset":18176,"inlinee":30520,"source_line":652,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":18256,"parent_record_offset":18228,"ancestor_record_offset":18176,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":18844,"parent_record_offset":18416,"ancestor_record_offset":18176,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$94::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7733
// 首条位置: {"module":534,"inlinee":30515}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":24164,"parent_record_offset":18008,"ancestor_record_offset":18008,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":19508,"parent_record_offset":18008,"ancestor_record_offset":18008,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":22144,"parent_record_offset":18008,"ancestor_record_offset":18008,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":24516,"parent_record_offset":18008,"ancestor_record_offset":18008,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":534,"record_offset":22172,"parent_record_offset":18008,"ancestor_record_offset":18008,"inlinee":30496,"source_line":159,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_key_value_collection
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":22348,"parent_record_offset":18008,"ancestor_record_offset":18008,"inlinee":30520,"source_line":652,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":19376,"parent_record_offset":18008,"ancestor_record_offset":18008,"inlinee":30524,"source_line":108,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":18120,"parent_record_offset":18008,"ancestor_record_offset":18008,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":18736,"parent_record_offset":18712,"ancestor_record_offset":18008,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":18760,"parent_record_offset":18736,"ancestor_record_offset":18008,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":18712,"parent_record_offset":18680,"ancestor_record_offset":18008,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":18628,"parent_record_offset":18600,"ancestor_record_offset":18008,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":18680,"parent_record_offset":18652,"ancestor_record_offset":18008,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":18652,"parent_record_offset":18628,"ancestor_record_offset":18008,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":18572,"parent_record_offset":18416,"ancestor_record_offset":18008,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":18600,"parent_record_offset":18572,"ancestor_record_offset":18008,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":534,"record_offset":18348,"parent_record_offset":18320,"ancestor_record_offset":18008,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":534,"record_offset":20964,"parent_record_offset":20932,"ancestor_record_offset":18008,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":534,"record_offset":19144,"parent_record_offset":19116,"ancestor_record_offset":18008,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":534,"record_offset":21024,"parent_record_offset":21000,"ancestor_record_offset":18008,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":534,"record_offset":21196,"parent_record_offset":21048,"ancestor_record_offset":18008,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":20732,"parent_record_offset":20524,"ancestor_record_offset":18008,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":21048,"parent_record_offset":21024,"ancestor_record_offset":18008,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":21000,"parent_record_offset":20868,"ancestor_record_offset":18008,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":21220,"parent_record_offset":21196,"ancestor_record_offset":18008,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":19116,"parent_record_offset":19092,"ancestor_record_offset":18008,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":534,"record_offset":20676,"parent_record_offset":20524,"ancestor_record_offset":18008,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":534,"record_offset":18544,"parent_record_offset":18416,"ancestor_record_offset":18008,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":534,"record_offset":18376,"parent_record_offset":18320,"ancestor_record_offset":18008,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":19308,"parent_record_offset":18416,"ancestor_record_offset":18008,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":534,"record_offset":18816,"parent_record_offset":18416,"ancestor_record_offset":18008,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_unaligned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":21108,"parent_record_offset":21076,"ancestor_record_offset":18008,"inlinee":6431,"source_line":1461,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":19252,"parent_record_offset":18416,"ancestor_record_offset":18008,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$25::from_iter
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":19172,"parent_record_offset":19116,"ancestor_record_offset":18008,"inlinee":30523,"source_line":1984,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":19212,"parent_record_offset":18868,"ancestor_record_offset":18008,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":534,"record_offset":20900,"parent_record_offset":20868,"ancestor_record_offset":18008,"inlinee":6916,"source_line":1103,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":534,"record_offset":20868,"parent_record_offset":20840,"ancestor_record_offset":18008,"inlinee":6915,"source_line":4480,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":534,"record_offset":21244,"parent_record_offset":21220,"ancestor_record_offset":18008,"inlinee":6918,"source_line":4492,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":534,"record_offset":20932,"parent_record_offset":20900,"ancestor_record_offset":18008,"inlinee":5655,"source_line":896,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":534,"record_offset":21912,"parent_record_offset":21888,"ancestor_record_offset":18008,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$25::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":19648,"parent_record_offset":19604,"ancestor_record_offset":18008,"inlinee":30528,"source_line":31,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$61::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":21604,"parent_record_offset":21580,"ancestor_record_offset":18008,"inlinee":30537,"source_line":69,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$79::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":21344,"parent_record_offset":21320,"ancestor_record_offset":18008,"inlinee":30535,"source_line":75,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$84::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":18868,"parent_record_offset":18844,"ancestor_record_offset":18008,"inlinee":30519,"source_line":78,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$89::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":18176,"parent_record_offset":18120,"ancestor_record_offset":18008,"inlinee":30517,"source_line":83,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$94::read_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":19556,"parent_record_offset":19508,"ancestor_record_offset":18008,"inlinee":30526,"source_line":88,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$94::read_from::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":23504,"parent_record_offset":23476,"ancestor_record_offset":18008,"inlinee":30538,"source_line":88,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$94::read_from::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":23532,"parent_record_offset":23504,"ancestor_record_offset":18008,"inlinee":30540,"source_line":88,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":534,"record_offset":20348,"parent_record_offset":20308,"ancestor_record_offset":18008,"inlinee":30513,"source_line":195,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":534,"record_offset":18228,"parent_record_offset":18176,"ancestor_record_offset":18008,"inlinee":30496,"source_line":159,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":18288,"parent_record_offset":18256,"ancestor_record_offset":18008,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_borrowed_from_reader
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":20572,"parent_record_offset":20524,"ancestor_record_offset":18008,"inlinee":30533,"source_line":123,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":18320,"parent_record_offset":18256,"ancestor_record_offset":18008,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$2::read_from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":534,"record_offset":20308,"parent_record_offset":20272,"ancestor_record_offset":18008,"inlinee":30532,"source_line":54,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":534,"record_offset":19728,"parent_record_offset":19700,"ancestor_record_offset":18008,"inlinee":30530,"source_line":116,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_collection
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":19092,"parent_record_offset":19068,"ancestor_record_offset":18008,"inlinee":30521,"source_line":630,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_key_value_collection
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":19068,"parent_record_offset":18868,"ancestor_record_offset":18008,"inlinee":30520,"source_line":652,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":18256,"parent_record_offset":18228,"ancestor_record_offset":18008,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":18844,"parent_record_offset":18416,"ancestor_record_offset":18008,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":18416,"parent_record_offset":18176,"ancestor_record_offset":18008,"inlinee":30501,"source_line":514,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":21076,"parent_record_offset":21048,"ancestor_record_offset":18008,"inlinee":30542,"source_line":22,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":534,"record_offset":20812,"parent_record_offset":20768,"ancestor_record_offset":18008,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$87::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":534,"record_offset":20768,"parent_record_offset":20732,"ancestor_record_offset":18008,"inlinee":11275,"source_line":3230,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$88::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":534,"record_offset":20840,"parent_record_offset":20768,"ancestor_record_offset":18008,"inlinee":11277,"source_line":3246,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$94::read_from::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x773E
// 首条位置: {"module":534,"inlinee":30526}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":22100,"parent_record_offset":19556,"ancestor_record_offset":19556,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":19604,"parent_record_offset":19556,"ancestor_record_offset":19556,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":534,"record_offset":19848,"parent_record_offset":19820,"ancestor_record_offset":19556,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":534,"record_offset":20964,"parent_record_offset":20932,"ancestor_record_offset":19556,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":534,"record_offset":20172,"parent_record_offset":20144,"ancestor_record_offset":19556,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":534,"record_offset":21024,"parent_record_offset":21000,"ancestor_record_offset":19556,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":534,"record_offset":21196,"parent_record_offset":21048,"ancestor_record_offset":19556,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":20732,"parent_record_offset":20524,"ancestor_record_offset":19556,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":21048,"parent_record_offset":21024,"ancestor_record_offset":19556,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":21000,"parent_record_offset":20868,"ancestor_record_offset":19556,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":21220,"parent_record_offset":21196,"ancestor_record_offset":19556,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":20144,"parent_record_offset":20120,"ancestor_record_offset":19556,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":534,"record_offset":20676,"parent_record_offset":20524,"ancestor_record_offset":19556,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":534,"record_offset":20700,"parent_record_offset":20676,"ancestor_record_offset":19556,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":534,"record_offset":19876,"parent_record_offset":19820,"ancestor_record_offset":19556,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":21840,"parent_record_offset":19648,"ancestor_record_offset":19556,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":534,"record_offset":21164,"parent_record_offset":21076,"ancestor_record_offset":19556,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_unaligned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":21108,"parent_record_offset":21076,"ancestor_record_offset":19556,"inlinee":6431,"source_line":1461,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$25::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":20200,"parent_record_offset":20144,"ancestor_record_offset":19556,"inlinee":30523,"source_line":1984,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":20240,"parent_record_offset":19648,"ancestor_record_offset":19556,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":534,"record_offset":20900,"parent_record_offset":20868,"ancestor_record_offset":19556,"inlinee":6916,"source_line":1103,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":534,"record_offset":20868,"parent_record_offset":20840,"ancestor_record_offset":19556,"inlinee":6915,"source_line":4480,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":534,"record_offset":21244,"parent_record_offset":21220,"ancestor_record_offset":19556,"inlinee":6918,"source_line":4492,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":534,"record_offset":20932,"parent_record_offset":20900,"ancestor_record_offset":19556,"inlinee":5655,"source_line":896,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":534,"record_offset":21912,"parent_record_offset":21888,"ancestor_record_offset":19556,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$25::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":19648,"parent_record_offset":19604,"ancestor_record_offset":19556,"inlinee":30528,"source_line":31,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$61::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":21604,"parent_record_offset":21580,"ancestor_record_offset":19556,"inlinee":30537,"source_line":69,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$79::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":21344,"parent_record_offset":21320,"ancestor_record_offset":19556,"inlinee":30535,"source_line":75,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":534,"record_offset":20348,"parent_record_offset":20308,"ancestor_record_offset":19556,"inlinee":30513,"source_line":195,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":534,"record_offset":19920,"parent_record_offset":19648,"ancestor_record_offset":19556,"inlinee":30496,"source_line":159,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":19792,"parent_record_offset":19756,"ancestor_record_offset":19556,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_borrowed_from_reader
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":20572,"parent_record_offset":20524,"ancestor_record_offset":19556,"inlinee":30533,"source_line":123,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":19820,"parent_record_offset":19756,"ancestor_record_offset":19556,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$2::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":534,"record_offset":20308,"parent_record_offset":20272,"ancestor_record_offset":19556,"inlinee":30532,"source_line":54,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":534,"record_offset":19728,"parent_record_offset":19700,"ancestor_record_offset":19556,"inlinee":30530,"source_line":116,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_collection
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":20120,"parent_record_offset":20096,"ancestor_record_offset":19556,"inlinee":30521,"source_line":630,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_key_value_collection
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":20096,"parent_record_offset":19648,"ancestor_record_offset":19556,"inlinee":30520,"source_line":652,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":19756,"parent_record_offset":19728,"ancestor_record_offset":19556,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":19700,"parent_record_offset":19648,"ancestor_record_offset":19556,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":21076,"parent_record_offset":21048,"ancestor_record_offset":19556,"inlinee":30542,"source_line":22,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":534,"record_offset":20812,"parent_record_offset":20768,"ancestor_record_offset":19556,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$87::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":534,"record_offset":20768,"parent_record_offset":20732,"ancestor_record_offset":19556,"inlinee":11275,"source_line":3230,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$88::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":534,"record_offset":20840,"parent_record_offset":20768,"ancestor_record_offset":19556,"inlinee":11277,"source_line":3246,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$94::read_from::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x774A
// 首条位置: {"module":534,"inlinee":30538}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] shader_pipeline::reflection::impl$94::read_from::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":23532,"parent_record_offset":23504,"ancestor_record_offset":23504,"inlinee":30540,"source_line":88,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":534,"record_offset":23692,"parent_record_offset":23664,"ancestor_record_offset":23504,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":534,"record_offset":23720,"parent_record_offset":23664,"ancestor_record_offset":23504,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":23636,"parent_record_offset":23608,"ancestor_record_offset":23504,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":23664,"parent_record_offset":23608,"ancestor_record_offset":23504,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":534,"record_offset":23584,"parent_record_offset":23560,"ancestor_record_offset":23504,"inlinee":30530,"source_line":116,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":23608,"parent_record_offset":23584,"ancestor_record_offset":23504,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":23560,"parent_record_offset":23532,"ancestor_record_offset":23504,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$94::read_from::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x774C
// 首条位置: {"module":534,"inlinee":30540}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":23560,"parent_record_offset":23532,"ancestor_record_offset":23532,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":534,"record_offset":23692,"parent_record_offset":23664,"ancestor_record_offset":23532,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":534,"record_offset":23720,"parent_record_offset":23664,"ancestor_record_offset":23532,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":23636,"parent_record_offset":23608,"ancestor_record_offset":23532,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":23664,"parent_record_offset":23608,"ancestor_record_offset":23532,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":534,"record_offset":23584,"parent_record_offset":23560,"ancestor_record_offset":23532,"inlinee":30530,"source_line":116,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":23608,"parent_record_offset":23584,"ancestor_record_offset":23532,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::reflection::impl$3::calculate_name_hashes::closure$0
// 记录形式: inlinee_declaration; instances=1
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12DA7
// 首条位置: {"module":1714,"inlinee":77223}
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
// IPI IDs: 0x12DA6
// 首条位置: {"module":1714,"inlinee":77222}
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
// IPI IDs: 0xAB45
// 首条位置: {"module":752,"inlinee":43845}
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
// IPI IDs: 0xAB4A
// 首条位置: {"module":752,"record_offset":404,"rva":13137504,"va":5381846624}
// 编译实例 RVAs: 0xc87660
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] const_fnv1a_hash::fnv1a_hash_64
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"const-fnv1a-hash","version":"1.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13303652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381846624,"raw_hex":"e864ffca00","size":5,"soft_fail":false,"target_va":5395150352,"va":5381846695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] shader_pipeline::reflection::len_without_suffix
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":752,"record_offset":540,"parent_record_offset":404,"ancestor_record_offset":404,"inlinee":43845,"source_line":117,"archive":"_pdb/B/modules/00752.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 19; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::BindingNameHash::new
// 信息 ID: 73e7f24afe2a55adacea03ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PathRaster> @ 0x3d8e14 -> 0xc87660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone3Raster> @ 0x3d9334 -> 0xc87660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::WaterRaster> @ 0x3d9854 -> 0xc87660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::GardenRaster> @ 0x3d9d74 -> 0xc87660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone1Raster> @ 0x3da294 -> 0xc87660 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone2Raster> @ 0x3da7b4 -> 0xc87660 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::BindingNameHash::with_suffix_dims
// 信息 ID: 2b8b45d52e886a72b70fb5fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 38 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x155511 -> 0xc87650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x15597b -> 0xc87650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x155e9a -> 0xc87650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x156363 -> 0xc87650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x2af86a -> 0xc87650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x2afd33 -> 0xc87650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset> @ 0x363eca -> 0xc87650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset> @ 0x364393 -> 0xc87650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> > @ 0x3729a1 -> 0xc87650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> > @ 0x372e51 -> 0xc87650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> > @ 0x373225 -> 0xc87650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> > @ 0x37307c -> 0xc87650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> > @ 0x372f2a -> 0xc87650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x8eddf1 -> 0xc87650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x8ee09b -> 0xc87650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x8ee341 -> 0xc87650 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$0::hash
// 信息 ID: 37c54089e6451bf4e4b4c2c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<shader_pipeline::reflection::BindingNameHash,u32>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<shader_pipeline::reflection::BindingNameHash,u32,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > >; module=1714, inline_record=3252; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<shader_pipeline::reflection::BindingNameHash,u32>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<shader_pipeline::reflection::BindingNameHash,u32,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > >; module=1714, inline_record=5648; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > >; module=1714, inline_record=9660; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > >; module=1714, inline_record=12056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$16::eq
// 信息 ID: 041ab1045e3e8cbdd1382288
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 203 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=61, inline_record=1288; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=61, inline_record=2456; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=61, inline_record=4996; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=61, inline_record=6720; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=61, inline_record=7888; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=61, inline_record=10556; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>; module=102, inline_record=1288; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>; module=102, inline_record=2456; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$18::read_from
// 信息 ID: 65a343e47bc0736aa51ade6a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,speedy::error::Error> >,core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > >,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> >,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; module=1610, inline_record=126688; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$25::read_from
// 信息 ID: dc9e9bc37968f75ef9ba0001
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=534, inline_record=19648; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$29::read_from
// 信息 ID: 81eba9d045e780f932a1f26a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,speedy::error::Error> >,core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,std::collections::hash::map::HashMap<alloc::string::String,shader_pipeline::reflection::ShaderInput,std::hash::random::RandomState> >,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,std::collections::hash::map::HashMap<alloc::string::String,shader_pipeline::reflection::ShaderInput,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,shader_pipeline::reflection::ShaderInput,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,shader_pipeline::reflection::ShaderInput>,std::collections::hash::map::HashMap<alloc::string::String,shader_pipeline::reflection::ShaderInput,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; module=1610, inline_record=118108; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$33::fmt
// 信息 ID: 7543159e2661ce7b4f9a4e1b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$37::read_from
// 信息 ID: 87b64fb184db4055679626c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,speedy::error::Error> >,core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> >,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; module=1610, inline_record=90752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$3::calculate_name_hashes::closure$0
// 信息 ID: 48bd905cfec189debfa807f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "shader_pipeline::reflection::impl$3::calculate_name_hashes", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$3::calculate_name_hashes::closure$1
// 信息 ID: 84b51af5dea565f683001133
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "shader_pipeline::reflection::impl$3::calculate_name_hashes", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$41::fmt
// 信息 ID: b3925ffd683ed6623f9ad2f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$45::read_from
// 信息 ID: 97ca69dc373bdad97b7f45ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,speedy::error::Error> >,core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> >,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; module=1610, inline_record=90976; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$48::fmt
// 信息 ID: 18a70483a3373e2b3d1a15bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<shader_pipeline::reflection::Binding>; module=3311, inline_record=404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$52::read_from
// 信息 ID: d1d07112443cfe992e005ecf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,speedy::error::Error> >,core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> >,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; module=1610, inline_record=90688; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$56::fmt
// 信息 ID: f036cae3d01f38ff6d79840f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<shader_pipeline::reflection::BindingIndex>; module=1729, inline_record=4172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$58::eq
// 信息 ID: 569415657ed61b8b4db4e67c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 15 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> >; module=149, inline_record=21280; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> >; module=149, inline_record=22316; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> >; module=502, inline_record=24936; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> >; module=502, inline_record=26032; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_external_storage_buffer; module=503, inline_record=24992; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>; module=503, inline_record=41896; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > > >; module=576, inline_record=2616; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >; module=1610, inline_record=113552; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$60::hash
// 信息 ID: 1a53035298aec1be34d8d23d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<shader_pipeline::reflection::BindingIndex> >; module=241, inline_record=63600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$61::read_from
// 信息 ID: ad57a9f1ecdea220a7037299
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=534, inline_record=21604; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,speedy::error::Error> >,core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> >,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; module=1610, inline_record=90416; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,speedy::error::Error> >,core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,std::hash::random::RandomState> >,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; module=1610, inline_record=105068; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,speedy::error::Error> >,core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > >,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> >,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; module=1610, inline_record=127316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$74::fmt
// 信息 ID: 4bcffe5d2e3ee5aec529e69e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<shader_pipeline::reflection::SetIndex>; module=1729, inline_record=3772; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$76::eq
// 信息 ID: 8d33b5296eef8bdf2acff423
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 149 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=61, inline_record=2932; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=61, inline_record=8364; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>; module=102, inline_record=2932; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>; module=145, inline_record=2908; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> >; module=149, inline_record=20456; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo>; module=238, inline_record=5960; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo>; module=238, inline_record=7400; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo>; module=238, inline_record=8840; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$79::read_from
// 信息 ID: b68275f4a7076e6ccde538c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=534, inline_record=21344; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,speedy::error::Error> >,core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,std::hash::random::RandomState> >,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<alloc::string::String,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,std::hash::random::RandomState> > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; module=1610, inline_record=104808; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,speedy::error::Error> >,core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > >,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> >,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,speedy::reader::Reader::read_collection::closure_env$1<speedy::readable::BufferReader<speedy::context::LittleEndian>,speedy::context::LittleEndian,tuple$<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex> >,std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingNameHash,tuple$<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::BindingIndex>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<shader_pipeline::reflection::BindingNameHash> > > > >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,speedy::error::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; module=1610, inline_record=127040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$84::read_from
// 信息 ID: d3a999e226246994afd71f94
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=534, inline_record=18868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$89::read_from
// 信息 ID: 09dc629f543826a202ac81f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=534, inline_record=18176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$94::read_from
// 信息 ID: 4ed289eb57ec5d43a2470034
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=534, inline_record=18008; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$94::read_from::closure$0
// 信息 ID: fec109d54496ad97fb434711
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "shader_pipeline::reflection::impl$94::read_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=534, inline_record=19556; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$94::read_from::closure$1
// 信息 ID: 366aca8b074cf27789aae187
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "shader_pipeline::reflection::impl$94::read_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=534, inline_record=23504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::impl$94::read_from::closure$1::closure$0
// 信息 ID: 73be6448b62d99bf7232b0b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "shader_pipeline::reflection::impl$94::read_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=534, inline_record=23532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::reflection::len_without_suffix
// 信息 ID: 8ea30bae940dfaeab907f38f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::reflection::BindingNameHash::new; module=752, inline_record=540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

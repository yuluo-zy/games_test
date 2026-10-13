// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/utils/src/selective_deserialize.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d242952ca071cd6f17c66c0c87a716bc158cfc9d72e1cc2a91b3b5d84466a1d2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: selective_deserialize.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1594, 1600
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::selective_deserialize::impl$0::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11B24
// 首条位置: {"module":1594,"inlinee":72484}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::selective_deserialize::impl$0::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11BB7
// 首条位置: {"module":1600,"inlinee":72631}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1600,"record_offset":243428,"parent_record_offset":241572,"ancestor_record_offset":241572,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1600,"record_offset":243396,"parent_record_offset":241572,"ancestor_record_offset":241572,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1600,"record_offset":241624,"parent_record_offset":241572,"ancestor_record_offset":241572,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1600,"record_offset":243268,"parent_record_offset":241572,"ancestor_record_offset":241572,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1600,"record_offset":243640,"parent_record_offset":243616,"ancestor_record_offset":241572,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1600,"record_offset":243616,"parent_record_offset":243524,"ancestor_record_offset":241572,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1600,"record_offset":243552,"parent_record_offset":243524,"ancestor_record_offset":241572,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1600,"record_offset":243524,"parent_record_offset":243500,"ancestor_record_offset":241572,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1600,"record_offset":243500,"parent_record_offset":243476,"ancestor_record_offset":241572,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1600,"record_offset":242780,"parent_record_offset":242680,"ancestor_record_offset":241572,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1600,"record_offset":242832,"parent_record_offset":242808,"ancestor_record_offset":241572,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1600,"record_offset":242096,"parent_record_offset":242072,"ancestor_record_offset":241572,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1600,"record_offset":243584,"parent_record_offset":243552,"ancestor_record_offset":241572,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1600,"record_offset":243452,"parent_record_offset":243428,"ancestor_record_offset":241572,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1600,"record_offset":242904,"parent_record_offset":242880,"ancestor_record_offset":241572,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1600,"record_offset":242928,"parent_record_offset":242904,"ancestor_record_offset":241572,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1600,"record_offset":242072,"parent_record_offset":242048,"ancestor_record_offset":241572,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1600,"record_offset":242048,"parent_record_offset":242004,"ancestor_record_offset":241572,"inlinee":18027,"source_line":499,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1600,"record_offset":242856,"parent_record_offset":242832,"ancestor_record_offset":241572,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1600,"record_offset":242608,"parent_record_offset":241664,"ancestor_record_offset":241572,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1600,"record_offset":243356,"parent_record_offset":243332,"ancestor_record_offset":241572,"inlinee":6469,"source_line":1150,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":242160,"parent_record_offset":241760,"ancestor_record_offset":241572,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":241760,"parent_record_offset":241664,"ancestor_record_offset":241572,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":241812,"parent_record_offset":241760,"ancestor_record_offset":241572,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek_error
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":243120,"parent_record_offset":241664,"ancestor_record_offset":241572,"inlinee":6727,"source_line":241,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":241664,"parent_record_offset":241624,"ancestor_record_offset":241572,"inlinee":6601,"source_line":1963,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":243292,"parent_record_offset":243268,"ancestor_record_offset":241572,"inlinee":6516,"source_line":1996,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":242680,"parent_record_offset":242656,"ancestor_record_offset":241572,"inlinee":6421,"source_line":2151,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1600,"record_offset":242656,"parent_record_offset":242632,"ancestor_record_offset":241572,"inlinee":6604,"source_line":126,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::iter::impl$1::next
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/iter.rs
//     evidence={"module":1600,"record_offset":241876,"parent_record_offset":241840,"ancestor_record_offset":241572,"inlinee":26197,"source_line":54,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$4::discard
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":242184,"parent_record_offset":242160,"ancestor_record_offset":241572,"inlinee":33541,"source_line":294,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$4::peek
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":241840,"parent_record_offset":241812,"ancestor_record_offset":241572,"inlinee":33540,"source_line":278,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufreader::buffer::Buffer::buffer
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader\buffer.rs
//     evidence={"module":1600,"record_offset":242004,"parent_record_offset":241972,"ancestor_record_offset":241572,"inlinee":5255,"source_line":50,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufreader::buffer::Buffer::consume_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader\buffer.rs
//     evidence={"module":1600,"record_offset":241972,"parent_record_offset":241936,"ancestor_record_offset":241572,"inlinee":18026,"source_line":91,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufreader::impl$4::spec_read_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader.rs
//     evidence={"module":1600,"record_offset":241936,"parent_record_offset":241908,"ancestor_record_offset":241572,"inlinee":26114,"source_line":321,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impl$20::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\mod.rs
//     evidence={"module":1600,"record_offset":241908,"parent_record_offset":241876,"ancestor_record_offset":241572,"inlinee":26112,"source_line":3085,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] utils::selective_deserialize::impl$4::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/selective_deserialize.rs
//     evidence={"module":1600,"record_offset":242808,"parent_record_offset":242680,"ancestor_record_offset":241572,"inlinee":72503,"source_line":139,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] utils::selective_deserialize::impl$5::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/selective_deserialize.rs
//     evidence={"module":1600,"record_offset":242632,"parent_record_offset":242608,"ancestor_record_offset":241572,"inlinee":72633,"source_line":156,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::selective_deserialize::impl$1::deserialize<alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global>,country_core::systems::save::snapshot_gc::SnapshotsKey,ref_mut$<serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1594,"record_offset":1488,"rva":20859280,"va":5389568400}
// 编译实例 RVAs: 0x13e4990
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: selective_deserialize.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14667818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389568400,"raw_hex":"e8d62f20ff","size":5,"soft_fail":false,"target_va":5374900736,"va":5389568549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: selective_deserialize.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14487036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389568400,"raw_hex":"e804f222ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5389568535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414186d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: selective_deserialize.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t212203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389568400,"raw_hex":"e8eb3c0300","size":5,"soft_fail":false,"target_va":5389780688,"va":5389568480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1594,"record_offset":1904,"parent_record_offset":1488,"ancestor_record_offset":1488,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1594,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":1488,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1594,"record_offset":2068,"parent_record_offset":2040,"ancestor_record_offset":1488,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1594,"record_offset":2040,"parent_record_offset":2016,"ancestor_record_offset":1488,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1594,"record_offset":2444,"parent_record_offset":2416,"ancestor_record_offset":1488,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1594,"record_offset":2416,"parent_record_offset":2392,"ancestor_record_offset":1488,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1594,"record_offset":2392,"parent_record_offset":2368,"ancestor_record_offset":1488,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1594,"record_offset":2468,"parent_record_offset":2444,"ancestor_record_offset":1488,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1594,"record_offset":1928,"parent_record_offset":1904,"ancestor_record_offset":1488,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01594.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::selective_deserialize::impl$1::deserialize<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >,country_core::systems::save::snapshot_gc::HistoryKey,ref_mut$<serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1594,"record_offset":260,"rva":20858928,"va":5389568048}
// 编译实例 RVAs: 0x13e4830
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: selective_deserialize.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14667466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389568048,"raw_hex":"e8363120ff","size":5,"soft_fail":false,"target_va":5374900736,"va":5389568197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: selective_deserialize.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14486684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389568048,"raw_hex":"e864f322ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5389568183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14142ce00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: selective_deserialize.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t296315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389568048,"raw_hex":"e87b850400","size":5,"soft_fail":false,"target_va":5389864448,"va":5389568128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1594,"record_offset":784,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1594,"record_offset":972,"parent_record_offset":948,"ancestor_record_offset":260,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1594,"record_offset":948,"parent_record_offset":920,"ancestor_record_offset":260,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1594,"record_offset":920,"parent_record_offset":896,"ancestor_record_offset":260,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1594,"record_offset":1348,"parent_record_offset":1320,"ancestor_record_offset":260,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1594,"record_offset":1320,"parent_record_offset":1296,"ancestor_record_offset":260,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1594,"record_offset":1296,"parent_record_offset":1272,"ancestor_record_offset":260,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1594,"record_offset":1372,"parent_record_offset":1348,"ancestor_record_offset":260,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1594,"record_offset":808,"parent_record_offset":784,"ancestor_record_offset":260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01594.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::selective_deserialize::impl$4::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 139
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11B37
// 首条位置: {"module":1600,"inlinee":72503}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1600,"record_offset":242832,"parent_record_offset":242808,"ancestor_record_offset":242808,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1600,"record_offset":242880,"parent_record_offset":242856,"ancestor_record_offset":242808,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1600,"record_offset":242904,"parent_record_offset":242880,"ancestor_record_offset":242808,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1600,"record_offset":242928,"parent_record_offset":242904,"ancestor_record_offset":242808,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1600,"record_offset":242856,"parent_record_offset":242832,"ancestor_record_offset":242808,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::selective_deserialize::impl$5::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 156
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11BB9
// 首条位置: {"module":1600,"inlinee":72633}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1600,"record_offset":242656,"parent_record_offset":242632,"ancestor_record_offset":242632,"inlinee":6604,"source_line":126,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1600,"record_offset":242780,"parent_record_offset":242680,"ancestor_record_offset":242632,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1600,"record_offset":242832,"parent_record_offset":242808,"ancestor_record_offset":242632,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1600,"record_offset":242904,"parent_record_offset":242880,"ancestor_record_offset":242632,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1600,"record_offset":242928,"parent_record_offset":242904,"ancestor_record_offset":242632,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1600,"record_offset":242856,"parent_record_offset":242832,"ancestor_record_offset":242632,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":242724,"parent_record_offset":242680,"ancestor_record_offset":242632,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":242680,"parent_record_offset":242656,"ancestor_record_offset":242632,"inlinee":6421,"source_line":2151,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$4::discard
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":242748,"parent_record_offset":242724,"ancestor_record_offset":242632,"inlinee":33541,"source_line":294,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] utils::selective_deserialize::impl$4::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/selective_deserialize.rs
//     evidence={"module":1600,"record_offset":242808,"parent_record_offset":242680,"ancestor_record_offset":242632,"inlinee":72503,"source_line":139,"archive":"_pdb/B/modules/01600.jsonl.gz"}
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
// 完整结构及原始类型记录: selective_deserialize.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::selective_deserialize::PartialVisitor
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >", "constructor": "utils::selective_deserialize::PartialVisitor", "arguments": ["utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impl$1::fmt<utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >", "constructor": "serde::de::impl$1::fmt", "arguments": ["utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >", "constructor": "serde_json::de::impl$7::deserialize_struct", "arguments": ["serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >", "utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >", "constructor": "utils::selective_deserialize::PartialVisitor", "arguments": ["utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > > >", "constructor": "serde_json::de::impl$7::deserialize_struct", "arguments": ["serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >", "utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::selective_deserialize::SelectiveDeserialize
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >", "constructor": "utils::selective_deserialize::SelectiveDeserialize", "arguments": ["country_core::systems::save::snapshot_gc::SnapshotsKey", "alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::selective_deserialize::impl$1::deserialize<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >,country_core::systems::save::snapshot_gc::HistoryKey,ref_mut$<serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > > > >", "constructor": "utils::selective_deserialize::impl$1::deserialize", "arguments": ["utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >", "country_core::systems::save::snapshot_gc::HistoryKey", "ref_mut$<serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >", "constructor": "utils::selective_deserialize::PartialVisitor", "arguments": ["utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impl$1::fmt<utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >", "constructor": "serde::de::impl$1::fmt", "arguments": ["utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >", "constructor": "serde_json::de::impl$7::deserialize_struct", "arguments": ["serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >", "utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >", "constructor": "utils::selective_deserialize::SelectiveDeserialize", "arguments": ["country_core::systems::save::snapshot_gc::HistoryKey", "utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >", "constructor": "utils::selective_deserialize::PartialVisitor", "arguments": ["utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > > >", "constructor": "serde_json::de::impl$7::deserialize_struct", "arguments": ["serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >", "utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: selective_deserialize.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: selective_deserialize.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::selective_deserialize::impl$0::expecting
// 信息 ID: 916f37b8853ec9b5e47f50a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >; module=1594, inline_record=2944; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::selective_deserialize::impl$0::visit_map
// 信息 ID: d944ade500c935b2edf87002
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >; module=1600, inline_record=241572; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > > >; module=1600, inline_record=387732; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::selective_deserialize::impl$1::deserialize<alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global>,country_core::systems::save::snapshot_gc::SnapshotsKey,ref_mut$<serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > > > >
// 信息 ID: 3c125786038ddc0ae9bdba95
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["country_core::resources::history::ResolvedHistoryChainSnapshot", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] std::io::buffered::bufreader::BufReader -> ["std::fs::File"]; 字段布局 = unknown。
// [类型关系] serde_json::read::IoRead -> ["std::io::buffered::bufreader::BufReader<std::fs::File>"]; 字段布局 = unknown。
// [类型关系] serde_json::de::Deserializer -> ["serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > >"]; 字段布局 = unknown。
// [类型关系] utils::selective_deserialize::impl$1::deserialize -> ["alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global>", "country_core::systems::save::snapshot_gc::SnapshotsKey", "ref_mut$<serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > > > @ 0x142d0fe -> 0x13e4990 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::selective_deserialize::impl$1::deserialize<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >,country_core::systems::save::snapshot_gc::HistoryKey,ref_mut$<serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > > > >
// 信息 ID: 0f880c4cdc145066467120f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["country_core::resources::history::ResolvedHistoryChainSnapshot", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] utils::selective_deserialize::SelectiveDeserialize -> ["country_core::systems::save::snapshot_gc::SnapshotsKey", "alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] std::io::buffered::bufreader::BufReader -> ["std::fs::File"]; 字段布局 = unknown。
// [类型关系] serde_json::read::IoRead -> ["std::io::buffered::bufreader::BufReader<std::fs::File>"]; 字段布局 = unknown。
// [类型关系] serde_json::de::Deserializer -> ["serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > >"]; 字段布局 = unknown。
// [类型关系] utils::selective_deserialize::impl$1::deserialize -> ["utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >", "country_core::systems::save::snapshot_gc::HistoryKey", "ref_mut$<serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::snapshot_gc::parse_save_file_snapshots<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x128679a -> 0x13e4830 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::selective_deserialize::impl$4::visit_str
// 信息 ID: 91938ccc9738b3f1c74bbdd5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >; module=1600, inline_record=242808; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > > >; module=1600, inline_record=388968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::selective_deserialize::impl$5::deserialize
// 信息 ID: 2457dc0a23dec9e3b8b12e2c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >; module=1600, inline_record=242632; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > > >; module=1600, inline_record=388792; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

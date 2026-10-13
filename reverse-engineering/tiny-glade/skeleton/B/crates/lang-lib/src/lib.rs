// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/lang-lib/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: cff24a38eaade08ea3e1a91b269be34a3f4d1f84f6725b806d01648fbb5ea80e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 636, 1825, 1826, 1829
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use arc_swap; // namespace candidate
//   package identities: [{"name":"arc-swap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.7.1"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use hashlink; // namespace candidate
//   package identities: [{"name":"hashlink","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.4"}]
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use paths; // namespace candidate
//   package identities: [{"name":"paths","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use yaml_rust2; // namespace candidate
//   package identities: [{"name":"yaml-rust2","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: lang_lib::language_library::LanguageLibrary::try_from_yml
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1825,"record_offset":6308,"rva":25462640,"va":5394171760}
// 编译实例 RVAs: 0x1848770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..collections..hash..map..HashMap$LT$K$C$V$C$S$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$$LP$K$C$V$RP$$GT$$GT$::from_iter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9337953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394171760,"raw_hex":"e8617c8e00","size":5,"soft_fail":false,"target_va":5403510144,"va":5394172186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19090581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394171760,"raw_hex":"e86bb3dcfe","size":5,"soft_fail":false,"target_va":5375081504,"va":5394172080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t17623571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394171760,"raw_hex":"e813ea0c01","size":5,"soft_fail":false,"target_va":5411795424,"va":5394171848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17369365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394171760,"raw_hex":"e815090901","size":5,"soft_fail":false,"target_va":5411541776,"va":5394172406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$yaml_rust2..yaml..Yaml$GT$$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394171760,"raw_hex":"e861110000","size":5,"soft_fail":false,"target_va":5394176800,"va":5394172346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11466350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394171760,"raw_hex":"e8920951ff","size":5,"soft_fail":false,"target_va":5382705472,"va":5394171817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11514286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394171760,"raw_hex":"e8524e50ff","size":5,"soft_fail":false,"target_va":5382657856,"va":5394172137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11418467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394171760,"raw_hex":"e89dc451ff","size":5,"soft_fail":false,"target_va":5382753792,"va":5394172254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] yaml_rust2::yaml::YamlLoader::load_from_str
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"yaml-rust2","version":"0.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9335684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394171760,"raw_hex":"e884738e00","size":5,"soft_fail":false,"target_va":5403507584,"va":5394171895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1825,"record_offset":6620,"parent_record_offset":6308,"ancestor_record_offset":6308,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1825,"record_offset":8004,"parent_record_offset":6308,"ancestor_record_offset":6308,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1825,"record_offset":7948,"parent_record_offset":6308,"ancestor_record_offset":6308,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1825,"record_offset":6676,"parent_record_offset":6308,"ancestor_record_offset":6308,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1825,"record_offset":6772,"parent_record_offset":6308,"ancestor_record_offset":6308,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1825,"record_offset":6524,"parent_record_offset":6308,"ancestor_record_offset":6308,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1825,"record_offset":6556,"parent_record_offset":6308,"ancestor_record_offset":6308,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_direct] hashlink::linked_hash_map::LinkedHashMap::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashlink","version":"0.8.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashlink-0.8.4/src/linked_hash_map.rs
//     evidence={"module":1825,"record_offset":6712,"parent_record_offset":6308,"ancestor_record_offset":6308,"inlinee":81984,"source_line":134,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_direct] lang_lib::language_library::LanguageLibrary::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/lang-lib/src/language_library.rs
//     evidence={"module":1825,"record_offset":8032,"parent_record_offset":6308,"ancestor_record_offset":6308,"inlinee":81985,"source_line":12,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1825,"record_offset":7228,"parent_record_offset":6308,"ancestor_record_offset":6308,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1825,"record_offset":6496,"parent_record_offset":6308,"ancestor_record_offset":6308,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_direct] yaml_rust2::yaml::Yaml::as_hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"yaml-rust2","version":"0.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/yaml-rust2-0.8.0/src/yaml.rs
//     evidence={"module":1825,"record_offset":6648,"parent_record_offset":6308,"ancestor_record_offset":6308,"inlinee":81982,"source_line":472,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1825,"record_offset":8132,"parent_record_offset":8108,"ancestor_record_offset":6308,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1825,"record_offset":8156,"parent_record_offset":8132,"ancestor_record_offset":6308,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1825,"record_offset":6948,"parent_record_offset":6924,"ancestor_record_offset":6308,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1825,"record_offset":8084,"parent_record_offset":8060,"ancestor_record_offset":6308,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1825,"record_offset":8108,"parent_record_offset":8084,"ancestor_record_offset":6308,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1825,"record_offset":6924,"parent_record_offset":6868,"ancestor_record_offset":6308,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1825,"record_offset":8060,"parent_record_offset":8032,"ancestor_record_offset":6308,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1825,"record_offset":6896,"parent_record_offset":6868,"ancestor_record_offset":6308,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1825,"record_offset":6868,"parent_record_offset":6844,"ancestor_record_offset":6308,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1825,"record_offset":6844,"parent_record_offset":6820,"ancestor_record_offset":6308,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1825,"record_offset":7776,"parent_record_offset":7744,"ancestor_record_offset":6308,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1825,"record_offset":7836,"parent_record_offset":7804,"ancestor_record_offset":6308,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1825,"record_offset":7972,"parent_record_offset":7948,"ancestor_record_offset":6308,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1825,"record_offset":7632,"parent_record_offset":7608,"ancestor_record_offset":6308,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1825,"record_offset":7804,"parent_record_offset":7744,"ancestor_record_offset":6308,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1825,"record_offset":7380,"parent_record_offset":7340,"ancestor_record_offset":6308,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1825,"record_offset":7496,"parent_record_offset":7468,"ancestor_record_offset":6308,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1825,"record_offset":6796,"parent_record_offset":6772,"ancestor_record_offset":6308,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1825,"record_offset":7584,"parent_record_offset":7468,"ancestor_record_offset":6308,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1825,"record_offset":7660,"parent_record_offset":7632,"ancestor_record_offset":6308,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1825,"record_offset":8180,"parent_record_offset":8156,"ancestor_record_offset":6308,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1825,"record_offset":7608,"parent_record_offset":7584,"ancestor_record_offset":6308,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1825,"record_offset":7688,"parent_record_offset":7632,"ancestor_record_offset":6308,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1825,"record_offset":7284,"parent_record_offset":7256,"ancestor_record_offset":6308,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1825,"record_offset":7544,"parent_record_offset":7520,"ancestor_record_offset":6308,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1825,"record_offset":7408,"parent_record_offset":7380,"ancestor_record_offset":6308,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1825,"record_offset":7744,"parent_record_offset":7340,"ancestor_record_offset":6308,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1825,"record_offset":7256,"parent_record_offset":7228,"ancestor_record_offset":6308,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1825,"record_offset":7436,"parent_record_offset":7408,"ancestor_record_offset":6308,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1825,"record_offset":7468,"parent_record_offset":7436,"ancestor_record_offset":6308,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1825,"record_offset":7340,"parent_record_offset":7312,"ancestor_record_offset":6308,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1825,"record_offset":7312,"parent_record_offset":7284,"ancestor_record_offset":6308,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::lang_pack_path
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8E18
// 首条位置: {"module":636,"inlinee":36376}
// 编译实例 RVAs: 0xb9c200
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5801612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380882944,"raw_hex":"e87479a7ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380883111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1957154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380882944,"raw_hex":"e822dd1d00","size":5,"soft_fail":false,"target_va":5382840208,"va":5380883049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1860988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380882944,"raw_hex":"e87c651c00","size":5,"soft_fail":false,"target_va":5382744080,"va":5380883087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":636,"record_offset":368,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":636,"record_offset":448,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":636,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":256,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":636,"record_offset":636,"parent_record_offset":580,"ancestor_record_offset":256,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":636,"record_offset":416,"parent_record_offset":392,"ancestor_record_offset":256,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":636,"record_offset":608,"parent_record_offset":580,"ancestor_record_offset":256,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":636,"record_offset":580,"parent_record_offset":556,"ancestor_record_offset":256,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":636,"record_offset":556,"parent_record_offset":532,"ancestor_record_offset":256,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":636,"record_offset":392,"parent_record_offset":368,"ancestor_record_offset":256,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":636,"record_offset":484,"parent_record_offset":448,"ancestor_record_offset":256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::load_locale
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":636,"record_offset":948,"rva":12174080,"va":5380883200}
// 编译实例 RVAs: 0xb9c300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5801830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883200,"raw_hex":"e89a78a7ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380883329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1944435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883200,"raw_hex":"e873ab1d00","size":5,"soft_fail":false,"target_va":5382827728,"va":5380883288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::load_locale_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883200,"raw_hex":"e87e010000","size":5,"soft_fail":false,"target_va":5380883616,"va":5380883229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::standard_app_root_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883200,"raw_hex":"e81d190000","size":5,"soft_fail":false,"target_va":5380889680,"va":5380883246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1822917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883200,"raw_hex":"e8c5d01b00","size":5,"soft_fail":false,"target_va":5382706224,"va":5380883302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1860795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883200,"raw_hex":"e8bb641c00","size":5,"soft_fail":false,"target_va":5382744080,"va":5380883280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5380883200,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5380883382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":636,"record_offset":1900,"parent_record_offset":948,"ancestor_record_offset":948,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":636,"record_offset":1092,"parent_record_offset":948,"ancestor_record_offset":948,"inlinee":9318,"source_line":347,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":636,"record_offset":1064,"parent_record_offset":948,"ancestor_record_offset":948,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":636,"record_offset":1520,"parent_record_offset":1496,"ancestor_record_offset":948,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":636,"record_offset":1496,"parent_record_offset":1440,"ancestor_record_offset":948,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":636,"record_offset":2236,"parent_record_offset":2208,"ancestor_record_offset":948,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":636,"record_offset":1468,"parent_record_offset":1440,"ancestor_record_offset":948,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":636,"record_offset":1440,"parent_record_offset":1416,"ancestor_record_offset":948,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":636,"record_offset":1416,"parent_record_offset":1396,"ancestor_record_offset":948,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":636,"record_offset":2264,"parent_record_offset":2236,"ancestor_record_offset":948,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":636,"record_offset":2288,"parent_record_offset":2264,"ancestor_record_offset":948,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":636,"record_offset":1296,"parent_record_offset":1092,"ancestor_record_offset":948,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":636,"record_offset":2036,"parent_record_offset":2012,"ancestor_record_offset":948,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":636,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":948,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":636,"record_offset":2060,"parent_record_offset":2036,"ancestor_record_offset":948,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":636,"record_offset":1200,"parent_record_offset":1176,"ancestor_record_offset":948,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":636,"record_offset":1248,"parent_record_offset":1224,"ancestor_record_offset":948,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":636,"record_offset":1224,"parent_record_offset":1200,"ancestor_record_offset":948,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":636,"record_offset":2012,"parent_record_offset":1984,"ancestor_record_offset":948,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":636,"record_offset":1984,"parent_record_offset":1956,"ancestor_record_offset":948,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":636,"record_offset":1176,"parent_record_offset":1152,"ancestor_record_offset":948,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":636,"record_offset":1152,"parent_record_offset":1128,"ancestor_record_offset":948,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":636,"record_offset":1128,"parent_record_offset":1092,"ancestor_record_offset":948,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::load_locale_impl
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":636,"record_offset":2728,"rva":12174496,"va":5380883616}
// 编译实例 RVAs: 0xb9c4a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1760194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883616,"raw_hex":"e8c2db1a00","size":5,"soft_fail":false,"target_va":5382644528,"va":5380884329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004fce0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11850587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883616,"raw_hex":"e8a52c4bff","size":5,"soft_fail":false,"target_va":5369036000,"va":5380886582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5802425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883616,"raw_hex":"e84776a7ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380883908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=15
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5802358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883616,"raw_hex":"e88a76a7ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380883857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5368124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883616,"raw_hex":"e8c416aeff","size":5,"soft_fail":false,"target_va":5375516032,"va":5380884151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1943857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883616,"raw_hex":"e831a91d00","size":5,"soft_fail":false,"target_va":5382827728,"va":5380883866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141848b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13288093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883616,"raw_hex":"e89dc2ca00","size":5,"soft_fail":false,"target_va":5394172752,"va":5380884654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30963221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883616,"raw_hex":"e81576d801","size":5,"soft_fail":false,"target_va":5411847168,"va":5380883942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30655052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883616,"raw_hex":"e84cc2d301","size":5,"soft_fail":false,"target_va":5411540992,"va":5380885935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t1956421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883616,"raw_hex":"e845da1d00","size":5,"soft_fail":false,"target_va":5382840208,"va":5380883782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t30655043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883616,"raw_hex":"e843c2d301","size":5,"soft_fail":false,"target_va":5411540963,"va":5380885915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t13289373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883616,"raw_hex":"e89dc7ca00","size":5,"soft_fail":false,"target_va":5394174448,"va":5380885070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1770270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883616,"raw_hex":"e81e031b00","size":5,"soft_fail":false,"target_va":5382655024,"va":5380884749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13289139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883616,"raw_hex":"e8b3c6ca00","size":5,"soft_fail":false,"target_va":5394174176,"va":5380885032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30655811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883616,"raw_hex":"e843c5d301","size":5,"soft_fail":false,"target_va":5411541904,"va":5380886088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::language_library::LanguageLibrary::try_from_yml
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13287767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883616,"raw_hex":"e857c1ca00","size":5,"soft_fail":false,"target_va":5394171760,"va":5380883988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::_var
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1814212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883616,"raw_hex":"e8c4ae1b00","size":5,"soft_fail":false,"target_va":5382699312,"va":5380885095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t1860245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380883616,"raw_hex":"e895621c00","size":5,"soft_fail":false,"target_va":5382744080,"va":5380883830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":636,"record_offset":6660,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::swap
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":636,"record_offset":7064,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":16235,"source_line":466,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":636,"record_offset":4464,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":636,"record_offset":3632,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":636,"record_offset":6008,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":636,"record_offset":9348,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":636,"record_offset":5976,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] lang_lib::lang_pack_path
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/lang-lib/src/lib.rs
//     evidence={"module":636,"record_offset":2884,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":36376,"source_line":17,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":636,"record_offset":4496,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":636,"record_offset":4380,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] std::env::var
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\env.rs
//     evidence={"module":636,"record_offset":7420,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":17230,"source_line":217,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":636,"record_offset":3492,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":636,"record_offset":4216,"parent_record_offset":4192,"ancestor_record_offset":2728,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":636,"record_offset":4240,"parent_record_offset":4216,"ancestor_record_offset":2728,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":636,"record_offset":3204,"parent_record_offset":3180,"ancestor_record_offset":2728,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":636,"record_offset":6732,"parent_record_offset":6696,"ancestor_record_offset":2728,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":636,"record_offset":4192,"parent_record_offset":4012,"ancestor_record_offset":2728,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":636,"record_offset":3180,"parent_record_offset":3124,"ancestor_record_offset":2728,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":636,"record_offset":6696,"parent_record_offset":6660,"ancestor_record_offset":2728,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":636,"record_offset":2912,"parent_record_offset":2884,"ancestor_record_offset":2728,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":636,"record_offset":2960,"parent_record_offset":2936,"ancestor_record_offset":2728,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":636,"record_offset":3944,"parent_record_offset":3912,"ancestor_record_offset":2728,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":636,"record_offset":3152,"parent_record_offset":3124,"ancestor_record_offset":2728,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":636,"record_offset":3124,"parent_record_offset":3100,"ancestor_record_offset":2728,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":636,"record_offset":4012,"parent_record_offset":3976,"ancestor_record_offset":2728,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":636,"record_offset":3976,"parent_record_offset":3944,"ancestor_record_offset":2728,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":636,"record_offset":3100,"parent_record_offset":3076,"ancestor_record_offset":2728,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":636,"record_offset":4052,"parent_record_offset":4012,"ancestor_record_offset":2728,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":636,"record_offset":3872,"parent_record_offset":3840,"ancestor_record_offset":2728,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":636,"record_offset":3840,"parent_record_offset":3808,"ancestor_record_offset":2728,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":636,"record_offset":3776,"parent_record_offset":3728,"ancestor_record_offset":2728,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":636,"record_offset":3808,"parent_record_offset":3776,"ancestor_record_offset":2728,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":636,"record_offset":3728,"parent_record_offset":3696,"ancestor_record_offset":2728,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":636,"record_offset":3696,"parent_record_offset":3664,"ancestor_record_offset":2728,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":636,"record_offset":3664,"parent_record_offset":3632,"ancestor_record_offset":2728,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":636,"record_offset":7288,"parent_record_offset":7264,"ancestor_record_offset":2728,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":636,"record_offset":3912,"parent_record_offset":3872,"ancestor_record_offset":2728,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay_all
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":636,"record_offset":7176,"parent_record_offset":7152,"ancestor_record_offset":2728,"inlinee":7948,"source_line":81,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$2::from_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":636,"record_offset":7208,"parent_record_offset":7064,"ancestor_record_offset":2728,"inlinee":7998,"source_line":169,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::wait_for_readers
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":636,"record_offset":7152,"parent_record_offset":7064,"ancestor_record_offset":2728,"inlinee":8000,"source_line":196,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":636,"record_offset":4076,"parent_record_offset":4052,"ancestor_record_offset":2728,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":636,"record_offset":4100,"parent_record_offset":4076,"ancestor_record_offset":2728,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":636,"record_offset":9420,"parent_record_offset":9388,"ancestor_record_offset":2728,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":636,"record_offset":4316,"parent_record_offset":4292,"ancestor_record_offset":2728,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":636,"record_offset":4148,"parent_record_offset":4124,"ancestor_record_offset":2728,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":636,"record_offset":4124,"parent_record_offset":4100,"ancestor_record_offset":2728,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":636,"record_offset":2936,"parent_record_offset":2912,"ancestor_record_offset":2728,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":636,"record_offset":4552,"parent_record_offset":4520,"ancestor_record_offset":2728,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":636,"record_offset":4580,"parent_record_offset":4520,"ancestor_record_offset":2728,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":636,"record_offset":4292,"parent_record_offset":3872,"ancestor_record_offset":2728,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=80
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":636,"record_offset":3028,"parent_record_offset":2992,"ancestor_record_offset":2728,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":636,"record_offset":6840,"parent_record_offset":6812,"ancestor_record_offset":2728,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":636,"record_offset":7096,"parent_record_offset":7064,"ancestor_record_offset":2728,"inlinee":11099,"source_line":1628,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":636,"record_offset":7324,"parent_record_offset":7288,"ancestor_record_offset":2728,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":636,"record_offset":4404,"parent_record_offset":4380,"ancestor_record_offset":2728,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":636,"record_offset":4428,"parent_record_offset":4404,"ancestor_record_offset":2728,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":636,"record_offset":7348,"parent_record_offset":7324,"ancestor_record_offset":2728,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":636,"record_offset":7120,"parent_record_offset":7096,"ancestor_record_offset":2728,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":636,"record_offset":7380,"parent_record_offset":7288,"ancestor_record_offset":2728,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":636,"record_offset":7004,"parent_record_offset":6980,"ancestor_record_offset":2728,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] lang_lib::load_locale_impl::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/lang-lib/src/lib.rs
//     evidence={"module":636,"record_offset":9388,"parent_record_offset":9348,"ancestor_record_offset":2728,"inlinee":36378,"source_line":58,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":636,"record_offset":4608,"parent_record_offset":4520,"ancestor_record_offset":2728,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":636,"record_offset":4520,"parent_record_offset":4496,"ancestor_record_offset":2728,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":636,"record_offset":4632,"parent_record_offset":4608,"ancestor_record_offset":2728,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":636,"record_offset":3540,"parent_record_offset":3516,"ancestor_record_offset":2728,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":636,"record_offset":3588,"parent_record_offset":3564,"ancestor_record_offset":2728,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":636,"record_offset":3564,"parent_record_offset":3540,"ancestor_record_offset":2728,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":636,"record_offset":2992,"parent_record_offset":2884,"ancestor_record_offset":2728,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":636,"record_offset":3516,"parent_record_offset":3492,"ancestor_record_offset":2728,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::load_locale_impl::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14048
// 首条位置: {"module":1826,"inlinee":81992}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1826,"record_offset":540,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01826.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1826,"record_offset":620,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01826.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1826,"record_offset":868,"parent_record_offset":844,"ancestor_record_offset":508,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01826.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1826,"record_offset":844,"parent_record_offset":788,"ancestor_record_offset":508,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01826.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1826,"record_offset":588,"parent_record_offset":564,"ancestor_record_offset":508,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01826.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1826,"record_offset":816,"parent_record_offset":788,"ancestor_record_offset":508,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01826.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1826,"record_offset":788,"parent_record_offset":764,"ancestor_record_offset":508,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01826.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1826,"record_offset":764,"parent_record_offset":740,"ancestor_record_offset":508,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01826.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1826,"record_offset":564,"parent_record_offset":540,"ancestor_record_offset":508,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01826.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1826,"record_offset":644,"parent_record_offset":620,"ancestor_record_offset":508,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01826.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::load_locale_impl::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8E1A
// 首条位置: {"module":636,"inlinee":36378}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":636,"record_offset":9420,"parent_record_offset":9388,"ancestor_record_offset":9388,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":636,"record_offset":9448,"parent_record_offset":9388,"ancestor_record_offset":9388,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::load_last_used_locale
// 记录形式: compiled_procedure; instances=1
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":636,"record_offset":12116,"rva":12177792,"va":5380886912}
// 编译实例 RVAs: 0xb9d180
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5805752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380886912,"raw_hex":"e84869a7ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380887235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=11
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5805576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380886912,"raw_hex":"e8f869a7ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380887075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1940727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380886912,"raw_hex":"e8f79c1d00","size":5,"soft_fail":false,"target_va":5382827728,"va":5380886996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30653562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380886912,"raw_hex":"e87abcd301","size":5,"soft_fail":false,"target_va":5411540963,"va":5380887396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1768056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380886912,"raw_hex":"e878fa1a00","size":5,"soft_fail":false,"target_va":5382655024,"va":5380886963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::load_locale_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380886912,"raw_hex":"e8abf1ffff","size":5,"soft_fail":false,"target_va":5380883616,"va":5380887280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::find_standard_app_root_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380886912,"raw_hex":"e8dd0a0000","size":5,"soft_fail":false,"target_va":5380889728,"va":5380886942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1818416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380886912,"raw_hex":"e830bf1b00","size":5,"soft_fail":false,"target_va":5382705472,"va":5380887051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1857048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380886912,"raw_hex":"e818561c00","size":5,"soft_fail":false,"target_va":5382744080,"va":5380887027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5380886912,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5380887146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":636,"record_offset":15208,"parent_record_offset":12116,"ancestor_record_offset":12116,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":636,"record_offset":12324,"parent_record_offset":12116,"ancestor_record_offset":12116,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":636,"record_offset":15680,"parent_record_offset":12116,"ancestor_record_offset":12116,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":636,"record_offset":12244,"parent_record_offset":12116,"ancestor_record_offset":12116,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":636,"record_offset":15524,"parent_record_offset":15500,"ancestor_record_offset":12116,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":636,"record_offset":15548,"parent_record_offset":15524,"ancestor_record_offset":12116,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":636,"record_offset":13008,"parent_record_offset":12984,"ancestor_record_offset":12116,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":636,"record_offset":15500,"parent_record_offset":15472,"ancestor_record_offset":12116,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":636,"record_offset":12984,"parent_record_offset":12928,"ancestor_record_offset":12116,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":636,"record_offset":13772,"parent_record_offset":13744,"ancestor_record_offset":12116,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":636,"record_offset":15412,"parent_record_offset":15384,"ancestor_record_offset":12116,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":636,"record_offset":12956,"parent_record_offset":12928,"ancestor_record_offset":12116,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":636,"record_offset":12928,"parent_record_offset":12904,"ancestor_record_offset":12116,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":636,"record_offset":15472,"parent_record_offset":15440,"ancestor_record_offset":12116,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":636,"record_offset":15440,"parent_record_offset":15412,"ancestor_record_offset":12116,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":636,"record_offset":12904,"parent_record_offset":12880,"ancestor_record_offset":12116,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":636,"record_offset":15348,"parent_record_offset":15320,"ancestor_record_offset":12116,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":636,"record_offset":15320,"parent_record_offset":15292,"ancestor_record_offset":12116,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":636,"record_offset":15264,"parent_record_offset":15236,"ancestor_record_offset":12116,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":636,"record_offset":15292,"parent_record_offset":15264,"ancestor_record_offset":12116,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":636,"record_offset":15236,"parent_record_offset":15208,"ancestor_record_offset":12116,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":636,"record_offset":15384,"parent_record_offset":15348,"ancestor_record_offset":12116,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":636,"record_offset":13800,"parent_record_offset":13772,"ancestor_record_offset":12116,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":636,"record_offset":15624,"parent_record_offset":15600,"ancestor_record_offset":12116,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":636,"record_offset":13824,"parent_record_offset":13800,"ancestor_record_offset":12116,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":636,"record_offset":15600,"parent_record_offset":15348,"ancestor_record_offset":12116,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=43
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":636,"record_offset":12272,"parent_record_offset":12244,"ancestor_record_offset":12116,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":636,"record_offset":13572,"parent_record_offset":13544,"ancestor_record_offset":12116,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":636,"record_offset":13620,"parent_record_offset":13596,"ancestor_record_offset":12116,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":636,"record_offset":13596,"parent_record_offset":13572,"ancestor_record_offset":12116,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":636,"record_offset":13388,"parent_record_offset":12356,"ancestor_record_offset":12116,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] lang_lib::load_last_used_locale::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/lang-lib/src/lib.rs
//     evidence={"module":636,"record_offset":12356,"parent_record_offset":12324,"ancestor_record_offset":12116,"inlinee":36380,"source_line":72,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":636,"record_offset":12436,"parent_record_offset":12412,"ancestor_record_offset":12116,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":636,"record_offset":12484,"parent_record_offset":12460,"ancestor_record_offset":12116,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":636,"record_offset":12460,"parent_record_offset":12436,"ancestor_record_offset":12116,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":636,"record_offset":12560,"parent_record_offset":12356,"ancestor_record_offset":12116,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":636,"record_offset":13544,"parent_record_offset":13516,"ancestor_record_offset":12116,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":636,"record_offset":13516,"parent_record_offset":13488,"ancestor_record_offset":12116,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":636,"record_offset":12528,"parent_record_offset":12356,"ancestor_record_offset":12116,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":636,"record_offset":12412,"parent_record_offset":12388,"ancestor_record_offset":12116,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":636,"record_offset":12388,"parent_record_offset":12356,"ancestor_record_offset":12116,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":636,"record_offset":12592,"parent_record_offset":12560,"ancestor_record_offset":12116,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::load_last_used_locale::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8E1C
// 首条位置: {"module":636,"inlinee":36380}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":636,"record_offset":14264,"parent_record_offset":12356,"ancestor_record_offset":12356,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":636,"record_offset":13388,"parent_record_offset":12356,"ancestor_record_offset":12356,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":636,"record_offset":12560,"parent_record_offset":12356,"ancestor_record_offset":12356,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":636,"record_offset":12528,"parent_record_offset":12356,"ancestor_record_offset":12356,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":636,"record_offset":12388,"parent_record_offset":12356,"ancestor_record_offset":12356,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":636,"record_offset":13008,"parent_record_offset":12984,"ancestor_record_offset":12356,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":636,"record_offset":12984,"parent_record_offset":12928,"ancestor_record_offset":12356,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":636,"record_offset":13772,"parent_record_offset":13744,"ancestor_record_offset":12356,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":636,"record_offset":12956,"parent_record_offset":12928,"ancestor_record_offset":12356,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":636,"record_offset":12928,"parent_record_offset":12904,"ancestor_record_offset":12356,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":636,"record_offset":12904,"parent_record_offset":12880,"ancestor_record_offset":12356,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":636,"record_offset":13800,"parent_record_offset":13772,"ancestor_record_offset":12356,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":636,"record_offset":13824,"parent_record_offset":13800,"ancestor_record_offset":12356,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":636,"record_offset":12760,"parent_record_offset":12560,"ancestor_record_offset":12356,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":636,"record_offset":13572,"parent_record_offset":13544,"ancestor_record_offset":12356,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":636,"record_offset":13620,"parent_record_offset":13596,"ancestor_record_offset":12356,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":636,"record_offset":13596,"parent_record_offset":13572,"ancestor_record_offset":12356,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":636,"record_offset":12436,"parent_record_offset":12412,"ancestor_record_offset":12356,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":636,"record_offset":12484,"parent_record_offset":12460,"ancestor_record_offset":12356,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":636,"record_offset":12460,"parent_record_offset":12436,"ancestor_record_offset":12356,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":636,"record_offset":13544,"parent_record_offset":13516,"ancestor_record_offset":12356,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":636,"record_offset":13516,"parent_record_offset":13488,"ancestor_record_offset":12356,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":636,"record_offset":12412,"parent_record_offset":12388,"ancestor_record_offset":12356,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":636,"record_offset":12616,"parent_record_offset":12592,"ancestor_record_offset":12356,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":636,"record_offset":12592,"parent_record_offset":12560,"ancestor_record_offset":12356,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::lookup
// 记录形式: compiled_procedure; instances=1
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":636,"record_offset":16140,"rva":12178560,"va":5380887680}
// 编译实例 RVAs: 0xb9d480
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1756135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380887680,"raw_hex":"e8e7cb1a00","size":5,"soft_fail":false,"target_va":5382644528,"va":5380888388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6208160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380887680,"raw_hex":"e86045a1ff","size":5,"soft_fail":false,"target_va":5374679664,"va":5380887819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5372228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380887680,"raw_hex":"e8bc06aeff","size":5,"soft_fail":false,"target_va":5375516032,"va":5380888255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141849750
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13286952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380887680,"raw_hex":"e828beca00","size":5,"soft_fail":false,"target_va":5394175824,"va":5380888867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30959207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380887680,"raw_hex":"e86766d801","size":5,"soft_fail":false,"target_va":5411847152,"va":5380887940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t13286297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380887680,"raw_hex":"e899bbca00","size":5,"soft_fail":false,"target_va":5394174448,"va":5380888146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13286181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380887680,"raw_hex":"e825bbca00","size":5,"soft_fail":false,"target_va":5394173952,"va":5380887766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::language_library::LanguageLibrary::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13282111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380887680,"raw_hex":"e83fabca00","size":5,"soft_fail":false,"target_va":5394170176,"va":5380888060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":636,"record_offset":17640,"parent_record_offset":16140,"ancestor_record_offset":16140,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":636,"record_offset":16268,"parent_record_offset":16140,"ancestor_record_offset":16140,"inlinee":16211,"source_line":453,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":636,"record_offset":18780,"parent_record_offset":16140,"ancestor_record_offset":16140,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":636,"record_offset":16324,"parent_record_offset":16140,"ancestor_record_offset":16140,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":636,"record_offset":17668,"parent_record_offset":16140,"ancestor_record_offset":16140,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] lang_lib::language_library::LanguageLibrary::try_get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/lang-lib/src/language_library.rs
//     evidence={"module":636,"record_offset":16352,"parent_record_offset":16140,"ancestor_record_offset":16140,"inlinee":36382,"source_line":48,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":636,"record_offset":18812,"parent_record_offset":16140,"ancestor_record_offset":16140,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":636,"record_offset":18696,"parent_record_offset":16140,"ancestor_record_offset":16140,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":636,"record_offset":18080,"parent_record_offset":18056,"ancestor_record_offset":16140,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":636,"record_offset":17924,"parent_record_offset":17716,"ancestor_record_offset":16140,"inlinee":7981,"source_line":65,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$2::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":636,"record_offset":17868,"parent_record_offset":17716,"ancestor_record_offset":16140,"inlinee":7980,"source_line":166,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$1::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":636,"record_offset":17716,"parent_record_offset":17692,"ancestor_record_offset":16140,"inlinee":7978,"source_line":104,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":636,"record_offset":16292,"parent_record_offset":16268,"ancestor_record_offset":16140,"inlinee":13273,"source_line":186,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":636,"record_offset":17172,"parent_record_offset":17144,"ancestor_record_offset":16140,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":636,"record_offset":16672,"parent_record_offset":16648,"ancestor_record_offset":16140,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":636,"record_offset":16756,"parent_record_offset":16732,"ancestor_record_offset":16140,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":636,"record_offset":16696,"parent_record_offset":16672,"ancestor_record_offset":16140,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":636,"record_offset":17348,"parent_record_offset":16536,"ancestor_record_offset":16140,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":636,"record_offset":18008,"parent_record_offset":17716,"ancestor_record_offset":16140,"inlinee":7982,"source_line":252,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":636,"record_offset":17776,"parent_record_offset":17752,"ancestor_record_offset":16140,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":636,"record_offset":16904,"parent_record_offset":16876,"ancestor_record_offset":16140,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":636,"record_offset":16876,"parent_record_offset":16848,"ancestor_record_offset":16140,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":636,"record_offset":17892,"parent_record_offset":17868,"ancestor_record_offset":16140,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":636,"record_offset":17524,"parent_record_offset":16352,"ancestor_record_offset":16140,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":636,"record_offset":17752,"parent_record_offset":17716,"ancestor_record_offset":16140,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":636,"record_offset":18868,"parent_record_offset":18836,"ancestor_record_offset":16140,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":636,"record_offset":18896,"parent_record_offset":18836,"ancestor_record_offset":16140,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":636,"record_offset":17692,"parent_record_offset":17668,"ancestor_record_offset":16140,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":636,"record_offset":17024,"parent_record_offset":17000,"ancestor_record_offset":16140,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":636,"record_offset":17804,"parent_record_offset":17776,"ancestor_record_offset":16140,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":636,"record_offset":17832,"parent_record_offset":17776,"ancestor_record_offset":16140,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":636,"record_offset":17256,"parent_record_offset":17228,"ancestor_record_offset":16140,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":636,"record_offset":17284,"parent_record_offset":17256,"ancestor_record_offset":16140,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":636,"record_offset":17200,"parent_record_offset":17172,"ancestor_record_offset":16140,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":636,"record_offset":17948,"parent_record_offset":17924,"ancestor_record_offset":16140,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":636,"record_offset":18116,"parent_record_offset":18080,"ancestor_record_offset":16140,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":636,"record_offset":18720,"parent_record_offset":18696,"ancestor_record_offset":16140,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":636,"record_offset":17972,"parent_record_offset":17948,"ancestor_record_offset":16140,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":636,"record_offset":18744,"parent_record_offset":18720,"ancestor_record_offset":16140,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":636,"record_offset":18140,"parent_record_offset":18116,"ancestor_record_offset":16140,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":636,"record_offset":18172,"parent_record_offset":18080,"ancestor_record_offset":16140,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":636,"record_offset":16816,"parent_record_offset":16788,"ancestor_record_offset":16140,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":636,"record_offset":16848,"parent_record_offset":16816,"ancestor_record_offset":16140,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":636,"record_offset":16788,"parent_record_offset":16536,"ancestor_record_offset":16140,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":636,"record_offset":16648,"parent_record_offset":16536,"ancestor_record_offset":16140,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":636,"record_offset":17376,"parent_record_offset":16536,"ancestor_record_offset":16140,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":636,"record_offset":16732,"parent_record_offset":16536,"ancestor_record_offset":16140,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":636,"record_offset":16620,"parent_record_offset":16536,"ancestor_record_offset":16140,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":636,"record_offset":17144,"parent_record_offset":17116,"ancestor_record_offset":16140,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":636,"record_offset":16404,"parent_record_offset":16380,"ancestor_record_offset":16140,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":636,"record_offset":16428,"parent_record_offset":16404,"ancestor_record_offset":16140,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":636,"record_offset":17116,"parent_record_offset":16948,"ancestor_record_offset":16140,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":636,"record_offset":16460,"parent_record_offset":16428,"ancestor_record_offset":16140,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":636,"record_offset":17056,"parent_record_offset":16972,"ancestor_record_offset":16140,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":636,"record_offset":17000,"parent_record_offset":16972,"ancestor_record_offset":16140,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":636,"record_offset":17468,"parent_record_offset":16536,"ancestor_record_offset":16140,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":636,"record_offset":16972,"parent_record_offset":16948,"ancestor_record_offset":16140,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":636,"record_offset":16512,"parent_record_offset":16488,"ancestor_record_offset":16140,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":636,"record_offset":16488,"parent_record_offset":16428,"ancestor_record_offset":16140,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":636,"record_offset":16536,"parent_record_offset":16512,"ancestor_record_offset":16140,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":636,"record_offset":16592,"parent_record_offset":16536,"ancestor_record_offset":16140,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":636,"record_offset":16948,"parent_record_offset":16536,"ancestor_record_offset":16140,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":636,"record_offset":18924,"parent_record_offset":18836,"ancestor_record_offset":16140,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":636,"record_offset":18836,"parent_record_offset":18812,"ancestor_record_offset":16140,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":636,"record_offset":18948,"parent_record_offset":18924,"ancestor_record_offset":16140,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":636,"record_offset":16380,"parent_record_offset":16352,"ancestor_record_offset":16140,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::game_locale
// 记录形式: compiled_procedure; instances=1
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":636,"record_offset":20348,"rva":12179888,"va":5380889008}
// 编译实例 RVAs: 0xb9d9b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1755152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380889008,"raw_hex":"e810c81a00","size":5,"soft_fail":false,"target_va":5382644528,"va":5380889371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5373197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380889008,"raw_hex":"e8f302aeff","size":5,"soft_fail":false,"target_va":5375516032,"va":5380889224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t13285316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380889008,"raw_hex":"e8c4b7ca00","size":5,"soft_fail":false,"target_va":5394174448,"va":5380889127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13284913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380889008,"raw_hex":"e831b6ca00","size":5,"soft_fail":false,"target_va":5394173952,"va":5380889034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":636,"record_offset":20480,"parent_record_offset":20348,"ancestor_record_offset":20348,"inlinee":16211,"source_line":453,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":636,"record_offset":21600,"parent_record_offset":20348,"ancestor_record_offset":20348,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":636,"record_offset":20536,"parent_record_offset":20348,"ancestor_record_offset":20348,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":636,"record_offset":20592,"parent_record_offset":20348,"ancestor_record_offset":20348,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] lang_lib::language_library::LanguageLibrary::locale
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/lang-lib/src/language_library.rs
//     evidence={"module":636,"record_offset":20564,"parent_record_offset":20348,"ancestor_record_offset":20348,"inlinee":36383,"source_line":19,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":636,"record_offset":21632,"parent_record_offset":20348,"ancestor_record_offset":20348,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":636,"record_offset":21516,"parent_record_offset":20348,"ancestor_record_offset":20348,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":636,"record_offset":20900,"parent_record_offset":20876,"ancestor_record_offset":20348,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":636,"record_offset":20748,"parent_record_offset":20632,"ancestor_record_offset":20348,"inlinee":7981,"source_line":65,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$1::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":636,"record_offset":20632,"parent_record_offset":20612,"ancestor_record_offset":20348,"inlinee":7978,"source_line":104,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":636,"record_offset":20504,"parent_record_offset":20480,"ancestor_record_offset":20348,"inlinee":13273,"source_line":186,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":636,"record_offset":20832,"parent_record_offset":20632,"ancestor_record_offset":20348,"inlinee":7982,"source_line":252,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":636,"record_offset":20688,"parent_record_offset":20664,"ancestor_record_offset":20348,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":636,"record_offset":20664,"parent_record_offset":20632,"ancestor_record_offset":20348,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":636,"record_offset":21688,"parent_record_offset":21656,"ancestor_record_offset":20348,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":636,"record_offset":21716,"parent_record_offset":21656,"ancestor_record_offset":20348,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":636,"record_offset":20612,"parent_record_offset":20592,"ancestor_record_offset":20348,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":636,"record_offset":20712,"parent_record_offset":20688,"ancestor_record_offset":20348,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":636,"record_offset":20772,"parent_record_offset":20748,"ancestor_record_offset":20348,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":636,"record_offset":20936,"parent_record_offset":20900,"ancestor_record_offset":20348,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":636,"record_offset":21540,"parent_record_offset":21516,"ancestor_record_offset":20348,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":636,"record_offset":20796,"parent_record_offset":20772,"ancestor_record_offset":20348,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":636,"record_offset":21564,"parent_record_offset":21540,"ancestor_record_offset":20348,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":636,"record_offset":20960,"parent_record_offset":20936,"ancestor_record_offset":20348,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":636,"record_offset":20992,"parent_record_offset":20900,"ancestor_record_offset":20348,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":636,"record_offset":21744,"parent_record_offset":21656,"ancestor_record_offset":20348,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":636,"record_offset":21656,"parent_record_offset":21632,"ancestor_record_offset":20348,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":636,"record_offset":21768,"parent_record_offset":21744,"ancestor_record_offset":20348,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00636.jsonl.gz"}
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
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]

// ------------------------------------------------------------------
// 函数: lang_lib::game_locale
// 信息 ID: 9f823a5c35e4ca91f6206033
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$7,ref$<str$> > @ 0xfbd1de -> 0xb9d9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$7,ref$<str$> > @ 0xfbd2a4 -> 0xb9d9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::daily_welcome::ui::closure_env$0,ref$<str$> > @ 0xfbeb1c -> 0xb9d9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::daily_welcome::ui::closure_env$0,ref$<str$> > @ 0xfbeba2 -> 0xb9d9b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::lang_pack_path
// 信息 ID: 0b42c75f63d5d74ae3ea2954
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] anyhow::context::impl$0::with_context<lang_lib::language_library::LanguageLibrary,anyhow::Error,alloc::string::String,lang_lib::load_locale_impl::closure_env$0> @ 0x1848bb9 -> 0xb9c200 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] lang_lib::load_locale_impl; module=636, inline_record=2884; 不是独立调用边。
// [PDB 内联宿主] lang_lib::load_locale_impl; module=636, inline_record=4672; 不是独立调用边。
// [PDB 内联宿主] lang_lib::load_locale_impl; module=636, inline_record=8044; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::language_library::LanguageLibrary::try_from_yml
// 信息 ID: 3a7c76d79092aa74b2fffbfb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] lang_lib::load_locale_impl @ 0xb9c614 -> 0x1848770 (direct_call); 同地址别名未裁定。
// [静态直接引用] lang_lib::load_locale_impl @ 0xb9cbdb -> 0x1848770 (direct_call); 同地址别名未裁定。
// [静态直接引用] lang_lib::load_locale_impl @ 0xb9c89e -> 0x1848770 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::load_last_used_locale
// 信息 ID: 0410c197fa28cde24ba582d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x3aefb9 -> 0xb9d180 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::load_last_used_locale::closure$0
// 信息 ID: 886c0aec3d3329e7e304a03f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "lang_lib::load_last_used_locale", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] lang_lib::load_last_used_locale; module=636, inline_record=12356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::load_locale
// 信息 ID: dd996e8d6cd6a2f0920e54b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::load_language_library::load_language_library @ 0x323699 -> 0xb9c300 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::NodeContext::with<tiny_glade::gui::language_menu::do_gui_language_menu::closure_env$0> @ 0x32d996 -> 0xb9c300 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::load_locale_impl
// 信息 ID: 7201fd6ea7a1ff6328de2501
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] lang_lib::load_locale @ 0xb9c31d -> 0xb9c4a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] lang_lib::load_last_used_locale @ 0xb9d2f0 -> 0xb9c4a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::load_locale_impl::closure$0
// 信息 ID: 950d2cece86d3798e927f76b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "lang_lib::load_locale_impl", "captures": "unknown"}
// [闭包环境类型引用] lang_lib::load_locale_impl::closure_env$0 <- anyhow::context::impl$0::with_context<lang_lib::language_library::LanguageLibrary,anyhow::Error,alloc::string::String,lang_lib::load_locale_impl::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<lang_lib::language_library::LanguageLibrary,anyhow::Error,alloc::string::String,lang_lib::load_locale_impl::closure_env$0>; module=1826, inline_record=508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::load_locale_impl::closure$1
// 信息 ID: f77f077770b3e3cf335c8c3a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "lang_lib::load_locale_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] lang_lib::load_locale_impl; module=636, inline_record=9388; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::lookup
// 信息 ID: 6f5d1a92e2f6016c1cf3edac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3cf25d5bff7c28a097953cac: iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] 96ed5be2881841498c025ae1: iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 引用方: 原程序直接转移 249 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_controls::impl$0::fmt @ 0x853e1 -> 0xb9d480 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu @ 0x854e3 -> 0xb9d480 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu @ 0x85565 -> 0xb9d480 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu @ 0x8564c -> 0xb9d480 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu @ 0x856c0 -> 0xb9d480 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu @ 0x85d14 -> 0xb9d480 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu @ 0x85d54 -> 0xb9d480 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu @ 0x85d94 -> 0xb9d480 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu @ 0x85dd4 -> 0xb9d480 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu @ 0x85e18 -> 0xb9d480 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu @ 0x85e60 -> 0xb9d480 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu @ 0x85ea8 -> 0xb9d480 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu @ 0x85eea -> 0xb9d480 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu @ 0x86459 -> 0xb9d480 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu @ 0x8649b -> 0xb9d480 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu @ 0x864db -> 0xb9d480 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

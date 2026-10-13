// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/lang-lib/src/language_library.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 5398725ceb75e29036451c0d1c4205741d0a279e9df6cbf9ade0f4dec2b4f3ef
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: language_library.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 636, 1825, 4205
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
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
// use std; // namespace candidate
//   package identities: []
// use yaml_rust2; // namespace candidate
//   package identities: [{"name":"yaml-rust2","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: lang_lib::language_library::LanguageLibrary::new
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14041
// 首条位置: {"module":1825,"inlinee":81985}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1825,"record_offset":8060,"parent_record_offset":8032,"ancestor_record_offset":8032,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1825,"record_offset":8132,"parent_record_offset":8108,"ancestor_record_offset":8032,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1825,"record_offset":8156,"parent_record_offset":8132,"ancestor_record_offset":8032,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1825,"record_offset":8084,"parent_record_offset":8060,"ancestor_record_offset":8032,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1825,"record_offset":8108,"parent_record_offset":8084,"ancestor_record_offset":8032,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1825,"record_offset":8180,"parent_record_offset":8156,"ancestor_record_offset":8032,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::language_library::LanguageLibrary::locale
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8E1F
// 首条位置: {"module":636,"inlinee":36383}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::language_library::LanguageLibrary::get
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1825,"record_offset":896,"rva":25461056,"va":5394170176}
// 编译实例 RVAs: 0x1848140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11526566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394170176,"raw_hex":"e85a1e50ff","size":5,"soft_fail":false,"target_va":5382644528,"va":5394171089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140043000
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25188078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394170176,"raw_hex":"e812a97ffe","size":5,"soft_fail":false,"target_va":5368983552,"va":5394171625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1a70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19490619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394170176,"raw_hex":"e8c598d6fe","size":5,"soft_fail":false,"target_va":5374679664,"va":5394170278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d2870
//     owner=unresolved_namespace; count=1
//     target_aliases=102 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19357400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394170176,"raw_hex":"e828a1d8fe","size":5,"soft_fail":false,"target_va":5374814320,"va":5394171715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19089741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394170176,"raw_hex":"e8b3b6dcfe","size":5,"soft_fail":false,"target_va":5375081488,"va":5394171224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18654926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394170176,"raw_hex":"e83259e3fe","size":5,"soft_fail":false,"target_va":5375516032,"va":5394170953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17676749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394170176,"raw_hex":"e8cdb90d01","size":5,"soft_fail":false,"target_va":5411847152,"va":5394170398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17675905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394170176,"raw_hex":"e881b60d01","size":5,"soft_fail":false,"target_va":5411847168,"va":5394171258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11330961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394170176,"raw_hex":"e86f1a53ff","size":5,"soft_fail":false,"target_va":5382840208,"va":5394171164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17369410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394170176,"raw_hex":"e842090901","size":5,"soft_fail":false,"target_va":5411540963,"va":5394171548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17371392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394170176,"raw_hex":"e800110901","size":5,"soft_fail":false,"target_va":5411542880,"va":5394171483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::map::HashMap$LT$K$C$V$C$S$C$A$GT$::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9324255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394170176,"raw_hex":"e8df468e00","size":5,"soft_fail":false,"target_va":5403495552,"va":5394171292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11512927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394170176,"raw_hex":"e8a15350ff","size":5,"soft_fail":false,"target_va":5382657856,"va":5394170778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17367194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394170176,"raw_hex":"e89a000901","size":5,"soft_fail":false,"target_va":5411538608,"va":5394171409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11417043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394170176,"raw_hex":"e82dca51ff","size":5,"soft_fail":false,"target_va":5382753792,"va":5394170830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17367370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394170176,"raw_hex":"e84a010901","size":5,"soft_fail":false,"target_va":5411538768,"va":5394171393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17367458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394170176,"raw_hex":"e8a2010901","size":5,"soft_fail":false,"target_va":5411538960,"va":5394171497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1825,"record_offset":5060,"parent_record_offset":896,"ancestor_record_offset":896,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1825,"record_offset":4948,"parent_record_offset":896,"ancestor_record_offset":896,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1825,"record_offset":5228,"parent_record_offset":896,"ancestor_record_offset":896,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1825,"record_offset":4740,"parent_record_offset":896,"ancestor_record_offset":896,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":1825,"record_offset":5032,"parent_record_offset":896,"ancestor_record_offset":896,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1825,"record_offset":2720,"parent_record_offset":896,"ancestor_record_offset":896,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1825,"record_offset":5892,"parent_record_offset":896,"ancestor_record_offset":896,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1825,"record_offset":2676,"parent_record_offset":896,"ancestor_record_offset":896,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1825,"record_offset":4772,"parent_record_offset":896,"ancestor_record_offset":896,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1825,"record_offset":4656,"parent_record_offset":896,"ancestor_record_offset":896,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1825,"record_offset":1076,"parent_record_offset":896,"ancestor_record_offset":896,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1825,"record_offset":5864,"parent_record_offset":896,"ancestor_record_offset":896,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":1825,"record_offset":2216,"parent_record_offset":896,"ancestor_record_offset":896,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1825,"record_offset":5132,"parent_record_offset":5108,"ancestor_record_offset":896,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1825,"record_offset":5156,"parent_record_offset":5132,"ancestor_record_offset":896,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1825,"record_offset":5084,"parent_record_offset":5060,"ancestor_record_offset":896,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1825,"record_offset":5108,"parent_record_offset":5084,"ancestor_record_offset":896,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1825,"record_offset":4996,"parent_record_offset":4972,"ancestor_record_offset":896,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1825,"record_offset":5448,"parent_record_offset":5420,"ancestor_record_offset":896,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1825,"record_offset":5508,"parent_record_offset":5476,"ancestor_record_offset":896,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1825,"record_offset":5476,"parent_record_offset":5448,"ancestor_record_offset":896,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1825,"record_offset":5544,"parent_record_offset":5508,"ancestor_record_offset":896,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1825,"record_offset":5384,"parent_record_offset":5356,"ancestor_record_offset":896,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1825,"record_offset":5356,"parent_record_offset":5328,"ancestor_record_offset":896,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1825,"record_offset":5300,"parent_record_offset":5256,"ancestor_record_offset":896,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1825,"record_offset":5328,"parent_record_offset":5300,"ancestor_record_offset":896,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1825,"record_offset":5256,"parent_record_offset":5228,"ancestor_record_offset":896,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1825,"record_offset":5420,"parent_record_offset":5384,"ancestor_record_offset":896,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1825,"record_offset":5568,"parent_record_offset":5544,"ancestor_record_offset":896,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1825,"record_offset":5592,"parent_record_offset":5568,"ancestor_record_offset":896,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1825,"record_offset":3332,"parent_record_offset":3300,"ancestor_record_offset":896,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1825,"record_offset":3392,"parent_record_offset":3360,"ancestor_record_offset":896,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1825,"record_offset":1864,"parent_record_offset":1836,"ancestor_record_offset":896,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1825,"record_offset":1364,"parent_record_offset":1340,"ancestor_record_offset":896,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1825,"record_offset":1448,"parent_record_offset":1424,"ancestor_record_offset":896,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1825,"record_offset":1388,"parent_record_offset":1364,"ancestor_record_offset":896,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1825,"record_offset":2040,"parent_record_offset":1232,"ancestor_record_offset":896,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1825,"record_offset":5640,"parent_record_offset":5616,"ancestor_record_offset":896,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1825,"record_offset":3188,"parent_record_offset":3164,"ancestor_record_offset":896,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1825,"record_offset":5616,"parent_record_offset":5592,"ancestor_record_offset":896,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1825,"record_offset":3360,"parent_record_offset":3300,"ancestor_record_offset":896,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1825,"record_offset":1596,"parent_record_offset":1568,"ancestor_record_offset":896,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1825,"record_offset":1568,"parent_record_offset":1540,"ancestor_record_offset":896,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1825,"record_offset":2756,"parent_record_offset":2720,"ancestor_record_offset":896,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1825,"record_offset":4972,"parent_record_offset":4948,"ancestor_record_offset":896,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1825,"record_offset":3052,"parent_record_offset":3024,"ancestor_record_offset":896,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1825,"record_offset":4828,"parent_record_offset":4796,"ancestor_record_offset":896,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1825,"record_offset":4856,"parent_record_offset":4796,"ancestor_record_offset":896,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1825,"record_offset":5784,"parent_record_offset":5384,"ancestor_record_offset":896,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1825,"record_offset":3140,"parent_record_offset":3024,"ancestor_record_offset":896,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1825,"record_offset":1716,"parent_record_offset":1692,"ancestor_record_offset":896,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1825,"record_offset":3216,"parent_record_offset":3188,"ancestor_record_offset":896,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1825,"record_offset":5180,"parent_record_offset":5156,"ancestor_record_offset":896,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1825,"record_offset":3164,"parent_record_offset":3140,"ancestor_record_offset":896,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1825,"record_offset":3244,"parent_record_offset":3188,"ancestor_record_offset":896,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1825,"record_offset":1948,"parent_record_offset":1920,"ancestor_record_offset":896,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1825,"record_offset":1976,"parent_record_offset":1948,"ancestor_record_offset":896,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1825,"record_offset":1892,"parent_record_offset":1864,"ancestor_record_offset":896,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1825,"record_offset":2596,"parent_record_offset":2568,"ancestor_record_offset":896,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1825,"record_offset":6144,"parent_record_offset":5956,"ancestor_record_offset":896,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1825,"record_offset":2288,"parent_record_offset":2256,"ancestor_record_offset":896,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1825,"record_offset":6236,"parent_record_offset":6204,"ancestor_record_offset":896,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1825,"record_offset":2500,"parent_record_offset":2468,"ancestor_record_offset":896,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1825,"record_offset":2312,"parent_record_offset":2288,"ancestor_record_offset":896,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1825,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":896,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1825,"record_offset":6168,"parent_record_offset":6144,"ancestor_record_offset":896,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1825,"record_offset":6260,"parent_record_offset":6236,"ancestor_record_offset":896,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1825,"record_offset":1508,"parent_record_offset":1480,"ancestor_record_offset":896,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1825,"record_offset":1540,"parent_record_offset":1508,"ancestor_record_offset":896,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1825,"record_offset":1480,"parent_record_offset":1232,"ancestor_record_offset":896,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1825,"record_offset":1340,"parent_record_offset":1232,"ancestor_record_offset":896,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1825,"record_offset":2068,"parent_record_offset":1232,"ancestor_record_offset":896,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1825,"record_offset":1424,"parent_record_offset":1232,"ancestor_record_offset":896,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1825,"record_offset":1312,"parent_record_offset":1232,"ancestor_record_offset":896,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1825,"record_offset":1836,"parent_record_offset":1808,"ancestor_record_offset":896,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1825,"record_offset":1100,"parent_record_offset":1076,"ancestor_record_offset":896,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1825,"record_offset":1124,"parent_record_offset":1100,"ancestor_record_offset":896,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1825,"record_offset":1808,"parent_record_offset":1640,"ancestor_record_offset":896,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1825,"record_offset":1156,"parent_record_offset":1124,"ancestor_record_offset":896,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1825,"record_offset":1748,"parent_record_offset":1664,"ancestor_record_offset":896,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1825,"record_offset":1692,"parent_record_offset":1664,"ancestor_record_offset":896,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1825,"record_offset":2160,"parent_record_offset":1232,"ancestor_record_offset":896,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1825,"record_offset":1664,"parent_record_offset":1640,"ancestor_record_offset":896,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1825,"record_offset":1208,"parent_record_offset":1184,"ancestor_record_offset":896,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1825,"record_offset":1184,"parent_record_offset":1124,"ancestor_record_offset":896,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1825,"record_offset":1232,"parent_record_offset":1208,"ancestor_record_offset":896,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1825,"record_offset":1284,"parent_record_offset":1232,"ancestor_record_offset":896,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1825,"record_offset":1640,"parent_record_offset":1232,"ancestor_record_offset":896,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1825,"record_offset":4884,"parent_record_offset":4796,"ancestor_record_offset":896,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1825,"record_offset":4796,"parent_record_offset":4772,"ancestor_record_offset":896,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1825,"record_offset":4908,"parent_record_offset":4884,"ancestor_record_offset":896,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1825,"record_offset":2784,"parent_record_offset":2756,"ancestor_record_offset":896,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1825,"record_offset":2840,"parent_record_offset":2812,"ancestor_record_offset":896,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1825,"record_offset":3100,"parent_record_offset":3076,"ancestor_record_offset":896,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1825,"record_offset":2964,"parent_record_offset":2936,"ancestor_record_offset":896,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1825,"record_offset":3300,"parent_record_offset":2896,"ancestor_record_offset":896,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1825,"record_offset":2812,"parent_record_offset":2784,"ancestor_record_offset":896,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":1825,"record_offset":2468,"parent_record_offset":2440,"ancestor_record_offset":896,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":1825,"record_offset":2440,"parent_record_offset":2412,"ancestor_record_offset":896,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":1825,"record_offset":5956,"parent_record_offset":5920,"ancestor_record_offset":896,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":1825,"record_offset":2568,"parent_record_offset":2376,"ancestor_record_offset":896,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":1825,"record_offset":2376,"parent_record_offset":2348,"ancestor_record_offset":896,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":1825,"record_offset":2348,"parent_record_offset":2216,"ancestor_record_offset":896,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":1825,"record_offset":5920,"parent_record_offset":5892,"ancestor_record_offset":896,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1825,"record_offset":2256,"parent_record_offset":2216,"ancestor_record_offset":896,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1825,"record_offset":6204,"parent_record_offset":5920,"ancestor_record_offset":896,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1825,"record_offset":2992,"parent_record_offset":2964,"ancestor_record_offset":896,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1825,"record_offset":3024,"parent_record_offset":2992,"ancestor_record_offset":896,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1825,"record_offset":2896,"parent_record_offset":2868,"ancestor_record_offset":896,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1825,"record_offset":2868,"parent_record_offset":2840,"ancestor_record_offset":896,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":1825,"record_offset":2412,"parent_record_offset":2376,"ancestor_record_offset":896,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/01825.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::language_library::LanguageLibrary::try_get
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8E1E
// 首条位置: {"module":636,"inlinee":36382}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":636,"record_offset":17524,"parent_record_offset":16352,"ancestor_record_offset":16352,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":636,"record_offset":16380,"parent_record_offset":16352,"ancestor_record_offset":16352,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":636,"record_offset":17172,"parent_record_offset":17144,"ancestor_record_offset":16352,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":636,"record_offset":16672,"parent_record_offset":16648,"ancestor_record_offset":16352,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":636,"record_offset":16756,"parent_record_offset":16732,"ancestor_record_offset":16352,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":636,"record_offset":16696,"parent_record_offset":16672,"ancestor_record_offset":16352,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":636,"record_offset":17348,"parent_record_offset":16536,"ancestor_record_offset":16352,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":636,"record_offset":16904,"parent_record_offset":16876,"ancestor_record_offset":16352,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":636,"record_offset":16876,"parent_record_offset":16848,"ancestor_record_offset":16352,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":636,"record_offset":17024,"parent_record_offset":17000,"ancestor_record_offset":16352,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":636,"record_offset":17256,"parent_record_offset":17228,"ancestor_record_offset":16352,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":636,"record_offset":17284,"parent_record_offset":17256,"ancestor_record_offset":16352,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":636,"record_offset":17200,"parent_record_offset":17172,"ancestor_record_offset":16352,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":636,"record_offset":16816,"parent_record_offset":16788,"ancestor_record_offset":16352,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":636,"record_offset":16848,"parent_record_offset":16816,"ancestor_record_offset":16352,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":636,"record_offset":16788,"parent_record_offset":16536,"ancestor_record_offset":16352,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":636,"record_offset":16648,"parent_record_offset":16536,"ancestor_record_offset":16352,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":636,"record_offset":17376,"parent_record_offset":16536,"ancestor_record_offset":16352,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":636,"record_offset":16732,"parent_record_offset":16536,"ancestor_record_offset":16352,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":636,"record_offset":16620,"parent_record_offset":16536,"ancestor_record_offset":16352,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":636,"record_offset":17144,"parent_record_offset":17116,"ancestor_record_offset":16352,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":636,"record_offset":16404,"parent_record_offset":16380,"ancestor_record_offset":16352,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":636,"record_offset":16428,"parent_record_offset":16404,"ancestor_record_offset":16352,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":636,"record_offset":17116,"parent_record_offset":16948,"ancestor_record_offset":16352,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":636,"record_offset":16460,"parent_record_offset":16428,"ancestor_record_offset":16352,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":636,"record_offset":17056,"parent_record_offset":16972,"ancestor_record_offset":16352,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":636,"record_offset":17000,"parent_record_offset":16972,"ancestor_record_offset":16352,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":636,"record_offset":17468,"parent_record_offset":16536,"ancestor_record_offset":16352,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":636,"record_offset":16972,"parent_record_offset":16948,"ancestor_record_offset":16352,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":636,"record_offset":16512,"parent_record_offset":16488,"ancestor_record_offset":16352,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":636,"record_offset":16488,"parent_record_offset":16428,"ancestor_record_offset":16352,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":636,"record_offset":16536,"parent_record_offset":16512,"ancestor_record_offset":16352,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":636,"record_offset":16592,"parent_record_offset":16536,"ancestor_record_offset":16352,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":636,"record_offset":16948,"parent_record_offset":16536,"ancestor_record_offset":16352,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00636.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::language_library::LanguageLibrary::try_from_yml
// 记录形式: compiled_procedure; instances=1
// 源码行: 52
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
//     target_aliases=1 candidates; complete list: language_library.rs.dependencies.json
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
// 函数符号: lang_lib::language_library::impl$0::try_from_yml::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F40A
// 首条位置: {"module":4205,"inlinee":128010}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":4205,"record_offset":1892,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":4205,"record_offset":1200,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4205,"record_offset":2088,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4205,"record_offset":2844,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_direct] yaml_rust2::yaml::Yaml::as_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"yaml-rust2","version":"0.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/yaml-rust2-0.8.0/src/yaml.rs
//     evidence={"module":4205,"record_offset":2060,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":128011,"source_line":472,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4205,"record_offset":1792,"parent_record_offset":1768,"ancestor_record_offset":1136,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4205,"record_offset":1816,"parent_record_offset":1792,"ancestor_record_offset":1136,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4205,"record_offset":3020,"parent_record_offset":2996,"ancestor_record_offset":1136,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4205,"record_offset":1916,"parent_record_offset":1892,"ancestor_record_offset":1136,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4205,"record_offset":1768,"parent_record_offset":1576,"ancestor_record_offset":1136,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4205,"record_offset":2996,"parent_record_offset":2940,"ancestor_record_offset":1136,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4205,"record_offset":1508,"parent_record_offset":1476,"ancestor_record_offset":1136,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4205,"record_offset":2968,"parent_record_offset":2940,"ancestor_record_offset":1136,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4205,"record_offset":2940,"parent_record_offset":2916,"ancestor_record_offset":1136,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4205,"record_offset":1576,"parent_record_offset":1540,"ancestor_record_offset":1136,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4205,"record_offset":1540,"parent_record_offset":1508,"ancestor_record_offset":1136,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4205,"record_offset":2916,"parent_record_offset":2892,"ancestor_record_offset":1136,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4205,"record_offset":1620,"parent_record_offset":1576,"ancestor_record_offset":1136,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4205,"record_offset":1380,"parent_record_offset":1344,"ancestor_record_offset":1136,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4205,"record_offset":1344,"parent_record_offset":1308,"ancestor_record_offset":1136,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4205,"record_offset":1272,"parent_record_offset":1236,"ancestor_record_offset":1136,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4205,"record_offset":1308,"parent_record_offset":1272,"ancestor_record_offset":1136,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4205,"record_offset":1236,"parent_record_offset":1200,"ancestor_record_offset":1136,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4205,"record_offset":1476,"parent_record_offset":1380,"ancestor_record_offset":1136,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":4205,"record_offset":1648,"parent_record_offset":1620,"ancestor_record_offset":1136,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":4205,"record_offset":1676,"parent_record_offset":1648,"ancestor_record_offset":1136,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4205,"record_offset":1444,"parent_record_offset":1420,"ancestor_record_offset":1136,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4205,"record_offset":1724,"parent_record_offset":1700,"ancestor_record_offset":1136,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4205,"record_offset":1700,"parent_record_offset":1676,"ancestor_record_offset":1136,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4205,"record_offset":1420,"parent_record_offset":1380,"ancestor_record_offset":1136,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4205,"record_offset":2868,"parent_record_offset":2844,"ancestor_record_offset":1136,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4205,"record_offset":2012,"parent_record_offset":1988,"ancestor_record_offset":1136,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/04205.jsonl.gz"}
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
// 完整结构及原始类型记录: language_library.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] lang_lib::language_library::LanguageLibrary
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 8; 过程引用数: 13
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$0::with_context<lang_lib::language_library::LanguageLibrary,anyhow::Error,alloc::string::String,lang_lib::load_locale_impl::closure_env$0>", "constructor": "anyhow::context::impl$0::with_context", "arguments": ["lang_lib::language_library::LanguageLibrary", "anyhow::Error", "alloc::string::String", "lang_lib::load_locale_impl::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["lang_lib::language_library::LanguageLibrary", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> >", "constructor": "core::option::Option", "arguments": ["alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >", "constructor": "enum2$", "arguments": ["core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::impl$5::load::closure$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>", "constructor": "arc_swap::strategy::hybrid::impl$5::load::closure$0", "arguments": ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >", "arc_swap::strategy::hybrid::DefaultConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>", "constructor": "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0", "arguments": ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >", "arc_swap::strategy::hybrid::DefaultConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::impl$2::pay_all::closure$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::debt::impl$2::pay_all::closure$0", "arguments": ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >", "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::HybridProtection<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > > >", "constructor": "arc_swap::strategy::hybrid::HybridProtection", "arguments": ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::impl$5::load::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>", "constructor": "arc_swap::strategy::hybrid::impl$5::load::closure_env$0", "arguments": ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >", "arc_swap::strategy::hybrid::DefaultConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::list::LocalNode::with<arc_swap::strategy::hybrid::HybridProtection<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > > >,arc_swap::strategy::hybrid::impl$5::load::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::debt::list::LocalNode::with", "arguments": ["arc_swap::strategy::hybrid::HybridProtection<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > > >", "arc_swap::strategy::hybrid::impl$5::load::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::impl$2::pay_all::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::debt::impl$2::pay_all::closure_env$0", "arguments": ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >", "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::list::LocalNode::with<tuple$<>,arc_swap::debt::impl$2::pay_all::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig> > >", "constructor": "arc_swap::debt::list::LocalNode::with", "arguments": ["tuple$<>", "arc_swap::debt::impl$2::pay_all::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lang_lib::language_library::LanguageLibrary", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::Guard<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::Guard", "arguments": ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >", "arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<arc_swap::Guard<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["arc_swap::Guard<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<arc_swap::strategy::hybrid::HybridProtection<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["arc_swap::strategy::hybrid::HybridProtection<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::HybridProtection::fallback<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > > >", "constructor": "arc_swap::strategy::hybrid::HybridProtection::fallback", "arguments": ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::helping::Slots::help<arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>,enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > > >", "constructor": "arc_swap::debt::helping::Slots::help", "arguments": ["arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>", "enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: language_library.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: language_library.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: lang_lib::language_library::LanguageLibrary::get
// 信息 ID: be75f6de2f10597613af2f4d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] lang_lib::lookup @ 0xb9d5fc -> 0x1848140 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::language_library::LanguageLibrary::locale
// 信息 ID: 1337b81324bcc2579d79d4b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] lang_lib::game_locale; module=636, inline_record=20564; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::language_library::LanguageLibrary::new
// 信息 ID: 4e73188e5df9b5f2a8f834fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] lang_lib::language_library::LanguageLibrary::try_from_yml; module=1825, inline_record=8032; 不是独立调用边。
// [PDB 内联宿主] lang_lib::language_library::LanguageLibrary::try_from_yml; module=1825, inline_record=8232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
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
// 函数: lang_lib::language_library::LanguageLibrary::try_get
// 信息 ID: bcdff7aa04a14691e588f9aa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] lang_lib::lookup; module=636, inline_record=16352; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::language_library::impl$0::try_from_yml::closure$0
// 信息 ID: bd373bc882ff94ea567b5d4f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "lang_lib::language_library::impl$0::try_from_yml", "captures": "unknown"}
// [闭包环境类型引用] lang_lib::language_library::impl$0::try_from_yml::closure_env$0 <- std::collections::hash::map::impl$87::from_iter<alloc::string::String,ref$<str$>,std::hash::random::RandomState,core::iter::adapters::map::Map<hashlink::linked_hash_map::Iter<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> >,lang_lib::language_library::impl$0::try_from_yml::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] lang_lib::language_library::impl$0::try_from_yml::closure_env$0 <- core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,ref$<str$> >,hashlink::linked_hash_map::Iter<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> >,lang_lib::language_library::impl$0::try_from_yml::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,ref$<str$> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,ref$<str$>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<hashlink::linked_hash_map::Iter<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> >,lang_lib::language_library::impl$0::try_from_yml::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] lang_lib::language_library::impl$0::try_from_yml::closure_env$0 <- core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,ref$<str$> >,hashlink::linked_hash_map::Iter<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> >,lang_lib::language_library::impl$0::try_from_yml::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,ref$<str$> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,ref$<str$>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<hashlink::linked_hash_map::Iter<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> >,lang_lib::language_library::impl$0::try_from_yml::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,ref$<str$> >,hashlink::linked_hash_map::Iter<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> >,lang_lib::language_library::impl$0::try_from_yml::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,ref$<str$> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,ref$<str$>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<hashlink::linked_hash_map::Iter<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> >,lang_lib::language_library::impl$0::try_from_yml::closure_env$0> > > >; module=4205, inline_record=1136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

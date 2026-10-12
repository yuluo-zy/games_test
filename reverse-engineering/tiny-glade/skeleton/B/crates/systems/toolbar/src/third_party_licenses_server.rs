// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/toolbar/src/third_party_licenses_server.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: ea719c34f1f345dc376803adbedfeecc7fab8f35c911d03827fb201996b4923e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: third_party_licenses_server.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3917, 4727
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_http; // namespace candidate
//   package identities: [{"name":"tiny_http","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::third_party_licenses_server::SingleFileServer::http<ref$<str$> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3917,"record_offset":384,"rva":37017232,"va":5405726352}
// 编译实例 RVAs: 0x234d690
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23082109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405726352,"raw_hex":"e883cb9ffe","size":5,"soft_fail":false,"target_va":5382644528,"va":5405726632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$str$u20$as$u20$std..net..socket_addr..ToSocketAddrs$GT$::to_socket_addrs
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22997783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405726352,"raw_hex":"e8e914a1fe","size":5,"soft_fail":false,"target_va":5382728880,"va":5405726658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30826268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405726352,"raw_hex":"e8e4a029fe","size":5,"soft_fail":false,"target_va":5374900736,"va":5405726999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30645335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405726352,"raw_hex":"e8a9632cfe","size":5,"soft_fail":false,"target_va":5375081488,"va":5405726818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30210471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405726352,"raw_hex":"e8590633fe","size":5,"soft_fail":false,"target_va":5375516032,"va":5405726498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fccbc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20450356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405726352,"raw_hex":"e8ccf3c7fe","size":5,"soft_fail":false,"target_va":5385276352,"va":5405726703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5814028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405726352,"raw_hex":"e80cb75800","size":5,"soft_fail":false,"target_va":5411540992,"va":5405726959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_http::Server::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5369730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405726352,"raw_hex":"e87e10aeff","size":5,"soft_fail":false,"target_va":5400357040,"va":5405726765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3917,"record_offset":648,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3917,"record_offset":1368,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3917,"record_offset":1412,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3917,"record_offset":680,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3917,"record_offset":564,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_direct] tiny_http::Server::http
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny_http-0.12.0/src/lib.rs
//     evidence={"module":3917,"record_offset":856,"parent_record_offset":384,"ancestor_record_offset":384,"inlinee":122708,"source_line":194,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3917,"record_offset":1256,"parent_record_offset":1232,"ancestor_record_offset":384,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3917,"record_offset":1280,"parent_record_offset":1256,"ancestor_record_offset":384,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3917,"record_offset":1200,"parent_record_offset":1164,"ancestor_record_offset":384,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3917,"record_offset":1232,"parent_record_offset":1200,"ancestor_record_offset":384,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::convert::impl$18::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed\convert.rs
//     evidence={"module":3917,"record_offset":1136,"parent_record_offset":1108,"ancestor_record_offset":384,"inlinee":8583,"source_line":589,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3917,"record_offset":1164,"parent_record_offset":1136,"ancestor_record_offset":384,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3917,"record_offset":1032,"parent_record_offset":1008,"ancestor_record_offset":384,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3917,"record_offset":1008,"parent_record_offset":984,"ancestor_record_offset":384,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":3917,"record_offset":736,"parent_record_offset":704,"ancestor_record_offset":384,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":3917,"record_offset":764,"parent_record_offset":704,"ancestor_record_offset":384,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3917,"record_offset":1308,"parent_record_offset":1280,"ancestor_record_offset":384,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3917,"record_offset":952,"parent_record_offset":896,"ancestor_record_offset":384,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3917,"record_offset":1076,"parent_record_offset":856,"ancestor_record_offset":384,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3917,"record_offset":1108,"parent_record_offset":856,"ancestor_record_offset":384,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3917,"record_offset":588,"parent_record_offset":564,"ancestor_record_offset":384,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3917,"record_offset":612,"parent_record_offset":588,"ancestor_record_offset":384,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3917,"record_offset":792,"parent_record_offset":704,"ancestor_record_offset":384,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3917,"record_offset":704,"parent_record_offset":680,"ancestor_record_offset":384,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3917,"record_offset":816,"parent_record_offset":792,"ancestor_record_offset":384,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_transitive] std::net::socket_addr::impl$10::to_socket_addrs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\net\socket_addr.rs
//     evidence={"module":3917,"record_offset":924,"parent_record_offset":896,"ancestor_record_offset":384,"inlinee":24002,"source_line":256,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_transitive] tiny_http::connection::ConfigListenAddr::from_socket_addrs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny_http-0.12.0/src/connection.rs
//     evidence={"module":3917,"record_offset":896,"parent_record_offset":856,"ancestor_record_offset":384,"inlinee":122709,"source_line":126,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   [pdb_inline_transitive] tiny_http::connection::impl$6::from_socket_addrs::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny_http-0.12.0/src/connection.rs
//     evidence={"module":3917,"record_offset":984,"parent_record_offset":952,"ancestor_record_offset":384,"inlinee":122711,"source_line":127,"archive":"_pdb/B/modules/03917.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::third_party_licenses_server::SingleFileServer::run
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4727,"record_offset":732,"rva":42385216,"va":5411094336}
// 编译实例 RVAs: 0x286bf40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28450403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e89de14dfe","size":5,"soft_fail":false,"target_va":5382644528,"va":5411094926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$tiny_http..IncomingRequests$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10733746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e84e375cff","size":5,"soft_fail":false,"target_va":5400360736,"va":5411094477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$tiny_http..common..Header$u20$as$u20$core..str..traits..FromStr$GT$::from_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5272203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e8758dafff","size":5,"soft_fail":false,"target_va":5405823072,"va":5411095270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d1240
//     owner=unresolved_namespace; count=2
//     target_aliases=8 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36287677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e8434bd6fd","size":5,"soft_fail":false,"target_va":5374808640,"va":5411096312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36195847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e8f9b1d7fd","size":5,"soft_fail":false,"target_va":5374900736,"va":5411096578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36013515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e8357adafd","size":5,"soft_fail":false,"target_va":5375081488,"va":5411094998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=11
//     target_aliases=1 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36012967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e8597cdafd","size":5,"soft_fail":false,"target_va":5375081504,"va":5411094466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35578711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e8a91ce1fd","size":5,"soft_fail":false,"target_va":5375516032,"va":5411094738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa5850
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25979873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e81f9473fe","size":5,"soft_fail":false,"target_va":5385115728,"va":5411095596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa7790
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25971858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e86eb373fe","size":5,"soft_fail":false,"target_va":5385123728,"va":5411095581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa9330
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25965865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e8d7ca73fe","size":5,"soft_fail":false,"target_va":5385130800,"va":5411096660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140faa070
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25961915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e845da73fe","size":5,"soft_fail":false,"target_va":5385134192,"va":5411096102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t752659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e8137c0b00","size":5,"soft_fail":false,"target_va":5411847168,"va":5411094504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t444804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e884c90600","size":5,"soft_fail":false,"target_va":5411540963,"va":5411096154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t445830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e886cd0600","size":5,"soft_fail":false,"target_va":5411542064,"va":5411096229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t446672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e8d0d00600","size":5,"soft_fail":false,"target_va":5411542880,"va":5411096203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-28385931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e875dd4efe","size":5,"soft_fail":false,"target_va":5382708672,"va":5411094598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::array::Channel$LT$T$GT$::send
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10770561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e87fa75bff","size":5,"soft_fail":false,"target_va":5400324992,"va":5411095548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::list::Channel$LT$T$GT$::send
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10776644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e8bc8f5bff","size":5,"soft_fail":false,"target_va":5400318896,"va":5411095535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::zero::Channel$LT$T$GT$::send
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5351454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e8e257aeff","size":5,"soft_fail":false,"target_va":5405744112,"va":5411095561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_http::request::Request::respond<std::io::util::Empty>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-1112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e8a8fbffff","size":5,"soft_fail":false,"target_va":5411093984,"va":5411095091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_http::request::Request::respond_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t163420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e85c7e0200","size":5,"soft_fail":false,"target_va":5411258880,"va":5411095455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_http::response::Response$LT$R$GT$::add_header
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t164638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e81e830200","size":5,"soft_fail":false,"target_va":5411260000,"va":5411095357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_http::response::Response$LT$std..fs..File$GT$::from_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5249599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411094336,"raw_hex":"e8c1e5afff","size":5,"soft_fail":false,"target_va":5405845648,"va":5411095242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5411094336,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5411095174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4727,"record_offset":1928,"parent_record_offset":732,"ancestor_record_offset":732,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4727,"record_offset":904,"parent_record_offset":732,"ancestor_record_offset":732,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4727,"record_offset":2580,"parent_record_offset":732,"ancestor_record_offset":732,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":4727,"record_offset":1960,"parent_record_offset":732,"ancestor_record_offset":732,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":4727,"record_offset":1844,"parent_record_offset":732,"ancestor_record_offset":732,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":4727,"record_offset":1728,"parent_record_offset":732,"ancestor_record_offset":732,"inlinee":11604,"source_line":382,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_direct] tiny_http::request::Request::respond
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny_http-0.12.0/src/request.rs
//     evidence={"module":4727,"record_offset":2612,"parent_record_offset":732,"ancestor_record_offset":732,"inlinee":135828,"source_line":432,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_direct] tiny_http::response::Response::empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny_http-0.12.0/src/response.rs
//     evidence={"module":4727,"record_offset":2136,"parent_record_offset":732,"ancestor_record_offset":732,"inlinee":99826,"source_line":545,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4727,"record_offset":2492,"parent_record_offset":2468,"ancestor_record_offset":732,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4727,"record_offset":2516,"parent_record_offset":2492,"ancestor_record_offset":732,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4727,"record_offset":1116,"parent_record_offset":1092,"ancestor_record_offset":732,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4727,"record_offset":2468,"parent_record_offset":2436,"ancestor_record_offset":732,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4727,"record_offset":1092,"parent_record_offset":1068,"ancestor_record_offset":732,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":4727,"record_offset":1068,"parent_record_offset":1036,"ancestor_record_offset":732,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4727,"record_offset":2360,"parent_record_offset":2324,"ancestor_record_offset":732,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4727,"record_offset":4676,"parent_record_offset":4644,"ancestor_record_offset":732,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4727,"record_offset":4644,"parent_record_offset":4616,"ancestor_record_offset":732,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4727,"record_offset":2436,"parent_record_offset":2396,"ancestor_record_offset":732,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4727,"record_offset":2396,"parent_record_offset":2360,"ancestor_record_offset":732,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4727,"record_offset":4616,"parent_record_offset":4588,"ancestor_record_offset":732,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4727,"record_offset":2180,"parent_record_offset":2136,"ancestor_record_offset":732,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4727,"record_offset":2204,"parent_record_offset":2180,"ancestor_record_offset":732,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4727,"record_offset":4428,"parent_record_offset":4404,"ancestor_record_offset":732,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":4727,"record_offset":1240,"parent_record_offset":1208,"ancestor_record_offset":732,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":4727,"record_offset":2696,"parent_record_offset":2672,"ancestor_record_offset":732,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":4727,"record_offset":1264,"parent_record_offset":1240,"ancestor_record_offset":732,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4727,"record_offset":5132,"parent_record_offset":5104,"ancestor_record_offset":732,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4727,"record_offset":2672,"parent_record_offset":2612,"ancestor_record_offset":732,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":4727,"record_offset":2016,"parent_record_offset":1984,"ancestor_record_offset":732,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":4727,"record_offset":2044,"parent_record_offset":1984,"ancestor_record_offset":732,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4727,"record_offset":936,"parent_record_offset":904,"ancestor_record_offset":732,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4727,"record_offset":1628,"parent_record_offset":1600,"ancestor_record_offset":732,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4727,"record_offset":1676,"parent_record_offset":1652,"ancestor_record_offset":732,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4727,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":732,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4727,"record_offset":2724,"parent_record_offset":2696,"ancestor_record_offset":732,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4727,"record_offset":2752,"parent_record_offset":2696,"ancestor_record_offset":732,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4727,"record_offset":2860,"parent_record_offset":2820,"ancestor_record_offset":732,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4727,"record_offset":2940,"parent_record_offset":2612,"ancestor_record_offset":732,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":4727,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":732,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":4727,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":732,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":4727,"record_offset":2072,"parent_record_offset":1984,"ancestor_record_offset":732,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":4727,"record_offset":1984,"parent_record_offset":1960,"ancestor_record_offset":732,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":4727,"record_offset":2096,"parent_record_offset":2072,"ancestor_record_offset":732,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":4727,"record_offset":1752,"parent_record_offset":1728,"ancestor_record_offset":732,"inlinee":11594,"source_line":1342,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":4727,"record_offset":1812,"parent_record_offset":1728,"ancestor_record_offset":732,"inlinee":11599,"source_line":1559,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":4727,"record_offset":1600,"parent_record_offset":968,"ancestor_record_offset":732,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":4727,"record_offset":968,"parent_record_offset":936,"ancestor_record_offset":732,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":4727,"record_offset":4360,"parent_record_offset":4336,"ancestor_record_offset":732,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":4727,"record_offset":2820,"parent_record_offset":2788,"ancestor_record_offset":732,"inlinee":13835,"source_line":392,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::impl$4::send::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":4727,"record_offset":2896,"parent_record_offset":2860,"ancestor_record_offset":732,"inlinee":13832,"source_line":398,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpsc.rs
//     evidence={"module":4727,"record_offset":2788,"parent_record_offset":2612,"ancestor_record_offset":732,"inlinee":7835,"source_line":607,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":4727,"record_offset":1780,"parent_record_offset":1752,"ancestor_record_offset":732,"inlinee":11605,"source_line":206,"archive":"_pdb/B/modules/04727.jsonl.gz"}
//   [pdb_inline_transitive] tiny_http::response::Response::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny_http-0.12.0/src/response.rs
//     evidence={"module":4727,"record_offset":2236,"parent_record_offset":2136,"ancestor_record_offset":732,"inlinee":122809,"source_line":196,"archive":"_pdb/B/modules/04727.jsonl.gz"}
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
// 完整结构及原始类型记录: third_party_licenses_server.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::third_party_licenses_server::SingleFileServer
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<toolbar::third_party_licenses_server::SingleFileServer>", "constructor": "core::ptr::drop_in_place", "arguments": ["toolbar::third_party_licenses_server::SingleFileServer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::third_party_licenses_server::SingleFileServer::http<ref$<str$> >", "constructor": "toolbar::third_party_licenses_server::SingleFileServer::http", "arguments": ["ref$<str$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: third_party_licenses_server.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: third_party_licenses_server.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: toolbar::third_party_licenses_server::SingleFileServer::http<ref$<str$> >
// 信息 ID: f92e3e31bb4a1f3315f7ac07
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] toolbar::third_party_licenses_server::SingleFileServer::http -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::credits::spawn_third_party_license_http_server @ 0x1dfb450 -> 0x234d690 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::third_party_licenses_server::SingleFileServer::run
// 信息 ID: 2935d1b4b86c90308a932068
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1,enum2$<core::result::Result<tuple$<>,anyhow::Error> > > @ 0x26272bf -> 0x286bf40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

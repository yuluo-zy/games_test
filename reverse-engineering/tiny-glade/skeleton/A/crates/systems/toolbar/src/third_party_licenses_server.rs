// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/toolbar/src/third_party_licenses_server.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: ea719c34f1f345dc376803adbedfeecc7fab8f35c911d03827fb201996b4923e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: third_party_licenses_server.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3648, 4310
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_http; // namespace candidate
//   package identities: [{"name":"tiny_http","source":"PDB_registry_path_only","version":"0.12.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::third_party_licenses_server::SingleFileServer::http<ref$<str$> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3648,"record_offset":372,"rva":28696480,"va":5397405600}
// 编译实例 RVAs: 0x1b5dfa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$str$u20$as$u20$std..net..socket_addr..ToSocketAddrs$GT$::to_socket_addrs
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19980715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397405600,"raw_hex":"e8551ecffe","size":5,"soft_fail":false,"target_va":5377425104,"va":5397405814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25091967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397405600,"raw_hex":"e8812081fe","size":5,"soft_fail":false,"target_va":5372314208,"va":5397406170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24656276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397405600,"raw_hex":"e86cc687fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5397405967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a69cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17777658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397405600,"raw_hex":"e806bcf0fe","size":5,"soft_fail":false,"target_va":5379628208,"va":5397405861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5572003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397405600,"raw_hex":"e8a3055500","size":5,"soft_fail":false,"target_va":5402978144,"va":5397406136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20052017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397405600,"raw_hex":"e8cf07cefe","size":5,"soft_fail":false,"target_va":5377353776,"va":5397405788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_http::Server::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5646834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397405600,"raw_hex":"e80ed6a9ff","size":5,"soft_fail":false,"target_va":5391759088,"va":5397405917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3648,"record_offset":640,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/03648.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3648,"record_offset":668,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03648.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3648,"record_offset":1232,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/03648.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3648,"record_offset":1272,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/03648.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":3648,"record_offset":696,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/03648.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":3648,"record_offset":556,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/03648.jsonl.gz"}
//   [pdb_inline_direct] tiny_http::Server::http
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tiny_http-0.12.0/src/lib.rs
//     evidence={"module":3648,"record_offset":724,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":108758,"source_line":194,"archive":"_pdb/A/modules/03648.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3648,"record_offset":1120,"parent_record_offset":1096,"ancestor_record_offset":372,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03648.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3648,"record_offset":1144,"parent_record_offset":1120,"ancestor_record_offset":372,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03648.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3648,"record_offset":1064,"parent_record_offset":1028,"ancestor_record_offset":372,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03648.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3648,"record_offset":1096,"parent_record_offset":1064,"ancestor_record_offset":372,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03648.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3648,"record_offset":1028,"parent_record_offset":1000,"ancestor_record_offset":372,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/03648.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$70::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3648,"record_offset":1000,"parent_record_offset":972,"ancestor_record_offset":372,"inlinee":6748,"source_line":2310,"archive":"_pdb/A/modules/03648.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3648,"record_offset":900,"parent_record_offset":876,"ancestor_record_offset":372,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/03648.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3648,"record_offset":876,"parent_record_offset":852,"ancestor_record_offset":372,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/03648.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3648,"record_offset":1172,"parent_record_offset":1144,"ancestor_record_offset":372,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03648.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3648,"record_offset":820,"parent_record_offset":764,"ancestor_record_offset":372,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/03648.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3648,"record_offset":944,"parent_record_offset":724,"ancestor_record_offset":372,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/03648.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3648,"record_offset":972,"parent_record_offset":724,"ancestor_record_offset":372,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/03648.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3648,"record_offset":580,"parent_record_offset":556,"ancestor_record_offset":372,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/03648.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3648,"record_offset":604,"parent_record_offset":580,"ancestor_record_offset":372,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/03648.jsonl.gz"}
//   [pdb_inline_transitive] std::net::socket_addr::impl$14::to_socket_addrs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\net\socket_addr.rs
//     evidence={"module":3648,"record_offset":792,"parent_record_offset":764,"ancestor_record_offset":372,"inlinee":20411,"source_line":304,"archive":"_pdb/A/modules/03648.jsonl.gz"}
//   [pdb_inline_transitive] tiny_http::connection::ConfigListenAddr::from_socket_addrs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tiny_http-0.12.0/src/connection.rs
//     evidence={"module":3648,"record_offset":764,"parent_record_offset":724,"ancestor_record_offset":372,"inlinee":108759,"source_line":126,"archive":"_pdb/A/modules/03648.jsonl.gz"}
//   [pdb_inline_transitive] tiny_http::connection::impl$6::from_socket_addrs::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tiny_http-0.12.0/src/connection.rs
//     evidence={"module":3648,"record_offset":852,"parent_record_offset":820,"ancestor_record_offset":372,"inlinee":108761,"source_line":127,"archive":"_pdb/A/modules/03648.jsonl.gz"}
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
// 首条位置: {"module":4310,"record_offset":848,"rva":31475456,"va":5400184576}
// 编译实例 RVAs: 0x1e04700
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$tiny_http..IncomingRequests$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8421983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400184576,"raw_hex":"e8a17d7fff","size":5,"soft_fail":false,"target_va":5391762720,"va":5400184698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$tiny_http..common..Header$u20$as$u20$core..str..traits..FromStr$GT$::from_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2691058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400184576,"raw_hex":"e80ef0d6ff","size":5,"soft_fail":false,"target_va":5397494224,"va":5400185277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035dea0
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27946573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400184576,"raw_hex":"e8b39155fe","size":5,"soft_fail":false,"target_va":5372239520,"va":5400186088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=4
//     target_aliases=89 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27871017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400184576,"raw_hex":"e8d7b856fe","size":5,"soft_fail":false,"target_va":5372314208,"va":5400185220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27435412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400184576,"raw_hex":"e86c5e5dfe","size":5,"soft_fail":false,"target_va":5372749696,"va":5400185103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a61490
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20592269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400184576,"raw_hex":"e873c9c5fe","size":5,"soft_fail":false,"target_va":5379593360,"va":5400185624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a63670
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20583575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400184576,"raw_hex":"e869ebc5fe","size":5,"soft_fail":false,"target_va":5379602032,"va":5400185602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a652d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20576583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400184576,"raw_hex":"e8b906c6fe","size":5,"soft_fail":false,"target_va":5379609296,"va":5400185874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a65510
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20576038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400184576,"raw_hex":"e8da08c6fe","size":5,"soft_fail":false,"target_va":5379609872,"va":5400185905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3408630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400184576,"raw_hex":"e8f6023400","size":5,"soft_fail":false,"target_va":5403593360,"va":5400184725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2792189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400184576,"raw_hex":"e8fd9a2a00","size":5,"soft_fail":false,"target_va":5402978144,"va":5400185950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2793126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400184576,"raw_hex":"e8a69e2a00","size":5,"soft_fail":false,"target_va":5402979152,"va":5400186021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2794128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400184576,"raw_hex":"e890a22a00","size":5,"soft_fail":false,"target_va":5402980128,"va":5400185995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22831260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400184576,"raw_hex":"e8649fa3fe","size":5,"soft_fail":false,"target_va":5377353776,"va":5400185031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-22776803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400184576,"raw_hex":"e81d74a4fe","size":5,"soft_fail":false,"target_va":5377408016,"va":5400184814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::array::Channel$LT$T$GT$::send
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8458482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400184576,"raw_hex":"e80eef7eff","size":5,"soft_fail":false,"target_va":5391727088,"va":5400185565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::list::Channel$LT$T$GT$::send
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8461445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400184576,"raw_hex":"e87be37eff","size":5,"soft_fail":false,"target_va":5391724112,"va":5400185552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::zero::Channel$LT$T$GT$::send
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2771103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400184576,"raw_hex":"e861b7d5ff","size":5,"soft_fail":false,"target_va":5397414480,"va":5400185578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_http::request::Request::respond<std::io::util::Empty>
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400184576,"raw_hex":"e82ffcffff","size":5,"soft_fail":false,"target_va":5400184224,"va":5400185196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_http::request::Request::respond_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t856859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400184576,"raw_hex":"e81b130d00","size":5,"soft_fail":false,"target_va":5401042336,"va":5400185472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_http::response::Response$LT$R$GT$::add_header
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t857485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400184576,"raw_hex":"e88d150d00","size":5,"soft_fail":false,"target_va":5401042864,"va":5400185374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_http::response::Response$LT$std..fs..File$GT$::from_file
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2668751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400184576,"raw_hex":"e83147d7ff","size":5,"soft_fail":false,"target_va":5397516496,"va":5400185242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":4310,"record_offset":1220,"parent_record_offset":848,"ancestor_record_offset":848,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":4310,"record_offset":1248,"parent_record_offset":848,"ancestor_record_offset":848,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4310,"record_offset":1756,"parent_record_offset":848,"ancestor_record_offset":848,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":4310,"record_offset":1276,"parent_record_offset":848,"ancestor_record_offset":848,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":4310,"record_offset":1136,"parent_record_offset":848,"ancestor_record_offset":848,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":4310,"record_offset":1020,"parent_record_offset":848,"ancestor_record_offset":848,"inlinee":5278,"source_line":374,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_direct] tiny_http::request::Request::respond
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tiny_http-0.12.0/src/request.rs
//     evidence={"module":4310,"record_offset":1788,"parent_record_offset":848,"ancestor_record_offset":848,"inlinee":120089,"source_line":432,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_direct] tiny_http::response::Response::empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tiny_http-0.12.0/src/response.rs
//     evidence={"module":4310,"record_offset":1304,"parent_record_offset":848,"ancestor_record_offset":848,"inlinee":83277,"source_line":545,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4310,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":848,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4310,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":848,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4310,"record_offset":1588,"parent_record_offset":1556,"ancestor_record_offset":848,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4310,"record_offset":1556,"parent_record_offset":1520,"ancestor_record_offset":848,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4310,"record_offset":1520,"parent_record_offset":1484,"ancestor_record_offset":848,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4310,"record_offset":1708,"parent_record_offset":1520,"ancestor_record_offset":848,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4310,"record_offset":1344,"parent_record_offset":1304,"ancestor_record_offset":848,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4310,"record_offset":1368,"parent_record_offset":1344,"ancestor_record_offset":848,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":4310,"record_offset":1872,"parent_record_offset":1848,"ancestor_record_offset":848,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4310,"record_offset":1848,"parent_record_offset":1788,"ancestor_record_offset":848,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4310,"record_offset":2136,"parent_record_offset":1788,"ancestor_record_offset":848,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4310,"record_offset":1900,"parent_record_offset":1872,"ancestor_record_offset":848,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4310,"record_offset":1664,"parent_record_offset":1636,"ancestor_record_offset":848,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4310,"record_offset":1928,"parent_record_offset":1872,"ancestor_record_offset":848,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4310,"record_offset":2028,"parent_record_offset":1992,"ancestor_record_offset":848,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4310,"record_offset":2100,"parent_record_offset":1788,"ancestor_record_offset":848,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":4310,"record_offset":1160,"parent_record_offset":1136,"ancestor_record_offset":848,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":4310,"record_offset":1184,"parent_record_offset":1160,"ancestor_record_offset":848,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":4310,"record_offset":1044,"parent_record_offset":1020,"ancestor_record_offset":848,"inlinee":5267,"source_line":925,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":4310,"record_offset":1104,"parent_record_offset":1020,"ancestor_record_offset":848,"inlinee":5272,"source_line":1130,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":4310,"record_offset":1992,"parent_record_offset":1964,"ancestor_record_offset":848,"inlinee":10646,"source_line":125,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::impl$4::send::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":4310,"record_offset":2060,"parent_record_offset":2028,"ancestor_record_offset":848,"inlinee":10648,"source_line":131,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":4310,"record_offset":1964,"parent_record_offset":1788,"ancestor_record_offset":848,"inlinee":5570,"source_line":612,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":4310,"record_offset":1072,"parent_record_offset":1044,"ancestor_record_offset":848,"inlinee":5279,"source_line":206,"archive":"_pdb/A/modules/04310.jsonl.gz"}
//   [pdb_inline_transitive] tiny_http::response::Response::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tiny_http-0.12.0/src/response.rs
//     evidence={"module":4310,"record_offset":1400,"parent_record_offset":1304,"ancestor_record_offset":848,"inlinee":108929,"source_line":196,"archive":"_pdb/A/modules/04310.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: toolbar::third_party_licenses_server::SingleFileServer::http<ref$<str$> >
// 信息 ID: f0137422d31ce0e8d78eee48
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] toolbar::third_party_licenses_server::SingleFileServer::http -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::credits::spawn_third_party_license_http_server @ 0x15ea711 -> 0x1b5dfa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::third_party_licenses_server::SingleFileServer::run
// 信息 ID: 8cd3cfcfd0c6b12faecc0b7e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys_common::backtrace::__rust_begin_short_backtrace<toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1,enum2$<core::result::Result<tuple$<>,anyhow::Error> > > @ 0x1b5efef -> 0x1e04700 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

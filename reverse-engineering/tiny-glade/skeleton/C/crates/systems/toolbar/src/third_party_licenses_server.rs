// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/toolbar/src/third_party_licenses_server.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: ea719c34f1f345dc376803adbedfeecc7fab8f35c911d03827fb201996b4923e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: third_party_licenses_server.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 4191, 5047
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
// 首条位置: {"module":4191,"record_offset":688,"rva":43349536,"va":5412058656}
// 编译实例 RVAs: 0x2957620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <str as std::net::socket_addr::ToSocketAddrs>::to_socket_addrs
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-27293595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412058656,"raw_hex":"e865885ffe","size":5,"soft_fail":false,"target_va":5384765328,"va":5412058918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-27449500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412058656,"raw_hex":"e864275dfe","size":5,"soft_fail":false,"target_va":5384609408,"va":5412058903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-36380380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412058656,"raw_hex":"e824e1d4fd","size":5,"soft_fail":false,"target_va":5375678688,"va":5412059063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-43336861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412058656,"raw_hex":"e863bb6afd","size":5,"soft_fail":false,"target_va":5368722192,"va":5412059048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-42720401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412058656,"raw_hex":"e86f2374fd","size":5,"soft_fail":false,"target_va":5369338848,"va":5412059244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411bfeb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24737953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412058656,"raw_hex":"e85f8786fe","size":5,"soft_fail":false,"target_va":5387321008,"va":5412058956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4561566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412058656,"raw_hex":"e89e9a4500","size":5,"soft_fail":false,"target_va":5416620769,"va":5412059198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_http::Server::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6463260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412058656,"raw_hex":"e8e4609dff","size":5,"soft_fail":false,"target_va":5405595760,"va":5412059015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4191,"record_offset":952,"parent_record_offset":688,"ancestor_record_offset":688,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4191,"record_offset":1640,"parent_record_offset":688,"ancestor_record_offset":688,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4191,"record_offset":1684,"parent_record_offset":688,"ancestor_record_offset":688,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":4191,"record_offset":1008,"parent_record_offset":688,"ancestor_record_offset":688,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":4191,"record_offset":868,"parent_record_offset":688,"ancestor_record_offset":688,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_direct] tiny_http::Server::http
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny_http-0.12.0/src/lib.rs
//     evidence={"module":4191,"record_offset":1120,"parent_record_offset":688,"ancestor_record_offset":688,"inlinee":136555,"source_line":194,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4191,"record_offset":1528,"parent_record_offset":1504,"ancestor_record_offset":688,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4191,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":688,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4191,"record_offset":1576,"parent_record_offset":1552,"ancestor_record_offset":688,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4191,"record_offset":1504,"parent_record_offset":1472,"ancestor_record_offset":688,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4191,"record_offset":1472,"parent_record_offset":1428,"ancestor_record_offset":688,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::convert::impl$16::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed\convert.rs
//     evidence={"module":4191,"record_offset":1400,"parent_record_offset":1372,"ancestor_record_offset":688,"inlinee":6824,"source_line":556,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4191,"record_offset":1428,"parent_record_offset":1400,"ancestor_record_offset":688,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4191,"record_offset":1296,"parent_record_offset":1272,"ancestor_record_offset":688,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4191,"record_offset":1272,"parent_record_offset":1248,"ancestor_record_offset":688,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4191,"record_offset":976,"parent_record_offset":952,"ancestor_record_offset":688,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4191,"record_offset":1216,"parent_record_offset":1160,"ancestor_record_offset":688,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4191,"record_offset":1340,"parent_record_offset":1120,"ancestor_record_offset":688,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4191,"record_offset":1372,"parent_record_offset":1120,"ancestor_record_offset":688,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4191,"record_offset":892,"parent_record_offset":868,"ancestor_record_offset":688,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4191,"record_offset":916,"parent_record_offset":892,"ancestor_record_offset":688,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":4191,"record_offset":1056,"parent_record_offset":1032,"ancestor_record_offset":688,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":4191,"record_offset":1032,"parent_record_offset":1008,"ancestor_record_offset":688,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":4191,"record_offset":1080,"parent_record_offset":1056,"ancestor_record_offset":688,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_transitive] std::net::socket_addr::impl$10::to_socket_addrs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\net\socket_addr.rs
//     evidence={"module":4191,"record_offset":1188,"parent_record_offset":1160,"ancestor_record_offset":688,"inlinee":25941,"source_line":257,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_transitive] tiny_http::connection::ConfigListenAddr::from_socket_addrs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny_http-0.12.0/src/connection.rs
//     evidence={"module":4191,"record_offset":1160,"parent_record_offset":1120,"ancestor_record_offset":688,"inlinee":136557,"source_line":126,"archive":"_pdb/C/modules/04191.jsonl.gz"}
//   [pdb_inline_transitive] tiny_http::connection::impl$6::from_socket_addrs::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny_http-0.12.0/src/connection.rs
//     evidence={"module":4191,"record_offset":1248,"parent_record_offset":1216,"ancestor_record_offset":688,"inlinee":136559,"source_line":127,"archive":"_pdb/C/modules/04191.jsonl.gz"}
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
// 首条位置: {"module":5047,"record_offset":264,"rva":47372960,"va":5416082080}
// 编译实例 RVAs: 0x2d2daa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::OpenOptions>::_open
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-31378793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e8973221fe","size":5,"soft_fail":false,"target_va":5384703536,"va":5416082324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-31473146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e806c21ffe","size":5,"soft_fail":false,"target_va":5384609408,"va":5416082549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$tiny_http..IncomingRequests$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10487643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e8a5f85fff","size":5,"soft_fail":false,"target_va":5405594560,"va":5416082198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$tiny_http..common..Header$u20$as$u20$core..str..traits..FromStr$GT$::from_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3815148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e814c9c5ff","size":5,"soft_fail":false,"target_va":5412267664,"va":5416082807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-40403937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e81f7c97fd","size":5,"soft_fail":false,"target_va":5375678688,"va":5416082620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-40404721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e80f7997fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5416083420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-47360418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e85e562dfd","size":5,"soft_fail":false,"target_va":5368722192,"va":5416082605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f920
//     owner=unresolved_namespace; count=2
//     target_aliases=16 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-46852093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e8031835fd","size":5,"soft_fail":false,"target_va":5369231648,"va":5416083736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-46743899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e8a5be36fd","size":5,"soft_fail":false,"target_va":5369338848,"va":5416082742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411846a0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29005863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e8d96745fe","size":5,"soft_fail":false,"target_va":5387077280,"va":5416083138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411872e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28994519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e8299445fe","size":5,"soft_fail":false,"target_va":5387088608,"va":5416083122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411891f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28987526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e87aaf45fe","size":5,"soft_fail":false,"target_va":5387096560,"va":5416084081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14118a0a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28983200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e860c045fe","size":5,"soft_fail":false,"target_va":5387100320,"va":5416083515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142975630
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3901554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e88e77c4ff","size":5,"soft_fail":false,"target_va":5412181552,"va":5416083101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: third_party_licenses_server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t974825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e8e9df0e00","size":5,"soft_fail":false,"target_va":5417057056,"va":5416082226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t537124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e824320800","size":5,"soft_fail":false,"target_va":5416620794,"va":5416083665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t538556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e8bc370800","size":5,"soft_fail":false,"target_va":5416622208,"va":5416083647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t538007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e897350800","size":5,"soft_fail":false,"target_va":5416621632,"va":5416083620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::array::Channel$LT$T$GT$::send
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10507940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e85ca95fff","size":5,"soft_fail":false,"target_va":5405575152,"va":5416083087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::list::Channel$LT$T$GT$::send
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10516678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e83a875fff","size":5,"soft_fail":false,"target_va":5405566400,"va":5416083073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_http::request::Request::respond<std::io::util::Empty>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e8e2070000","size":5,"soft_fail":false,"target_va":5416084736,"va":5416082713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_http::request::Request::respond_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t215691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e88b4a0300","size":5,"soft_fail":false,"target_va":5416298688,"va":5416082992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_http::response::Response$LT$R$GT$::add_header
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t206990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e88e280300","size":5,"soft_fail":false,"target_va":5416289888,"va":5416082893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_http::response::Response$LT$std..fs..File$GT$::from_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-3792415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416082080,"raw_hex":"e8e121c6ff","size":5,"soft_fail":false,"target_va":5412290368,"va":5416082778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*10077816(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5416082080,"raw_hex":"ff1578c69900","size":6,"soft_fail":false,"va":5416083354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":5047,"record_offset":636,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":5047,"record_offset":2100,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":5047,"record_offset":1272,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":5047,"record_offset":692,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":5047,"record_offset":552,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":5047,"record_offset":436,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7533,"source_line":570,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_direct] tiny_http::request::Request::respond
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny_http-0.12.0/src/request.rs
//     evidence={"module":5047,"record_offset":1304,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":148085,"source_line":432,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_direct] tiny_http::response::Response::empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny_http-0.12.0/src/response.rs
//     evidence={"module":5047,"record_offset":804,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":111588,"source_line":545,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":5047,"record_offset":1156,"parent_record_offset":1132,"ancestor_record_offset":264,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":5047,"record_offset":1180,"parent_record_offset":1156,"ancestor_record_offset":264,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":5047,"record_offset":2592,"parent_record_offset":2568,"ancestor_record_offset":264,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":5047,"record_offset":2616,"parent_record_offset":2592,"ancestor_record_offset":264,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":5047,"record_offset":1204,"parent_record_offset":1180,"ancestor_record_offset":264,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":5047,"record_offset":2640,"parent_record_offset":2616,"ancestor_record_offset":264,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":5047,"record_offset":1132,"parent_record_offset":1100,"ancestor_record_offset":264,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":5047,"record_offset":2568,"parent_record_offset":2504,"ancestor_record_offset":264,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":5047,"record_offset":1024,"parent_record_offset":988,"ancestor_record_offset":264,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":5047,"record_offset":2536,"parent_record_offset":2504,"ancestor_record_offset":264,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":5047,"record_offset":2504,"parent_record_offset":2476,"ancestor_record_offset":264,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":5047,"record_offset":1100,"parent_record_offset":1060,"ancestor_record_offset":264,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":5047,"record_offset":1060,"parent_record_offset":1024,"ancestor_record_offset":264,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":5047,"record_offset":2476,"parent_record_offset":2448,"ancestor_record_offset":264,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":5047,"record_offset":844,"parent_record_offset":804,"ancestor_record_offset":264,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":5047,"record_offset":868,"parent_record_offset":844,"ancestor_record_offset":264,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":5047,"record_offset":2288,"parent_record_offset":2264,"ancestor_record_offset":264,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":5047,"record_offset":1388,"parent_record_offset":1364,"ancestor_record_offset":264,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":5047,"record_offset":3104,"parent_record_offset":3076,"ancestor_record_offset":264,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":5047,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":264,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":5047,"record_offset":1364,"parent_record_offset":1304,"ancestor_record_offset":264,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":5047,"record_offset":1600,"parent_record_offset":1304,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":5047,"record_offset":1496,"parent_record_offset":1456,"ancestor_record_offset":264,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":5047,"record_offset":1572,"parent_record_offset":1304,"ancestor_record_offset":264,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":5047,"record_offset":576,"parent_record_offset":552,"ancestor_record_offset":264,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":5047,"record_offset":600,"parent_record_offset":576,"ancestor_record_offset":264,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":5047,"record_offset":740,"parent_record_offset":716,"ancestor_record_offset":264,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":5047,"record_offset":716,"parent_record_offset":692,"ancestor_record_offset":264,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":5047,"record_offset":764,"parent_record_offset":740,"ancestor_record_offset":264,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":5047,"record_offset":460,"parent_record_offset":436,"ancestor_record_offset":264,"inlinee":7535,"source_line":1692,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":5047,"record_offset":520,"parent_record_offset":436,"ancestor_record_offset":264,"inlinee":7538,"source_line":1915,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":5047,"record_offset":2220,"parent_record_offset":2196,"ancestor_record_offset":264,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpmc\mod.rs
//     evidence={"module":5047,"record_offset":1456,"parent_record_offset":1424,"ancestor_record_offset":264,"inlinee":5892,"source_line":393,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::impl$4::send::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpmc\mod.rs
//     evidence={"module":5047,"record_offset":1532,"parent_record_offset":1496,"ancestor_record_offset":264,"inlinee":5887,"source_line":399,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":5047,"record_offset":1424,"parent_record_offset":1304,"ancestor_record_offset":264,"inlinee":12867,"source_line":607,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":5047,"record_offset":488,"parent_record_offset":460,"ancestor_record_offset":264,"inlinee":7537,"source_line":215,"archive":"_pdb/C/modules/05047.jsonl.gz"}
//   [pdb_inline_transitive] tiny_http::response::Response::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"tiny_http","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny_http-0.12.0/src/response.rs
//     evidence={"module":5047,"record_offset":900,"parent_record_offset":804,"ancestor_record_offset":264,"inlinee":136822,"source_line":196,"archive":"_pdb/C/modules/05047.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: toolbar::third_party_licenses_server::SingleFileServer::http<ref$<str$> >
// 信息 ID: 78cf30f8bd74e9aa373081ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] toolbar::third_party_licenses_server::SingleFileServer::http -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::credits::spawn_third_party_license_http_server @ 0x22f55bd -> 0x2957620 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::third_party_licenses_server::SingleFileServer::run
// 信息 ID: 36b99e0c66038ab19151cc72
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1,enum2$<core::result::Result<tuple$<>,anyhow::Error> > > @ 0x2b2e51c -> 0x2d2daa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

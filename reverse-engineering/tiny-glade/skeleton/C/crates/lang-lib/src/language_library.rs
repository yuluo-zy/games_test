// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/lang-lib/src/language_library.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 5398725ceb75e29036451c0d1c4205741d0a279e9df6cbf9ade0f4dec2b4f3ef
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: language_library.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 720, 2107, 3597
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
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
// IPI IDs: 0x177B4
// 首条位置: {"module":2107,"inlinee":96180}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2107,"record_offset":2664,"parent_record_offset":2636,"ancestor_record_offset":2636,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":2736,"parent_record_offset":2712,"ancestor_record_offset":2636,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":2760,"parent_record_offset":2736,"ancestor_record_offset":2636,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":2784,"parent_record_offset":2760,"ancestor_record_offset":2636,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":2712,"parent_record_offset":2688,"ancestor_record_offset":2636,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2107,"record_offset":2688,"parent_record_offset":2664,"ancestor_record_offset":2636,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::language_library::LanguageLibrary::locale
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB6CE
// 首条位置: {"module":720,"inlinee":46798}
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
// 首条位置: {"module":2107,"record_offset":5044,"rva":31290064,"va":5399999184}
// 编译实例 RVAs: 0x1dd72d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15354863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399999184,"raw_hex":"e811b415ff","size":5,"soft_fail":false,"target_va":5384644912,"va":5399999770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t16615838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399999184,"raw_hex":"e89e89fd00","size":5,"soft_fail":false,"target_va":5416616112,"va":5400000269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t16615934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399999184,"raw_hex":"e8fe89fd00","size":5,"soft_fail":false,"target_va":5416616304,"va":5400000365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15390597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399999184,"raw_hex":"e87b2815ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5400000000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24321422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399999184,"raw_hex":"e872e28cfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5400000105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31277867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399999184,"raw_hex":"e8d5bc22fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5400000054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081b80
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30760046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399999184,"raw_hex":"e892a32afe","size":5,"soft_fail":false,"target_va":5369240448,"va":5400000489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=1
//     target_aliases=122 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30735464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399999184,"raw_hex":"e898032bfe","size":5,"soft_fail":false,"target_va":5369265120,"va":5400000579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26518385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399999184,"raw_hex":"e88f5c6bfe","size":5,"soft_fail":false,"target_va":5373480896,"va":5399999276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17057641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399999184,"raw_hex":"e869470401","size":5,"soft_fail":false,"target_va":5417057040,"va":5399999394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17056924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399999184,"raw_hex":"e89c440401","size":5,"soft_fail":false,"target_va":5417057056,"va":5400000127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15205181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399999184,"raw_hex":"e8c3fc17ff","size":5,"soft_fail":false,"target_va":5384794864,"va":5400000040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16620716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399999184,"raw_hex":"e8ac9cfd00","size":5,"soft_fail":false,"target_va":5416620794,"va":5400000073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16621277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399999184,"raw_hex":"e8dd9efd00","size":5,"soft_fail":false,"target_va":5416621632,"va":5400000350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::map::HashMap$LT$K$C$V$C$S$C$A$GT$::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9632104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399999184,"raw_hex":"e868f99200","size":5,"soft_fail":false,"target_va":5409632288,"va":5400000179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16619412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399999184,"raw_hex":"e89497fd00","size":5,"soft_fail":false,"target_va":5416619696,"va":5400000279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15273692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399999184,"raw_hex":"e824f116ff","size":5,"soft_fail":false,"target_va":5384726688,"va":5400000375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2107,"record_offset":8940,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2107,"record_offset":8828,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":2107,"record_offset":9108,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2107,"record_offset":8660,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":2107,"record_offset":8912,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2107,"record_offset":6772,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":4101,"source_line":1805,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2107,"record_offset":9712,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2107,"record_offset":6728,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":2107,"record_offset":8716,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2107,"record_offset":8576,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2107,"record_offset":5224,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2107,"record_offset":9684,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":2107,"record_offset":6268,"parent_record_offset":5044,"ancestor_record_offset":5044,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":9012,"parent_record_offset":8988,"ancestor_record_offset":5044,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":9036,"parent_record_offset":9012,"ancestor_record_offset":5044,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":9060,"parent_record_offset":9036,"ancestor_record_offset":5044,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":8988,"parent_record_offset":8964,"ancestor_record_offset":5044,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2107,"record_offset":8964,"parent_record_offset":8940,"ancestor_record_offset":5044,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2107,"record_offset":8876,"parent_record_offset":8852,"ancestor_record_offset":5044,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2107,"record_offset":9272,"parent_record_offset":9248,"ancestor_record_offset":5044,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2107,"record_offset":9328,"parent_record_offset":9296,"ancestor_record_offset":5044,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2107,"record_offset":9296,"parent_record_offset":9272,"ancestor_record_offset":5044,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2107,"record_offset":9368,"parent_record_offset":9328,"ancestor_record_offset":5044,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2107,"record_offset":9172,"parent_record_offset":9136,"ancestor_record_offset":5044,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2107,"record_offset":9196,"parent_record_offset":9172,"ancestor_record_offset":5044,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2107,"record_offset":9220,"parent_record_offset":9196,"ancestor_record_offset":5044,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2107,"record_offset":9136,"parent_record_offset":9108,"ancestor_record_offset":5044,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2107,"record_offset":9248,"parent_record_offset":9220,"ancestor_record_offset":5044,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":2107,"record_offset":9392,"parent_record_offset":9368,"ancestor_record_offset":5044,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2107,"record_offset":7376,"parent_record_offset":7348,"ancestor_record_offset":5044,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2107,"record_offset":7276,"parent_record_offset":7072,"ancestor_record_offset":5044,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2107,"record_offset":5928,"parent_record_offset":5900,"ancestor_record_offset":5044,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2107,"record_offset":5488,"parent_record_offset":5464,"ancestor_record_offset":5044,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2107,"record_offset":5572,"parent_record_offset":5548,"ancestor_record_offset":5044,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2107,"record_offset":6096,"parent_record_offset":5356,"ancestor_record_offset":5044,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2107,"record_offset":9440,"parent_record_offset":9416,"ancestor_record_offset":5044,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":2107,"record_offset":7240,"parent_record_offset":7216,"ancestor_record_offset":5044,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2107,"record_offset":9416,"parent_record_offset":9392,"ancestor_record_offset":5044,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2107,"record_offset":7404,"parent_record_offset":7348,"ancestor_record_offset":5044,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2107,"record_offset":5720,"parent_record_offset":5692,"ancestor_record_offset":5044,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2107,"record_offset":5692,"parent_record_offset":5664,"ancestor_record_offset":5044,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2107,"record_offset":6808,"parent_record_offset":6772,"ancestor_record_offset":5044,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2107,"record_offset":8684,"parent_record_offset":8660,"ancestor_record_offset":5044,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2107,"record_offset":8852,"parent_record_offset":8828,"ancestor_record_offset":5044,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2107,"record_offset":7104,"parent_record_offset":7072,"ancestor_record_offset":5044,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2107,"record_offset":9608,"parent_record_offset":9220,"ancestor_record_offset":5044,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2107,"record_offset":5512,"parent_record_offset":5488,"ancestor_record_offset":5044,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2107,"record_offset":7192,"parent_record_offset":7072,"ancestor_record_offset":5044,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2107,"record_offset":5836,"parent_record_offset":5812,"ancestor_record_offset":5044,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2107,"record_offset":7216,"parent_record_offset":7192,"ancestor_record_offset":5044,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2107,"record_offset":6012,"parent_record_offset":5984,"ancestor_record_offset":5044,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2107,"record_offset":6040,"parent_record_offset":6012,"ancestor_record_offset":5044,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2107,"record_offset":5956,"parent_record_offset":5928,"ancestor_record_offset":5044,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2107,"record_offset":6648,"parent_record_offset":6620,"ancestor_record_offset":5044,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2107,"record_offset":9972,"parent_record_offset":9784,"ancestor_record_offset":5044,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2107,"record_offset":6340,"parent_record_offset":6308,"ancestor_record_offset":5044,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2107,"record_offset":10064,"parent_record_offset":10032,"ancestor_record_offset":5044,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2107,"record_offset":6552,"parent_record_offset":6520,"ancestor_record_offset":5044,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2107,"record_offset":6364,"parent_record_offset":6340,"ancestor_record_offset":5044,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2107,"record_offset":6576,"parent_record_offset":6552,"ancestor_record_offset":5044,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2107,"record_offset":9996,"parent_record_offset":9972,"ancestor_record_offset":5044,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2107,"record_offset":10088,"parent_record_offset":10064,"ancestor_record_offset":5044,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2107,"record_offset":5632,"parent_record_offset":5604,"ancestor_record_offset":5044,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2107,"record_offset":5664,"parent_record_offset":5632,"ancestor_record_offset":5044,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2107,"record_offset":5604,"parent_record_offset":5356,"ancestor_record_offset":5044,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2107,"record_offset":5464,"parent_record_offset":5356,"ancestor_record_offset":5044,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2107,"record_offset":6124,"parent_record_offset":5356,"ancestor_record_offset":5044,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2107,"record_offset":5548,"parent_record_offset":5356,"ancestor_record_offset":5044,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":2107,"record_offset":5408,"parent_record_offset":5356,"ancestor_record_offset":5044,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":2107,"record_offset":5900,"parent_record_offset":5872,"ancestor_record_offset":5044,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2107,"record_offset":5248,"parent_record_offset":5224,"ancestor_record_offset":5044,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2107,"record_offset":5872,"parent_record_offset":5764,"ancestor_record_offset":5044,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2107,"record_offset":5280,"parent_record_offset":5248,"ancestor_record_offset":5044,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2107,"record_offset":5812,"parent_record_offset":5788,"ancestor_record_offset":5044,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2107,"record_offset":6216,"parent_record_offset":5356,"ancestor_record_offset":5044,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2107,"record_offset":5788,"parent_record_offset":5764,"ancestor_record_offset":5044,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2107,"record_offset":5332,"parent_record_offset":5308,"ancestor_record_offset":5044,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2107,"record_offset":5308,"parent_record_offset":5248,"ancestor_record_offset":5044,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2107,"record_offset":5356,"parent_record_offset":5332,"ancestor_record_offset":5044,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2107,"record_offset":5436,"parent_record_offset":5356,"ancestor_record_offset":5044,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2107,"record_offset":5764,"parent_record_offset":5356,"ancestor_record_offset":5044,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2107,"record_offset":8764,"parent_record_offset":8740,"ancestor_record_offset":5044,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":2107,"record_offset":8740,"parent_record_offset":8716,"ancestor_record_offset":5044,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2107,"record_offset":8788,"parent_record_offset":8764,"ancestor_record_offset":5044,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2107,"record_offset":6836,"parent_record_offset":6808,"ancestor_record_offset":5044,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":2107,"record_offset":6892,"parent_record_offset":6864,"ancestor_record_offset":5044,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":2107,"record_offset":7152,"parent_record_offset":7128,"ancestor_record_offset":5044,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":2107,"record_offset":7012,"parent_record_offset":6984,"ancestor_record_offset":5044,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":2107,"record_offset":7348,"parent_record_offset":6948,"ancestor_record_offset":5044,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":2107,"record_offset":6864,"parent_record_offset":6836,"ancestor_record_offset":5044,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":2107,"record_offset":6520,"parent_record_offset":6492,"ancestor_record_offset":5044,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":2107,"record_offset":6492,"parent_record_offset":6464,"ancestor_record_offset":5044,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":2107,"record_offset":9784,"parent_record_offset":9744,"ancestor_record_offset":5044,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":2107,"record_offset":6620,"parent_record_offset":6428,"ancestor_record_offset":5044,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":2107,"record_offset":6428,"parent_record_offset":6400,"ancestor_record_offset":5044,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":2107,"record_offset":6400,"parent_record_offset":6268,"ancestor_record_offset":5044,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":2107,"record_offset":9744,"parent_record_offset":9712,"ancestor_record_offset":5044,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":2107,"record_offset":6308,"parent_record_offset":6268,"ancestor_record_offset":5044,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":2107,"record_offset":10032,"parent_record_offset":9744,"ancestor_record_offset":5044,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":2107,"record_offset":7040,"parent_record_offset":7012,"ancestor_record_offset":5044,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":2107,"record_offset":7072,"parent_record_offset":7040,"ancestor_record_offset":5044,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":2107,"record_offset":6464,"parent_record_offset":6428,"ancestor_record_offset":5044,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2107,"record_offset":6948,"parent_record_offset":6920,"ancestor_record_offset":5044,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2107,"record_offset":6920,"parent_record_offset":6892,"ancestor_record_offset":5044,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::language_library::LanguageLibrary::try_get
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB6D3
// 首条位置: {"module":720,"inlinee":46803}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":720,"record_offset":16456,"parent_record_offset":15380,"ancestor_record_offset":15380,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":720,"record_offset":15408,"parent_record_offset":15380,"ancestor_record_offset":15380,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":720,"record_offset":16116,"parent_record_offset":16088,"ancestor_record_offset":15380,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":720,"record_offset":15676,"parent_record_offset":15652,"ancestor_record_offset":15380,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":720,"record_offset":15760,"parent_record_offset":15736,"ancestor_record_offset":15380,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":720,"record_offset":16284,"parent_record_offset":15540,"ancestor_record_offset":15380,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":720,"record_offset":15908,"parent_record_offset":15880,"ancestor_record_offset":15380,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":720,"record_offset":15880,"parent_record_offset":15852,"ancestor_record_offset":15380,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":720,"record_offset":15700,"parent_record_offset":15676,"ancestor_record_offset":15380,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":720,"record_offset":16024,"parent_record_offset":16000,"ancestor_record_offset":15380,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":720,"record_offset":16200,"parent_record_offset":16172,"ancestor_record_offset":15380,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":720,"record_offset":16228,"parent_record_offset":16200,"ancestor_record_offset":15380,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":720,"record_offset":16144,"parent_record_offset":16116,"ancestor_record_offset":15380,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":720,"record_offset":15820,"parent_record_offset":15792,"ancestor_record_offset":15380,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":720,"record_offset":15852,"parent_record_offset":15820,"ancestor_record_offset":15380,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":720,"record_offset":15792,"parent_record_offset":15540,"ancestor_record_offset":15380,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":720,"record_offset":15652,"parent_record_offset":15540,"ancestor_record_offset":15380,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":720,"record_offset":16312,"parent_record_offset":15540,"ancestor_record_offset":15380,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":720,"record_offset":15736,"parent_record_offset":15540,"ancestor_record_offset":15380,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":720,"record_offset":15596,"parent_record_offset":15540,"ancestor_record_offset":15380,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":720,"record_offset":16088,"parent_record_offset":16060,"ancestor_record_offset":15380,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":720,"record_offset":15432,"parent_record_offset":15408,"ancestor_record_offset":15380,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":720,"record_offset":16060,"parent_record_offset":15952,"ancestor_record_offset":15380,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":720,"record_offset":15464,"parent_record_offset":15432,"ancestor_record_offset":15380,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":720,"record_offset":16000,"parent_record_offset":15976,"ancestor_record_offset":15380,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":720,"record_offset":16404,"parent_record_offset":15540,"ancestor_record_offset":15380,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":720,"record_offset":15976,"parent_record_offset":15952,"ancestor_record_offset":15380,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":720,"record_offset":15516,"parent_record_offset":15492,"ancestor_record_offset":15380,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":720,"record_offset":15492,"parent_record_offset":15432,"ancestor_record_offset":15380,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":720,"record_offset":15540,"parent_record_offset":15516,"ancestor_record_offset":15380,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":720,"record_offset":15624,"parent_record_offset":15540,"ancestor_record_offset":15380,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":720,"record_offset":15952,"parent_record_offset":15540,"ancestor_record_offset":15380,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00720.jsonl.gz"}
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
// 首条位置: {"module":2107,"record_offset":924,"rva":31288944,"va":5399998064}
// 编译实例 RVAs: 0x1dd6e70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15353512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e858b915ff","size":5,"soft_fail":false,"target_va":5384644912,"va":5399998419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-24319661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e853e98cfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5399998360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31276265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e817c322fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5399998452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e83d0
//     owner=unresolved_namespace; count=1
//     target_aliases=34 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30338142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e8a21331fe","size":5,"soft_fail":false,"target_va":5369660368,"va":5399998505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142706e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9633194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e8aafd9200","size":5,"soft_fail":false,"target_va":5409631776,"va":5399998577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t17002489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e8f96f0301","size":5,"soft_fail":false,"target_va":5417000640,"va":5399998146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16622893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e82da5fd00","size":5,"soft_fail":false,"target_va":5416621600,"va":5399998702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$yaml_rust2..yaml..Yaml$GT$$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e82cf9ffff","size":5,"soft_fail":false,"target_va":5399996896,"va":5399998639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$std..collections..hash..map..HashMap$LT$alloc..string..String$C$$RF$str$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e8bafaffff","size":5,"soft_fail":false,"target_va":5399997808,"va":5399999153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::RawTable$LT$T$C$A$GT$::reserve_rehash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16999351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e8b7630301","size":5,"soft_fail":false,"target_va":5416998080,"va":5399998724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15246858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e8f65917ff","size":5,"soft_fail":false,"target_va":5384751264,"va":5399998117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15272052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e88cf716ff","size":5,"soft_fail":false,"target_va":5384726688,"va":5399998735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16620585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e8299cfd00","size":5,"soft_fail":false,"target_va":5416619360,"va":5399998770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] yaml_rust2::yaml::YamlLoader::load_from_str
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"yaml-rust2","version":"0.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t18927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e8ef490000","size":5,"soft_fail":false,"target_va":5400017120,"va":5399998188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2107,"record_offset":1232,"parent_record_offset":924,"ancestor_record_offset":924,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2107,"record_offset":2836,"parent_record_offset":924,"ancestor_record_offset":924,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2107,"record_offset":1288,"parent_record_offset":924,"ancestor_record_offset":924,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2107,"record_offset":1384,"parent_record_offset":924,"ancestor_record_offset":924,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2107,"record_offset":1136,"parent_record_offset":924,"ancestor_record_offset":924,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2107,"record_offset":1168,"parent_record_offset":924,"ancestor_record_offset":924,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] hashlink::linked_hash_map::LinkedHashMap::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashlink","version":"0.8.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashlink-0.8.4/src/linked_hash_map.rs
//     evidence={"module":2107,"record_offset":1324,"parent_record_offset":924,"ancestor_record_offset":924,"inlinee":46836,"source_line":134,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] lang_lib::language_library::LanguageLibrary::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/lang-lib/src/language_library.rs
//     evidence={"module":2107,"record_offset":2636,"parent_record_offset":924,"ancestor_record_offset":924,"inlinee":96180,"source_line":12,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2107,"record_offset":1952,"parent_record_offset":924,"ancestor_record_offset":924,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":2107,"record_offset":1108,"parent_record_offset":924,"ancestor_record_offset":924,"inlinee":8116,"source_line":383,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] yaml_rust2::yaml::Yaml::as_hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"yaml-rust2","version":"0.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/yaml-rust2-0.8.0/src/yaml.rs
//     evidence={"module":2107,"record_offset":1260,"parent_record_offset":924,"ancestor_record_offset":924,"inlinee":96179,"source_line":472,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":2736,"parent_record_offset":2712,"ancestor_record_offset":924,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":2760,"parent_record_offset":2736,"ancestor_record_offset":924,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":1560,"parent_record_offset":1536,"ancestor_record_offset":924,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":1584,"parent_record_offset":1560,"ancestor_record_offset":924,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":2784,"parent_record_offset":2760,"ancestor_record_offset":924,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":1608,"parent_record_offset":1584,"ancestor_record_offset":924,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":2712,"parent_record_offset":2688,"ancestor_record_offset":924,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":1536,"parent_record_offset":1480,"ancestor_record_offset":924,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2107,"record_offset":2688,"parent_record_offset":2664,"ancestor_record_offset":924,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2107,"record_offset":2664,"parent_record_offset":2636,"ancestor_record_offset":924,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2107,"record_offset":1508,"parent_record_offset":1480,"ancestor_record_offset":924,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2107,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":924,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2107,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":924,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2107,"record_offset":2492,"parent_record_offset":2464,"ancestor_record_offset":924,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2107,"record_offset":2392,"parent_record_offset":2188,"ancestor_record_offset":924,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2107,"record_offset":3124,"parent_record_offset":3068,"ancestor_record_offset":924,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2107,"record_offset":3376,"parent_record_offset":3344,"ancestor_record_offset":924,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2107,"record_offset":3412,"parent_record_offset":3284,"ancestor_record_offset":924,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":2107,"record_offset":2356,"parent_record_offset":2332,"ancestor_record_offset":924,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2107,"record_offset":2520,"parent_record_offset":2464,"ancestor_record_offset":924,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2107,"record_offset":2100,"parent_record_offset":2064,"ancestor_record_offset":924,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2107,"record_offset":2220,"parent_record_offset":2188,"ancestor_record_offset":924,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2107,"record_offset":3040,"parent_record_offset":3012,"ancestor_record_offset":924,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2107,"record_offset":1408,"parent_record_offset":1384,"ancestor_record_offset":924,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2107,"record_offset":2308,"parent_record_offset":2188,"ancestor_record_offset":924,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2107,"record_offset":2332,"parent_record_offset":2308,"ancestor_record_offset":924,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2107,"record_offset":3316,"parent_record_offset":3284,"ancestor_record_offset":924,"inlinee":8392,"source_line":1103,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2107,"record_offset":3284,"parent_record_offset":3256,"ancestor_record_offset":924,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2107,"record_offset":3344,"parent_record_offset":3316,"ancestor_record_offset":924,"inlinee":8393,"source_line":955,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2107,"record_offset":3228,"parent_record_offset":2872,"ancestor_record_offset":924,"inlinee":12224,"source_line":362,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$78::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2107,"record_offset":2872,"parent_record_offset":2836,"ancestor_record_offset":924,"inlinee":12269,"source_line":2919,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$79::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2107,"record_offset":3256,"parent_record_offset":2872,"ancestor_record_offset":924,"inlinee":12226,"source_line":2936,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":2107,"record_offset":2008,"parent_record_offset":1980,"ancestor_record_offset":924,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":2107,"record_offset":2268,"parent_record_offset":2244,"ancestor_record_offset":924,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":2107,"record_offset":2128,"parent_record_offset":2100,"ancestor_record_offset":924,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":2107,"record_offset":2464,"parent_record_offset":2064,"ancestor_record_offset":924,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":2107,"record_offset":1980,"parent_record_offset":1952,"ancestor_record_offset":924,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":2107,"record_offset":2156,"parent_record_offset":2128,"ancestor_record_offset":924,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":2107,"record_offset":2188,"parent_record_offset":2156,"ancestor_record_offset":924,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2107,"record_offset":2064,"parent_record_offset":2036,"ancestor_record_offset":924,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2107,"record_offset":2036,"parent_record_offset":2008,"ancestor_record_offset":924,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::language_library::impl$0::try_from_yml::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F8EE
// 首条位置: {"module":3597,"inlinee":129262}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3597,"record_offset":2548,"parent_record_offset":1140,"ancestor_record_offset":1140,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":3597,"record_offset":1256,"parent_record_offset":1140,"ancestor_record_offset":1140,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3597,"record_offset":1220,"parent_record_offset":1140,"ancestor_record_offset":1140,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3597,"record_offset":2716,"parent_record_offset":1140,"ancestor_record_offset":1140,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_direct] yaml_rust2::yaml::Yaml::as_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"yaml-rust2","version":"0.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/yaml-rust2-0.8.0/src/yaml.rs
//     evidence={"module":3597,"record_offset":1192,"parent_record_offset":1140,"ancestor_record_offset":1140,"inlinee":129263,"source_line":472,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3597,"record_offset":1708,"parent_record_offset":1684,"ancestor_record_offset":1140,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3597,"record_offset":1732,"parent_record_offset":1708,"ancestor_record_offset":1140,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3597,"record_offset":2892,"parent_record_offset":2868,"ancestor_record_offset":1140,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3597,"record_offset":2916,"parent_record_offset":2892,"ancestor_record_offset":1140,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3597,"record_offset":1756,"parent_record_offset":1732,"ancestor_record_offset":1140,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3597,"record_offset":2940,"parent_record_offset":2916,"ancestor_record_offset":1140,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3597,"record_offset":1684,"parent_record_offset":1528,"ancestor_record_offset":1140,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3597,"record_offset":2868,"parent_record_offset":2812,"ancestor_record_offset":1140,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3597,"record_offset":2572,"parent_record_offset":2548,"ancestor_record_offset":1140,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3597,"record_offset":1456,"parent_record_offset":1424,"ancestor_record_offset":1140,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3597,"record_offset":2840,"parent_record_offset":2812,"ancestor_record_offset":1140,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3597,"record_offset":2812,"parent_record_offset":2788,"ancestor_record_offset":1140,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3597,"record_offset":1528,"parent_record_offset":1488,"ancestor_record_offset":1140,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3597,"record_offset":1488,"parent_record_offset":1456,"ancestor_record_offset":1140,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3597,"record_offset":2788,"parent_record_offset":2764,"ancestor_record_offset":1140,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3597,"record_offset":1572,"parent_record_offset":1528,"ancestor_record_offset":1140,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3597,"record_offset":1320,"parent_record_offset":1288,"ancestor_record_offset":1140,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3597,"record_offset":1352,"parent_record_offset":1320,"ancestor_record_offset":1140,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3597,"record_offset":1384,"parent_record_offset":1352,"ancestor_record_offset":1140,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3597,"record_offset":1288,"parent_record_offset":1256,"ancestor_record_offset":1140,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3597,"record_offset":1424,"parent_record_offset":1384,"ancestor_record_offset":1140,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":3597,"record_offset":1596,"parent_record_offset":1572,"ancestor_record_offset":1140,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":3597,"record_offset":1644,"parent_record_offset":1620,"ancestor_record_offset":1140,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3597,"record_offset":1620,"parent_record_offset":1596,"ancestor_record_offset":1140,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3597,"record_offset":1812,"parent_record_offset":1384,"ancestor_record_offset":1140,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3597,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":1140,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/03597.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3597,"record_offset":2740,"parent_record_offset":2716,"ancestor_record_offset":1140,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03597.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 8; 过程引用数: 14
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["lang_lib::language_library::LanguageLibrary", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> >", "constructor": "core::option::Option", "arguments": ["alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >", "constructor": "enum2$", "arguments": ["core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::impl$5::load::closure$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>", "constructor": "arc_swap::strategy::hybrid::impl$5::load::closure$0", "arguments": ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >", "arc_swap::strategy::hybrid::DefaultConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>", "constructor": "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0", "arguments": ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >", "arc_swap::strategy::hybrid::DefaultConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::impl$2::pay_all::closure$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::debt::impl$2::pay_all::closure$0", "arguments": ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >", "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::impl$2::pay_all::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::debt::impl$2::pay_all::closure_env$0", "arguments": ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >", "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::list::LocalNode::with<tuple$<>,arc_swap::debt::impl$2::pay_all::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig> > >", "constructor": "arc_swap::debt::list::LocalNode::with", "arguments": ["tuple$<>", "arc_swap::debt::impl$2::pay_all::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::HybridProtection<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > > >", "constructor": "arc_swap::strategy::hybrid::HybridProtection", "arguments": ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::impl$5::load::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>", "constructor": "arc_swap::strategy::hybrid::impl$5::load::closure_env$0", "arguments": ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >", "arc_swap::strategy::hybrid::DefaultConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::list::LocalNode::with<arc_swap::strategy::hybrid::HybridProtection<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > > >,arc_swap::strategy::hybrid::impl$5::load::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::debt::list::LocalNode::with", "arguments": ["arc_swap::strategy::hybrid::HybridProtection<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > > >", "arc_swap::strategy::hybrid::impl$5::load::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lang_lib::language_library::LanguageLibrary", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::Guard<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::Guard", "arguments": ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >", "arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<arc_swap::Guard<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["arc_swap::Guard<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::ArcInner<lang_lib::language_library::LanguageLibrary>", "constructor": "alloc::sync::ArcInner", "arguments": ["lang_lib::language_library::LanguageLibrary"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::ArcInner<lang_lib::language_library::LanguageLibrary> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::ArcInner<lang_lib::language_library::LanguageLibrary>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$0::with_context<lang_lib::language_library::LanguageLibrary,anyhow::Error,alloc::string::String,lang_lib::load_locale_impl::closure_env$0>", "constructor": "anyhow::context::impl$0::with_context", "arguments": ["lang_lib::language_library::LanguageLibrary", "anyhow::Error", "alloc::string::String", "lang_lib::load_locale_impl::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: lang_lib::language_library::LanguageLibrary::get
// 信息 ID: e85c3370d56e2fc7633b500a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] lang_lib::lookup @ 0xdc1f77 -> 0x1dd72d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::language_library::LanguageLibrary::locale
// 信息 ID: 0e9c6ff422e0800d83d66dbf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] lang_lib::game_locale; module=720, inline_record=480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::language_library::LanguageLibrary::new
// 信息 ID: 69d0d92b679db70b7783a1c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] lang_lib::language_library::LanguageLibrary::try_from_yml; module=2107, inline_record=2636; 不是独立调用边。
// [PDB 内联宿主] lang_lib::language_library::LanguageLibrary::try_from_yml; module=2107, inline_record=3456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::language_library::LanguageLibrary::try_from_yml
// 信息 ID: 16693596572e0e7a3de4179e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] lang_lib::load_locale_impl @ 0xdc11f3 -> 0x1dd6e70 (direct_call); 同地址别名未裁定。
// [静态直接引用] lang_lib::load_locale_impl @ 0xdc1713 -> 0x1dd6e70 (direct_call); 同地址别名未裁定。
// [静态直接引用] lang_lib::load_locale_impl @ 0xdc140c -> 0x1dd6e70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::language_library::LanguageLibrary::try_get
// 信息 ID: d81cce7125db8a32a7227cc7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] lang_lib::lookup; module=720, inline_record=15380; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::language_library::impl$0::try_from_yml::closure$0
// 信息 ID: 94e92105a1faa797f48afef7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "lang_lib::language_library::impl$0::try_from_yml", "captures": "unknown"}
// [闭包环境类型引用] lang_lib::language_library::impl$0::try_from_yml::closure_env$0 <- core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,ref$<str$> >,hashlink::linked_hash_map::Iter<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> >,lang_lib::language_library::impl$0::try_from_yml::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,ref$<str$> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,ref$<str$>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<hashlink::linked_hash_map::Iter<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> >,lang_lib::language_library::impl$0::try_from_yml::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] lang_lib::language_library::impl$0::try_from_yml::closure_env$0 <- core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,ref$<str$> >,hashlink::linked_hash_map::Iter<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> >,lang_lib::language_library::impl$0::try_from_yml::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,ref$<str$> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,ref$<str$>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<hashlink::linked_hash_map::Iter<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> >,lang_lib::language_library::impl$0::try_from_yml::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,ref$<str$> >,hashlink::linked_hash_map::Iter<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> >,lang_lib::language_library::impl$0::try_from_yml::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,ref$<str$> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,ref$<str$>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<hashlink::linked_hash_map::Iter<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> >,lang_lib::language_library::impl$0::try_from_yml::closure_env$0> > > >; module=3597, inline_record=1140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

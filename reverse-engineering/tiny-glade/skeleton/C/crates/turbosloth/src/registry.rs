// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/turbosloth/src/registry.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 72ee4edd54fe1e7625c748f128095f0d5f697b482a7fe21b7259bd14e037166b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: registry.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 3241, 4407
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use inventory; // namespace candidate
//   package identities: [{"name":"inventory","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.3.24"}]
// use std; // namespace candidate
//   package identities: []
// use turbosloth; // namespace candidate
//   package identities: [{"name":"turbosloth","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: turbosloth::registry::registered_version
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3241,"record_offset":264,"rva":39330368,"va":5408039488}
// 编译实例 RVAs: 0x2582240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::rwlock::futex::RwLock>::read_contended
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8576360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408039488,"raw_hex":"e868dd8200","size":5,"soft_fail":false,"target_va":5416616320,"va":5408039955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::rwlock::futex::RwLock>::wake_writer_or_readers
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8577011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408039488,"raw_hex":"e8f3df8200","size":5,"soft_fail":false,"target_va":5416617072,"va":5408040056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34558744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408039488,"raw_hex":"e8e8acf0fd","size":5,"soft_fail":false,"target_va":5373480896,"va":5408039635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e19041
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9006150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408039488,"raw_hex":"e8466c8900","size":5,"soft_fail":false,"target_va":5417046081,"va":5408039926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9017273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408039488,"raw_hex":"e8b9978900","size":5,"soft_fail":false,"target_va":5417057040,"va":5408039762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8581595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408039488,"raw_hex":"e8dbf18200","size":5,"soft_fail":false,"target_va":5416621632,"va":5408040032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3241,"record_offset":2368,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3241,"record_offset":2396,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3241,"record_offset":1080,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3241,"record_offset":1320,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::rwlock::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\rwlock.rs
//     evidence={"module":3241,"record_offset":676,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7930,"source_line":408,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::registry::versions
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/registry.rs
//     evidence={"module":3241,"record_offset":408,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":118457,"source_line":34,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3241,"record_offset":2028,"parent_record_offset":2000,"ancestor_record_offset":264,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3241,"record_offset":1588,"parent_record_offset":1564,"ancestor_record_offset":264,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3241,"record_offset":1672,"parent_record_offset":1648,"ancestor_record_offset":264,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":3241,"record_offset":2196,"parent_record_offset":1452,"ancestor_record_offset":264,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":3241,"record_offset":1820,"parent_record_offset":1792,"ancestor_record_offset":264,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":3241,"record_offset":1792,"parent_record_offset":1764,"ancestor_record_offset":264,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3241,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":264,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3241,"record_offset":1124,"parent_record_offset":1080,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3241,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":264,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3241,"record_offset":2112,"parent_record_offset":2084,"ancestor_record_offset":264,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3241,"record_offset":2140,"parent_record_offset":2112,"ancestor_record_offset":264,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3241,"record_offset":2056,"parent_record_offset":2028,"ancestor_record_offset":264,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3241,"record_offset":1000,"parent_record_offset":972,"ancestor_record_offset":264,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3241,"record_offset":856,"parent_record_offset":716,"ancestor_record_offset":264,"inlinee":7934,"source_line":3097,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3241,"record_offset":1244,"parent_record_offset":1208,"ancestor_record_offset":264,"inlinee":4762,"source_line":3165,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3241,"record_offset":592,"parent_record_offset":568,"ancestor_record_offset":264,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3241,"record_offset":880,"parent_record_offset":856,"ancestor_record_offset":264,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3241,"record_offset":616,"parent_record_offset":592,"ancestor_record_offset":264,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3241,"record_offset":1268,"parent_record_offset":1244,"ancestor_record_offset":264,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":3241,"record_offset":1732,"parent_record_offset":1704,"ancestor_record_offset":264,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":3241,"record_offset":1764,"parent_record_offset":1732,"ancestor_record_offset":264,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":3241,"record_offset":1704,"parent_record_offset":1452,"ancestor_record_offset":264,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":3241,"record_offset":1564,"parent_record_offset":1452,"ancestor_record_offset":264,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":3241,"record_offset":2224,"parent_record_offset":1452,"ancestor_record_offset":264,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":3241,"record_offset":1648,"parent_record_offset":1452,"ancestor_record_offset":264,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":3241,"record_offset":1508,"parent_record_offset":1452,"ancestor_record_offset":264,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":3241,"record_offset":2000,"parent_record_offset":1972,"ancestor_record_offset":264,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":3241,"record_offset":1344,"parent_record_offset":1320,"ancestor_record_offset":264,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":3241,"record_offset":1972,"parent_record_offset":1864,"ancestor_record_offset":264,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":3241,"record_offset":1376,"parent_record_offset":1344,"ancestor_record_offset":264,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3241,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":264,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3241,"record_offset":2316,"parent_record_offset":1452,"ancestor_record_offset":264,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3241,"record_offset":1888,"parent_record_offset":1864,"ancestor_record_offset":264,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3241,"record_offset":1428,"parent_record_offset":1404,"ancestor_record_offset":264,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3241,"record_offset":1404,"parent_record_offset":1344,"ancestor_record_offset":264,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3241,"record_offset":1452,"parent_record_offset":1428,"ancestor_record_offset":264,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3241,"record_offset":1536,"parent_record_offset":1452,"ancestor_record_offset":264,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3241,"record_offset":1864,"parent_record_offset":1452,"ancestor_record_offset":264,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":3241,"record_offset":544,"parent_record_offset":520,"ancestor_record_offset":264,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":3241,"record_offset":496,"parent_record_offset":464,"ancestor_record_offset":264,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":3241,"record_offset":436,"parent_record_offset":408,"ancestor_record_offset":264,"inlinee":14249,"source_line":317,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get_or_try_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":3241,"record_offset":464,"parent_record_offset":436,"ancestor_record_offset":264,"inlinee":14250,"source_line":398,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":3241,"record_offset":520,"parent_record_offset":496,"ancestor_record_offset":264,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":3241,"record_offset":944,"parent_record_offset":916,"ancestor_record_offset":264,"inlinee":4343,"source_line":112,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":3241,"record_offset":972,"parent_record_offset":944,"ancestor_record_offset":264,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLockReadGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\rwlock.rs
//     evidence={"module":3241,"record_offset":916,"parent_record_offset":676,"ancestor_record_offset":264,"inlinee":7936,"source_line":740,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::impl$19::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\rwlock.rs
//     evidence={"module":3241,"record_offset":1180,"parent_record_offset":1152,"ancestor_record_offset":264,"inlinee":4759,"source_line":1119,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":3241,"record_offset":568,"parent_record_offset":544,"ancestor_record_offset":264,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":3241,"record_offset":716,"parent_record_offset":676,"ancestor_record_offset":264,"inlinee":7931,"source_line":94,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read_unlock
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":3241,"record_offset":1208,"parent_record_offset":1180,"ancestor_record_offset":264,"inlinee":4761,"source_line":110,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::is_read_lockable
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":3241,"record_offset":824,"parent_record_offset":716,"ancestor_record_offset":264,"inlinee":7933,"source_line":47,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::registry::versions
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CEB9
// 首条位置: {"module":3241,"inlinee":118457}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::sync::once_lock::OnceLock::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":3241,"record_offset":436,"parent_record_offset":408,"ancestor_record_offset":408,"inlinee":14249,"source_line":317,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3241,"record_offset":592,"parent_record_offset":568,"ancestor_record_offset":408,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3241,"record_offset":616,"parent_record_offset":592,"ancestor_record_offset":408,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":3241,"record_offset":544,"parent_record_offset":520,"ancestor_record_offset":408,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":3241,"record_offset":496,"parent_record_offset":464,"ancestor_record_offset":408,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get_or_try_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":3241,"record_offset":464,"parent_record_offset":436,"ancestor_record_offset":408,"inlinee":14250,"source_line":398,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":3241,"record_offset":520,"parent_record_offset":496,"ancestor_record_offset":408,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":3241,"record_offset":568,"parent_record_offset":544,"ancestor_record_offset":408,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/03241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::registry::versions::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x22007
// 首条位置: {"module":4407,"inlinee":139271}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] inventory::_::impl$0::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"inventory","version":"0.3.24","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/inventory-0.3.24/src/lib.rs
//     evidence={"module":4407,"record_offset":1772,"parent_record_offset":992,"ancestor_record_offset":992,"inlinee":139273,"source_line":344,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_direct] inventory::_::impl$2::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"inventory","version":"0.3.24","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/inventory-0.3.24/src/lib.rs
//     evidence={"module":4407,"record_offset":1884,"parent_record_offset":992,"ancestor_record_offset":992,"inlinee":139277,"source_line":365,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4407,"record_offset":1944,"parent_record_offset":992,"ancestor_record_offset":992,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4407,"record_offset":1048,"parent_record_offset":992,"ancestor_record_offset":992,"inlinee":6553,"source_line":271,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4407,"record_offset":1624,"parent_record_offset":1600,"ancestor_record_offset":992,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4407,"record_offset":1528,"parent_record_offset":1324,"ancestor_record_offset":992,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":4407,"record_offset":1492,"parent_record_offset":1468,"ancestor_record_offset":992,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4407,"record_offset":1652,"parent_record_offset":1600,"ancestor_record_offset":992,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4407,"record_offset":1236,"parent_record_offset":1204,"ancestor_record_offset":992,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4407,"record_offset":1356,"parent_record_offset":1324,"ancestor_record_offset":992,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4407,"record_offset":1912,"parent_record_offset":1884,"ancestor_record_offset":992,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4407,"record_offset":1444,"parent_record_offset":1324,"ancestor_record_offset":992,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4407,"record_offset":1468,"parent_record_offset":1444,"ancestor_record_offset":992,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4407,"record_offset":1820,"parent_record_offset":1796,"ancestor_record_offset":992,"inlinee":4833,"source_line":1700,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4407,"record_offset":1844,"parent_record_offset":1820,"ancestor_record_offset":992,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_transitive] inventory::_::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"inventory","version":"0.3.24","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/inventory-0.3.24/src/lib.rs
//     evidence={"module":4407,"record_offset":1796,"parent_record_offset":1772,"ancestor_record_offset":992,"inlinee":139275,"source_line":331,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4407,"record_offset":1076,"parent_record_offset":1048,"ancestor_record_offset":992,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4407,"record_offset":1148,"parent_record_offset":1104,"ancestor_record_offset":992,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":4407,"record_offset":1404,"parent_record_offset":1380,"ancestor_record_offset":992,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":4407,"record_offset":1264,"parent_record_offset":1236,"ancestor_record_offset":992,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4407,"record_offset":1600,"parent_record_offset":1204,"ancestor_record_offset":992,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4407,"record_offset":1104,"parent_record_offset":1076,"ancestor_record_offset":992,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":4407,"record_offset":1292,"parent_record_offset":1264,"ancestor_record_offset":992,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":4407,"record_offset":1324,"parent_record_offset":1292,"ancestor_record_offset":992,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":4407,"record_offset":1204,"parent_record_offset":1176,"ancestor_record_offset":992,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/04407.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":4407,"record_offset":1176,"parent_record_offset":1148,"ancestor_record_offset":992,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/04407.jsonl.gz"}
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
// 完整结构及原始类型记录: registry.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] turbosloth::registry::RecipeRegistration
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "inventory::impl$1::submit<turbosloth::registry::RecipeRegistration>", "constructor": "inventory::impl$1::submit", "arguments": ["turbosloth::registry::RecipeRegistration"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: registry.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: registry.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: turbosloth::registry::registered_version
// 信息 ID: 0179db452482a7d1862ea28c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 15 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] turbosloth::asset::BuildContext::build<asset_pipe::texture::BuildTextureLayer> @ 0x170e7ef -> 0x2582240 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::asset::BuildContext::require<clutter_pack_pipe::ImportClutterGlb> @ 0x170ecb3 -> 0x2582240 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::asset::BuildContext::require<enum2$<asset_pipe::texture::BuildTextureAsset> > @ 0x170fdcf -> 0x2582240 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::asset::BuildContext::require<clutter_pack_pipe::BakeClutterIcon> @ 0x17103f5 -> 0x2582240 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::asset::BuildContext::require<clutter_pack_pipe::mod_icons::BakeModClutterIcon> @ 0x171096f -> 0x2582240 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::asset::BuildContext::require<clutter_pack_pipe::ExtractRoughnessChannel> @ 0x1710e1b -> 0x2582240 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::store<asset_pipe::texture::TextureAssetList> @ 0x1b0f153 -> 0x2582240 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::store<clutter_pack_pipe::ClutterObjectList> @ 0x1b10013 -> 0x2582240 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::store<clutter_pack_pipe::ClutterMeshAsset> @ 0x1b10ec3 -> 0x2582240 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::store<turbosloth::asset::FileAsset> @ 0x1b11d73 -> 0x2582240 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::store<clutter_pack_pipe::ClutterGlbImport> @ 0x1b12c23 -> 0x2582240 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::store<asset::texture::blob::TextureAssetBlob> @ 0x1b13ad3 -> 0x2582240 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::store<clutter_pack_pipe::ClutterIconImage> @ 0x1b14993 -> 0x2582240 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::store<asset::texture::SourceImageAsset> @ 0x1b15843 -> 0x2582240 (direct_call); 同地址别名未裁定。
// [静态直接引用] turbosloth::store::DiskStore::recipe_outcome_if_fresh @ 0x1b1f074 -> 0x2582240 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::registry::versions
// 信息 ID: 7f97a7f80fa90c3ba647557e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::registry::registered_version; module=3241, inline_record=408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::registry::versions::closure$0
// 信息 ID: a376c8b292021784ab0ee169
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "turbosloth::registry::versions", "captures": "unknown"}
// [闭包环境类型引用] turbosloth::registry::versions::closure_env$0 <- std::sync::once::impl$2::call_once_force::closure$0<std::sync::once_lock::impl$0::initialize::closure_env$0<std::sync::poison::rwlock::RwLock<std::collections::hash::map::HashMap<ref$<str$>,usize,std::hash::random::RandomState,alloc::alloc::Global> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::rwlock::RwLock<std::collections::hash::map::HashMap<ref$<str$>,usize,std::hash::random::RandomState,alloc::alloc::Global> >,turbosloth::registry::versions::closure_env$0>,never$> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] turbosloth::registry::versions::closure_env$0 <- std::sync::once_lock::OnceLock::initialize<std::sync::poison::rwlock::RwLock<std::collections::hash::map::HashMap<ref$<str$>,usize,std::hash::random::RandomState,alloc::alloc::Global> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::rwlock::RwLock<std::collections::hash::map::HashMap<ref$<str$>,usize,std::hash::random::RandomState,alloc::alloc::Global> >,turbosloth::registry::versions::closure_env$0>,never$>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] turbosloth::registry::versions::closure_env$0 <- core::ops::function::FnOnce::call_once<std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<std::sync::poison::rwlock::RwLock<std::collections::hash::map::HashMap<ref$<str$>,usize,std::hash::random::RandomState,alloc::alloc::Global> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::rwlock::RwLock<std::collections::hash::map::HashMap<ref$<str$>,usize,std::hash::random::RandomState,alloc::alloc::Global> >,turbosloth::registry::versions::closure_env$0>,never$> >,tuple$<ref$<std::sync::once::OnceState> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] std::sync::once::impl$2::call_once_force::closure$0<std::sync::once_lock::impl$0::initialize::closure_env$0<std::sync::poison::rwlock::RwLock<std::collections::hash::map::HashMap<ref$<str$>,usize,std::hash::random::RandomState,alloc::alloc::Global> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::rwlock::RwLock<std::collections::hash::map::HashMap<ref$<str$>,usize,std::hash::random::RandomState,alloc::alloc::Global> >,turbosloth::registry::versions::closure_env$0>,never$> >; module=4407, inline_record=992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

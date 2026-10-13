// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tg-package-integrity/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: b52f8642f8d60fddef21bdefeb7e0c285b7dcb8d269560720f8c73d127291fc0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 327, 1160, 1162, 1163, 1164, 1165
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use blake3; // namespace candidate
//   package identities: [{"name":"blake3","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.5.3"}]
// use core; // namespace candidate
//   package identities: []
// use easy_parallel; // namespace candidate
//   package identities: [{"name":"easy-parallel","source":"registry+https://github.com/rust-lang/crates.io-index","version":"3.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use relative_path; // namespace candidate
//   package identities: [{"name":"relative-path","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.9.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use tg_package_integrity; // namespace candidate
//   package identities: [{"name":"tg-package-integrity","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::verify_package_manifest
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":327,"record_offset":1812,"rva":6800336,"va":5375509456}
// 编译实例 RVAs: 0x67c3d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7336099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375509456,"raw_hex":"e8a3f06f00","size":5,"soft_fail":false,"target_va":5382847888,"va":5375511784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5286417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375509456,"raw_hex":"e811aa5000","size":5,"soft_fail":false,"target_va":5380796496,"va":5375510074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..io..error..Error$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7203697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375509456,"raw_hex":"e871eb6d00","size":5,"soft_fail":false,"target_va":5382713360,"va":5375509658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004e480
//     owner=unresolved_namespace; count=4
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6481848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375509456,"raw_hex":"e848189dff","size":5,"soft_fail":false,"target_va":5369029760,"va":5375511603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d1240
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-704222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375509456,"raw_hex":"e82241f5ff","size":5,"soft_fail":false,"target_va":5374808640,"va":5375512857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-609690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375509456,"raw_hex":"e866b2f6ff","size":5,"soft_fail":false,"target_va":5374900736,"va":5375510421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e88a0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-607843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375509456,"raw_hex":"e89db9f6ff","size":5,"soft_fail":false,"target_va":5374904480,"va":5375512318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f8f70
//     owner=unresolved_namespace; count=6
//     target_aliases=18 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-539907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375509456,"raw_hex":"e8fdc2f7ff","size":5,"soft_fail":false,"target_va":5374971760,"va":5375511662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=18
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-428272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375509456,"raw_hex":"e81077f9ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5375509771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f739a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9401322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375509456,"raw_hex":"e8ea738f00","size":5,"soft_fail":false,"target_va":5384911264,"va":5375509937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f76810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9412362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375509456,"raw_hex":"e80a9f8f00","size":5,"soft_fail":false,"target_va":5384923152,"va":5375510785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f77ad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9417178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375509456,"raw_hex":"e8dab18f00","size":5,"soft_fail":false,"target_va":5384927952,"va":5375510769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f79650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9423578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375509456,"raw_hex":"e8daca8f00","size":5,"soft_fail":false,"target_va":5384934992,"va":5375511409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1154164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375509456,"raw_hex":"e88c63eeff","size":5,"soft_fail":false,"target_va":5374356912,"va":5375511071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7576359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375509456,"raw_hex":"e8279b7300","size":5,"soft_fail":false,"target_va":5383086224,"va":5375509860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::array::_$LT$impl$u20$core..hash..Hash$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$::hash
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9400934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375509456,"raw_hex":"e866728f00","size":5,"soft_fail":false,"target_va":5384910832,"va":5375509893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$tg_package_integrity..verify_package_manifest..CheckOutcome$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9407656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375509456,"raw_hex":"e8a88c8f00","size":5,"soft_fail":false,"target_va":5384920240,"va":5375512579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36031024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375509456,"raw_hex":"e830ca2502","size":5,"soft_fail":false,"target_va":5411542880,"va":5375511851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::map::HashMap$LT$K$C$V$C$S$C$A$GT$::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9427624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375509456,"raw_hex":"e8a8da8f00","size":5,"soft_fail":false,"target_va":5384939424,"va":5375511795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::RawTable$LT$T$C$A$GT$::reserve_rehash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36078935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375509456,"raw_hex":"e857852602","size":5,"soft_fail":false,"target_va":5411590688,"va":5375511748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7195294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375509456,"raw_hex":"e89eca6d00","size":5,"soft_fail":false,"target_va":5382704816,"va":5375509517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7147628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375509456,"raw_hex":"e86c106d00","size":5,"soft_fail":false,"target_va":5382657856,"va":5375510223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7243226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375509456,"raw_hex":"e8da856e00","size":5,"soft_fail":false,"target_va":5382753792,"va":5375510561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5375509456,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5375509742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":327,"record_offset":8784,"parent_record_offset":1812,"ancestor_record_offset":1812,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":327,"record_offset":7116,"parent_record_offset":1812,"ancestor_record_offset":1812,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":327,"record_offset":7392,"parent_record_offset":1812,"ancestor_record_offset":1812,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":327,"record_offset":7172,"parent_record_offset":1812,"ancestor_record_offset":1812,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":327,"record_offset":7228,"parent_record_offset":1812,"ancestor_record_offset":1812,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":327,"record_offset":4400,"parent_record_offset":1812,"ancestor_record_offset":1812,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":327,"record_offset":5864,"parent_record_offset":1812,"ancestor_record_offset":1812,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":327,"record_offset":2024,"parent_record_offset":1812,"ancestor_record_offset":1812,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":327,"record_offset":6668,"parent_record_offset":1812,"ancestor_record_offset":1812,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":327,"record_offset":3332,"parent_record_offset":1812,"ancestor_record_offset":1812,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_direct] easy_parallel::Parallel::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"easy-parallel","version":"3.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/easy-parallel-3.2.0/src/lib.rs
//     evidence={"module":327,"record_offset":7088,"parent_record_offset":1812,"ancestor_record_offset":1812,"inlinee":19622,"source_line":85,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_direct] relative_path::RelativePathBuf::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"1.9.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-1.9.0/src/lib.rs
//     evidence={"module":327,"record_offset":8756,"parent_record_offset":1812,"ancestor_record_offset":1812,"inlinee":19624,"source_line":638,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":327,"record_offset":1996,"parent_record_offset":1812,"ancestor_record_offset":1812,"inlinee":12456,"source_line":267,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_direct] tg_package_integrity::is_valid_manifest_checksum
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":327,"record_offset":3360,"parent_record_offset":1812,"ancestor_record_offset":1812,"inlinee":19615,"source_line":31,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":327,"record_offset":2464,"parent_record_offset":2440,"ancestor_record_offset":1812,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":327,"record_offset":2440,"parent_record_offset":2384,"ancestor_record_offset":1812,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":327,"record_offset":2836,"parent_record_offset":2808,"ancestor_record_offset":1812,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":327,"record_offset":7996,"parent_record_offset":7964,"ancestor_record_offset":1812,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":327,"record_offset":2412,"parent_record_offset":2384,"ancestor_record_offset":1812,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":327,"record_offset":2384,"parent_record_offset":2360,"ancestor_record_offset":1812,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":327,"record_offset":8044,"parent_record_offset":8020,"ancestor_record_offset":1812,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":327,"record_offset":8020,"parent_record_offset":7996,"ancestor_record_offset":1812,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":327,"record_offset":2360,"parent_record_offset":2336,"ancestor_record_offset":1812,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":327,"record_offset":2196,"parent_record_offset":2148,"ancestor_record_offset":1812,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":327,"record_offset":2112,"parent_record_offset":2076,"ancestor_record_offset":1812,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":327,"record_offset":2148,"parent_record_offset":2112,"ancestor_record_offset":1812,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":327,"record_offset":7964,"parent_record_offset":7932,"ancestor_record_offset":1812,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":327,"record_offset":5888,"parent_record_offset":5864,"ancestor_record_offset":1812,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":327,"record_offset":8140,"parent_record_offset":8116,"ancestor_record_offset":1812,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":327,"record_offset":8484,"parent_record_offset":8460,"ancestor_record_offset":1812,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] blake3::impl$22::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":327,"record_offset":3416,"parent_record_offset":3388,"ancestor_record_offset":1812,"inlinee":19619,"source_line":221,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":327,"record_offset":2868,"parent_record_offset":2836,"ancestor_record_offset":1812,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":327,"record_offset":5008,"parent_record_offset":4976,"ancestor_record_offset":1812,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":327,"record_offset":5068,"parent_record_offset":5036,"ancestor_record_offset":1812,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":327,"record_offset":2224,"parent_record_offset":2148,"ancestor_record_offset":1812,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":327,"record_offset":5352,"parent_record_offset":5320,"ancestor_record_offset":1812,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":327,"record_offset":5412,"parent_record_offset":5388,"ancestor_record_offset":1812,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":327,"record_offset":5520,"parent_record_offset":5460,"ancestor_record_offset":1812,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":327,"record_offset":5388,"parent_record_offset":5260,"ancestor_record_offset":1812,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":327,"record_offset":5596,"parent_record_offset":5572,"ancestor_record_offset":1812,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":327,"record_offset":4864,"parent_record_offset":4840,"ancestor_record_offset":1812,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":327,"record_offset":2892,"parent_record_offset":2868,"ancestor_record_offset":1812,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":327,"record_offset":5660,"parent_record_offset":5460,"ancestor_record_offset":1812,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":327,"record_offset":6560,"parent_record_offset":6532,"ancestor_record_offset":1812,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":327,"record_offset":5036,"parent_record_offset":4976,"ancestor_record_offset":1812,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":327,"record_offset":4612,"parent_record_offset":4572,"ancestor_record_offset":1812,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":327,"record_offset":4728,"parent_record_offset":4700,"ancestor_record_offset":1812,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=56
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":327,"record_offset":2288,"parent_record_offset":2148,"ancestor_record_offset":1812,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":327,"record_offset":7196,"parent_record_offset":7172,"ancestor_record_offset":1812,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":327,"record_offset":4816,"parent_record_offset":4700,"ancestor_record_offset":1812,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":327,"record_offset":2636,"parent_record_offset":2608,"ancestor_record_offset":1812,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":327,"record_offset":2684,"parent_record_offset":2660,"ancestor_record_offset":1812,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":327,"record_offset":2660,"parent_record_offset":2636,"ancestor_record_offset":1812,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":327,"record_offset":5492,"parent_record_offset":5460,"ancestor_record_offset":1812,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":327,"record_offset":7324,"parent_record_offset":7228,"ancestor_record_offset":1812,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":327,"record_offset":7288,"parent_record_offset":7228,"ancestor_record_offset":1812,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":327,"record_offset":4892,"parent_record_offset":4864,"ancestor_record_offset":1812,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":327,"record_offset":4840,"parent_record_offset":4816,"ancestor_record_offset":1812,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":327,"record_offset":4920,"parent_record_offset":4864,"ancestor_record_offset":1812,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":327,"record_offset":2252,"parent_record_offset":2148,"ancestor_record_offset":1812,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":327,"record_offset":5460,"parent_record_offset":5436,"ancestor_record_offset":1812,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":327,"record_offset":5292,"parent_record_offset":5260,"ancestor_record_offset":1812,"inlinee":6916,"source_line":1103,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":327,"record_offset":5260,"parent_record_offset":5232,"ancestor_record_offset":1812,"inlinee":6915,"source_line":4480,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":327,"record_offset":5620,"parent_record_offset":5596,"ancestor_record_offset":1812,"inlinee":6918,"source_line":4492,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":327,"record_offset":5320,"parent_record_offset":5292,"ancestor_record_offset":1812,"inlinee":5655,"source_line":896,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":327,"record_offset":5812,"parent_record_offset":5788,"ancestor_record_offset":1812,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":327,"record_offset":5232,"parent_record_offset":5204,"ancestor_record_offset":1812,"inlinee":6932,"source_line":1307,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":327,"record_offset":5176,"parent_record_offset":4440,"ancestor_record_offset":1812,"inlinee":13714,"source_line":392,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$7::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":327,"record_offset":4440,"parent_record_offset":4400,"ancestor_record_offset":1812,"inlinee":13713,"source_line":1067,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":327,"record_offset":5204,"parent_record_offset":4440,"ancestor_record_offset":1812,"inlinee":13716,"source_line":1117,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":327,"record_offset":4516,"parent_record_offset":4488,"ancestor_record_offset":1812,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":327,"record_offset":4776,"parent_record_offset":4752,"ancestor_record_offset":1812,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":327,"record_offset":4640,"parent_record_offset":4612,"ancestor_record_offset":1812,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":327,"record_offset":4976,"parent_record_offset":4572,"ancestor_record_offset":1812,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":327,"record_offset":4488,"parent_record_offset":4440,"ancestor_record_offset":1812,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":327,"record_offset":2608,"parent_record_offset":2580,"ancestor_record_offset":1812,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":327,"record_offset":2580,"parent_record_offset":2552,"ancestor_record_offset":1812,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":327,"record_offset":4668,"parent_record_offset":4640,"ancestor_record_offset":1812,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":327,"record_offset":4700,"parent_record_offset":4668,"ancestor_record_offset":1812,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":327,"record_offset":4572,"parent_record_offset":4544,"ancestor_record_offset":1812,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":327,"record_offset":4544,"parent_record_offset":4516,"ancestor_record_offset":1812,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] tg_package_integrity::is_valid_manifest_checksum::calculate_fnv_hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":327,"record_offset":3388,"parent_record_offset":3360,"ancestor_record_offset":1812,"inlinee":19617,"source_line":32,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] tg_package_integrity::verify_package_manifest::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":327,"record_offset":2076,"parent_record_offset":2024,"ancestor_record_offset":1812,"inlinee":19613,"source_line":46,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] tg_package_integrity::verify_package_manifest::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":327,"record_offset":3504,"parent_record_offset":3452,"ancestor_record_offset":1812,"inlinee":19620,"source_line":55,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] tg_package_integrity::verify_package_manifest::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":327,"record_offset":5544,"parent_record_offset":5520,"ancestor_record_offset":1812,"inlinee":19625,"source_line":64,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_json::de::Deserializer::parse_exponent<serde_json::read::SliceRead>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1160,"record_offset":5396,"rva":360976,"va":5369070096}
// 编译实例 RVAs: 0x58210
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a0fa0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42240917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369070096,"raw_hex":"e8958b8402","size":5,"soft_fail":false,"target_va":5411311520,"va":5369070598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a4c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42256808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369070096,"raw_hex":"e8a8c98402","size":5,"soft_fail":false,"target_va":5411327120,"va":5369070307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$2::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":1160,"record_offset":5984,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":6492,"source_line":1668,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$2::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":1160,"record_offset":6012,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":6493,"source_line":1714,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":5664,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::f64_from_parts
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":6040,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":6482,"source_line":562,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::next_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":5768,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":6476,"source_line":224,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::peek_or_null
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":5576,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":6480,"source_line":216,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_infinite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":1160,"record_offset":6232,"parent_record_offset":6040,"ancestor_record_offset":5396,"inlinee":6486,"source_line":546,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::wrapping_abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":1160,"record_offset":6108,"parent_record_offset":6040,"ancestor_record_offset":5396,"inlinee":6484,"source_line":2192,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1160,"record_offset":6196,"parent_record_offset":6164,"ancestor_record_offset":5396,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1160,"record_offset":6164,"parent_record_offset":6108,"ancestor_record_offset":5396,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":5600,"parent_record_offset":5576,"ancestor_record_offset":5396,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1160,"record_offset":5684,"parent_record_offset":5664,"ancestor_record_offset":5396,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1160,"record_offset":5792,"parent_record_offset":5768,"ancestor_record_offset":5396,"inlinee":6477,"source_line":489,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1160,"record_offset":5624,"parent_record_offset":5600,"ancestor_record_offset":5396,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1160,"record_offset":2868,"rva":6374144,"va":5375083264}
// 编译实例 RVAs: 0x614300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a0920
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36225908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375083264,"raw_hex":"e874c32802","size":5,"soft_fail":false,"target_va":5411309856,"va":5375083943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a0fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36227724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375083264,"raw_hex":"e88cca2802","size":5,"soft_fail":false,"target_va":5411311520,"va":5375083791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a4990
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36242800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375083264,"raw_hex":"e870052902","size":5,"soft_fail":false,"target_va":5411326352,"va":5375083547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::de::Deserializer::parse_decimal<serde_json::read::SliceRead>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-1134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375083264,"raw_hex":"e892fbffff","size":5,"soft_fail":false,"target_va":5375082688,"va":5375083817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::de::Deserializer::parse_exponent<serde_json::read::SliceRead>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6013291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375083264,"raw_hex":"e8953ea4ff","size":5,"soft_fail":false,"target_va":5369070096,"va":5375083382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":3408,"parent_record_offset":2868,"ancestor_record_offset":2868,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::next_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":3036,"parent_record_offset":2868,"ancestor_record_offset":2868,"inlinee":6476,"source_line":224,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::parse_number
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":3176,"parent_record_offset":2868,"ancestor_record_offset":2868,"inlinee":6728,"source_line":435,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::peek_or_null
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":3092,"parent_record_offset":2868,"ancestor_record_offset":2868,"inlinee":6480,"source_line":216,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":3116,"parent_record_offset":3092,"ancestor_record_offset":2868,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1160,"record_offset":3432,"parent_record_offset":3408,"ancestor_record_offset":2868,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1160,"record_offset":3056,"parent_record_offset":3036,"ancestor_record_offset":2868,"inlinee":6477,"source_line":489,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1160,"record_offset":3140,"parent_record_offset":3116,"ancestor_record_offset":2868,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF087
// 首条位置: {"module":1160,"inlinee":61575}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF08B
// 首条位置: {"module":1160,"inlinee":61579}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1160,"record_offset":21944,"parent_record_offset":21916,"ancestor_record_offset":21916,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1160,"record_offset":21968,"parent_record_offset":21944,"ancestor_record_offset":21916,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1160,"record_offset":21992,"parent_record_offset":21968,"ancestor_record_offset":21916,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1160,"record_offset":22016,"parent_record_offset":21992,"ancestor_record_offset":21916,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF08A
// 首条位置: {"module":1160,"inlinee":61578}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1160,"record_offset":21756,"parent_record_offset":21728,"ancestor_record_offset":21728,"inlinee":6604,"source_line":126,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1160,"record_offset":21888,"parent_record_offset":21784,"ancestor_record_offset":21728,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1160,"record_offset":21968,"parent_record_offset":21944,"ancestor_record_offset":21728,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1160,"record_offset":21992,"parent_record_offset":21968,"ancestor_record_offset":21728,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1160,"record_offset":22016,"parent_record_offset":21992,"ancestor_record_offset":21728,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1160,"record_offset":21944,"parent_record_offset":21916,"ancestor_record_offset":21728,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":21832,"parent_record_offset":21784,"ancestor_record_offset":21728,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":21784,"parent_record_offset":21756,"ancestor_record_offset":21728,"inlinee":6421,"source_line":2151,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1160,"record_offset":21856,"parent_record_offset":21832,"ancestor_record_offset":21728,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] tg_package_integrity::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":1160,"record_offset":21916,"parent_record_offset":21784,"ancestor_record_offset":21728,"inlinee":61579,"source_line":24,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4CAE
// 首条位置: {"module":1163,"inlinee":19630}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF088
// 首条位置: {"module":1160,"inlinee":61576}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1160,"record_offset":22200,"parent_record_offset":21072,"ancestor_record_offset":21072,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1160,"record_offset":25944,"parent_record_offset":21072,"ancestor_record_offset":21072,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1160,"record_offset":25876,"parent_record_offset":21072,"ancestor_record_offset":21072,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1160,"record_offset":26776,"parent_record_offset":21072,"ancestor_record_offset":21072,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1160,"record_offset":21144,"parent_record_offset":21072,"ancestor_record_offset":21072,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1160,"record_offset":22232,"parent_record_offset":21072,"ancestor_record_offset":21072,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1160,"record_offset":26276,"parent_record_offset":26252,"ancestor_record_offset":21072,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1160,"record_offset":26252,"parent_record_offset":26188,"ancestor_record_offset":21072,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1160,"record_offset":24464,"parent_record_offset":24432,"ancestor_record_offset":21072,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1160,"record_offset":26220,"parent_record_offset":26188,"ancestor_record_offset":21072,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1160,"record_offset":26188,"parent_record_offset":26160,"ancestor_record_offset":21072,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1160,"record_offset":24512,"parent_record_offset":24488,"ancestor_record_offset":21072,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1160,"record_offset":24488,"parent_record_offset":24464,"ancestor_record_offset":21072,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1160,"record_offset":26160,"parent_record_offset":26132,"ancestor_record_offset":21072,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1160,"record_offset":24432,"parent_record_offset":24400,"ancestor_record_offset":21072,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1160,"record_offset":21888,"parent_record_offset":21784,"ancestor_record_offset":21072,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1160,"record_offset":24400,"parent_record_offset":23120,"ancestor_record_offset":21072,"inlinee":6530,"source_line":2503,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1160,"record_offset":23628,"parent_record_offset":23120,"ancestor_record_offset":21072,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1160,"record_offset":26000,"parent_record_offset":25972,"ancestor_record_offset":21072,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1160,"record_offset":21968,"parent_record_offset":21944,"ancestor_record_offset":21072,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1160,"record_offset":26672,"parent_record_offset":26644,"ancestor_record_offset":21072,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1160,"record_offset":26024,"parent_record_offset":26000,"ancestor_record_offset":21072,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1160,"record_offset":24836,"parent_record_offset":24804,"ancestor_record_offset":21072,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1160,"record_offset":21992,"parent_record_offset":21968,"ancestor_record_offset":21072,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1160,"record_offset":22016,"parent_record_offset":21992,"ancestor_record_offset":21072,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1160,"record_offset":21944,"parent_record_offset":21916,"ancestor_record_offset":21072,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1160,"record_offset":26848,"parent_record_offset":26824,"ancestor_record_offset":21072,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1160,"record_offset":26824,"parent_record_offset":26800,"ancestor_record_offset":21072,"inlinee":6672,"source_line":126,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1160,"record_offset":26936,"parent_record_offset":26912,"ancestor_record_offset":21072,"inlinee":28015,"source_line":126,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1160,"record_offset":23032,"parent_record_offset":22988,"ancestor_record_offset":21072,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1160,"record_offset":21700,"parent_record_offset":21192,"ancestor_record_offset":21072,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1160,"record_offset":25836,"parent_record_offset":25812,"ancestor_record_offset":21072,"inlinee":6469,"source_line":1150,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1160,"record_offset":26912,"parent_record_offset":26888,"ancestor_record_offset":21072,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":21416,"parent_record_offset":21280,"ancestor_record_offset":21072,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::ignore_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":23120,"parent_record_offset":23076,"ancestor_record_offset":21072,"inlinee":6520,"source_line":1023,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::next_char
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":23836,"parent_record_offset":23768,"ancestor_record_offset":21072,"inlinee":6476,"source_line":224,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_ident
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":23768,"parent_record_offset":23120,"ancestor_record_offset":21072,"inlinee":6495,"source_line":371,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":22316,"parent_record_offset":22272,"ancestor_record_offset":21072,"inlinee":5761,"source_line":982,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":21280,"parent_record_offset":21192,"ancestor_record_offset":21072,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":21340,"parent_record_offset":21280,"ancestor_record_offset":21072,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":21192,"parent_record_offset":21144,"ancestor_record_offset":21072,"inlinee":6601,"source_line":1963,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":22272,"parent_record_offset":22232,"ancestor_record_offset":21072,"inlinee":6516,"source_line":1996,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":21784,"parent_record_offset":21756,"ancestor_record_offset":21072,"inlinee":6421,"source_line":2151,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1160,"record_offset":21756,"parent_record_offset":21728,"ancestor_record_offset":21072,"inlinee":6604,"source_line":126,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$7::deserialize_ignored_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":23076,"parent_record_offset":23032,"ancestor_record_offset":21072,"inlinee":6519,"source_line":1893,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1160,"record_offset":21440,"parent_record_offset":21416,"ancestor_record_offset":21072,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1160,"record_offset":23872,"parent_record_offset":23836,"ancestor_record_offset":21072,"inlinee":6477,"source_line":489,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1160,"record_offset":21372,"parent_record_offset":21340,"ancestor_record_offset":21072,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] tg_package_integrity::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":1160,"record_offset":21916,"parent_record_offset":21784,"ancestor_record_offset":21072,"inlinee":61579,"source_line":24,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] tg_package_integrity::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":1160,"record_offset":21728,"parent_record_offset":21700,"ancestor_record_offset":21072,"inlinee":61578,"source_line":24,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF08C
// 首条位置: {"module":1160,"inlinee":61580}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1160,"record_offset":27112,"parent_record_offset":27060,"ancestor_record_offset":27060,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1160,"record_offset":27936,"parent_record_offset":27420,"ancestor_record_offset":27060,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1160,"record_offset":27960,"parent_record_offset":27936,"ancestor_record_offset":27060,"inlinee":6469,"source_line":1150,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":27304,"parent_record_offset":27172,"ancestor_record_offset":27060,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":27172,"parent_record_offset":27136,"ancestor_record_offset":27060,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":27232,"parent_record_offset":27172,"ancestor_record_offset":27060,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1160,"record_offset":27136,"parent_record_offset":27112,"ancestor_record_offset":27060,"inlinee":6533,"source_line":1916,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1160,"record_offset":27328,"parent_record_offset":27304,"ancestor_record_offset":27060,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1160,"record_offset":27264,"parent_record_offset":27232,"ancestor_record_offset":27060,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/01160.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::is_valid_manifest_checksum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4C9F
// 首条位置: {"module":327,"inlinee":19615}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tg_package_integrity::is_valid_manifest_checksum::calculate_fnv_hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-package-integrity/src/lib.rs
//     evidence={"module":327,"record_offset":3388,"parent_record_offset":3360,"ancestor_record_offset":3360,"inlinee":19617,"source_line":32,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] blake3::impl$22::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":327,"record_offset":3416,"parent_record_offset":3388,"ancestor_record_offset":3360,"inlinee":19619,"source_line":221,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::is_valid_manifest_checksum::calculate_fnv_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4CA1
// 首条位置: {"module":327,"inlinee":19617}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] blake3::impl$22::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":327,"record_offset":3416,"parent_record_offset":3388,"ancestor_record_offset":3388,"inlinee":19619,"source_line":221,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::verify_package_manifest::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4C9D
// 首条位置: {"module":327,"inlinee":19613}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":327,"record_offset":2112,"parent_record_offset":2076,"ancestor_record_offset":2076,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":327,"record_offset":2524,"parent_record_offset":2076,"ancestor_record_offset":2076,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":327,"record_offset":2464,"parent_record_offset":2440,"ancestor_record_offset":2076,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":327,"record_offset":2440,"parent_record_offset":2384,"ancestor_record_offset":2076,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":327,"record_offset":2836,"parent_record_offset":2808,"ancestor_record_offset":2076,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":327,"record_offset":2412,"parent_record_offset":2384,"ancestor_record_offset":2076,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":327,"record_offset":2384,"parent_record_offset":2360,"ancestor_record_offset":2076,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":327,"record_offset":2360,"parent_record_offset":2336,"ancestor_record_offset":2076,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":327,"record_offset":2196,"parent_record_offset":2148,"ancestor_record_offset":2076,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":327,"record_offset":2148,"parent_record_offset":2112,"ancestor_record_offset":2076,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":327,"record_offset":2868,"parent_record_offset":2836,"ancestor_record_offset":2076,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":327,"record_offset":2224,"parent_record_offset":2148,"ancestor_record_offset":2076,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":327,"record_offset":2892,"parent_record_offset":2868,"ancestor_record_offset":2076,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":327,"record_offset":2288,"parent_record_offset":2148,"ancestor_record_offset":2076,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":327,"record_offset":2636,"parent_record_offset":2608,"ancestor_record_offset":2076,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":327,"record_offset":2684,"parent_record_offset":2660,"ancestor_record_offset":2076,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":327,"record_offset":2660,"parent_record_offset":2636,"ancestor_record_offset":2076,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":327,"record_offset":2252,"parent_record_offset":2148,"ancestor_record_offset":2076,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":327,"record_offset":2608,"parent_record_offset":2580,"ancestor_record_offset":2076,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":327,"record_offset":2580,"parent_record_offset":2552,"ancestor_record_offset":2076,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::verify_package_manifest::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4CA4
// 首条位置: {"module":327,"inlinee":19620}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":327,"record_offset":3548,"parent_record_offset":3504,"ancestor_record_offset":3504,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":327,"record_offset":3964,"parent_record_offset":3504,"ancestor_record_offset":3504,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":327,"record_offset":3904,"parent_record_offset":3880,"ancestor_record_offset":3504,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":327,"record_offset":3880,"parent_record_offset":3824,"ancestor_record_offset":3504,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":327,"record_offset":4112,"parent_record_offset":4084,"ancestor_record_offset":3504,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":327,"record_offset":3852,"parent_record_offset":3824,"ancestor_record_offset":3504,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":327,"record_offset":3824,"parent_record_offset":3800,"ancestor_record_offset":3504,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":327,"record_offset":3800,"parent_record_offset":3776,"ancestor_record_offset":3504,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":327,"record_offset":3636,"parent_record_offset":3584,"ancestor_record_offset":3504,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":327,"record_offset":3584,"parent_record_offset":3548,"ancestor_record_offset":3504,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":327,"record_offset":3664,"parent_record_offset":3584,"ancestor_record_offset":3504,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":327,"record_offset":3728,"parent_record_offset":3584,"ancestor_record_offset":3504,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":327,"record_offset":3692,"parent_record_offset":3584,"ancestor_record_offset":3504,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00327.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::verify_package_manifest::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4CA9
// 首条位置: {"module":327,"inlinee":19625}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::verify_package_manifest::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF096
// 首条位置: {"module":1162,"inlinee":61590}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1162,"record_offset":10828,"parent_record_offset":9500,"ancestor_record_offset":9500,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$90::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1162,"record_offset":9600,"parent_record_offset":9500,"ancestor_record_offset":9500,"inlinee":26646,"source_line":2533,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1162,"record_offset":11244,"parent_record_offset":9500,"ancestor_record_offset":9500,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1162,"record_offset":9772,"parent_record_offset":9500,"ancestor_record_offset":9500,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1162,"record_offset":9744,"parent_record_offset":9500,"ancestor_record_offset":9500,"inlinee":12456,"source_line":267,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1162,"record_offset":10220,"parent_record_offset":10196,"ancestor_record_offset":9500,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1162,"record_offset":10196,"parent_record_offset":10112,"ancestor_record_offset":9500,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1162,"record_offset":10112,"parent_record_offset":10084,"ancestor_record_offset":9500,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1162,"record_offset":10724,"parent_record_offset":10696,"ancestor_record_offset":9500,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1162,"record_offset":10696,"parent_record_offset":10672,"ancestor_record_offset":9500,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1162,"record_offset":10672,"parent_record_offset":10648,"ancestor_record_offset":9500,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1162,"record_offset":10916,"parent_record_offset":10864,"ancestor_record_offset":9500,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1162,"record_offset":10864,"parent_record_offset":10828,"ancestor_record_offset":9500,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$78::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1162,"record_offset":11268,"parent_record_offset":11244,"ancestor_record_offset":9500,"inlinee":5606,"source_line":359,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":1162,"record_offset":11292,"parent_record_offset":11268,"ancestor_record_offset":9500,"inlinee":5608,"source_line":15,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1162,"record_offset":10140,"parent_record_offset":10112,"ancestor_record_offset":9500,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1162,"record_offset":9648,"parent_record_offset":9624,"ancestor_record_offset":9500,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1162,"record_offset":10944,"parent_record_offset":10864,"ancestor_record_offset":9500,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1162,"record_offset":10164,"parent_record_offset":10140,"ancestor_record_offset":9500,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1162,"record_offset":9800,"parent_record_offset":9772,"ancestor_record_offset":9500,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1162,"record_offset":9912,"parent_record_offset":9884,"ancestor_record_offset":9500,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1162,"record_offset":9960,"parent_record_offset":9936,"ancestor_record_offset":9500,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1162,"record_offset":9936,"parent_record_offset":9912,"ancestor_record_offset":9500,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1162,"record_offset":10972,"parent_record_offset":10864,"ancestor_record_offset":9500,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1162,"record_offset":9672,"parent_record_offset":9648,"ancestor_record_offset":9500,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1162,"record_offset":9696,"parent_record_offset":9672,"ancestor_record_offset":9500,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1162,"record_offset":9624,"parent_record_offset":9600,"ancestor_record_offset":9500,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1162,"record_offset":9884,"parent_record_offset":9856,"ancestor_record_offset":9500,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1162,"record_offset":9856,"parent_record_offset":9828,"ancestor_record_offset":9500,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/01162.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::find_extra_files<ref$<str$> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1163,"record_offset":12276,"rva":16225872,"va":5384934992}
// 编译实例 RVAs: 0xf79650
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$relative_path..Components$u20$as$u20$core..cmp..PartialEq$GT$::eq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t335155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384934992,"raw_hex":"e8331d0500","size":5,"soft_fail":false,"target_va":5385271296,"va":5384936136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2226561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384934992,"raw_hex":"e87f06deff","size":5,"soft_fail":false,"target_va":5382708864,"va":5384935420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..sys..pal..windows..fs..FindNextFileHandle$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2168876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384934992,"raw_hex":"e8d4e7deff","size":5,"soft_fail":false,"target_va":5382768048,"va":5384936919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10256729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384934992,"raw_hex":"e8a77e63ff","size":5,"soft_fail":false,"target_va":5374679664,"va":5384936388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e69f0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10042379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384934992,"raw_hex":"e8f5c366ff","size":5,"soft_fail":false,"target_va":5374896624,"va":5384938998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=29
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9853695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384934992,"raw_hex":"e801a569ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5384935194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d58e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2231217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384934992,"raw_hex":"e84ff4ddff","size":5,"soft_fail":false,"target_va":5382704752,"va":5384935964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2108227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384934992,"raw_hex":"e8bdd4dfff","size":5,"soft_fail":false,"target_va":5382827728,"va":5384935950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26910630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384934992,"raw_hex":"e8a69f9a01","size":5,"soft_fail":false,"target_va":5411847152,"va":5384936517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26911916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384934992,"raw_hex":"e8aca49a01","size":5,"soft_fail":false,"target_va":5411847168,"va":5384935247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10579919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384934992,"raw_hex":"e831905eff","size":5,"soft_fail":false,"target_va":5374356912,"va":5384936826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-2252673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384934992,"raw_hex":"e87fa0ddff","size":5,"soft_fail":false,"target_va":5382683584,"va":5384936252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9839765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384934992,"raw_hex":"e86bdb69ff","size":5,"soft_fail":false,"target_va":5375099104,"va":5384938864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] relative_path::RelativePath::join
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"relative-path","version":"1.9.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t14467116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384934992,"raw_hex":"e82cc0dc00","size":5,"soft_fail":false,"target_va":5399403168,"va":5384936047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2226391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384934992,"raw_hex":"e82907deff","size":5,"soft_fail":false,"target_va":5382709552,"va":5384935938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_type
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2225981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384934992,"raw_hex":"e8c308deff","size":5,"soft_fail":false,"target_va":5382709520,"va":5384935496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2227003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384934992,"raw_hex":"e8c504deff","size":5,"soft_fail":false,"target_va":5382709008,"va":5384936006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::readdir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2164523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384934992,"raw_hex":"e8d5f8deff","size":5,"soft_fail":false,"target_va":5382770560,"va":5384935078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_package_integrity::find_extra_files<std::path::PathBuf>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384934992,"raw_hex":"e869e9ffff","size":5,"soft_fail":false,"target_va":5384930864,"va":5384936642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5384934992,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5384935165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1163,"record_offset":20380,"parent_record_offset":12276,"ancestor_record_offset":12276,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1163,"record_offset":12512,"parent_record_offset":12276,"ancestor_record_offset":12276,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_direct] relative_path::impl$112::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"1.9.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-1.9.0/src/lib.rs
//     evidence={"module":1163,"record_offset":16768,"parent_record_offset":12276,"ancestor_record_offset":12276,"inlinee":61596,"source_line":2280,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1163,"record_offset":18708,"parent_record_offset":12276,"ancestor_record_offset":12276,"inlinee":4470,"source_line":689,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1163,"record_offset":16736,"parent_record_offset":12276,"ancestor_record_offset":12276,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1163,"record_offset":16680,"parent_record_offset":12276,"ancestor_record_offset":12276,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_direct] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1163,"record_offset":18004,"parent_record_offset":12276,"ancestor_record_offset":12276,"inlinee":13008,"source_line":2036,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1163,"record_offset":12440,"parent_record_offset":12276,"ancestor_record_offset":12276,"inlinee":12459,"source_line":2936,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1163,"record_offset":12964,"parent_record_offset":12940,"ancestor_record_offset":12276,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1163,"record_offset":12940,"parent_record_offset":12852,"ancestor_record_offset":12276,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1163,"record_offset":12852,"parent_record_offset":12824,"ancestor_record_offset":12276,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1163,"record_offset":20460,"parent_record_offset":20428,"ancestor_record_offset":12276,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1163,"record_offset":17056,"parent_record_offset":17028,"ancestor_record_offset":12276,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1163,"record_offset":17028,"parent_record_offset":17004,"ancestor_record_offset":12276,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1163,"record_offset":20508,"parent_record_offset":20484,"ancestor_record_offset":12276,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1163,"record_offset":20484,"parent_record_offset":20460,"ancestor_record_offset":12276,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1163,"record_offset":17004,"parent_record_offset":16980,"ancestor_record_offset":12276,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1163,"record_offset":17868,"parent_record_offset":17832,"ancestor_record_offset":12276,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1163,"record_offset":17832,"parent_record_offset":17808,"ancestor_record_offset":12276,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1163,"record_offset":20428,"parent_record_offset":20380,"ancestor_record_offset":12276,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1163,"record_offset":12884,"parent_record_offset":12852,"ancestor_record_offset":12276,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1163,"record_offset":19496,"parent_record_offset":19468,"ancestor_record_offset":12276,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1163,"record_offset":18996,"parent_record_offset":18972,"ancestor_record_offset":12276,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1163,"record_offset":19080,"parent_record_offset":19056,"ancestor_record_offset":12276,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1163,"record_offset":19020,"parent_record_offset":18996,"ancestor_record_offset":12276,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1163,"record_offset":19672,"parent_record_offset":18888,"ancestor_record_offset":12276,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1163,"record_offset":12908,"parent_record_offset":12884,"ancestor_record_offset":12276,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1163,"record_offset":19228,"parent_record_offset":19200,"ancestor_record_offset":12276,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1163,"record_offset":19200,"parent_record_offset":19172,"ancestor_record_offset":12276,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=92
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1163,"record_offset":12540,"parent_record_offset":12512,"ancestor_record_offset":12276,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1163,"record_offset":19348,"parent_record_offset":19324,"ancestor_record_offset":12276,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1163,"record_offset":12652,"parent_record_offset":12624,"ancestor_record_offset":12276,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1163,"record_offset":12700,"parent_record_offset":12676,"ancestor_record_offset":12276,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1163,"record_offset":12676,"parent_record_offset":12652,"ancestor_record_offset":12276,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1163,"record_offset":17900,"parent_record_offset":17868,"ancestor_record_offset":12276,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1163,"record_offset":20548,"parent_record_offset":20380,"ancestor_record_offset":12276,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1163,"record_offset":12472,"parent_record_offset":12440,"ancestor_record_offset":12276,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1163,"record_offset":19580,"parent_record_offset":19552,"ancestor_record_offset":12276,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1163,"record_offset":19608,"parent_record_offset":19580,"ancestor_record_offset":12276,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1163,"record_offset":19524,"parent_record_offset":19496,"ancestor_record_offset":12276,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1163,"record_offset":17928,"parent_record_offset":17868,"ancestor_record_offset":12276,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1163,"record_offset":17960,"parent_record_offset":17832,"ancestor_record_offset":12276,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1163,"record_offset":19140,"parent_record_offset":19112,"ancestor_record_offset":12276,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1163,"record_offset":19172,"parent_record_offset":19140,"ancestor_record_offset":12276,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1163,"record_offset":19112,"parent_record_offset":18888,"ancestor_record_offset":12276,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1163,"record_offset":18972,"parent_record_offset":18888,"ancestor_record_offset":12276,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1163,"record_offset":19700,"parent_record_offset":18888,"ancestor_record_offset":12276,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1163,"record_offset":19056,"parent_record_offset":18888,"ancestor_record_offset":12276,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1163,"record_offset":18944,"parent_record_offset":18888,"ancestor_record_offset":12276,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1163,"record_offset":19468,"parent_record_offset":19440,"ancestor_record_offset":12276,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1163,"record_offset":18756,"parent_record_offset":18732,"ancestor_record_offset":12276,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1163,"record_offset":18780,"parent_record_offset":18756,"ancestor_record_offset":12276,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1163,"record_offset":19440,"parent_record_offset":19272,"ancestor_record_offset":12276,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1163,"record_offset":18812,"parent_record_offset":18780,"ancestor_record_offset":12276,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1163,"record_offset":19380,"parent_record_offset":19296,"ancestor_record_offset":12276,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1163,"record_offset":19324,"parent_record_offset":19296,"ancestor_record_offset":12276,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1163,"record_offset":19792,"parent_record_offset":18888,"ancestor_record_offset":12276,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1163,"record_offset":19296,"parent_record_offset":19272,"ancestor_record_offset":12276,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1163,"record_offset":18864,"parent_record_offset":18840,"ancestor_record_offset":12276,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1163,"record_offset":18840,"parent_record_offset":18780,"ancestor_record_offset":12276,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1163,"record_offset":18888,"parent_record_offset":18864,"ancestor_record_offset":12276,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1163,"record_offset":19272,"parent_record_offset":18888,"ancestor_record_offset":12276,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1163,"record_offset":18732,"parent_record_offset":18708,"ancestor_record_offset":12276,"inlinee":4472,"source_line":862,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::Components::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"relative-path","version":"1.9.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-1.9.0/src/lib.rs
//     evidence={"module":1163,"record_offset":16840,"parent_record_offset":16816,"ancestor_record_offset":12276,"inlinee":30561,"source_line":518,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::RelativePath::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"1.9.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-1.9.0/src/lib.rs
//     evidence={"module":1163,"record_offset":16816,"parent_record_offset":16792,"ancestor_record_offset":12276,"inlinee":30559,"source_line":1108,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::impl$46::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"1.9.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-1.9.0/src/lib.rs
//     evidence={"module":1163,"record_offset":16792,"parent_record_offset":16768,"ancestor_record_offset":12276,"inlinee":30558,"source_line":1976,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1163,"record_offset":16704,"parent_record_offset":16680,"ancestor_record_offset":12276,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1163,"record_offset":12624,"parent_record_offset":12596,"ancestor_record_offset":12276,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1163,"record_offset":12596,"parent_record_offset":12568,"ancestor_record_offset":12276,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":1163,"record_offset":18036,"parent_record_offset":18004,"ancestor_record_offset":12276,"inlinee":13010,"source_line":1126,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_symlink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":1163,"record_offset":18060,"parent_record_offset":18036,"ancestor_record_offset":12276,"inlinee":13011,"source_line":1132,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_package_integrity::find_extra_files<std::path::PathBuf>
// 记录形式: compiled_procedure; instances=1
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1163,"record_offset":836,"rva":16221744,"va":5384930864}
// 编译实例 RVAs: 0xf78630
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$relative_path..Components$u20$as$u20$core..cmp..PartialEq$GT$::eq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t339267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384930864,"raw_hex":"e8432d0500","size":5,"soft_fail":false,"target_va":5385271296,"va":5384932024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2222449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384930864,"raw_hex":"e88f16deff","size":5,"soft_fail":false,"target_va":5382708864,"va":5384931308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..sys..pal..windows..fs..FindNextFileHandle$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2164760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384930864,"raw_hex":"e8e8f7deff","size":5,"soft_fail":false,"target_va":5382768048,"va":5384932803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10252617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384930864,"raw_hex":"e8b78e63ff","size":5,"soft_fail":false,"target_va":5374679664,"va":5384932276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e69f0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10038331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384930864,"raw_hex":"e8c5d366ff","size":5,"soft_fail":false,"target_va":5374896624,"va":5384934950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=31
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9849582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384930864,"raw_hex":"e812b569ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5384931081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d58e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2227105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384930864,"raw_hex":"e85f04deff","size":5,"soft_fail":false,"target_va":5382704752,"va":5384931852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2103224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384930864,"raw_hex":"e848e8dfff","size":5,"soft_fail":false,"target_va":5382827728,"va":5384930947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26914742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384930864,"raw_hex":"e8b6af9a01","size":5,"soft_fail":false,"target_va":5411847152,"va":5384932405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26916022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384930864,"raw_hex":"e8b6b49a01","size":5,"soft_fail":false,"target_va":5411847168,"va":5384931141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10575810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384930864,"raw_hex":"e83ea05eff","size":5,"soft_fail":false,"target_va":5374356912,"va":5384932717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-2248561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384930864,"raw_hex":"e88fb0ddff","size":5,"soft_fail":false,"target_va":5382683584,"va":5384932140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9835717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384930864,"raw_hex":"e83beb69ff","size":5,"soft_fail":false,"target_va":5375099104,"va":5384934816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] relative_path::RelativePath::join
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"relative-path","version":"1.9.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t14471228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384930864,"raw_hex":"e83cd0dc00","size":5,"soft_fail":false,"target_va":5399403168,"va":5384931935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2222279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384930864,"raw_hex":"e83917deff","size":5,"soft_fail":false,"target_va":5382709552,"va":5384931826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_type
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2221869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384930864,"raw_hex":"e8d318deff","size":5,"soft_fail":false,"target_va":5382709520,"va":5384931384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2222891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384930864,"raw_hex":"e8d514deff","size":5,"soft_fail":false,"target_va":5382709008,"va":5384931894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::readdir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2160410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384930864,"raw_hex":"e8e608dfff","size":5,"soft_fail":false,"target_va":5382770560,"va":5384930965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_package_integrity::find_extra_files<std::path::PathBuf>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384930864,"raw_hex":"e879f9ffff","size":5,"soft_fail":false,"target_va":5384930864,"va":5384932530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5384930864,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5384931052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1163,"record_offset":9120,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1163,"record_offset":1252,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_direct] relative_path::impl$112::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"1.9.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-1.9.0/src/lib.rs
//     evidence={"module":1163,"record_offset":5508,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":61596,"source_line":2280,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1163,"record_offset":7448,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":4470,"source_line":689,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1163,"record_offset":5476,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1163,"record_offset":5420,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_direct] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1163,"record_offset":6744,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":13008,"source_line":2036,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1163,"record_offset":1180,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":12459,"source_line":2936,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1163,"record_offset":1012,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1163,"record_offset":1704,"parent_record_offset":1680,"ancestor_record_offset":836,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1163,"record_offset":1680,"parent_record_offset":1592,"ancestor_record_offset":836,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1163,"record_offset":1592,"parent_record_offset":1564,"ancestor_record_offset":836,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1163,"record_offset":9196,"parent_record_offset":9164,"ancestor_record_offset":836,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1163,"record_offset":5796,"parent_record_offset":5768,"ancestor_record_offset":836,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1163,"record_offset":5768,"parent_record_offset":5744,"ancestor_record_offset":836,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1163,"record_offset":9244,"parent_record_offset":9220,"ancestor_record_offset":836,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1163,"record_offset":9220,"parent_record_offset":9196,"ancestor_record_offset":836,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1163,"record_offset":5744,"parent_record_offset":5720,"ancestor_record_offset":836,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1163,"record_offset":6608,"parent_record_offset":6572,"ancestor_record_offset":836,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1163,"record_offset":6572,"parent_record_offset":6548,"ancestor_record_offset":836,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1163,"record_offset":9164,"parent_record_offset":9120,"ancestor_record_offset":836,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1163,"record_offset":1624,"parent_record_offset":1592,"ancestor_record_offset":836,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1163,"record_offset":8236,"parent_record_offset":8208,"ancestor_record_offset":836,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1163,"record_offset":7736,"parent_record_offset":7712,"ancestor_record_offset":836,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1163,"record_offset":7820,"parent_record_offset":7796,"ancestor_record_offset":836,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1163,"record_offset":7760,"parent_record_offset":7736,"ancestor_record_offset":836,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1163,"record_offset":8412,"parent_record_offset":7628,"ancestor_record_offset":836,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1163,"record_offset":1648,"parent_record_offset":1624,"ancestor_record_offset":836,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1163,"record_offset":7968,"parent_record_offset":7940,"ancestor_record_offset":836,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1163,"record_offset":7940,"parent_record_offset":7912,"ancestor_record_offset":836,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=102
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1163,"record_offset":1280,"parent_record_offset":1252,"ancestor_record_offset":836,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1163,"record_offset":8088,"parent_record_offset":8064,"ancestor_record_offset":836,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1163,"record_offset":1392,"parent_record_offset":1364,"ancestor_record_offset":836,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1163,"record_offset":1440,"parent_record_offset":1416,"ancestor_record_offset":836,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1163,"record_offset":1416,"parent_record_offset":1392,"ancestor_record_offset":836,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1163,"record_offset":6640,"parent_record_offset":6608,"ancestor_record_offset":836,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1163,"record_offset":9284,"parent_record_offset":9120,"ancestor_record_offset":836,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1163,"record_offset":1212,"parent_record_offset":1180,"ancestor_record_offset":836,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1163,"record_offset":8320,"parent_record_offset":8292,"ancestor_record_offset":836,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1163,"record_offset":8348,"parent_record_offset":8320,"ancestor_record_offset":836,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1163,"record_offset":8264,"parent_record_offset":8236,"ancestor_record_offset":836,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1163,"record_offset":6668,"parent_record_offset":6608,"ancestor_record_offset":836,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1163,"record_offset":6700,"parent_record_offset":6572,"ancestor_record_offset":836,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1163,"record_offset":7880,"parent_record_offset":7852,"ancestor_record_offset":836,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1163,"record_offset":7912,"parent_record_offset":7880,"ancestor_record_offset":836,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1163,"record_offset":7852,"parent_record_offset":7628,"ancestor_record_offset":836,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1163,"record_offset":7712,"parent_record_offset":7628,"ancestor_record_offset":836,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1163,"record_offset":8440,"parent_record_offset":7628,"ancestor_record_offset":836,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1163,"record_offset":7796,"parent_record_offset":7628,"ancestor_record_offset":836,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1163,"record_offset":7684,"parent_record_offset":7628,"ancestor_record_offset":836,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1163,"record_offset":8208,"parent_record_offset":8180,"ancestor_record_offset":836,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1163,"record_offset":7496,"parent_record_offset":7472,"ancestor_record_offset":836,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1163,"record_offset":7520,"parent_record_offset":7496,"ancestor_record_offset":836,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1163,"record_offset":8180,"parent_record_offset":8012,"ancestor_record_offset":836,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1163,"record_offset":7552,"parent_record_offset":7520,"ancestor_record_offset":836,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1163,"record_offset":8120,"parent_record_offset":8036,"ancestor_record_offset":836,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1163,"record_offset":8064,"parent_record_offset":8036,"ancestor_record_offset":836,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1163,"record_offset":8532,"parent_record_offset":7628,"ancestor_record_offset":836,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1163,"record_offset":8036,"parent_record_offset":8012,"ancestor_record_offset":836,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1163,"record_offset":7604,"parent_record_offset":7580,"ancestor_record_offset":836,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1163,"record_offset":7580,"parent_record_offset":7520,"ancestor_record_offset":836,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1163,"record_offset":7628,"parent_record_offset":7604,"ancestor_record_offset":836,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1163,"record_offset":8012,"parent_record_offset":7628,"ancestor_record_offset":836,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1163,"record_offset":7472,"parent_record_offset":7448,"ancestor_record_offset":836,"inlinee":4472,"source_line":862,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::Components::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"relative-path","version":"1.9.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-1.9.0/src/lib.rs
//     evidence={"module":1163,"record_offset":5580,"parent_record_offset":5556,"ancestor_record_offset":836,"inlinee":30561,"source_line":518,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::RelativePath::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"1.9.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-1.9.0/src/lib.rs
//     evidence={"module":1163,"record_offset":5556,"parent_record_offset":5532,"ancestor_record_offset":836,"inlinee":30559,"source_line":1108,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::impl$46::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"1.9.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-1.9.0/src/lib.rs
//     evidence={"module":1163,"record_offset":5532,"parent_record_offset":5508,"ancestor_record_offset":836,"inlinee":30558,"source_line":1976,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1163,"record_offset":1084,"parent_record_offset":1060,"ancestor_record_offset":836,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1163,"record_offset":1132,"parent_record_offset":1108,"ancestor_record_offset":836,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1163,"record_offset":1108,"parent_record_offset":1084,"ancestor_record_offset":836,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1163,"record_offset":1364,"parent_record_offset":1336,"ancestor_record_offset":836,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1163,"record_offset":1336,"parent_record_offset":1308,"ancestor_record_offset":836,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1163,"record_offset":1060,"parent_record_offset":1036,"ancestor_record_offset":836,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1163,"record_offset":1036,"parent_record_offset":1012,"ancestor_record_offset":836,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":1163,"record_offset":6776,"parent_record_offset":6744,"ancestor_record_offset":836,"inlinee":13010,"source_line":1126,"archive":"_pdb/B/modules/01163.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_symlink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":1163,"record_offset":6800,"parent_record_offset":6776,"ancestor_record_offset":836,"inlinee":13011,"source_line":1132,"archive":"_pdb/B/modules/01163.jsonl.gz"}
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
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_package_integrity::PackageManifest
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::from_slice<tg_package_integrity::PackageManifest>", "constructor": "serde_json::de::from_slice", "arguments": ["tg_package_integrity::PackageManifest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tg_package_integrity::PackageManifest,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["tg_package_integrity::PackageManifest", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tg_package_integrity::PackageManifest,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<tg_package_integrity::PackageManifest,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tg_package_integrity::PackageManifest,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tg_package_integrity::PackageManifest,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tg_package_integrity::PackageManifest>", "constructor": "core::ptr::drop_in_place", "arguments": ["tg_package_integrity::PackageManifest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_package_integrity::PackageVerificationError
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tg_package_integrity::PackageVerificationError>", "constructor": "enum2$", "arguments": ["tg_package_integrity::PackageVerificationError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<>,enum2$<tg_package_integrity::PackageVerificationError> >", "constructor": "core::result::Result", "arguments": ["tuple$<>", "enum2$<tg_package_integrity::PackageVerificationError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<>,enum2$<tg_package_integrity::PackageVerificationError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<>,enum2$<tg_package_integrity::PackageVerificationError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<tg_package_integrity::PackageVerificationError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tuple$<>,enum2$<tg_package_integrity::PackageVerificationError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<tg_package_integrity::PackageVerificationError> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<tg_package_integrity::PackageVerificationError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_package_integrity::verify_package_manifest::CheckOutcome
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 42
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>", "constructor": "enum2$", "arguments": ["tg_package_integrity::verify_package_manifest::CheckOutcome"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpsc::Receiver", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "easy_parallel::Parallel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "easy_parallel::Parallel", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<easy_parallel::Parallel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["easy_parallel::Parallel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Sender<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpsc::Sender", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Sender<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Sender<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::SendError<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpsc::SendError", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::SendError<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::SendError<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Packet<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::zero::Packet", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::zero::Packet<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::zero::Packet<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::list::Channel", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::list::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::array::Channel", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::array::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "assoc$", "arguments": ["Output", "enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "core::ops::function::FnOnce", "arguments": ["tuple$<>", "assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>", "constructor": "dyn$", "arguments": ["core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "core::marker::Send"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "easy_parallel::Parallel::run<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "easy_parallel::Parallel::run", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "easy_parallel::Parallel::each<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,tuple$<alloc::string::String,alloc::string::String>,alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,tg_package_integrity::verify_package_manifest::closure_env$3>", "constructor": "easy_parallel::Parallel::each", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>", "tuple$<alloc::string::String,alloc::string::String>", "alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>", "tg_package_integrity::verify_package_manifest::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "easy_parallel::impl$0::finish::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "easy_parallel::impl$0::finish::closure_env$0", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>", "alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>", "enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<easy_parallel::impl$0::finish::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,tuple$<> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["easy_parallel::impl$0::finish::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "easy_parallel::impl$0::each::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,tuple$<alloc::string::String,alloc::string::String>,alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,tg_package_integrity::verify_package_manifest::closure_env$3>", "constructor": "easy_parallel::impl$0::each::closure_env$0", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>", "tuple$<alloc::string::String,alloc::string::String>", "alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>", "tg_package_integrity::verify_package_manifest::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<easy_parallel::impl$0::each::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,tuple$<alloc::string::String,alloc::string::String>,alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,tg_package_integrity::verify_package_manifest::closure_env$3>,tuple$<> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["easy_parallel::impl$0::each::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,tuple$<alloc::string::String,alloc::string::String>,alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,tg_package_integrity::verify_package_manifest::closure_env$3>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<easy_parallel::impl$0::each::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,tuple$<alloc::string::String,alloc::string::String>,alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,tg_package_integrity::verify_package_manifest::closure_env$3> >", "constructor": "core::ptr::drop_in_place", "arguments": ["easy_parallel::impl$0::each::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,tuple$<alloc::string::String,alloc::string::String>,alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,tg_package_integrity::verify_package_manifest::closure_env$3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<easy_parallel::impl$0::finish::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["easy_parallel::impl$0::finish::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >,core::marker::Send>,alloc::alloc::Global>,enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_senders<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::list::Channel::disconnect_senders", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_receivers<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::list::Channel::disconnect_receivers", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::recv<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::list::Channel::recv", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::impl$3::recv::closure$1<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::list::impl$3::recv::closure$1", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::send<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::list::Channel::send", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::zero::Channel", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::impl$6::drop::closure_env$2", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Sender::release<std::sync::mpmc::zero::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "std::sync::mpmc::counter::Sender::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$20::drop::closure_env$2<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::impl$20::drop::closure_env$2", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Receiver::release<std::sync::mpmc::zero::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,std::sync::mpmc::impl$20::drop::closure_env$2<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "std::sync::mpmc::counter::Receiver::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "std::sync::mpmc::impl$20::drop::closure_env$2<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::disconnect_receivers<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::array::Channel::disconnect_receivers", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::recv<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::array::Channel::recv", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::impl$1::recv::closure$1<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::array::impl$1::recv::closure$1", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::send<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::array::Channel::send", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::impl$1::send::closure$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::array::impl$1::send::closure$0", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::disconnect<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::zero::Channel::disconnect", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::recv<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::zero::Channel::recv", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::recv::closure$1<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::zero::impl$3::recv::closure$1", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::send<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::zero::Channel::send", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure$0", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::recv::closure_env$1<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::zero::impl$3::recv::closure_env$1", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<std::sync::mpmc::zero::impl$3::recv::closure_env$1<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "core::option::Option", "arguments": ["std::sync::mpmc::zero::impl$3::recv::closure_env$1<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::recv::closure_env$1<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > > >", "constructor": "enum2$", "arguments": ["core::option::Option<std::sync::mpmc::zero::impl$3::recv::closure_env$1<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::recv::closure_env$1<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::recv::closure_env$1<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure_env$0", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >", "constructor": "core::option::Option", "arguments": ["std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > > >", "constructor": "enum2$", "arguments": ["core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::Sender::send<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpmc::Sender::send", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::impl$18::fmt<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >", "constructor": "std::sync::mpsc::impl$18::fmt", "arguments": ["enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: serde_json::de::Deserializer::parse_exponent<serde_json::read::SliceRead>
// 信息 ID: 3d775db911774ef6e0660b41
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::Deserializer::parse_exponent -> ["serde_json::read::SliceRead"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::Deserializer::parse_number<serde_json::read::SliceRead> @ 0x5742c -> 0x58210 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_number<serde_json::read::SliceRead> @ 0x57579 -> 0x58210 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_decimal<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal<serde_json::read::SliceRead> @ 0x614196 -> 0x58210 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> @ 0x614376 -> 0x58210 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> @ 0x614479 -> 0x58210 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_long_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_long_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_long_integer<serde_json::read::SliceRead> @ 0x28a49f7 -> 0x58210 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_decimal_overflow<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal_overflow<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal_overflow<serde_json::read::SliceRead> @ 0x28a4b83 -> 0x58210 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead>
// 信息 ID: f706ce6298aa4eb3f5229dde
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::Deserializer::parse_integer -> ["serde_json::read::SliceRead"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impl$5::deserialize<i64,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0x615826 -> 0x614300 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::de::impl$5::deserialize<i64,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0x61585b -> 0x614300 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::de::impl$5::deserialize<u32,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0xf74193 -> 0x614300 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::de::impl$5::deserialize<u32,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0xf741cd -> 0x614300 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0x1406989 -> 0x614300 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0x14069d1 -> 0x614300 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x28a46fe -> 0x614300 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x28a4775 -> 0x614300 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x28e4f8e -> 0x614300 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x28e5005 -> 0x614300 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::_::impl$0::deserialize
// 信息 ID: d7720225ec7eb2148dde1b00
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::from_slice<tg_package_integrity::PackageManifest>; module=1160, inline_record=540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 0d006dc49a95759e215eb2e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tg_package_integrity::_::impl$0::deserialize::__Visitor>; module=1160, inline_record=21916; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 5cbb1b4d446339d745813883
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tg_package_integrity::_::impl$0::deserialize::__Visitor>; module=1160, inline_record=21728; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 6eec9841e7b134b911c2b5e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<tg_package_integrity::_::impl$0::deserialize::__Visitor>; module=1163, inline_record=432; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 1f6a4f789cc627ebe66245de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tg_package_integrity::_::impl$0::deserialize::__Visitor>; module=1160, inline_record=21072; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 00f5977db89d86f48b57e923
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,tg_package_integrity::_::impl$0::deserialize::__Visitor>; module=1160, inline_record=27060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::find_extra_files<ref$<str$> >
// 信息 ID: 476ca7c319a0019f5679972e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tg_package_integrity::find_extra_files -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_package_integrity::verify_package_manifest @ 0x67cb71 -> 0xf79650 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::find_extra_files<std::path::PathBuf>
// 信息 ID: da77d401e2b6eeb2160c28ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_package_integrity::find_extra_files -> ["std::path::PathBuf"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_package_integrity::find_extra_files<std::path::PathBuf> @ 0xf78cb2 -> 0xf78630 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_package_integrity::find_extra_files<ref$<str$> > @ 0xf79cc2 -> 0xf78630 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::is_valid_manifest_checksum
// 信息 ID: bcea37bfa5e022c13935b28a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_package_integrity::verify_package_manifest; module=327, inline_record=3360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::is_valid_manifest_checksum::calculate_fnv_hash
// 信息 ID: cd298e238cc0152e998e3e11
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_package_integrity::verify_package_manifest; module=327, inline_record=3388; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::verify_package_manifest
// 信息 ID: 46692ce276d398a53cf69254
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::integrity_report_main::verify_package_integrity_and_show_report @ 0x2552f5 -> 0x67c3d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x3b040f -> 0x67c3d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::verify_package_manifest::closure$0
// 信息 ID: cc64eff2225dc3f5de8f8540
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_package_integrity::verify_package_manifest", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_package_integrity::verify_package_manifest; module=327, inline_record=2076; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::verify_package_manifest::closure$1
// 信息 ID: e471265d2f19952bdb80e3d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tg_package_integrity::verify_package_manifest", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_package_integrity::verify_package_manifest; module=327, inline_record=3504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::verify_package_manifest::closure$2
// 信息 ID: 20942d1ae069b82b2e3aa866
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tg_package_integrity::verify_package_manifest", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_package_integrity::verify_package_manifest; module=327, inline_record=5544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_package_integrity::verify_package_manifest::closure$3
// 信息 ID: e5e41538a25a452b564380a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tg_package_integrity::verify_package_manifest", "captures": "unknown"}
// [闭包环境类型引用] tg_package_integrity::verify_package_manifest::closure_env$3 <- easy_parallel::Parallel::each<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,tuple$<alloc::string::String,alloc::string::String>,alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,tg_package_integrity::verify_package_manifest::closure_env$3>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_package_integrity::verify_package_manifest::closure_env$3 <- core::ops::function::FnOnce::call_once<easy_parallel::impl$0::each::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,tuple$<alloc::string::String,alloc::string::String>,alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,tg_package_integrity::verify_package_manifest::closure_env$3>,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_package_integrity::verify_package_manifest::closure_env$3 <- core::ptr::drop_in_place<easy_parallel::impl$0::each::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,tuple$<alloc::string::String,alloc::string::String>,alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,tg_package_integrity::verify_package_manifest::closure_env$3> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<easy_parallel::impl$0::each::closure_env$0<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome>,tuple$<alloc::string::String,alloc::string::String>,alloc::vec::Vec<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,tg_package_integrity::verify_package_manifest::closure_env$3>,tuple$<> >; module=1162, inline_record=9500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

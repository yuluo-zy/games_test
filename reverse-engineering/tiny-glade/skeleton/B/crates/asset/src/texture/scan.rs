// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/asset/src/texture/scan.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: ca0ff088b2b52adc3e1529c042e08cf0bf6a6bec77a576fc7ac655e1514a1735
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: scan.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3167, 4092
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use relative_path; // namespace candidate
//   package identities: [{"name":"relative-path","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.9.0"}]
// use ron; // namespace candidate
//   package identities: [{"name":"ron","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968","version":"0.8.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset::texture::scan::scan_asset_tree
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3167,"record_offset":2288,"rva":34442144,"va":5403151264}
// 编译实例 RVAs: 0x20d8ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] asset::texture::scan::scan_asset_tree_walker
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151264,"raw_hex":"e8a1000000","size":5,"soft_fail":false,"target_va":5403151616,"va":5403151450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t22536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151264,"raw_hex":"e808580000","size":5,"soft_fail":false,"target_va":5403174032,"va":5403151491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20493451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151264,"raw_hex":"e8754bc7fe","size":5,"soft_fail":false,"target_va":5382657856,"va":5403151302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20397545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151264,"raw_hex":"e817c2c8fe","size":5,"soft_fail":false,"target_va":5382753792,"va":5403151332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::texture::config::InheritableTextureOpts::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/config.rs
//     evidence={"module":3167,"record_offset":3124,"parent_record_offset":2288,"ancestor_record_offset":2288,"inlinee":108127,"source_line":46,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":3240,"parent_record_offset":2288,"ancestor_record_offset":2288,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3167,"record_offset":3212,"parent_record_offset":2288,"ancestor_record_offset":2288,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3167,"record_offset":2424,"parent_record_offset":2288,"ancestor_record_offset":2288,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$2::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3167,"record_offset":3176,"parent_record_offset":3152,"ancestor_record_offset":2288,"inlinee":108131,"source_line":90,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$6::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3167,"record_offset":3152,"parent_record_offset":3124,"ancestor_record_offset":2288,"inlinee":108129,"source_line":152,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3167,"record_offset":2924,"parent_record_offset":2896,"ancestor_record_offset":2288,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3167,"record_offset":2984,"parent_record_offset":2952,"ancestor_record_offset":2288,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3167,"record_offset":2788,"parent_record_offset":2764,"ancestor_record_offset":2288,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3167,"record_offset":2952,"parent_record_offset":2896,"ancestor_record_offset":2288,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3167,"record_offset":2552,"parent_record_offset":2520,"ancestor_record_offset":2288,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3167,"record_offset":2656,"parent_record_offset":2632,"ancestor_record_offset":2288,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":3264,"parent_record_offset":3240,"ancestor_record_offset":2288,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3167,"record_offset":2740,"parent_record_offset":2632,"ancestor_record_offset":2288,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":2812,"parent_record_offset":2788,"ancestor_record_offset":2288,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":2764,"parent_record_offset":2740,"ancestor_record_offset":2288,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":2840,"parent_record_offset":2788,"ancestor_record_offset":2288,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3167,"record_offset":3312,"parent_record_offset":3288,"ancestor_record_offset":2288,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3167,"record_offset":3092,"parent_record_offset":2424,"ancestor_record_offset":2288,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3167,"record_offset":2472,"parent_record_offset":2448,"ancestor_record_offset":2288,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3167,"record_offset":2700,"parent_record_offset":2680,"ancestor_record_offset":2288,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3167,"record_offset":2576,"parent_record_offset":2552,"ancestor_record_offset":2288,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3167,"record_offset":2896,"parent_record_offset":2520,"ancestor_record_offset":2288,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3167,"record_offset":2448,"parent_record_offset":2424,"ancestor_record_offset":2288,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3167,"record_offset":2604,"parent_record_offset":2576,"ancestor_record_offset":2288,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3167,"record_offset":2632,"parent_record_offset":2604,"ancestor_record_offset":2288,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3167,"record_offset":2520,"parent_record_offset":2496,"ancestor_record_offset":2288,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3167,"record_offset":2496,"parent_record_offset":2472,"ancestor_record_offset":2288,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::scan::scan_asset_tree_walker
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3167,"record_offset":3468,"rva":34442496,"va":5403151616}
// 编译实例 RVAs: 0x20d8d00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$relative_path..RelativePathBuf$u20$as$u20$core..cmp..PartialEq$GT$::eq
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-17884264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8981beffe","size":5,"soft_fail":false,"target_va":5385271792,"va":5403156051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20444451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8dd0ac8fe","size":5,"soft_fail":false,"target_va":5382708864,"va":5403153310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..sys..pal..windows..fs..FindNextFileHandle$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20389164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8d4e2c8fe","size":5,"soft_fail":false,"target_va":5382768048,"va":5403157207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28475848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8387e4dfe","size":5,"soft_fail":false,"target_va":5374679664,"va":5403155507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e69f0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28265227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8f5b450fe","size":5,"soft_fail":false,"target_va":5374896624,"va":5403161846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=38
//     target_aliases=1 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28070374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e81aae53fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5403151873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140889e60
//     owner=unresolved_namespace; count=7
//     target_aliases=3 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25493737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e817ff7afe","size":5,"soft_fail":false,"target_va":5377662560,"va":5403156292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b770c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22424480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e860d4a9fe","size":5,"soft_fail":false,"target_va":5380731072,"va":5403155547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d58e70
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20448739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e81dfac7fe","size":5,"soft_fail":false,"target_va":5382704752,"va":5403153486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=8
//     target_aliases=5 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20324020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e84ce1c9fe","size":5,"soft_fail":false,"target_va":5382827728,"va":5403151743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d8d6c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20235066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8c63ccbfe","size":5,"soft_fail":false,"target_va":5382919872,"va":5403154933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420d4ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e854b5ffff","size":5,"soft_fail":false,"target_va":5403135136,"va":5403154247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142421c80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3440304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8b07e3400","size":5,"soft_fail":false,"target_va":5406596224,"va":5403155915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142421dd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3441561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e899833400","size":5,"soft_fail":false,"target_va":5406596560,"va":5403154994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142423660
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3446581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e835973400","size":5,"soft_fail":false,"target_va":5406602848,"va":5403156262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142425b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3459030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8d6c73400","size":5,"soft_fail":false,"target_va":5406612224,"va":5403153189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142425dd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3460711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e867ce3400","size":5,"soft_fail":false,"target_va":5406612944,"va":5403152228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: scan.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8693997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8eda88400","size":5,"soft_fail":false,"target_va":5411847168,"va":5403153166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-20312654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8b20dcafe","size":5,"soft_fail":false,"target_va":5382840208,"va":5403152857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-28796499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8ad9948fe","size":5,"soft_fail":false,"target_va":5374358848,"va":5403155342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t-20470832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8d0a3c7fe","size":5,"soft_fail":false,"target_va":5382683584,"va":5403154411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8628555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e84ba98300","size":5,"soft_fail":false,"target_va":5411786032,"va":5403157472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h99f34d5a19ceb942 (.llvm.4711583917080959974)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8663710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e89e328400","size":5,"soft_fail":false,"target_va":5411816640,"va":5403152925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8657901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8ed1b8400","size":5,"soft_fail":false,"target_va":5411815824,"va":5403157918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8665330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8f2388400","size":5,"soft_fail":false,"target_va":5411817328,"va":5403151993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20504100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8dc21c7fe","size":5,"soft_fail":false,"target_va":5382655024,"va":5403159119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::texture::config::InheritableTextureOpts::into_concrete
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3654061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8adc13700","size":5,"soft_fail":false,"target_va":5406810704,"va":5403156638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::texture::scan::scan_asset_tree_walker
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e84cf6ffff","size":5,"soft_fail":false,"target_va":5403151616,"va":5403154095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t8383262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e81eeb7f00","size":5,"soft_fail":false,"target_va":5411541776,"va":5403158509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8383656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8a8ec7f00","size":5,"soft_fail":false,"target_va":5411542296,"va":5403158635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$asset..texture..desc..TextureAssetDesc$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-25099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8f59dffff","size":5,"soft_fail":false,"target_va":5403136096,"va":5403161190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8384300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e82cef7f00","size":5,"soft_fail":false,"target_va":5411542880,"va":5403158575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::RawTable$LT$T$C$A$GT$::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3453167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8efb03400","size":5,"soft_fail":false,"target_va":5406608192,"va":5403155020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e80b4d0000","size":5,"soft_fail":false,"target_va":5403174032,"va":5403154304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] relative_path::RelativePath::join
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"relative-path","version":"1.9.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3458408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e868c53400","size":5,"soft_fail":false,"target_va":5406612592,"va":5403154179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_name
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-20443913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8f70cc8fe","size":5,"soft_fail":false,"target_va":5382709552,"va":5403153460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::metadata
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20443968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8c00cc8fe","size":5,"soft_fail":false,"target_va":5382709424,"va":5403153387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::path
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-20444725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8cb09c8fe","size":5,"soft_fail":false,"target_va":5382709008,"va":5403153728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20446492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8e402c8fe","size":5,"soft_fail":false,"target_va":5382705472,"va":5403151959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-20494050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e81e49c7fe","size":5,"soft_fail":false,"target_va":5382657856,"va":5403151901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20407660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8949ac8fe","size":5,"soft_fail":false,"target_va":5382744080,"va":5403151735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::extension
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20409795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e83d92c8fe","size":5,"soft_fail":false,"target_va":5382743968,"va":5403153758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::file_stem
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20411574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e84a8bc8fe","size":5,"soft_fail":false,"target_va":5382743808,"va":5403155377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::readdir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20382550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e8aafcc8fe","size":5,"soft_fail":false,"target_va":5382770560,"va":5403153105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::stat
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20375174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e87a19c9fe","size":5,"soft_fail":false,"target_va":5382776592,"va":5403151761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-20398219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403151616,"raw_hex":"e875bfc8fe","size":5,"soft_fail":false,"target_va":5382753792,"va":5403152006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5403151616,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5403151844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3167,"record_offset":20396,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3167,"record_offset":13568,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":10360,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":13280,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":15320,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::with_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":3167,"record_offset":7076,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":12467,"source_line":58,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::config::InheritableTextureOpts::inherit_from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/config.rs
//     evidence={"module":3167,"record_offset":6700,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":108136,"source_line":33,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::config::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/config.rs
//     evidence={"module":3167,"record_offset":4836,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":108133,"source_line":7,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::scan::load_texture_group_config
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/scan.rs
//     evidence={"module":3167,"record_offset":5732,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":108135,"source_line":162,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3167,"record_offset":19244,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3167,"record_offset":13484,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3167,"record_offset":9324,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3167,"record_offset":14364,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":5145,"source_line":1277,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3167,"record_offset":7720,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3167,"record_offset":15288,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":5059,"source_line":1048,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":8900,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3167,"record_offset":7388,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3167,"record_offset":18936,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::strip_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3167,"record_offset":7756,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":9143,"source_line":2397,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3167,"record_offset":9456,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3167,"record_offset":15416,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3167,"record_offset":16436,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3167,"record_offset":10388,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":4955,"source_line":605,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3167,"record_offset":13984,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":8292,"source_line":1244,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3167,"record_offset":7952,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3167,"record_offset":7688,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsString::as_os_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3167,"record_offset":7604,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":46522,"source_line":202,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3167,"record_offset":8700,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":108143,"source_line":698,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::fs::Metadata::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":3167,"record_offset":7492,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":16199,"source_line":1622,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":3167,"record_offset":7324,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":12459,"source_line":2936,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":3167,"record_offset":3892,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":13003,"source_line":3025,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":3167,"record_offset":3724,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":3167,"record_offset":3752,"parent_record_offset":3468,"ancestor_record_offset":3468,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3167,"record_offset":4452,"parent_record_offset":4428,"ancestor_record_offset":3468,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3167,"record_offset":4428,"parent_record_offset":4340,"ancestor_record_offset":3468,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":3167,"record_offset":4340,"parent_record_offset":4312,"ancestor_record_offset":3468,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3167,"record_offset":7140,"parent_record_offset":7116,"ancestor_record_offset":3468,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3167,"record_offset":7188,"parent_record_offset":7164,"ancestor_record_offset":3468,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3167,"record_offset":13364,"parent_record_offset":13332,"ancestor_record_offset":3468,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3167,"record_offset":6108,"parent_record_offset":6080,"ancestor_record_offset":3468,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3167,"record_offset":6080,"parent_record_offset":6056,"ancestor_record_offset":3468,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3167,"record_offset":13412,"parent_record_offset":13388,"ancestor_record_offset":3468,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3167,"record_offset":13388,"parent_record_offset":13364,"ancestor_record_offset":3468,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3167,"record_offset":6056,"parent_record_offset":6032,"ancestor_record_offset":3468,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3167,"record_offset":13656,"parent_record_offset":13604,"ancestor_record_offset":3468,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3167,"record_offset":13604,"parent_record_offset":13568,"ancestor_record_offset":3468,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3167,"record_offset":11624,"parent_record_offset":11588,"ancestor_record_offset":3468,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3167,"record_offset":11588,"parent_record_offset":11564,"ancestor_record_offset":3468,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":13332,"parent_record_offset":13280,"ancestor_record_offset":3468,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3167,"record_offset":10468,"parent_record_offset":10444,"ancestor_record_offset":3468,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":3167,"record_offset":7252,"parent_record_offset":7228,"ancestor_record_offset":3468,"inlinee":12447,"source_line":316,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":3167,"record_offset":7228,"parent_record_offset":7076,"ancestor_record_offset":3468,"inlinee":12446,"source_line":33,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3167,"record_offset":6764,"parent_record_offset":6700,"ancestor_record_offset":3468,"inlinee":108137,"source_line":30,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3167,"record_offset":6800,"parent_record_offset":6764,"ancestor_record_offset":3468,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::scan::scan_asset_tree_walker::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/scan.rs
//     evidence={"module":3167,"record_offset":7116,"parent_record_offset":7076,"ancestor_record_offset":3468,"inlinee":108141,"source_line":34,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::scan::scan_asset_tree_walker::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/scan.rs
//     evidence={"module":3167,"record_offset":14400,"parent_record_offset":14364,"ancestor_record_offset":3468,"inlinee":108145,"source_line":72,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::scan::scan_asset_tree_walker::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/scan.rs
//     evidence={"module":3167,"record_offset":9356,"parent_record_offset":9324,"ancestor_record_offset":3468,"inlinee":108144,"source_line":110,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":3167,"record_offset":4372,"parent_record_offset":4340,"ancestor_record_offset":3468,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3167,"record_offset":5416,"parent_record_offset":5384,"ancestor_record_offset":3468,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3167,"record_offset":5476,"parent_record_offset":5444,"ancestor_record_offset":3468,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3167,"record_offset":13532,"parent_record_offset":13508,"ancestor_record_offset":3468,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3167,"record_offset":7852,"parent_record_offset":7828,"ancestor_record_offset":3468,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3167,"record_offset":15724,"parent_record_offset":15700,"ancestor_record_offset":3468,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3167,"record_offset":15808,"parent_record_offset":15784,"ancestor_record_offset":3468,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3167,"record_offset":14424,"parent_record_offset":14400,"ancestor_record_offset":3468,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3167,"record_offset":13684,"parent_record_offset":13604,"ancestor_record_offset":3468,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3167,"record_offset":15748,"parent_record_offset":15724,"ancestor_record_offset":3468,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3167,"record_offset":16260,"parent_record_offset":15588,"ancestor_record_offset":3468,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3167,"record_offset":13508,"parent_record_offset":13484,"ancestor_record_offset":3468,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3167,"record_offset":5272,"parent_record_offset":5248,"ancestor_record_offset":3468,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3167,"record_offset":4396,"parent_record_offset":4372,"ancestor_record_offset":3468,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3167,"record_offset":20084,"parent_record_offset":20056,"ancestor_record_offset":3468,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3167,"record_offset":5444,"parent_record_offset":5384,"ancestor_record_offset":3468,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3167,"record_offset":15960,"parent_record_offset":15932,"ancestor_record_offset":3468,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3167,"record_offset":15932,"parent_record_offset":15904,"ancestor_record_offset":3468,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3167,"record_offset":5020,"parent_record_offset":4980,"ancestor_record_offset":3468,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3167,"record_offset":7164,"parent_record_offset":7140,"ancestor_record_offset":3468,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3167,"record_offset":9412,"parent_record_offset":9356,"ancestor_record_offset":3468,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3167,"record_offset":5136,"parent_record_offset":5108,"ancestor_record_offset":3468,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3167,"record_offset":6828,"parent_record_offset":6800,"ancestor_record_offset":3468,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=180
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":3996,"parent_record_offset":3892,"ancestor_record_offset":3468,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3167,"record_offset":5224,"parent_record_offset":5108,"ancestor_record_offset":3468,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3167,"record_offset":16080,"parent_record_offset":16056,"ancestor_record_offset":3468,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3167,"record_offset":4140,"parent_record_offset":4112,"ancestor_record_offset":3468,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3167,"record_offset":4188,"parent_record_offset":4164,"ancestor_record_offset":3468,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3167,"record_offset":4164,"parent_record_offset":4140,"ancestor_record_offset":3468,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3167,"record_offset":11656,"parent_record_offset":11624,"ancestor_record_offset":3468,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":5300,"parent_record_offset":5272,"ancestor_record_offset":3468,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":5248,"parent_record_offset":5224,"ancestor_record_offset":3468,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":5328,"parent_record_offset":5272,"ancestor_record_offset":3468,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3167,"record_offset":13712,"parent_record_offset":13604,"ancestor_record_offset":3468,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3167,"record_offset":3960,"parent_record_offset":3932,"ancestor_record_offset":3468,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3167,"record_offset":5824,"parent_record_offset":5732,"ancestor_record_offset":3468,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3167,"record_offset":5860,"parent_record_offset":5732,"ancestor_record_offset":3468,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3167,"record_offset":7876,"parent_record_offset":7852,"ancestor_record_offset":3468,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3167,"record_offset":7900,"parent_record_offset":7876,"ancestor_record_offset":3468,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3167,"record_offset":7828,"parent_record_offset":7804,"ancestor_record_offset":3468,"inlinee":5361,"source_line":2581,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3167,"record_offset":15348,"parent_record_offset":15320,"ancestor_record_offset":3468,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3167,"record_offset":15376,"parent_record_offset":15348,"ancestor_record_offset":3468,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3167,"record_offset":7804,"parent_record_offset":7780,"ancestor_record_offset":3468,"inlinee":8748,"source_line":981,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3167,"record_offset":7780,"parent_record_offset":7756,"ancestor_record_offset":3468,"inlinee":9146,"source_line":1013,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3167,"record_offset":11684,"parent_record_offset":11624,"ancestor_record_offset":3468,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3167,"record_offset":11716,"parent_record_offset":11588,"ancestor_record_offset":3468,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3167,"record_offset":15868,"parent_record_offset":15840,"ancestor_record_offset":3468,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3167,"record_offset":15904,"parent_record_offset":15868,"ancestor_record_offset":3468,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3167,"record_offset":15840,"parent_record_offset":15588,"ancestor_record_offset":3468,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3167,"record_offset":15700,"parent_record_offset":15588,"ancestor_record_offset":3468,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3167,"record_offset":16288,"parent_record_offset":15588,"ancestor_record_offset":3468,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3167,"record_offset":15784,"parent_record_offset":15588,"ancestor_record_offset":3468,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":3167,"record_offset":15672,"parent_record_offset":15588,"ancestor_record_offset":3468,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":3167,"record_offset":16196,"parent_record_offset":16172,"ancestor_record_offset":3468,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3167,"record_offset":15448,"parent_record_offset":15416,"ancestor_record_offset":3468,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3167,"record_offset":15480,"parent_record_offset":15448,"ancestor_record_offset":3468,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3167,"record_offset":10412,"parent_record_offset":10388,"ancestor_record_offset":3468,"inlinee":4956,"source_line":845,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3167,"record_offset":14008,"parent_record_offset":13984,"ancestor_record_offset":3468,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3167,"record_offset":14040,"parent_record_offset":14008,"ancestor_record_offset":3468,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3167,"record_offset":16172,"parent_record_offset":16004,"ancestor_record_offset":3468,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3167,"record_offset":14068,"parent_record_offset":14040,"ancestor_record_offset":3468,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3167,"record_offset":16112,"parent_record_offset":16028,"ancestor_record_offset":3468,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3167,"record_offset":16056,"parent_record_offset":16028,"ancestor_record_offset":3468,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3167,"record_offset":16380,"parent_record_offset":15588,"ancestor_record_offset":3468,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3167,"record_offset":16028,"parent_record_offset":16004,"ancestor_record_offset":3468,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3167,"record_offset":15564,"parent_record_offset":15540,"ancestor_record_offset":3468,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3167,"record_offset":15540,"parent_record_offset":15480,"ancestor_record_offset":3468,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3167,"record_offset":15588,"parent_record_offset":15564,"ancestor_record_offset":3468,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3167,"record_offset":15644,"parent_record_offset":15588,"ancestor_record_offset":3468,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3167,"record_offset":10828,"parent_record_offset":10804,"ancestor_record_offset":3468,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3167,"record_offset":16004,"parent_record_offset":15588,"ancestor_record_offset":3468,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::from_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":3167,"record_offset":5888,"parent_record_offset":5732,"ancestor_record_offset":3468,"inlinee":13006,"source_line":108,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/options.rs
//     evidence={"module":3167,"record_offset":5912,"parent_record_offset":5888,"ancestor_record_offset":3468,"inlinee":12914,"source_line":51,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3167,"record_offset":8668,"parent_record_offset":7952,"ancestor_record_offset":3468,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3167,"record_offset":4868,"parent_record_offset":4836,"ancestor_record_offset":3468,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3167,"record_offset":9384,"parent_record_offset":9356,"ancestor_record_offset":3468,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$44::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3167,"record_offset":8804,"parent_record_offset":8780,"ancestor_record_offset":3468,"inlinee":12384,"source_line":1427,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$45::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3167,"record_offset":8780,"parent_record_offset":8700,"ancestor_record_offset":3468,"inlinee":46232,"source_line":1435,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3167,"record_offset":3800,"parent_record_offset":3776,"ancestor_record_offset":3468,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3167,"record_offset":3848,"parent_record_offset":3824,"ancestor_record_offset":3468,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3167,"record_offset":3824,"parent_record_offset":3800,"ancestor_record_offset":3468,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":3167,"record_offset":7516,"parent_record_offset":7492,"ancestor_record_offset":3468,"inlinee":13008,"source_line":2036,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":3167,"record_offset":3932,"parent_record_offset":3892,"ancestor_record_offset":3468,"inlinee":12458,"source_line":2381,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":3167,"record_offset":5792,"parent_record_offset":5732,"ancestor_record_offset":3468,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3167,"record_offset":4924,"parent_record_offset":4896,"ancestor_record_offset":3468,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3167,"record_offset":5184,"parent_record_offset":5160,"ancestor_record_offset":3468,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3167,"record_offset":5048,"parent_record_offset":5020,"ancestor_record_offset":3468,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3167,"record_offset":5384,"parent_record_offset":4980,"ancestor_record_offset":3468,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3167,"record_offset":4896,"parent_record_offset":4868,"ancestor_record_offset":3468,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":3167,"record_offset":4112,"parent_record_offset":4080,"ancestor_record_offset":3468,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":3167,"record_offset":4080,"parent_record_offset":4052,"ancestor_record_offset":3468,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":3167,"record_offset":3776,"parent_record_offset":3752,"ancestor_record_offset":3468,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":3167,"record_offset":7540,"parent_record_offset":7516,"ancestor_record_offset":3468,"inlinee":13010,"source_line":1126,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::FileType::is_symlink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":3167,"record_offset":7564,"parent_record_offset":7540,"ancestor_record_offset":3468,"inlinee":13011,"source_line":1132,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3167,"record_offset":5076,"parent_record_offset":5048,"ancestor_record_offset":3468,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3167,"record_offset":5108,"parent_record_offset":5076,"ancestor_record_offset":3468,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3167,"record_offset":4980,"parent_record_offset":4952,"ancestor_record_offset":3468,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3167,"record_offset":4952,"parent_record_offset":4924,"ancestor_record_offset":3468,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::scan::scan_asset_tree_walker::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A66D
// 首条位置: {"module":3167,"inlinee":108141}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3167,"record_offset":7140,"parent_record_offset":7116,"ancestor_record_offset":7116,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3167,"record_offset":7188,"parent_record_offset":7164,"ancestor_record_offset":7116,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3167,"record_offset":7164,"parent_record_offset":7140,"ancestor_record_offset":7116,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::scan::scan_asset_tree_walker::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E84F
// 首条位置: {"module":4092,"inlinee":125007}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4092,"record_offset":520,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/04092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4092,"record_offset":568,"parent_record_offset":544,"ancestor_record_offset":500,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/04092.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4092,"record_offset":544,"parent_record_offset":520,"ancestor_record_offset":500,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/04092.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::scan::scan_asset_tree_walker::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A671
// 首条位置: {"module":3167,"inlinee":108145}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3167,"record_offset":14424,"parent_record_offset":14400,"ancestor_record_offset":14400,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::scan::scan_asset_tree_walker::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A670
// 首条位置: {"module":3167,"inlinee":108144}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3167,"record_offset":9412,"parent_record_offset":9356,"ancestor_record_offset":9356,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3167,"record_offset":9384,"parent_record_offset":9356,"ancestor_record_offset":9356,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::scan::load_texture_group_config
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 162
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A667
// 首条位置: {"module":3167,"inlinee":108135}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":5984,"parent_record_offset":5732,"ancestor_record_offset":5732,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3167,"record_offset":5824,"parent_record_offset":5732,"ancestor_record_offset":5732,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3167,"record_offset":5860,"parent_record_offset":5732,"ancestor_record_offset":5732,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] ron::de::from_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":3167,"record_offset":5888,"parent_record_offset":5732,"ancestor_record_offset":5732,"inlinee":13006,"source_line":108,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":3167,"record_offset":5792,"parent_record_offset":5732,"ancestor_record_offset":5732,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3167,"record_offset":6160,"parent_record_offset":6136,"ancestor_record_offset":5732,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3167,"record_offset":6136,"parent_record_offset":6080,"ancestor_record_offset":5732,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3167,"record_offset":6108,"parent_record_offset":6080,"ancestor_record_offset":5732,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3167,"record_offset":6080,"parent_record_offset":6056,"ancestor_record_offset":5732,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3167,"record_offset":6056,"parent_record_offset":6032,"ancestor_record_offset":5732,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":6008,"parent_record_offset":5984,"ancestor_record_offset":5732,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/options.rs
//     evidence={"module":3167,"record_offset":5912,"parent_record_offset":5888,"ancestor_record_offset":5732,"inlinee":12914,"source_line":51,"archive":"_pdb/B/modules/03167.jsonl.gz"}
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
// 完整结构及原始类型记录: scan.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: scan.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: scan.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: asset::texture::scan::load_texture_group_config
// 信息 ID: 04a71946247da9d7a3a5e5bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3167, inline_record=5732; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::scan::scan_asset_tree
// 信息 ID: 545a87b9e5790c924e8fd4a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture::compile_texture_assets @ 0x17b7c2b -> 0x20d8ba0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::scan::scan_asset_tree_walker
// 信息 ID: 01e23c48c53bf1fb2207af93
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::texture::scan::scan_asset_tree @ 0x20d8c5a -> 0x20d8d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::texture::scan::scan_asset_tree_walker @ 0x20d96af -> 0x20d8d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::texture::scan::scan_asset_tree_walker @ 0x20d9878 -> 0x20d8d00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::scan::scan_asset_tree_walker::closure$0
// 信息 ID: 846dea734061e194235aff30
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset::texture::scan::scan_asset_tree_walker", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3167, inline_record=7116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::scan::scan_asset_tree_walker::closure$1
// 信息 ID: 7fc26aca55fb0ed65bee2c7f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "asset::texture::scan::scan_asset_tree_walker", "captures": "unknown"}
// [闭包环境类型引用] asset::texture::scan::scan_asset_tree_walker::closure_env$1 <- anyhow::context::impl$0::with_context<std::fs::ReadDir,std::io::error::Error,alloc::string::String,asset::texture::scan::scan_asset_tree_walker::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<std::fs::ReadDir,std::io::error::Error,alloc::string::String,asset::texture::scan::scan_asset_tree_walker::closure_env$1>; module=4092, inline_record=500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::scan::scan_asset_tree_walker::closure$2
// 信息 ID: ec0c47733504181f64e7850b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "asset::texture::scan::scan_asset_tree_walker", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3167, inline_record=14400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::scan::scan_asset_tree_walker::closure$3
// 信息 ID: 7ddfdbbbb1a18c6b8ffe2e45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "asset::texture::scan::scan_asset_tree_walker", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3167, inline_record=9356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

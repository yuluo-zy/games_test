// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/collision_detection.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 25c114dd1137b57fd78bf3394abd55693b98703e683bc46ced0200004a7d2b1a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: collision_detection.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1522, 1536, 3051
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::collision_detection::intersect_wall_decorators<system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3051,"record_offset":5608,"rva":33446144,"va":5402155264}
// 编译实例 RVAs: 0x1fe5900
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141fe50e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: collision_detection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402155264,"raw_hex":"e8fbf6ffff","size":5,"soft_fail":false,"target_va":5402153184,"va":5402155488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141fe5310
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: collision_detection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402155264,"raw_hex":"e8bbf7ffff","size":5,"soft_fail":false,"target_va":5402153744,"va":5402155856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9184386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402155264,"raw_hex":"e882248c00","size":5,"soft_fail":false,"target_va":5411340064,"va":5402155673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] system_decorator::collision_detection::intersect_wall_decorators_impl
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/collision_detection.rs
//     evidence={"module":3051,"record_offset":5868,"parent_record_offset":5608,"ancestor_record_offset":5608,"inlinee":106216,"source_line":52,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3051,"record_offset":7400,"parent_record_offset":7368,"ancestor_record_offset":5608,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3051,"record_offset":7540,"parent_record_offset":7516,"ancestor_record_offset":5608,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3051,"record_offset":7448,"parent_record_offset":7424,"ancestor_record_offset":5608,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3051,"record_offset":7424,"parent_record_offset":7400,"ancestor_record_offset":5608,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3051,"record_offset":7564,"parent_record_offset":7540,"ancestor_record_offset":5608,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3051,"record_offset":7368,"parent_record_offset":6360,"ancestor_record_offset":5608,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3051,"record_offset":6360,"parent_record_offset":6336,"ancestor_record_offset":5608,"inlinee":11829,"source_line":3524,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3051,"record_offset":7340,"parent_record_offset":6360,"ancestor_record_offset":5608,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3051,"record_offset":7516,"parent_record_offset":6360,"ancestor_record_offset":5608,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3051,"record_offset":7488,"parent_record_offset":6360,"ancestor_record_offset":5608,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3051,"record_offset":6284,"parent_record_offset":5868,"ancestor_record_offset":5608,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3051,"record_offset":6336,"parent_record_offset":6284,"ancestor_record_offset":5608,"inlinee":11862,"source_line":18,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3051,"record_offset":6140,"parent_record_offset":6112,"ancestor_record_offset":5608,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3051,"record_offset":6200,"parent_record_offset":6168,"ancestor_record_offset":5608,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":3051,"record_offset":6408,"parent_record_offset":6360,"ancestor_record_offset":5608,"inlinee":9843,"source_line":63,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":3051,"record_offset":6308,"parent_record_offset":6284,"ancestor_record_offset":5608,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3051,"record_offset":6432,"parent_record_offset":6408,"ancestor_record_offset":5608,"inlinee":6090,"source_line":2852,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3051,"record_offset":7300,"parent_record_offset":6456,"ancestor_record_offset":5608,"inlinee":6096,"source_line":2859,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3051,"record_offset":6456,"parent_record_offset":6432,"ancestor_record_offset":5608,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3051,"record_offset":6824,"parent_record_offset":6792,"ancestor_record_offset":5608,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3051,"record_offset":6792,"parent_record_offset":6760,"ancestor_record_offset":5608,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3051,"record_offset":6084,"parent_record_offset":6052,"ancestor_record_offset":5608,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3051,"record_offset":6928,"parent_record_offset":6904,"ancestor_record_offset":5608,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3051,"record_offset":6716,"parent_record_offset":6668,"ancestor_record_offset":5608,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3051,"record_offset":6760,"parent_record_offset":6716,"ancestor_record_offset":5608,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3051,"record_offset":6868,"parent_record_offset":6668,"ancestor_record_offset":5608,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3051,"record_offset":6668,"parent_record_offset":6600,"ancestor_record_offset":5608,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3051,"record_offset":6112,"parent_record_offset":6052,"ancestor_record_offset":5608,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3051,"record_offset":6168,"parent_record_offset":6112,"ancestor_record_offset":5608,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3051,"record_offset":7016,"parent_record_offset":6600,"ancestor_record_offset":5608,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3051,"record_offset":5916,"parent_record_offset":5892,"ancestor_record_offset":5608,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3051,"record_offset":6516,"parent_record_offset":6488,"ancestor_record_offset":5608,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":7232,"parent_record_offset":7208,"ancestor_record_offset":5608,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":7208,"parent_record_offset":6516,"ancestor_record_offset":5608,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":6904,"parent_record_offset":6600,"ancestor_record_offset":5608,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":6052,"parent_record_offset":5968,"ancestor_record_offset":5608,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":6600,"parent_record_offset":6552,"ancestor_record_offset":5608,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":5944,"parent_record_offset":5916,"ancestor_record_offset":5608,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":6024,"parent_record_offset":5968,"ancestor_record_offset":5608,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":5996,"parent_record_offset":5968,"ancestor_record_offset":5608,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":5968,"parent_record_offset":5944,"ancestor_record_offset":5608,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":6552,"parent_record_offset":6516,"ancestor_record_offset":5608,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3051,"record_offset":5892,"parent_record_offset":5868,"ancestor_record_offset":5608,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3051,"record_offset":6488,"parent_record_offset":6456,"ancestor_record_offset":5608,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::collision_detection::intersect_wall_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3051,"record_offset":9376,"rva":33446944,"va":5402156064}
// 编译实例 RVAs: 0x1fe5c20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141fe5530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: collision_detection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402156064,"raw_hex":"e81bf8ffff","size":5,"soft_fail":false,"target_va":5402154288,"va":5402156304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141fe5720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: collision_detection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402156064,"raw_hex":"e87bf8ffff","size":5,"soft_fail":false,"target_va":5402154784,"va":5402156704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9183570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402156064,"raw_hex":"e852218c00","size":5,"soft_fail":false,"target_va":5411340064,"va":5402156489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] system_decorator::collision_detection::intersect_wall_decorators_impl
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/collision_detection.rs
//     evidence={"module":3051,"record_offset":9644,"parent_record_offset":9376,"ancestor_record_offset":9376,"inlinee":106216,"source_line":52,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3051,"record_offset":11180,"parent_record_offset":11148,"ancestor_record_offset":9376,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3051,"record_offset":11320,"parent_record_offset":11296,"ancestor_record_offset":9376,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3051,"record_offset":11228,"parent_record_offset":11204,"ancestor_record_offset":9376,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3051,"record_offset":11204,"parent_record_offset":11180,"ancestor_record_offset":9376,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3051,"record_offset":11344,"parent_record_offset":11320,"ancestor_record_offset":9376,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3051,"record_offset":11148,"parent_record_offset":10140,"ancestor_record_offset":9376,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3051,"record_offset":10140,"parent_record_offset":10116,"ancestor_record_offset":9376,"inlinee":11829,"source_line":3524,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3051,"record_offset":11120,"parent_record_offset":10140,"ancestor_record_offset":9376,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3051,"record_offset":11296,"parent_record_offset":10140,"ancestor_record_offset":9376,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3051,"record_offset":11268,"parent_record_offset":10140,"ancestor_record_offset":9376,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3051,"record_offset":10064,"parent_record_offset":9644,"ancestor_record_offset":9376,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3051,"record_offset":10116,"parent_record_offset":10064,"ancestor_record_offset":9376,"inlinee":11862,"source_line":18,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3051,"record_offset":9920,"parent_record_offset":9892,"ancestor_record_offset":9376,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3051,"record_offset":9980,"parent_record_offset":9948,"ancestor_record_offset":9376,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":3051,"record_offset":10188,"parent_record_offset":10140,"ancestor_record_offset":9376,"inlinee":9843,"source_line":63,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":3051,"record_offset":10088,"parent_record_offset":10064,"ancestor_record_offset":9376,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3051,"record_offset":10212,"parent_record_offset":10188,"ancestor_record_offset":9376,"inlinee":6090,"source_line":2852,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3051,"record_offset":11080,"parent_record_offset":10236,"ancestor_record_offset":9376,"inlinee":6096,"source_line":2859,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3051,"record_offset":10236,"parent_record_offset":10212,"ancestor_record_offset":9376,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3051,"record_offset":10604,"parent_record_offset":10572,"ancestor_record_offset":9376,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3051,"record_offset":10572,"parent_record_offset":10540,"ancestor_record_offset":9376,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3051,"record_offset":9864,"parent_record_offset":9832,"ancestor_record_offset":9376,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3051,"record_offset":10708,"parent_record_offset":10684,"ancestor_record_offset":9376,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3051,"record_offset":10496,"parent_record_offset":10448,"ancestor_record_offset":9376,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3051,"record_offset":10540,"parent_record_offset":10496,"ancestor_record_offset":9376,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3051,"record_offset":10648,"parent_record_offset":10448,"ancestor_record_offset":9376,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3051,"record_offset":10448,"parent_record_offset":10380,"ancestor_record_offset":9376,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3051,"record_offset":9892,"parent_record_offset":9832,"ancestor_record_offset":9376,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3051,"record_offset":9948,"parent_record_offset":9892,"ancestor_record_offset":9376,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3051,"record_offset":10796,"parent_record_offset":10380,"ancestor_record_offset":9376,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3051,"record_offset":9692,"parent_record_offset":9668,"ancestor_record_offset":9376,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3051,"record_offset":10296,"parent_record_offset":10268,"ancestor_record_offset":9376,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":11012,"parent_record_offset":10988,"ancestor_record_offset":9376,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":10988,"parent_record_offset":10296,"ancestor_record_offset":9376,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":10684,"parent_record_offset":10380,"ancestor_record_offset":9376,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":9832,"parent_record_offset":9744,"ancestor_record_offset":9376,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":10380,"parent_record_offset":10332,"ancestor_record_offset":9376,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":9720,"parent_record_offset":9692,"ancestor_record_offset":9376,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":9804,"parent_record_offset":9744,"ancestor_record_offset":9376,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":9776,"parent_record_offset":9744,"ancestor_record_offset":9376,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":9744,"parent_record_offset":9720,"ancestor_record_offset":9376,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":10332,"parent_record_offset":10296,"ancestor_record_offset":9376,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3051,"record_offset":9668,"parent_record_offset":9644,"ancestor_record_offset":9376,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3051,"record_offset":10268,"parent_record_offset":10236,"ancestor_record_offset":9376,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::collision_detection::intersect_wall_decorators_impl
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19EE8
// 首条位置: {"module":3051,"inlinee":106216}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3051,"record_offset":6284,"parent_record_offset":5868,"ancestor_record_offset":5868,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3051,"record_offset":5892,"parent_record_offset":5868,"ancestor_record_offset":5868,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3051,"record_offset":7400,"parent_record_offset":7368,"ancestor_record_offset":5868,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3051,"record_offset":7540,"parent_record_offset":7516,"ancestor_record_offset":5868,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3051,"record_offset":7448,"parent_record_offset":7424,"ancestor_record_offset":5868,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3051,"record_offset":7424,"parent_record_offset":7400,"ancestor_record_offset":5868,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3051,"record_offset":7564,"parent_record_offset":7540,"ancestor_record_offset":5868,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3051,"record_offset":7368,"parent_record_offset":6360,"ancestor_record_offset":5868,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3051,"record_offset":6360,"parent_record_offset":6336,"ancestor_record_offset":5868,"inlinee":11829,"source_line":3524,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3051,"record_offset":7340,"parent_record_offset":6360,"ancestor_record_offset":5868,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3051,"record_offset":7516,"parent_record_offset":6360,"ancestor_record_offset":5868,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3051,"record_offset":7488,"parent_record_offset":6360,"ancestor_record_offset":5868,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3051,"record_offset":6336,"parent_record_offset":6284,"ancestor_record_offset":5868,"inlinee":11862,"source_line":18,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3051,"record_offset":6140,"parent_record_offset":6112,"ancestor_record_offset":5868,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3051,"record_offset":6200,"parent_record_offset":6168,"ancestor_record_offset":5868,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":3051,"record_offset":6408,"parent_record_offset":6360,"ancestor_record_offset":5868,"inlinee":9843,"source_line":63,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":3051,"record_offset":6308,"parent_record_offset":6284,"ancestor_record_offset":5868,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3051,"record_offset":6432,"parent_record_offset":6408,"ancestor_record_offset":5868,"inlinee":6090,"source_line":2852,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3051,"record_offset":7300,"parent_record_offset":6456,"ancestor_record_offset":5868,"inlinee":6096,"source_line":2859,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3051,"record_offset":6456,"parent_record_offset":6432,"ancestor_record_offset":5868,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3051,"record_offset":6824,"parent_record_offset":6792,"ancestor_record_offset":5868,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3051,"record_offset":6792,"parent_record_offset":6760,"ancestor_record_offset":5868,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3051,"record_offset":6084,"parent_record_offset":6052,"ancestor_record_offset":5868,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3051,"record_offset":6928,"parent_record_offset":6904,"ancestor_record_offset":5868,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3051,"record_offset":6716,"parent_record_offset":6668,"ancestor_record_offset":5868,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3051,"record_offset":6760,"parent_record_offset":6716,"ancestor_record_offset":5868,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3051,"record_offset":6868,"parent_record_offset":6668,"ancestor_record_offset":5868,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3051,"record_offset":6668,"parent_record_offset":6600,"ancestor_record_offset":5868,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3051,"record_offset":6112,"parent_record_offset":6052,"ancestor_record_offset":5868,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3051,"record_offset":6168,"parent_record_offset":6112,"ancestor_record_offset":5868,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3051,"record_offset":7016,"parent_record_offset":6600,"ancestor_record_offset":5868,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3051,"record_offset":5916,"parent_record_offset":5892,"ancestor_record_offset":5868,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3051,"record_offset":6516,"parent_record_offset":6488,"ancestor_record_offset":5868,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":7232,"parent_record_offset":7208,"ancestor_record_offset":5868,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":7208,"parent_record_offset":6516,"ancestor_record_offset":5868,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":6904,"parent_record_offset":6600,"ancestor_record_offset":5868,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":6052,"parent_record_offset":5968,"ancestor_record_offset":5868,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":6600,"parent_record_offset":6552,"ancestor_record_offset":5868,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":5944,"parent_record_offset":5916,"ancestor_record_offset":5868,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":6024,"parent_record_offset":5968,"ancestor_record_offset":5868,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":5996,"parent_record_offset":5968,"ancestor_record_offset":5868,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":5968,"parent_record_offset":5944,"ancestor_record_offset":5868,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3051,"record_offset":6552,"parent_record_offset":6516,"ancestor_record_offset":5868,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3051,"record_offset":6488,"parent_record_offset":6456,"ancestor_record_offset":5868,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::collision_detection::intersect_wall_decorators_impl::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19EE3
// 首条位置: {"module":3051,"inlinee":106211}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3051,"record_offset":4104,"parent_record_offset":3652,"ancestor_record_offset":3652,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3051,"record_offset":1184,"parent_record_offset":784,"ancestor_record_offset":784,"inlinee":25147,"source_line":285,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3051,"record_offset":1460,"parent_record_offset":784,"ancestor_record_offset":784,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::impl$0::snapping_direction
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/lib.rs
//     evidence={"module":3051,"record_offset":1432,"parent_record_offset":784,"ancestor_record_offset":784,"inlinee":25468,"source_line":63,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/merge_proposal_wall_decorators.rs
//     evidence={"module":3051,"record_offset":852,"parent_record_offset":784,"ancestor_record_offset":784,"inlinee":106213,"source_line":202,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":3051,"record_offset":3736,"parent_record_offset":3652,"ancestor_record_offset":3652,"inlinee":106214,"source_line":546,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceRegion::intersects
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":3051,"record_offset":1552,"parent_record_offset":784,"ancestor_record_offset":784,"inlinee":25152,"source_line":70,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3051,"record_offset":872,"parent_record_offset":852,"ancestor_record_offset":784,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3051,"record_offset":3076,"parent_record_offset":3052,"ancestor_record_offset":2188,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3051,"record_offset":1304,"parent_record_offset":1280,"ancestor_record_offset":784,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3051,"record_offset":1492,"parent_record_offset":1460,"ancestor_record_offset":784,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3051,"record_offset":1248,"parent_record_offset":1220,"ancestor_record_offset":784,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3051,"record_offset":1084,"parent_record_offset":1056,"ancestor_record_offset":784,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3051,"record_offset":1008,"parent_record_offset":984,"ancestor_record_offset":784,"inlinee":12925,"source_line":236,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3051,"record_offset":1116,"parent_record_offset":1032,"ancestor_record_offset":784,"inlinee":12927,"source_line":237,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3051,"record_offset":984,"parent_record_offset":852,"ancestor_record_offset":784,"inlinee":12923,"source_line":2551,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3051,"record_offset":1032,"parent_record_offset":1008,"ancestor_record_offset":784,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3051,"record_offset":1056,"parent_record_offset":1032,"ancestor_record_offset":784,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3051,"record_offset":896,"parent_record_offset":872,"ancestor_record_offset":784,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3051,"record_offset":1340,"parent_record_offset":1220,"ancestor_record_offset":784,"inlinee":25150,"source_line":118,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3051,"record_offset":1220,"parent_record_offset":1184,"ancestor_record_offset":784,"inlinee":25148,"source_line":241,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3051,"record_offset":1280,"parent_record_offset":1248,"ancestor_record_offset":784,"inlinee":25160,"source_line":244,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$0::regions_intersect
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3051,"record_offset":1596,"parent_record_offset":1552,"ancestor_record_offset":784,"inlinee":25154,"source_line":39,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$2::distance
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3051,"record_offset":3052,"parent_record_offset":3028,"ancestor_record_offset":2188,"inlinee":44783,"source_line":91,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3051,"record_offset":1364,"parent_record_offset":1340,"ancestor_record_offset":784,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3051,"record_offset":1392,"parent_record_offset":1340,"ancestor_record_offset":784,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::collision_detection::DecoratorWorldSpace::project_to_wall_space
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11782
// 首条位置: {"module":1522,"inlinee":71554}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1522,"record_offset":2408,"parent_record_offset":2376,"ancestor_record_offset":2376,"inlinee":24561,"source_line":207,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1522,"record_offset":2432,"parent_record_offset":2408,"ancestor_record_offset":2376,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01522.jsonl.gz"}
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
// 完整结构及原始类型记录: collision_detection.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_decorator::collision_detection::DecoratorIntersection
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "ref$<system_decorator::collision_detection::DecoratorIntersection>", "constructor": "ref$", "arguments": ["system_decorator::collision_detection::DecoratorIntersection"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<system_decorator::collision_detection::DecoratorIntersection> >", "constructor": "tuple$", "arguments": ["ref$<system_decorator::collision_detection::DecoratorIntersection>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$3::call_mut<tuple$<ref$<system_decorator::collision_detection::DecoratorIntersection> >,system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure_env$0>", "constructor": "core::ops::function::impls::impl$3::call_mut", "arguments": ["tuple$<ref$<system_decorator::collision_detection::DecoratorIntersection> >", "system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_decorator::collision_detection::DecoratorWorldSpace
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: collision_detection.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: collision_detection.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::collision_detection::DecoratorWorldSpace::project_to_wall_space
// 信息 ID: 3d8e7b1050407d969cc4a467
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1522, inline_record=2376; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::collision_detection::intersect_wall_decorators<system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1>
// 信息 ID: 2a7b4bf8005c01ca8416f220
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_decorator::collision_detection::intersect_wall_decorators -> ["system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner @ 0x20022a4 -> 0x1fe5900 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::collision_detection::intersect_wall_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0>
// 信息 ID: 30d1488d93ad71f09eec0ea1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_decorator::collision_detection::intersect_wall_decorators -> ["system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0x127d298 -> 0x1fe5c20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::collision_detection::intersect_wall_decorators_impl
// 信息 ID: 6175d2aed770c1ff0e8e2543
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::collision_detection::intersect_wall_decorators<system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1>; module=3051, inline_record=5868; 不是独立调用边。
// [PDB 内联宿主] system_decorator::collision_detection::intersect_wall_decorators<system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1>; module=3051, inline_record=7616; 不是独立调用边。
// [PDB 内联宿主] system_decorator::collision_detection::intersect_wall_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0>; module=3051, inline_record=9644; 不是独立调用边。
// [PDB 内联宿主] system_decorator::collision_detection::intersect_wall_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0>; module=3051, inline_record=11396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::collision_detection::intersect_wall_decorators_impl::closure$0
// 信息 ID: 6a0ff1315402c2b497f7a981
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::collision_detection::intersect_wall_decorators_impl", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; module=3051, inline_record=784; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; module=3051, inline_record=2188; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; module=3051, inline_record=3652; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; module=3051, inline_record=4868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/rg/resource_info.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 0f92cb3e59af073ef66c194a9ebbaafc349625df27209c37cb85962ac5539f4c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: resource_info.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 66, 104, 105, 184, 217, 219, 533, 676, 696, 698, 1918, 1937, 3274, 4223, 4224
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use vk_sync_fork; // namespace candidate
//   package identities: [{"name":"vk-sync-fork","source":"local_or_path","version":"0.4.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::impl$3::default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2728
// 首条位置: {"module":66,"inlinee":10024}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":66,"record_offset":26784,"parent_record_offset":26748,"ancestor_record_offset":26748,"inlinee":4786,"source_line":1161,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":66,"record_offset":27040,"parent_record_offset":27012,"ancestor_record_offset":26748,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":66,"record_offset":27092,"parent_record_offset":27068,"ancestor_record_offset":26748,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":66,"record_offset":27068,"parent_record_offset":27012,"ancestor_record_offset":26748,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":66,"record_offset":27116,"parent_record_offset":27092,"ancestor_record_offset":26748,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":66,"record_offset":26984,"parent_record_offset":26956,"ancestor_record_offset":26748,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":27140,"parent_record_offset":27116,"ancestor_record_offset":26748,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$7::default
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":66,"record_offset":26840,"parent_record_offset":26812,"ancestor_record_offset":26748,"inlinee":4790,"source_line":2070,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$12::default
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":66,"record_offset":26812,"parent_record_offset":26784,"ancestor_record_offset":26748,"inlinee":4788,"source_line":1355,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":66,"record_offset":26896,"parent_record_offset":26868,"ancestor_record_offset":26748,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":66,"record_offset":27012,"parent_record_offset":26956,"ancestor_record_offset":26748,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":66,"record_offset":26868,"parent_record_offset":26840,"ancestor_record_offset":26748,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":66,"record_offset":26956,"parent_record_offset":26924,"ancestor_record_offset":26748,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":66,"record_offset":26924,"parent_record_offset":26896,"ancestor_record_offset":26748,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources>
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":217,"record_offset":19676,"rva":5440560,"va":5374149680}
// 编译实例 RVAs: 0x530430
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048da00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_info.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-666362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374149680,"raw_hex":"e806d5f5ff","size":5,"soft_fail":false,"target_va":5373483520,"va":5374149877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd4b40
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: resource_info.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6964161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374149680,"raw_hex":"e8c1436a00","size":5,"soft_fail":false,"target_va":5381114688,"va":5374150522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd4b80
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: resource_info.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6964343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374149680,"raw_hex":"e877446a00","size":5,"soft_fail":false,"target_va":5381114752,"va":5374150404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bece40
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: resource_info.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7063451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374149680,"raw_hex":"e89bc76b00","size":5,"soft_fail":false,"target_va":5381213760,"va":5374150304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: resource_info.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37697236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374149680,"raw_hex":"e8d4363f02","size":5,"soft_fail":false,"target_va":5411847200,"va":5374149959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t208584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374149680,"raw_hex":"e8c82e0300","size":5,"soft_fail":false,"target_va":5374359040,"va":5374150451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t37167347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374149680,"raw_hex":"e8f3203702","size":5,"soft_fail":false,"target_va":5411318240,"va":5374150888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t37391374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374149680,"raw_hex":"e80e8c3a02","size":5,"soft_fail":false,"target_va":5411542296,"va":5374150917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource::ResourceOrigin::ty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6972656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374149680,"raw_hex":"e8f0646a00","size":5,"soft_fail":false,"target_va":5381123344,"va":5374150683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_info::ResourceInfo::calculate_lifetime_events
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6963109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374149680,"raw_hex":"e8a53f6a00","size":5,"soft_fail":false,"target_va":5381113952,"va":5374150838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_info::ResourceInfo::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6963130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374149680,"raw_hex":"e8ba3f6a00","size":5,"soft_fail":false,"target_va":5381112848,"va":5374149713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_info::ResourceInfo::extend_usage_flags_for_exports
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6963032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374149680,"raw_hex":"e8583f6a00","size":5,"soft_fail":false,"target_va":5381113184,"va":5374150147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_info::ResourceInfo::extract_creation_flags
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6962867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374149680,"raw_hex":"e8b33e6a00","size":5,"soft_fail":false,"target_va":5381113008,"va":5374150136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_info::ResourceInfo::link_up_next_access_chains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6962973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374149680,"raw_hex":"e81d3f6a00","size":5,"soft_fail":false,"target_va":5381113808,"va":5374150830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_info::buffer_access_mask_to_usage_flags
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6963871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374149680,"raw_hex":"e89f426a00","size":5,"soft_fail":false,"target_va":5381114624,"va":5374150748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_info::image_access_mask_to_usage_flags
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6963979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374149680,"raw_hex":"e80b436a00","size":5,"soft_fail":false,"target_va":5381114320,"va":5374150336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] vk_sync_fork::get_access_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"vk-sync-fork","version":"0.4.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7210461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374149680,"raw_hex":"e8dd056e00","size":5,"soft_fail":false,"target_va":5381361104,"va":5374150638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::resource_info::ResourceInfo::allocate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":217,"record_offset":19884,"parent_record_offset":19676,"ancestor_record_offset":19676,"inlinee":15137,"source_line":55,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_info::ResourceInfo::extract_pass_resource_access
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":217,"record_offset":21808,"parent_record_offset":19676,"ancestor_record_offset":19676,"inlinee":15139,"source_line":124,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":217,"record_offset":20348,"parent_record_offset":20324,"ancestor_record_offset":19676,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":217,"record_offset":20176,"parent_record_offset":20148,"ancestor_record_offset":19676,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":217,"record_offset":20272,"parent_record_offset":20244,"ancestor_record_offset":19676,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":217,"record_offset":20244,"parent_record_offset":20208,"ancestor_record_offset":19676,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":217,"record_offset":20396,"parent_record_offset":20372,"ancestor_record_offset":19676,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":217,"record_offset":20372,"parent_record_offset":20348,"ancestor_record_offset":19676,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":217,"record_offset":20208,"parent_record_offset":20176,"ancestor_record_offset":19676,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":20324,"parent_record_offset":20108,"ancestor_record_offset":19676,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":22296,"parent_record_offset":22272,"ancestor_record_offset":19676,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":20108,"parent_record_offset":20080,"ancestor_record_offset":19676,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":20592,"parent_record_offset":20532,"ancestor_record_offset":19676,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":20564,"parent_record_offset":20532,"ancestor_record_offset":19676,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":22064,"parent_record_offset":21808,"ancestor_record_offset":19676,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":20148,"parent_record_offset":20108,"ancestor_record_offset":19676,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":20532,"parent_record_offset":19884,"ancestor_record_offset":19676,"inlinee":5294,"source_line":3002,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":23112,"parent_record_offset":21808,"ancestor_record_offset":19676,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":22408,"parent_record_offset":21808,"ancestor_record_offset":19676,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":22248,"parent_record_offset":21808,"ancestor_record_offset":19676,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":20052,"parent_record_offset":19884,"ancestor_record_offset":19676,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":22272,"parent_record_offset":22248,"ancestor_record_offset":19676,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":217,"record_offset":21132,"parent_record_offset":20592,"ancestor_record_offset":19676,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":217,"record_offset":20492,"parent_record_offset":20108,"ancestor_record_offset":19676,"inlinee":6531,"source_line":13,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":217,"record_offset":20080,"parent_record_offset":20052,"ancestor_record_offset":19676,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":217,"record_offset":22960,"parent_record_offset":21808,"ancestor_record_offset":19676,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":217,"record_offset":23016,"parent_record_offset":22988,"ancestor_record_offset":19676,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":217,"record_offset":21036,"parent_record_offset":21004,"ancestor_record_offset":19676,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":217,"record_offset":22624,"parent_record_offset":21808,"ancestor_record_offset":19676,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":217,"record_offset":21956,"parent_record_offset":21808,"ancestor_record_offset":19676,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":217,"record_offset":21004,"parent_record_offset":20972,"ancestor_record_offset":19676,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":217,"record_offset":20972,"parent_record_offset":20592,"ancestor_record_offset":19676,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":217,"record_offset":20436,"parent_record_offset":20108,"ancestor_record_offset":19676,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":217,"record_offset":20460,"parent_record_offset":20436,"ancestor_record_offset":19676,"inlinee":6052,"source_line":796,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":217,"record_offset":19940,"parent_record_offset":19884,"ancestor_record_offset":19676,"inlinee":15138,"source_line":653,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":217,"record_offset":22872,"parent_record_offset":21808,"ancestor_record_offset":19676,"inlinee":15140,"source_line":1642,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":217,"record_offset":22896,"parent_record_offset":22872,"ancestor_record_offset":19676,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":217,"record_offset":20016,"parent_record_offset":19992,"ancestor_record_offset":19676,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":217,"record_offset":22024,"parent_record_offset":21984,"ancestor_record_offset":19676,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":217,"record_offset":21076,"parent_record_offset":20592,"ancestor_record_offset":19676,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":217,"record_offset":19968,"parent_record_offset":19884,"ancestor_record_offset":19676,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":217,"record_offset":23140,"parent_record_offset":23112,"ancestor_record_offset":19676,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":217,"record_offset":22444,"parent_record_offset":22408,"ancestor_record_offset":19676,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":217,"record_offset":23168,"parent_record_offset":23140,"ancestor_record_offset":19676,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":217,"record_offset":22480,"parent_record_offset":22444,"ancestor_record_offset":19676,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":217,"record_offset":19992,"parent_record_offset":19968,"ancestor_record_offset":19676,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":217,"record_offset":21984,"parent_record_offset":21956,"ancestor_record_offset":19676,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_info::ResourceAccessIndex::valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":217,"record_offset":22988,"parent_record_offset":21808,"ancestor_record_offset":19676,"inlinee":11912,"source_line":228,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_info::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":217,"record_offset":23044,"parent_record_offset":23016,"ancestor_record_offset":19676,"inlinee":11914,"source_line":252,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::ResourceInfo::clear
// 记录形式: compiled_procedure; instances=1
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":676,"record_offset":364,"rva":12403728,"va":5381112848}
// 编译实例 RVAs: 0xbd4410
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: resource_info.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30734263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381112848,"raw_hex":"e8b7f7d401","size":5,"soft_fail":false,"target_va":5411847200,"va":5381112932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":676,"record_offset":500,"parent_record_offset":364,"ancestor_record_offset":364,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":676,"record_offset":640,"parent_record_offset":364,"ancestor_record_offset":364,"inlinee":17337,"source_line":362,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":676,"record_offset":996,"parent_record_offset":972,"ancestor_record_offset":364,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":676,"record_offset":740,"parent_record_offset":712,"ancestor_record_offset":364,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":676,"record_offset":972,"parent_record_offset":948,"ancestor_record_offset":364,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::impl$2::fill_tag
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":676,"record_offset":948,"parent_record_offset":888,"ancestor_record_offset":364,"inlinee":6225,"source_line":79,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":676,"record_offset":688,"parent_record_offset":664,"ancestor_record_offset":364,"inlinee":17339,"source_line":1009,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":676,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":364,"inlinee":10622,"source_line":813,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":676,"record_offset":804,"parent_record_offset":784,"ancestor_record_offset":364,"inlinee":10625,"source_line":807,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":676,"record_offset":828,"parent_record_offset":804,"ancestor_record_offset":364,"inlinee":10626,"source_line":3020,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::ctrl_slice
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":676,"record_offset":888,"parent_record_offset":828,"ancestor_record_offset":364,"inlinee":10627,"source_line":2482,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":676,"record_offset":860,"parent_record_offset":828,"ancestor_record_offset":364,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::num_ctrl_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":676,"record_offset":920,"parent_record_offset":888,"ancestor_record_offset":364,"inlinee":6148,"source_line":2504,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":676,"record_offset":1036,"parent_record_offset":828,"ancestor_record_offset":364,"inlinee":6127,"source_line":128,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::clear::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":676,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":364,"inlinee":10624,"source_line":819,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::scopeguard::impl$3::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\scopeguard.rs
//     evidence={"module":676,"record_offset":760,"parent_record_offset":740,"ancestor_record_offset":364,"inlinee":6123,"source_line":69,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":676,"record_offset":664,"parent_record_offset":640,"ancestor_record_offset":364,"inlinee":17338,"source_line":430,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::ResourceInfo::allocate
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3B21
// 首条位置: {"module":217,"inlinee":15137}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":20532,"parent_record_offset":19884,"ancestor_record_offset":19884,"inlinee":5294,"source_line":3002,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":20052,"parent_record_offset":19884,"ancestor_record_offset":19884,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":217,"record_offset":19940,"parent_record_offset":19884,"ancestor_record_offset":19884,"inlinee":15138,"source_line":653,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":217,"record_offset":19968,"parent_record_offset":19884,"ancestor_record_offset":19884,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":217,"record_offset":20348,"parent_record_offset":20324,"ancestor_record_offset":19884,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":217,"record_offset":20176,"parent_record_offset":20148,"ancestor_record_offset":19884,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":217,"record_offset":20272,"parent_record_offset":20244,"ancestor_record_offset":19884,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":217,"record_offset":20244,"parent_record_offset":20208,"ancestor_record_offset":19884,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":217,"record_offset":20396,"parent_record_offset":20372,"ancestor_record_offset":19884,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":217,"record_offset":20372,"parent_record_offset":20348,"ancestor_record_offset":19884,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":217,"record_offset":20208,"parent_record_offset":20176,"ancestor_record_offset":19884,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":20324,"parent_record_offset":20108,"ancestor_record_offset":19884,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":20108,"parent_record_offset":20080,"ancestor_record_offset":19884,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":20592,"parent_record_offset":20532,"ancestor_record_offset":19884,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":20564,"parent_record_offset":20532,"ancestor_record_offset":19884,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":20148,"parent_record_offset":20108,"ancestor_record_offset":19884,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":217,"record_offset":21132,"parent_record_offset":20592,"ancestor_record_offset":19884,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":217,"record_offset":20492,"parent_record_offset":20108,"ancestor_record_offset":19884,"inlinee":6531,"source_line":13,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":217,"record_offset":20080,"parent_record_offset":20052,"ancestor_record_offset":19884,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":217,"record_offset":21036,"parent_record_offset":21004,"ancestor_record_offset":19884,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":217,"record_offset":21004,"parent_record_offset":20972,"ancestor_record_offset":19884,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":217,"record_offset":20972,"parent_record_offset":20592,"ancestor_record_offset":19884,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":217,"record_offset":20436,"parent_record_offset":20108,"ancestor_record_offset":19884,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":217,"record_offset":20460,"parent_record_offset":20436,"ancestor_record_offset":19884,"inlinee":6052,"source_line":796,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":217,"record_offset":20016,"parent_record_offset":19992,"ancestor_record_offset":19884,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":217,"record_offset":21076,"parent_record_offset":20592,"ancestor_record_offset":19884,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":217,"record_offset":19992,"parent_record_offset":19968,"ancestor_record_offset":19884,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::impl$0::allocate::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2E4A
// 首条位置: {"module":104,"inlinee":11850}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":184,"record_offset":22204,"parent_record_offset":22164,"ancestor_record_offset":22164,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::ResourceInfo::extract_creation_flags
// 记录形式: compiled_procedure; instances=1
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":676,"record_offset":1132,"rva":12403888,"va":5381113008}
// 编译实例 RVAs: 0xbd44b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30429140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381113008,"raw_hex":"e8d44fd001","size":5,"soft_fail":false,"target_va":5411542296,"va":5381113151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":676,"record_offset":1392,"parent_record_offset":1132,"ancestor_record_offset":1132,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":676,"record_offset":1288,"parent_record_offset":1132,"ancestor_record_offset":1132,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":676,"record_offset":1356,"parent_record_offset":1316,"ancestor_record_offset":1132,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":676,"record_offset":1420,"parent_record_offset":1392,"ancestor_record_offset":1132,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":676,"record_offset":1448,"parent_record_offset":1420,"ancestor_record_offset":1132,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":676,"record_offset":1316,"parent_record_offset":1288,"ancestor_record_offset":1132,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::ResourceInfo::extend_usage_flags_for_exports
// 记录形式: compiled_procedure; instances=1
// 源码行: 102
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":676,"record_offset":1576,"rva":12404064,"va":5381113184}
// 编译实例 RVAs: 0xbd4560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1418adc70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_info.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13473401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381113184,"raw_hex":"e87996cd00","size":5,"soft_fail":false,"target_va":5394586736,"va":5381113330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30428505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381113184,"raw_hex":"e8594dd001","size":5,"soft_fail":false,"target_va":5411542296,"va":5381113786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30428169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381113184,"raw_hex":"e8094cd001","size":5,"soft_fail":false,"target_va":5411541904,"va":5381113730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30429104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381113184,"raw_hex":"e8b04fd001","size":5,"soft_fail":false,"target_va":5411542880,"va":5381113771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] vk_sync_fork::get_access_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"vk-sync-fork","version":"0.4.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t247797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381113184,"raw_hex":"e8f5c70300","size":5,"soft_fail":false,"target_va":5381361104,"va":5381113302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":676,"record_offset":2212,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":676,"record_offset":1776,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::AccessFlags::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":676,"record_offset":2184,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":38950,"source_line":23,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$286::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":676,"record_offset":2408,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":15804,"source_line":45,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$354::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":676,"record_offset":2000,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":15813,"source_line":45,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":676,"record_offset":2092,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":676,"record_offset":1852,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::graph::ExportableResource::index
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":676,"record_offset":2036,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":11958,"source_line":33,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_info::buffer_access_mask_to_usage_flags
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":676,"record_offset":2308,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":38951,"source_line":320,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_info::image_access_mask_to_usage_flags
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":676,"record_offset":1928,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":38948,"source_line":275,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":676,"record_offset":2128,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":676,"record_offset":1816,"parent_record_offset":1796,"ancestor_record_offset":1576,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":676,"record_offset":1796,"parent_record_offset":1776,"ancestor_record_offset":1576,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::AccessFlags::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":676,"record_offset":2348,"parent_record_offset":2308,"ancestor_record_offset":1576,"inlinee":38952,"source_line":27,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::AccessFlags::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":676,"record_offset":2372,"parent_record_offset":2348,"ancestor_record_offset":1576,"inlinee":38950,"source_line":23,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":676,"record_offset":1968,"parent_record_offset":1928,"ancestor_record_offset":1576,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":676,"record_offset":1892,"parent_record_offset":1852,"ancestor_record_offset":1576,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":676,"record_offset":2240,"parent_record_offset":2212,"ancestor_record_offset":1576,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":676,"record_offset":2268,"parent_record_offset":2240,"ancestor_record_offset":1576,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":676,"record_offset":2152,"parent_record_offset":2128,"ancestor_record_offset":1576,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::graph::ExportableResource::raw
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":676,"record_offset":2060,"parent_record_offset":2036,"ancestor_record_offset":1576,"inlinee":11959,"source_line":25,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::ResourceInfo::extract_pass_resource_access
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 124
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3B23
// 首条位置: {"module":217,"inlinee":15139}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":22064,"parent_record_offset":21808,"ancestor_record_offset":21808,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":23112,"parent_record_offset":21808,"ancestor_record_offset":21808,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":22408,"parent_record_offset":21808,"ancestor_record_offset":21808,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":22248,"parent_record_offset":21808,"ancestor_record_offset":21808,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":217,"record_offset":22960,"parent_record_offset":21808,"ancestor_record_offset":21808,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":217,"record_offset":22624,"parent_record_offset":21808,"ancestor_record_offset":21808,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":217,"record_offset":21956,"parent_record_offset":21808,"ancestor_record_offset":21808,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":217,"record_offset":22872,"parent_record_offset":21808,"ancestor_record_offset":21808,"inlinee":15140,"source_line":1642,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":217,"record_offset":22332,"parent_record_offset":21808,"ancestor_record_offset":21808,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_info::ResourceAccessIndex::valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":217,"record_offset":22988,"parent_record_offset":21808,"ancestor_record_offset":21808,"inlinee":11912,"source_line":228,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":217,"record_offset":22128,"parent_record_offset":22104,"ancestor_record_offset":21808,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":217,"record_offset":22176,"parent_record_offset":22152,"ancestor_record_offset":21808,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":217,"record_offset":22152,"parent_record_offset":22128,"ancestor_record_offset":21808,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":22104,"parent_record_offset":22064,"ancestor_record_offset":21808,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":22296,"parent_record_offset":22272,"ancestor_record_offset":21808,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":217,"record_offset":22272,"parent_record_offset":22248,"ancestor_record_offset":21808,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":217,"record_offset":23016,"parent_record_offset":22988,"ancestor_record_offset":21808,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":217,"record_offset":22896,"parent_record_offset":22872,"ancestor_record_offset":21808,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":217,"record_offset":22024,"parent_record_offset":21984,"ancestor_record_offset":21808,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":217,"record_offset":22216,"parent_record_offset":22064,"ancestor_record_offset":21808,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":217,"record_offset":23140,"parent_record_offset":23112,"ancestor_record_offset":21808,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":217,"record_offset":22444,"parent_record_offset":22408,"ancestor_record_offset":21808,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":217,"record_offset":23168,"parent_record_offset":23140,"ancestor_record_offset":21808,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":217,"record_offset":22480,"parent_record_offset":22444,"ancestor_record_offset":21808,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":217,"record_offset":21984,"parent_record_offset":21956,"ancestor_record_offset":21808,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_info::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":217,"record_offset":23044,"parent_record_offset":23016,"ancestor_record_offset":21808,"inlinee":11914,"source_line":252,"archive":"_pdb/B/modules/00217.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::ResourceInfo::link_up_next_access_chains
// 记录形式: compiled_procedure; instances=1
// 源码行: 179
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":676,"record_offset":2536,"rva":12404688,"va":5381113808}
// 编译实例 RVAs: 0xbd47d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30428394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381113808,"raw_hex":"e8ea4cd001","size":5,"soft_fail":false,"target_va":5411542296,"va":5381113897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30428937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381113808,"raw_hex":"e8094fd001","size":5,"soft_fail":false,"target_va":5411542880,"va":5381113938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":676,"record_offset":2928,"parent_record_offset":2536,"ancestor_record_offset":2536,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":676,"record_offset":2776,"parent_record_offset":2536,"ancestor_record_offset":2536,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":676,"record_offset":2700,"parent_record_offset":2536,"ancestor_record_offset":2536,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":676,"record_offset":2740,"parent_record_offset":2720,"ancestor_record_offset":2536,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":676,"record_offset":2832,"parent_record_offset":2804,"ancestor_record_offset":2536,"inlinee":9618,"source_line":799,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::ptr_try_from_impls::impl$2::try_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":676,"record_offset":2856,"parent_record_offset":2832,"ancestor_record_offset":2536,"inlinee":14742,"source_line":255,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":676,"record_offset":2720,"parent_record_offset":2700,"ancestor_record_offset":2536,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":676,"record_offset":2888,"parent_record_offset":2804,"ancestor_record_offset":2536,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":676,"record_offset":2956,"parent_record_offset":2928,"ancestor_record_offset":2536,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":676,"record_offset":2984,"parent_record_offset":2956,"ancestor_record_offset":2536,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_info::impl$23::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":676,"record_offset":2804,"parent_record_offset":2776,"ancestor_record_offset":2536,"inlinee":38954,"source_line":241,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::ResourceInfo::calculate_lifetime_events
// 记录形式: compiled_procedure; instances=1
// 源码行: 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":676,"record_offset":3028,"rva":12404832,"va":5381113952}
// 编译实例 RVAs: 0xbd4860
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140958a90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: resource_info.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2604853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381113952,"raw_hex":"e8cb40d8ff","size":5,"soft_fail":false,"target_va":5378509456,"va":5381114304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-479383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381113952,"raw_hex":"e869aff8ff","size":5,"soft_fail":false,"target_va":5380634736,"va":5381114114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":676,"record_offset":3308,"parent_record_offset":3028,"ancestor_record_offset":3028,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":676,"record_offset":3188,"parent_record_offset":3028,"ancestor_record_offset":3028,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::sort_unstable
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":676,"record_offset":3668,"parent_record_offset":3028,"ancestor_record_offset":3028,"inlinee":24453,"source_line":2972,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":676,"record_offset":3368,"parent_record_offset":3344,"ancestor_record_offset":3028,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":676,"record_offset":3416,"parent_record_offset":3392,"ancestor_record_offset":3028,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":676,"record_offset":3392,"parent_record_offset":3368,"ancestor_record_offset":3028,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":676,"record_offset":3344,"parent_record_offset":3308,"ancestor_record_offset":3028,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":676,"record_offset":4032,"parent_record_offset":4008,"ancestor_record_offset":3028,"inlinee":5087,"source_line":1371,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":676,"record_offset":4080,"parent_record_offset":4056,"ancestor_record_offset":3028,"inlinee":8353,"source_line":1820,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":676,"record_offset":3972,"parent_record_offset":3948,"ancestor_record_offset":3028,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":676,"record_offset":3732,"parent_record_offset":3696,"ancestor_record_offset":3028,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":676,"record_offset":4008,"parent_record_offset":3884,"ancestor_record_offset":3028,"inlinee":4196,"source_line":166,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":676,"record_offset":3924,"parent_record_offset":3884,"ancestor_record_offset":3028,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":676,"record_offset":3828,"parent_record_offset":3788,"ancestor_record_offset":3028,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":676,"record_offset":3264,"parent_record_offset":3220,"ancestor_record_offset":3028,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":676,"record_offset":3456,"parent_record_offset":3308,"ancestor_record_offset":3028,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":676,"record_offset":3220,"parent_record_offset":3188,"ancestor_record_offset":3028,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::impl$7::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":676,"record_offset":3948,"parent_record_offset":3924,"ancestor_record_offset":3028,"inlinee":10771,"source_line":302,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insert_tail
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":676,"record_offset":3884,"parent_record_offset":3788,"ancestor_record_offset":3028,"inlinee":10769,"source_line":542,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insertion_sort_shift_left
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":676,"record_offset":3788,"parent_record_offset":3696,"ancestor_record_offset":3028,"inlinee":7664,"source_line":580,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::unstable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\unstable\mod.rs
//     evidence={"module":676,"record_offset":3696,"parent_record_offset":3668,"ancestor_record_offset":3028,"inlinee":7632,"source_line":20,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_info::impl$31::partial_cmp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":676,"record_offset":4056,"parent_record_offset":4032,"ancestor_record_offset":3028,"inlinee":38956,"source_line":353,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::ResourceAccessIndex::index
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 214
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9A36
// 首条位置: {"module":698,"inlinee":39478}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::ResourceAccessIndex::try_index
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 219
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2E87
// 首条位置: {"module":104,"inlinee":11911}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::resource_info::ResourceAccessIndex::valid
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":104,"record_offset":161216,"parent_record_offset":161192,"ancestor_record_offset":161192,"inlinee":11912,"source_line":228,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":104,"record_offset":161240,"parent_record_offset":161216,"ancestor_record_offset":161192,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_info::impl$17::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":104,"record_offset":161264,"parent_record_offset":161240,"ancestor_record_offset":161192,"inlinee":11914,"source_line":252,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::ResourceAccessIndex::valid
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 228
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2E88
// 首条位置: {"module":104,"inlinee":11912}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":104,"record_offset":161240,"parent_record_offset":161216,"ancestor_record_offset":161216,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_info::impl$17::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":104,"record_offset":161264,"parent_record_offset":161240,"ancestor_record_offset":161216,"inlinee":11914,"source_line":252,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::impl$13::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 241
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":676,"record_offset":4696,"rva":12405568,"va":5381114688}
// 编译实例 RVAs: 0xbd4b40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30428131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381114688,"raw_hex":"e8e34bd001","size":5,"soft_fail":false,"target_va":5411542880,"va":5381114744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":676,"record_offset":4828,"parent_record_offset":4696,"ancestor_record_offset":4696,"inlinee":9618,"source_line":799,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":676,"record_offset":4876,"parent_record_offset":4696,"ancestor_record_offset":4696,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::ptr_try_from_impls::impl$2::try_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":676,"record_offset":4848,"parent_record_offset":4828,"ancestor_record_offset":4696,"inlinee":14742,"source_line":255,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::impl$23::from
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 241
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x982A
// 首条位置: {"module":676,"inlinee":38954}
// 编译实例 RVAs: 0xbd4b80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30428067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381114752,"raw_hex":"e8a34bd001","size":5,"soft_fail":false,"target_va":5411542880,"va":5381114808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":676,"record_offset":2832,"parent_record_offset":2804,"ancestor_record_offset":2804,"inlinee":9618,"source_line":799,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":676,"record_offset":2888,"parent_record_offset":2804,"ancestor_record_offset":2804,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::ptr_try_from_impls::impl$2::try_from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":676,"record_offset":2856,"parent_record_offset":2832,"ancestor_record_offset":2804,"inlinee":14742,"source_line":255,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::impl$7::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 248
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3B37
// 首条位置: {"module":219,"inlinee":15159}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::impl$17::eq
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 252
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2E8A
// 首条位置: {"module":104,"inlinee":11914}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::image_access_mask_to_usage_flags
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9824
// 首条位置: {"module":676,"inlinee":38948}
// 编译实例 RVAs: 0xbd49d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30427293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381114320,"raw_hex":"e89d48d001","size":5,"soft_fail":false,"target_va":5411541904,"va":5381114606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":676,"record_offset":1968,"parent_record_offset":1928,"ancestor_record_offset":1928,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::buffer_access_mask_to_usage_flags
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 320, 323
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9827
// 首条位置: {"module":676,"inlinee":38951}
// 编译实例 RVAs: 0xbd4b00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::bitflags::AccessFlags::intersects
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":676,"record_offset":2348,"parent_record_offset":2308,"ancestor_record_offset":2308,"inlinee":38952,"source_line":27,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::AccessFlags::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":676,"record_offset":2372,"parent_record_offset":2348,"ancestor_record_offset":2308,"inlinee":38950,"source_line":23,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::impl$31::partial_cmp
// 记录形式: inline_instance, inlinee_declaration; instances=31
// 源码行: 353
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x982C
// 首条位置: {"module":676,"inlinee":38956}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$64::partial_cmp
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":676,"record_offset":4080,"parent_record_offset":4056,"ancestor_record_offset":4056,"inlinee":8353,"source_line":1820,"archive":"_pdb/B/modules/00676.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::ResourceLifetimeEvent::is_acquire
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 383
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x982F
// 首条位置: {"module":676,"inlinee":38959}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::ResourceLifetimeEvent::resource_idx
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 387
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x982E
// 首条位置: {"module":676,"inlinee":38958}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: resource_info.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::resource_info::ResourceAccessIndex
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ops::range::Range<rhapsody::rg::resource_info::ResourceAccessIndex>", "constructor": "core::ops::range::Range", "arguments": ["rhapsody::rg::resource_info::ResourceAccessIndex"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<core::ops::range::Range<rhapsody::rg::resource_info::ResourceAccessIndex>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["core::ops::range::Range<rhapsody::rg::resource_info::ResourceAccessIndex>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::resource_info::ResourceAccessInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<rhapsody::rg::resource_info::ResourceAccessInfo,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["rhapsody::rg::resource_info::ResourceAccessInfo", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::resource_info::ResourceInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 9; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::resource_info::ResourceInfo::calculate", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::resource_info::ResourceInfo>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::resource_info::ResourceInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::resource_info::ResourceLifetime
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime>", "constructor": "core::slice::iter::Iter", "arguments": ["rhapsody::rg::resource_info::ResourceLifetime"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >", "constructor": "core::iter::adapters::enumerate::Enumerate", "arguments": ["core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> >", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >", "rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::rg::resource::ResourceHandle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["rhapsody::rg::resource::ResourceHandle", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::rg::resource_info::ResourceLifetime,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::rg::resource::ResourceOrigin> >,rhapsody::rg::resource_info::impl$0::allocate::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["rhapsody::rg::resource_info::ResourceLifetime", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::rg::resource::ResourceOrigin> >,rhapsody::rg::resource_info::impl$0::allocate::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::rg::resource_info::ResourceLifetime,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::rg::resource_info::ResourceLifetime,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::rg::resource::ResourceOrigin> >,rhapsody::rg::resource_info::impl$0::allocate::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["rhapsody::rg::resource_info::ResourceLifetime", "alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::rg::resource_info::ResourceLifetime,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::rg::resource::ResourceOrigin> >,rhapsody::rg::resource_info::impl$0::allocate::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<rhapsody::rg::resource_info::ResourceLifetime,core::slice::iter::Iter<enum2$<rhapsody::rg::resource::ResourceOrigin> >,rhapsody::rg::resource_info::impl$0::allocate::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::rg::resource_info::ResourceLifetime,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::rg::resource_info::ResourceLifetime,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::rg::resource::ResourceOrigin> >,rhapsody::rg::resource_info::impl$0::allocate::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["rhapsody::rg::resource_info::ResourceLifetime", "core::slice::iter::Iter<enum2$<rhapsody::rg::resource::ResourceOrigin> >", "rhapsody::rg::resource_info::impl$0::allocate::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources>", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::rg::resource_info::ResourceLifetime,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::rg::resource_info::ResourceLifetime,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::rg::resource::ResourceOrigin> >,rhapsody::rg::resource_info::impl$0::allocate::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::resource_info::ResourceLifetimeEvent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>", "constructor": "ref$", "arguments": ["rhapsody::rg::resource_info::ResourceLifetimeEvent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::ipnsort<rhapsody::rg::resource_info::ResourceLifetimeEvent,bool (*)(ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>,ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>)>", "constructor": "core::slice::sort::unstable::ipnsort", "arguments": ["rhapsody::rg::resource_info::ResourceLifetimeEvent", "bool (*)(ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>,ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_network<rhapsody::rg::resource_info::ResourceLifetimeEvent,bool (*)(ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>,ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>)>", "constructor": "core::slice::sort::shared::smallsort::small_sort_network", "arguments": ["rhapsody::rg::resource_info::ResourceLifetimeEvent", "bool (*)(ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>,ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::quicksort::quicksort<rhapsody::rg::resource_info::ResourceLifetimeEvent,bool (*)(ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>,ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>)>", "constructor": "core::slice::sort::unstable::quicksort::quicksort", "arguments": ["rhapsody::rg::resource_info::ResourceLifetimeEvent", "bool (*)(ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>,ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<rhapsody::rg::resource_info::ResourceLifetimeEvent,bool (*)(ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>,ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>)>", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["rhapsody::rg::resource_info::ResourceLifetimeEvent", "bool (*)(ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>,ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::heapsort::heapsort<rhapsody::rg::resource_info::ResourceLifetimeEvent,bool (*)(ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>,ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>)>", "constructor": "core::slice::sort::unstable::heapsort::heapsort", "arguments": ["rhapsody::rg::resource_info::ResourceLifetimeEvent", "bool (*)(ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>,ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: resource_info.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: resource_info.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 11; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>,ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>)
// 候选参数类型: ["ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>", "ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] std::ffi::os_str::OsString (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] serde::__private::de::content::ContentDeserializer<serde_json::error::Error> (*)(enum2$<serde::__private::de::content::Content>)
// 候选参数类型: ["enum2$<serde::__private::de::content::Content>"]
// [文件级接口候选，具体函数归属 unknown] sentry_types::protocol::v7::Breadcrumb (*)(ref$<sentry_types::protocol::v7::Breadcrumb>)
// 候选参数类型: ["ref$<sentry_types::protocol::v7::Breadcrumb>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::ResourceAccessIndex::index
// 信息 ID: f13f5e2989e32ae2a97306d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::compiled::pass_access_range_indices; module=698, inline_record=960; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::compiled::pass_access_range_indices; module=698, inline_record=984; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::ResourceAccessIndex::try_index
// 信息 ID: b697d28a3ef756f896ccf527
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::rg::resource::ResourceHandle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > >; module=104, inline_record=161192; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::rg::resource::ResourceHandle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > >; module=104, inline_record=162028; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::ResourceAccessIndex::valid
// 信息 ID: c29ca13508dc56ed36ba3f1a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::rg::resource::ResourceHandle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > >; module=104, inline_record=161216; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::rg::resource::ResourceHandle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > >; module=104, inline_record=162052; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources>; module=217, inline_record=22988; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::ResourceInfo::allocate
// 信息 ID: 1cdf18081b4cb268e2130649
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources>; module=217, inline_record=19884; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources>
// 信息 ID: 1c8a9ff0e2d81098b22b2b76
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::resource_info::ResourceInfo::calculate -> ["country_core::render::pass_builder::pass_resources::PassResources"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x163ac9 -> 0x530430 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0> @ 0x1657f8 -> 0x530430 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0> @ 0x166f9f -> 0x530430 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::ResourceInfo::calculate_lifetime_events
// 信息 ID: dfffe29c269633c3e553b6c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x5308b6 -> 0xbd4860 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::ResourceInfo::clear
// 信息 ID: bee08590540055d00a64ee45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x530451 -> 0xbd4410 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::ResourceInfo::extend_usage_flags_for_exports
// 信息 ID: 1128482b42581bf0252ecd5e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x530603 -> 0xbd4560 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::ResourceInfo::extract_creation_flags
// 信息 ID: 386a5cf831d1f8fede12d158
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x5305f8 -> 0xbd44b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::ResourceInfo::extract_pass_resource_access
// 信息 ID: d4eff9ca3a9c8b9b2526ab99
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources>; module=217, inline_record=21808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::ResourceInfo::link_up_next_access_chains
// 信息 ID: 29476afc08feb9ab25b8c8c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x5308ae -> 0xbd47d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::ResourceLifetimeEvent::is_acquire
// 信息 ID: 35efd640a15ca4b90fdcedd5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::resource_registry::ResourceRegistry::populate_resources; module=696, inline_record=8416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::ResourceLifetimeEvent::resource_idx
// 信息 ID: 0a4f3d9803dd03c3243fb6f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::resource_registry::ResourceRegistry::populate_resources; module=696, inline_record=8388; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::buffer_access_mask_to_usage_flags
// 信息 ID: 41a4473b0ba83e3ceb8c45eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x53085c -> 0xbd4b00 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::rg::resource_info::ResourceInfo::extend_usage_flags_for_exports; module=676, inline_record=2308; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::image_access_mask_to_usage_flags
// 信息 ID: c27b6df4a1e036738216b487
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x5306c0 -> 0xbd49d0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::rg::resource_info::ResourceInfo::extend_usage_flags_for_exports; module=676, inline_record=1928; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::impl$0::allocate::closure$0
// 信息 ID: 9c6466c08df2fc7703975fa5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::rg::resource_info::impl$0::allocate", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::rg::resource_info::impl$0::allocate::closure_env$0 <- core::iter::adapters::map::impl$2::fold<rhapsody::rg::resource_info::ResourceLifetime,core::slice::iter::Iter<enum2$<rhapsody::rg::resource::ResourceOrigin> >,rhapsody::rg::resource_info::impl$0::allocate::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::rg::resource_info::ResourceLifetime,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::rg::resource_info::ResourceLifetime,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::rg::resource::ResourceOrigin> >,rhapsody::rg::resource_info::impl$0::allocate::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::rg::resource_info::impl$0::allocate::closure_env$0 <- core::iter::adapters::map::impl$2::fold<rhapsody::rg::resource_info::ResourceLifetime,core::slice::iter::Iter<enum2$<rhapsody::rg::resource::ResourceOrigin> >,rhapsody::rg::resource_info::impl$0::allocate::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::rg::resource_info::ResourceLifetime,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::rg::resource_info::ResourceLifetime,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::rg::resource::ResourceOrigin> >,rhapsody::rg::resource_info::impl$0::allocate::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::rg::resource_info::ResourceLifetime,core::slice::iter::Iter<enum2$<rhapsody::rg::resource::ResourceOrigin> >,rhapsody::rg::resource_info::impl$0::allocate::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::rg::resource_info::ResourceLifetime,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::rg::resource_info::ResourceLifetime,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::rg::resource::ResourceOrigin> >,rhapsody::rg::resource_info::impl$0::allocate::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > > > >; module=184, inline_record=22164; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::impl$13::from
// 信息 ID: f31e4a205283b623bdfa7fff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 27 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,alloc::alloc::Global>,ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,rhapsody::rg::compiled::impl$1::record_batch::closure_env$2<country_core::render::pass_builder::pass_resources::PassResources>,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > > > > @ 0xf8a06 -> 0xbd4b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$1::record_batch::closure$1<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2c374c -> 0xbd4b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x31dc85 -> 0xbd4b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<rhapsody::rg::resource_info::ResourceLifetime,core::slice::iter::Iter<enum2$<rhapsody::rg::resource::ResourceOrigin> >,rhapsody::rg::resource_info::impl$0::allocate::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::rg::resource_info::ResourceLifetime,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::rg::resource_info::ResourceLifetime,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::rg::resource::ResourceOrigin> >,rhapsody::rg::resource_info::impl$0::allocate::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > > > > @ 0x48dad7 -> 0xbd4b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x53077a -> 0xbd4b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x530799 -> 0xbd4b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x5307c3 -> 0xbd4b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<alloc::string::String> @ 0x587843 -> 0xbd4b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> > @ 0x588052 -> 0xbd4b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_stream::RasterPass::new_impl @ 0x8679d5 -> 0xbd4b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args @ 0x90a9b9 -> 0xbd4b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::rg_buffer_copy_from_slice<u32> @ 0x90da55 -> 0xbd4b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::clear_rg_buffer @ 0x9ddbda -> 0xbd4b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::clear_rg_color_image @ 0x9ddeba -> 0xbd4b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketFlowers> @ 0x15fd10f -> 0xbd4b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResAllDof> @ 0x15fda1f -> 0xbd4b40 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::impl$17::eq
// 信息 ID: 59814d86307264b0d2ad350e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::rg::resource::ResourceHandle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > >; module=104, inline_record=161264; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::rg::resource::ResourceHandle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > >; module=104, inline_record=162100; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources>; module=217, inline_record=23044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::impl$23::from
// 信息 ID: d9de11e9b5d32d1202aeb27b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x530704 -> 0xbd4b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x530875 -> 0xbd4b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x530880 -> 0xbd4b80 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::rg::resource_info::ResourceInfo::link_up_next_access_chains; module=676, inline_record=2804; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::impl$31::partial_cmp
// 信息 ID: f77145ee6912880520dfc918
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 62dd3fceb4bb7390701a47c2: bool (*)(ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>,ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>)
// 引用方: 原程序直接转移 0 条；内联宿主 24 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::resource_info::ResourceInfo::calculate_lifetime_events; module=676, inline_record=4056; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::resource_info::ResourceInfo::calculate_lifetime_events; module=676, inline_record=4196; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::unstable::ipnsort<rhapsody::rg::resource_info::ResourceLifetimeEvent,bool (*)(ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>,ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>)>; module=1918, inline_record=664; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::unstable::ipnsort<rhapsody::rg::resource_info::ResourceLifetimeEvent,bool (*)(ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>,ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>)>; module=1918, inline_record=780; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::unstable::ipnsort<rhapsody::rg::resource_info::ResourceLifetimeEvent,bool (*)(ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>,ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>)>; module=1918, inline_record=892; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_network<rhapsody::rg::resource_info::ResourceLifetimeEvent,bool (*)(ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>,ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>)>; module=1937, inline_record=12540; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_network<rhapsody::rg::resource_info::ResourceLifetimeEvent,bool (*)(ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>,ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>)>; module=1937, inline_record=12680; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_network<rhapsody::rg::resource_info::ResourceLifetimeEvent,bool (*)(ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>,ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>)>; module=1937, inline_record=13000; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::impl$3::default
// 信息 ID: f7b933a6225fde27b5412de4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>; module=66, inline_record=26748; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0>; module=66, inline_record=40996; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0>; module=66, inline_record=48996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::impl$7::eq
// 信息 ID: 598eabbe87f3d8678f73b636
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::pass_builder::PassBuilder::add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>; module=219, inline_record=60988; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::pass_builder::PassBuilder::add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>; module=533, inline_record=17076; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

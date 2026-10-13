// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/rg/binding.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 773a596f0e6045a0c2a61c6c26ad172ce709642ab9cf493b99805703b57454e9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: binding.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 61, 102, 145, 149, 500, 502, 503, 518, 1680, 1935, 3095
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use vk_sync_fork; // namespace candidate
//   package identities: [{"name":"vk-sync-fork","source":"local_or_path","version":"0.4.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::binding::rg_binding_to_descriptor_binding
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3095,"record_offset":2300,"rva":33943536,"va":5402652656}
// 编译实例 RVAs: 0x205eff0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1407cea70
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25758661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402652656,"raw_hex":"e83bf476fe","size":5,"soft_fail":false,"target_va":5376895600,"va":5402654256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f6d90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9864552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402652656,"raw_hex":"e8987a69ff","size":5,"soft_fail":false,"target_va":5392788880,"va":5402653427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b6ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8030240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402652656,"raw_hex":"e8e07785ff","size":5,"soft_fail":false,"target_va":5394623152,"va":5402653387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428caa40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8828115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402652656,"raw_hex":"e8d3b48600","size":5,"soft_fail":false,"target_va":5411482176,"va":5402654056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428caa60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8828356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402652656,"raw_hex":"e8c4b58600","size":5,"soft_fail":false,"target_va":5411482208,"va":5402653847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t8887828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402652656,"raw_hex":"e8149e8700","size":5,"soft_fail":false,"target_va":5411541776,"va":5402653943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t8828029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402652656,"raw_hex":"e87db48600","size":5,"soft_fail":false,"target_va":5411481622,"va":5402653588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t8888152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402652656,"raw_hex":"e8589f8700","size":5,"soft_fail":false,"target_va":5411542064,"va":5402653907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t8888426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402652656,"raw_hex":"e86aa08700","size":5,"soft_fail":false,"target_va":5411542296,"va":5402653865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8888069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402652656,"raw_hex":"e8059f8700","size":5,"soft_fail":false,"target_va":5411541904,"va":5402653830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8888840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402652656,"raw_hex":"e808a28700","size":5,"soft_fail":false,"target_va":5411542880,"va":5402654035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_registry::ResourceRegistry::image_view
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21461183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402652656,"raw_hex":"e84187b8fe","size":5,"soft_fail":false,"target_va":5381192288,"va":5402653466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] vk_sync_fork::get_access_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"vk-sync-fork","version":"0.4.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21292339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402652656,"raw_hex":"e8cd1abbfe","size":5,"soft_fail":false,"target_va":5381361104,"va":5402653438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3095,"record_offset":4284,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03095.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3095,"record_offset":3904,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/03095.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$6::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3095,"record_offset":3784,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":9142,"source_line":653,"archive":"_pdb/B/modules/03095.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3095,"record_offset":2476,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":8083,"source_line":1483,"archive":"_pdb/B/modules/03095.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3095,"record_offset":2880,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03095.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3095,"record_offset":3812,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/03095.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3095,"record_offset":3988,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03095.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":3095,"record_offset":2760,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/03095.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_registry::ResourceRegistry::buffer
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_registry.rs
//     evidence={"module":3095,"record_offset":2516,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":39457,"source_line":97,"archive":"_pdb/B/modules/03095.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_registry::ResourceRegistry::image_from_raw_handle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_registry.rs
//     evidence={"module":3095,"record_offset":3316,"parent_record_offset":2300,"ancestor_record_offset":2300,"inlinee":39456,"source_line":86,"archive":"_pdb/B/modules/03095.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3095,"record_offset":2548,"parent_record_offset":2516,"ancestor_record_offset":2300,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/03095.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3095,"record_offset":3928,"parent_record_offset":3904,"ancestor_record_offset":2300,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/03095.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3095,"record_offset":3868,"parent_record_offset":3844,"ancestor_record_offset":2300,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03095.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":3095,"record_offset":3952,"parent_record_offset":3928,"ancestor_record_offset":2300,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/03095.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3095,"record_offset":2572,"parent_record_offset":2548,"ancestor_record_offset":2300,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/03095.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3095,"record_offset":2596,"parent_record_offset":2572,"ancestor_record_offset":2300,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/03095.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":3095,"record_offset":2792,"parent_record_offset":2760,"ancestor_record_offset":2300,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/03095.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":3095,"record_offset":2848,"parent_record_offset":2760,"ancestor_record_offset":2300,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/03095.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":3095,"record_offset":2820,"parent_record_offset":2760,"ancestor_record_offset":2300,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/03095.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/binding.rs
//     evidence={"module":3095,"record_offset":3844,"parent_record_offset":3812,"ancestor_record_offset":2300,"inlinee":106399,"source_line":44,"archive":"_pdb/B/modules/03095.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::RuntimeResource::borrow
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":3095,"record_offset":2728,"parent_record_offset":2516,"ancestor_record_offset":2300,"inlinee":39156,"source_line":207,"archive":"_pdb/B/modules/03095.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::pass_api::RenderPassApi::find_resource_access_type
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1680,"record_offset":256,"rva":24079760,"va":5392788880}
// 编译实例 RVAs: 0x16f6d90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18753168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392788880,"raw_hex":"e890261e01","size":5,"soft_fail":false,"target_va":5411542296,"va":5392789123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18752794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392788880,"raw_hex":"e81a251e01","size":5,"soft_fail":false,"target_va":5411541904,"va":5392789105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18754082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392788880,"raw_hex":"e8222a1e01","size":5,"soft_fail":false,"target_va":5411543104,"va":5392789017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18754116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392788880,"raw_hex":"e8442a1e01","size":5,"soft_fail":false,"target_va":5411543120,"va":5392788999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1680,"record_offset":412,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1680,"record_offset":792,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1680,"record_offset":648,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4466,"source_line":296,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1680,"record_offset":840,"parent_record_offset":816,"ancestor_record_offset":256,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1680,"record_offset":608,"parent_record_offset":572,"ancestor_record_offset":256,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1680,"record_offset":704,"parent_record_offset":676,"ancestor_record_offset":256,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1680,"record_offset":440,"parent_record_offset":412,"ancestor_record_offset":256,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1680,"record_offset":468,"parent_record_offset":440,"ancestor_record_offset":256,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1680,"record_offset":572,"parent_record_offset":540,"ancestor_record_offset":256,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1680,"record_offset":676,"parent_record_offset":648,"ancestor_record_offset":256,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::binding::impl$0::find_resource_access_type::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/binding.rs
//     evidence={"module":1680,"record_offset":736,"parent_record_offset":648,"ancestor_record_offset":256,"inlinee":76721,"source_line":24,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::binding::impl$0::find_resource_access_type::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/binding.rs
//     evidence={"module":1680,"record_offset":816,"parent_record_offset":792,"ancestor_record_offset":256,"inlinee":76724,"source_line":25,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::impl$37::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":1680,"record_offset":756,"parent_record_offset":736,"ancestor_record_offset":256,"inlinee":76723,"source_line":271,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::binding::impl$0::find_resource_access_type::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12BB1
// 首条位置: {"module":1680,"inlinee":76721}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::resource::impl$37::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":1680,"record_offset":756,"parent_record_offset":736,"ancestor_record_offset":736,"inlinee":76723,"source_line":271,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::binding::impl$0::find_resource_access_type::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12BB4
// 首条位置: {"module":1680,"inlinee":76724}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1680,"record_offset":840,"parent_record_offset":816,"ancestor_record_offset":816,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01680.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19F9F
// 首条位置: {"module":3095,"inlinee":106399}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3095,"record_offset":3868,"parent_record_offset":3844,"ancestor_record_offset":3844,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03095.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x152EA
// 首条位置: {"module":1935,"inlinee":86762}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1935,"record_offset":19764,"parent_record_offset":19712,"ancestor_record_offset":19712,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01935.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::binding::PassResourceSlot::new
// 记录形式: inline_instance, inlinee_declaration; instances=136
// 源码行: 176
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2590
// 首条位置: {"module":61,"inlinee":9616}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=128
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":61,"record_offset":3484,"parent_record_offset":3456,"ancestor_record_offset":3456,"inlinee":9618,"source_line":799,"archive":"_pdb/B/modules/00061.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=128
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":61,"record_offset":3540,"parent_record_offset":3456,"ancestor_record_offset":3456,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00061.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::ptr_try_from_impls::impl$0::try_from
//     owner=rust_standard_library; count=128
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":61,"record_offset":3508,"parent_record_offset":3484,"ancestor_record_offset":3456,"inlinee":9620,"source_line":255,"archive":"_pdb/B/modules/00061.jsonl.gz"}
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
// 完整结构及原始类型记录: binding.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::binding::PassResourceSlot
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 14
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<ref$<str$>,rhapsody::rg::binding::PassResourceSlot,std::hash::random::RandomState>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["ref$<str$>", "rhapsody::rg::binding::PassResourceSlot", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::HashMap<ref$<str$>,rhapsody::rg::binding::PassResourceSlot,std::hash::random::RandomState> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::HashMap<ref$<str$>,rhapsody::rg::binding::PassResourceSlot,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>", "constructor": "tuple$", "arguments": ["shader_pipeline::reflection::BindingIndex", "rhapsody::rg::binding::PassResourceSlot"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>", "country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>", "core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>", "country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>", "core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > >", "constructor": "rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > >", "constructor": "ref_mut$", "arguments": ["rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$3::call_mut<tuple$<ref$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > > >", "constructor": "core::ops::function::impls::impl$3::call_mut", "arguments": ["tuple$<ref$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > > >", "ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >", "constructor": "core::slice::iter::Iter", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >", "country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>", "core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > >", "constructor": "hashbrown::map::impl$82::extend::closure_env$0", "arguments": ["shader_pipeline::reflection::BindingIndex", "alloc::string::String", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>", "hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,tuple$<>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >", "tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>", "tuple$<>", "country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$44::fold::closure_env$0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,tuple$<>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >", "constructor": "hashbrown::map::impl$44::fold::closure_env$0", "arguments": ["shader_pipeline::reflection::BindingIndex", "rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>", "tuple$<>", "core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,tuple$<>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawIterRange::fold_impl<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,hashbrown::map::impl$44::fold::closure_env$0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,tuple$<>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >,tuple$<> >", "constructor": "hashbrown::raw::RawIterRange::fold_impl", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "hashbrown::map::impl$44::fold::closure_env$0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,tuple$<>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::render::pass_builder::draw_stream::TextureBindingKey,rhapsody::rg::binding::PassResourceSlot>", "constructor": "tuple$", "arguments": ["country_core::render::pass_builder::draw_stream::TextureBindingKey", "rhapsody::rg::binding::PassResourceSlot"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::render::pass_builder::draw_stream::TextureBindingKey,rhapsody::rg::binding::PassResourceSlot,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::render::pass_builder::draw_stream::TextureBindingKey", "rhapsody::rg::binding::PassResourceSlot", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::render::pass_builder::draw_stream::TextureBindingKey,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::render::pass_builder::draw_stream::TextureBindingKey,rhapsody::rg::binding::PassResourceSlot,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::render::pass_builder::draw_stream::TextureBindingKey,rhapsody::rg::binding::PassResourceSlot>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::render::pass_builder::draw_stream::TextureBindingKey,rhapsody::rg::binding::PassResourceSlot,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<str$>,rhapsody::rg::binding::PassResourceSlot>", "constructor": "tuple$", "arguments": ["ref$<str$>", "rhapsody::rg::binding::PassResourceSlot"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<ref$<str$>,rhapsody::rg::binding::PassResourceSlot,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["ref$<str$>", "rhapsody::rg::binding::PassResourceSlot", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<ref$<str$>,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<ref$<str$>,rhapsody::rg::binding::PassResourceSlot,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<ref$<str$>,rhapsody::rg::binding::PassResourceSlot>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<ref$<str$>,rhapsody::rg::binding::PassResourceSlot,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > >", "constructor": "ref_mut$", "arguments": ["core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1,tuple$<>,ref_mut$<core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>", "core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >", "country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1", "tuple$<>", "ref_mut$<core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > > >", "constructor": "core::iter::traits::iterator::Iterator::find::check::closure_env$0", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >", "constructor": "core::iter::adapters::map::impl$2::try_fold", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >", "country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2", "tuple$<>", "core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > > >", "enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >", "constructor": "rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > >", "constructor": "ref_mut$", "arguments": ["rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > >", "constructor": "core::iter::traits::iterator::Iterator::find::check::closure_env$0", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >", "constructor": "core::iter::adapters::map::impl$2::try_fold", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4", "tuple$<>", "core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > >", "enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > >", "constructor": "hashbrown::map::impl$82::extend", "arguments": ["shader_pipeline::reflection::BindingIndex", "alloc::string::String", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::rustc_entry<ref$<str$>,rhapsody::rg::binding::PassResourceSlot,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::rustc_entry", "arguments": ["ref$<str$>", "rhapsody::rg::binding::PassResourceSlot", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >", "constructor": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > >", "constructor": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: binding.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: binding.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::rg::binding::PassResourceSlot::new
// 信息 ID: 4a3fa3b6cce6d7b76218a357
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 128 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=61, inline_record=3456; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=61, inline_record=9016; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked>; module=102, inline_record=3584; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo>; module=102, inline_record=6128; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo>; module=102, inline_record=6800; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo>; module=102, inline_record=7472; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo>; module=102, inline_record=8136; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo>; module=102, inline_record=8804; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::binding::impl$0::find_resource_access_type::closure$0
// 信息 ID: 7b1a202ce6513232a5663526
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::rg::binding::impl$0::find_resource_access_type", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::pass_api::RenderPassApi::find_resource_access_type; module=1680, inline_record=736; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::binding::impl$0::find_resource_access_type::closure$1
// 信息 ID: 91ff3be36a139e87f1401cd8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::rg::binding::impl$0::find_resource_access_type", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::pass_api::RenderPassApi::find_resource_access_type; module=1680, inline_record=816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::binding::rg_binding_to_descriptor_binding
// 信息 ID: f69073191c07a4f2ec3bcb3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::rg::pass_api::RenderPassBinding> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$0> > @ 0x12fc451 -> 0x205eff0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure$1
// 信息 ID: c6d9f5e6c923164691cc408b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::rg::binding::rg_binding_to_descriptor_binding", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::binding::rg_binding_to_descriptor_binding; module=3095, inline_record=3844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure$2
// 信息 ID: 00d6ccb390825c0c3d598c36
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "rhapsody::rg::binding::rg_binding_to_descriptor_binding", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure_env$2 <- core::iter::adapters::map::impl$2::fold<ash::vk::definitions::DescriptorImageInfo,core::ops::range::Range<u32>,rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::DescriptorImageInfo,alloc::vec::impl$20::extend_trusted::closure_env$0<ash::vk::definitions::DescriptorImageInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<u32>,rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure_env$2> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure_env$2 <- core::iter::adapters::map::impl$2::fold<ash::vk::definitions::DescriptorImageInfo,core::ops::range::Range<u32>,rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::DescriptorImageInfo,alloc::vec::impl$20::extend_trusted::closure_env$0<ash::vk::definitions::DescriptorImageInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<u32>,rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure_env$2> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::DescriptorImageInfo,core::iter::adapters::map::Map<core::ops::range::Range<u32>,rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<ash::vk::definitions::DescriptorImageInfo,core::ops::range::Range<u32>,rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::DescriptorImageInfo,alloc::vec::impl$20::extend_trusted::closure_env$0<ash::vk::definitions::DescriptorImageInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<u32>,rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure_env$2> > > >; module=1935, inline_record=19712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::pass_api::RenderPassApi::find_resource_access_type
// 信息 ID: 49cf38aae45fd6ccaad799fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<ash::vk::definitions::DescriptorImageInfo,core::ops::range::Range<u32>,rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::DescriptorImageInfo,alloc::vec::impl$20::extend_trusted::closure_env$0<ash::vk::definitions::DescriptorImageInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<u32>,rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure_env$2> > > > @ 0x18b478d -> 0x16f6d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::binding::rg_binding_to_descriptor_binding @ 0x205f2f3 -> 0x16f6d90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

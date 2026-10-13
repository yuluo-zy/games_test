// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/rg/resource_registry.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 9c810c4d2f157ae56ec3ea7e654cff343940748a8404c1e6dd479f52b0008f06
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: resource_registry.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 66, 197, 503, 520, 696, 1655, 1680, 1741, 1936, 3095, 3314
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_registry::impl$2::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":197,"record_offset":4872,"rva":5144560,"va":5373853680}
// 编译实例 RVAs: 0x4e7ff0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t9038731","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5373853680,"raw_hex":"e98beb8900","size":5,"soft_fail":false,"target_va":5382892432,"va":5373853696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_registry::impl$5::default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2724
// 首条位置: {"module":66,"inlinee":10020}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$26::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":26692,"parent_record_offset":26644,"ancestor_record_offset":26644,"inlinee":10022,"source_line":3797,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_info::impl$3::default
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":66,"record_offset":26748,"parent_record_offset":26644,"ancestor_record_offset":26644,"inlinee":10024,"source_line":12,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":26716,"parent_record_offset":26692,"ancestor_record_offset":26644,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":66,"record_offset":27040,"parent_record_offset":27012,"ancestor_record_offset":26644,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":66,"record_offset":27092,"parent_record_offset":27068,"ancestor_record_offset":26644,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":66,"record_offset":27068,"parent_record_offset":27012,"ancestor_record_offset":26644,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":66,"record_offset":27116,"parent_record_offset":27092,"ancestor_record_offset":26644,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":66,"record_offset":26984,"parent_record_offset":26956,"ancestor_record_offset":26644,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":27140,"parent_record_offset":27116,"ancestor_record_offset":26644,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$7::default
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":66,"record_offset":26840,"parent_record_offset":26812,"ancestor_record_offset":26644,"inlinee":4790,"source_line":2070,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$12::default
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":66,"record_offset":26812,"parent_record_offset":26784,"ancestor_record_offset":26644,"inlinee":4788,"source_line":1355,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":66,"record_offset":26784,"parent_record_offset":26748,"ancestor_record_offset":26644,"inlinee":4786,"source_line":1161,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":66,"record_offset":26896,"parent_record_offset":26868,"ancestor_record_offset":26644,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":66,"record_offset":27012,"parent_record_offset":26956,"ancestor_record_offset":26644,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":66,"record_offset":26868,"parent_record_offset":26840,"ancestor_record_offset":26644,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":66,"record_offset":26956,"parent_record_offset":26924,"ancestor_record_offset":26644,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":66,"record_offset":26924,"parent_record_offset":26896,"ancestor_record_offset":26644,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_registry::ResourceRegistry::resource
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9A1F
// 首条位置: {"module":696,"record_offset":3048,"rva":12482736,"va":5381191856}
// 编译实例 RVAs: 0xbe78b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t30350341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381191856,"raw_hex":"e8051ccf01","size":5,"soft_fail":false,"target_va":5411542296,"va":5381191950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":696,"record_offset":3192,"parent_record_offset":3048,"ancestor_record_offset":3048,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":696,"record_offset":3216,"parent_record_offset":3192,"ancestor_record_offset":3048,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":696,"record_offset":3240,"parent_record_offset":3216,"ancestor_record_offset":3048,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_registry::ResourceRegistry::image
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6EDE
// 首条位置: {"module":503,"inlinee":28382}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_registry::ResourceRegistry::image_from_raw_handle
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9A20
// 首条位置: {"module":696,"record_offset":3484,"rva":12482880,"va":5381192000}
// 编译实例 RVAs: 0xbe7940
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428caa40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30290096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192000,"raw_hex":"e8b030ce01","size":5,"soft_fail":false,"target_va":5411482176,"va":5381192075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30349936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192000,"raw_hex":"e8701acf01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381192123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30350204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192000,"raw_hex":"e87c1bcf01","size":5,"soft_fail":false,"target_va":5411542296,"va":5381192087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":696,"record_offset":3644,"parent_record_offset":3484,"ancestor_record_offset":3484,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::RuntimeResource::borrow
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":696,"record_offset":3836,"parent_record_offset":3484,"ancestor_record_offset":3484,"inlinee":39156,"source_line":207,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":696,"record_offset":3668,"parent_record_offset":3644,"ancestor_record_offset":3484,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":696,"record_offset":3692,"parent_record_offset":3668,"ancestor_record_offset":3484,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_registry::ResourceRegistry::buffer
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9A21
// 首条位置: {"module":696,"record_offset":3880,"rva":12483024,"va":5381192144}
// 编译实例 RVAs: 0xbe79d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428caa60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30289980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192144,"raw_hex":"e83c30ce01","size":5,"soft_fail":false,"target_va":5411482208,"va":5381192223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30349788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192144,"raw_hex":"e8dc19cf01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381192271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30350056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192144,"raw_hex":"e8e81acf01","size":5,"soft_fail":false,"target_va":5411542296,"va":5381192235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":696,"record_offset":4028,"parent_record_offset":3880,"ancestor_record_offset":3880,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::RuntimeResource::borrow
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":696,"record_offset":4220,"parent_record_offset":3880,"ancestor_record_offset":3880,"inlinee":39156,"source_line":207,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":696,"record_offset":4052,"parent_record_offset":4028,"ancestor_record_offset":3880,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":696,"record_offset":4076,"parent_record_offset":4052,"ancestor_record_offset":3880,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_registry::ResourceRegistry::image_view
// 记录形式: compiled_procedure; instances=1
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":696,"record_offset":4260,"rva":12483168,"va":5381192288}
// 编译实例 RVAs: 0xbe7a60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428caa80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30289699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192288,"raw_hex":"e8232fce01","size":5,"soft_fail":false,"target_va":5411482240,"va":5381192536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30349193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192288,"raw_hex":"e88917cf01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381192578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30289140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192288,"raw_hex":"e8f42cce01","size":5,"soft_fail":false,"target_va":5411481622,"va":5381192477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30349457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192288,"raw_hex":"e89118cf01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381192602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30349740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192288,"raw_hex":"e8ac19cf01","size":5,"soft_fail":false,"target_va":5411542296,"va":5381192551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::texture::ManagedTexture::view
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-46363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192288,"raw_hex":"e8e54affff","size":5,"soft_fail":false,"target_va":5381146064,"va":5381192422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":696,"record_offset":4420,"parent_record_offset":4260,"ancestor_record_offset":4260,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":696,"record_offset":4788,"parent_record_offset":4260,"ancestor_record_offset":4260,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":696,"record_offset":4664,"parent_record_offset":4260,"ancestor_record_offset":4260,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::RuntimeResource::borrow
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":696,"record_offset":4624,"parent_record_offset":4260,"ancestor_record_offset":4260,"inlinee":39156,"source_line":207,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":696,"record_offset":4448,"parent_record_offset":4420,"ancestor_record_offset":4260,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":696,"record_offset":4476,"parent_record_offset":4448,"ancestor_record_offset":4260,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":696,"record_offset":4704,"parent_record_offset":4664,"ancestor_record_offset":4260,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":696,"record_offset":4756,"parent_record_offset":4664,"ancestor_record_offset":4260,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":696,"record_offset":4732,"parent_record_offset":4664,"ancestor_record_offset":4260,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_registry::ResourceRegistry::populate_resources
// 记录形式: compiled_procedure; instances=1
// 源码行: 153
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":696,"record_offset":4828,"rva":12483488,"va":5381192608}
// 编译实例 RVAs: 0xbe7ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14014a360
//     owner=unresolved_namespace; count=2
//     target_aliases=27 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11134415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e8311a56ff","size":5,"soft_fail":false,"target_va":5370061664,"va":5381196074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6114419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e88db3a2ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381195918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407cea70
//     owner=unresolved_namespace; count=4
//     target_aliases=10 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4300447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e86161beff","size":5,"soft_fail":false,"target_va":5376895600,"va":5381196042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdebf0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-40264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e8b862ffff","size":5,"soft_fail":false,"target_va":5381155824,"va":5381196083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdfc90
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e83d74ffff","size":5,"soft_fail":false,"target_va":5381160080,"va":5381195854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be0b30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e8a77cffff","size":5,"soft_fail":false,"target_va":5381163824,"va":5381197444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e81e97ffff","size":5,"soft_fail":false,"target_va":5381170352,"va":5381197197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140beec30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e8c3660000","size":5,"soft_fail":false,"target_va":5381221424,"va":5381195112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bfe650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t90096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e8f05f0100","size":5,"soft_fail":false,"target_va":5381285456,"va":5381195355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c03460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t110495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e89faf0100","size":5,"soft_fail":false,"target_va":5381305440,"va":5381194940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c07950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t128300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e82cf50100","size":5,"soft_fail":false,"target_va":5381323088,"va":5381194783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417e7c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12581984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e860fcbf00","size":5,"soft_fail":false,"target_va":5393775664,"va":5381193675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417e7f20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12582507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e86bfebf00","size":5,"soft_fail":false,"target_va":5393776416,"va":5381193904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418bcc30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13454229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e8954bcd00","size":5,"soft_fail":false,"target_va":5394648112,"va":5381193878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418bce70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13455034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e8ba4ecd00","size":5,"soft_fail":false,"target_va":5394648688,"va":5381193649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30654202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e8fabed301","size":5,"soft_fail":false,"target_va":5411847200,"va":5381192993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6840401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e8af9f97ff","size":5,"soft_fail":false,"target_va":5374355376,"va":5381195772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30285730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e8a21fce01","size":5,"soft_fail":false,"target_va":5411481920,"va":5381196185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t30345950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e8de0acf01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381196109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t30346000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e8100bcf01","size":5,"soft_fail":false,"target_va":5411542296,"va":5381196291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30345107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e89307cf01","size":5,"soft_fail":false,"target_va":5411541904,"va":5381196792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<rhapsody::rg::resource::ResourceImportInfo> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e8dee8ffff","size":5,"soft_fail":false,"target_va":5381191440,"va":5381197357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30346324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e8540ccf01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381196551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::format::vk_format_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11865355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e80b0db500","size":5,"soft_fail":false,"target_va":5393059936,"va":5381194576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_pool::ResourcePool::take_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e86eedffff","size":5,"soft_fail":false,"target_va":5381190112,"va":5381194861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_pool::ResourcePool::take_texture
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e8d7e7ffff","size":5,"soft_fail":false,"target_va":5381188496,"va":5381194676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1464828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e8fc591600","size":5,"soft_fail":false,"target_va":5382657856,"va":5381193023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1560734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381192608,"raw_hex":"e89ed01700","size":5,"soft_fail":false,"target_va":5382753792,"va":5381193053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":696,"record_offset":11012,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":696,"record_offset":5076,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":696,"record_offset":8024,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":696,"record_offset":5160,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":5294,"source_line":3002,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":696,"record_offset":8628,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":696,"record_offset":8444,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":696,"record_offset":7764,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::BufferUsageFlags::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":696,"record_offset":8724,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":39447,"source_line":23,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::ImageUsageFlags::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":696,"record_offset":9156,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":39448,"source_line":23,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":696,"record_offset":10552,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":696,"record_offset":10728,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":9618,"source_line":799,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":696,"record_offset":11984,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":696,"record_offset":8540,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":696,"record_offset":11080,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":696,"record_offset":10784,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":696,"record_offset":8224,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::format::impl$1::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/format.rs
//     evidence={"module":696,"record_offset":10636,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":39450,"source_line":197,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$24::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":696,"record_offset":10608,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":12549,"source_line":49,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_info::ResourceLifetimeEvent::is_acquire
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":696,"record_offset":8416,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":38959,"source_line":383,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_info::ResourceLifetimeEvent::resource_idx
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":696,"record_offset":8388,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":38958,"source_line":387,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":696,"record_offset":9492,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":12667,"source_line":378,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":696,"record_offset":8752,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":8292,"source_line":1244,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$58::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":696,"record_offset":9852,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":11860,"source_line":2516,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":696,"record_offset":6308,"parent_record_offset":4828,"ancestor_record_offset":4828,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":696,"record_offset":11476,"parent_record_offset":11448,"ancestor_record_offset":4828,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":696,"record_offset":11448,"parent_record_offset":11232,"ancestor_record_offset":4828,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":696,"record_offset":6012,"parent_record_offset":5956,"ancestor_record_offset":4828,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":696,"record_offset":5288,"parent_record_offset":5260,"ancestor_record_offset":4828,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":696,"record_offset":5904,"parent_record_offset":5880,"ancestor_record_offset":4828,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":696,"record_offset":5348,"parent_record_offset":5316,"ancestor_record_offset":4828,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":696,"record_offset":6068,"parent_record_offset":6040,"ancestor_record_offset":4828,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":696,"record_offset":6040,"parent_record_offset":6012,"ancestor_record_offset":4828,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":696,"record_offset":5316,"parent_record_offset":5288,"ancestor_record_offset":4828,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":696,"record_offset":5956,"parent_record_offset":5736,"ancestor_record_offset":4828,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":696,"record_offset":7844,"parent_record_offset":7820,"ancestor_record_offset":4828,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":696,"record_offset":7820,"parent_record_offset":7788,"ancestor_record_offset":4828,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":696,"record_offset":5192,"parent_record_offset":5160,"ancestor_record_offset":4828,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":696,"record_offset":5708,"parent_record_offset":5676,"ancestor_record_offset":4828,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":696,"record_offset":5260,"parent_record_offset":5192,"ancestor_record_offset":4828,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":696,"record_offset":7788,"parent_record_offset":7764,"ancestor_record_offset":4828,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":696,"record_offset":5640,"parent_record_offset":5192,"ancestor_record_offset":4828,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":696,"record_offset":10668,"parent_record_offset":10636,"ancestor_record_offset":4828,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":696,"record_offset":10576,"parent_record_offset":10552,"ancestor_record_offset":4828,"inlinee":9478,"source_line":146,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":696,"record_offset":6836,"parent_record_offset":6804,"ancestor_record_offset":4828,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":696,"record_offset":6896,"parent_record_offset":6864,"ancestor_record_offset":4828,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":696,"record_offset":5508,"parent_record_offset":5472,"ancestor_record_offset":4828,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::ptr_try_from_impls::impl$1::try_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":696,"record_offset":10752,"parent_record_offset":10728,"ancestor_record_offset":4828,"inlinee":38980,"source_line":255,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":696,"record_offset":9704,"parent_record_offset":9676,"ancestor_record_offset":4828,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":696,"record_offset":9764,"parent_record_offset":9732,"ancestor_record_offset":4828,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":696,"record_offset":5472,"parent_record_offset":5436,"ancestor_record_offset":4828,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":696,"record_offset":5436,"parent_record_offset":5192,"ancestor_record_offset":4828,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":696,"record_offset":6692,"parent_record_offset":6668,"ancestor_record_offset":4828,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":696,"record_offset":11352,"parent_record_offset":11324,"ancestor_record_offset":4828,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":696,"record_offset":11380,"parent_record_offset":11352,"ancestor_record_offset":4828,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":696,"record_offset":6864,"parent_record_offset":6804,"ancestor_record_offset":4828,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":696,"record_offset":10156,"parent_record_offset":10124,"ancestor_record_offset":4828,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":696,"record_offset":10124,"parent_record_offset":10092,"ancestor_record_offset":4828,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":696,"record_offset":6440,"parent_record_offset":6404,"ancestor_record_offset":4828,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":696,"record_offset":6556,"parent_record_offset":6528,"ancestor_record_offset":4828,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":696,"record_offset":9800,"parent_record_offset":9648,"ancestor_record_offset":4828,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":696,"record_offset":11044,"parent_record_offset":11012,"ancestor_record_offset":4828,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":696,"record_offset":5388,"parent_record_offset":5192,"ancestor_record_offset":4828,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":696,"record_offset":6644,"parent_record_offset":6528,"ancestor_record_offset":4828,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":696,"record_offset":10416,"parent_record_offset":10392,"ancestor_record_offset":4828,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":696,"record_offset":8348,"parent_record_offset":8324,"ancestor_record_offset":4828,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":696,"record_offset":8268,"parent_record_offset":8224,"ancestor_record_offset":4828,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":696,"record_offset":6720,"parent_record_offset":6692,"ancestor_record_offset":4828,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":696,"record_offset":6668,"parent_record_offset":6644,"ancestor_record_offset":4828,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":696,"record_offset":5548,"parent_record_offset":5192,"ancestor_record_offset":4828,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":696,"record_offset":8656,"parent_record_offset":8628,"ancestor_record_offset":4828,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":696,"record_offset":8472,"parent_record_offset":8444,"ancestor_record_offset":4828,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":696,"record_offset":8684,"parent_record_offset":8656,"ancestor_record_offset":4828,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":696,"record_offset":8500,"parent_record_offset":8472,"ancestor_record_offset":4828,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":696,"record_offset":7960,"parent_record_offset":7788,"ancestor_record_offset":4828,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":696,"record_offset":8324,"parent_record_offset":8300,"ancestor_record_offset":4828,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":696,"record_offset":8300,"parent_record_offset":8224,"ancestor_record_offset":4828,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":696,"record_offset":10052,"parent_record_offset":10012,"ancestor_record_offset":4828,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":696,"record_offset":10092,"parent_record_offset":10052,"ancestor_record_offset":4828,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":696,"record_offset":10200,"parent_record_offset":10012,"ancestor_record_offset":4828,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":696,"record_offset":10012,"parent_record_offset":9960,"ancestor_record_offset":4828,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":696,"record_offset":9676,"parent_record_offset":9648,"ancestor_record_offset":4828,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":696,"record_offset":9732,"parent_record_offset":9676,"ancestor_record_offset":4828,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":696,"record_offset":10292,"parent_record_offset":9960,"ancestor_record_offset":4828,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":696,"record_offset":9540,"parent_record_offset":9516,"ancestor_record_offset":4828,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":696,"record_offset":8776,"parent_record_offset":8752,"ancestor_record_offset":4828,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":696,"record_offset":8800,"parent_record_offset":8776,"ancestor_record_offset":4828,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":696,"record_offset":9900,"parent_record_offset":9876,"ancestor_record_offset":4828,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":696,"record_offset":8828,"parent_record_offset":8800,"ancestor_record_offset":4828,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":696,"record_offset":10392,"parent_record_offset":9960,"ancestor_record_offset":4828,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":696,"record_offset":9648,"parent_record_offset":9588,"ancestor_record_offset":4828,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":696,"record_offset":9960,"parent_record_offset":9924,"ancestor_record_offset":4828,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":696,"record_offset":9564,"parent_record_offset":9540,"ancestor_record_offset":4828,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":696,"record_offset":11264,"parent_record_offset":11232,"ancestor_record_offset":4828,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":696,"record_offset":11296,"parent_record_offset":11264,"ancestor_record_offset":4828,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":696,"record_offset":9620,"parent_record_offset":9588,"ancestor_record_offset":4828,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":696,"record_offset":11176,"parent_record_offset":11152,"ancestor_record_offset":4828,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":696,"record_offset":11232,"parent_record_offset":11176,"ancestor_record_offset":4828,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":696,"record_offset":11204,"parent_record_offset":11176,"ancestor_record_offset":4828,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":696,"record_offset":9588,"parent_record_offset":9564,"ancestor_record_offset":4828,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":696,"record_offset":11324,"parent_record_offset":11264,"ancestor_record_offset":4828,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":696,"record_offset":11152,"parent_record_offset":11128,"ancestor_record_offset":4828,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":696,"record_offset":9924,"parent_record_offset":9900,"ancestor_record_offset":4828,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":696,"record_offset":9516,"parent_record_offset":9492,"ancestor_record_offset":4828,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":696,"record_offset":7004,"parent_record_offset":6308,"ancestor_record_offset":4828,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":696,"record_offset":9876,"parent_record_offset":9852,"ancestor_record_offset":4828,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":696,"record_offset":6356,"parent_record_offset":6332,"ancestor_record_offset":4828,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":696,"record_offset":6604,"parent_record_offset":6580,"ancestor_record_offset":4828,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":696,"record_offset":6468,"parent_record_offset":6440,"ancestor_record_offset":4828,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":696,"record_offset":6804,"parent_record_offset":6404,"ancestor_record_offset":4828,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":696,"record_offset":6332,"parent_record_offset":6308,"ancestor_record_offset":4828,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":696,"record_offset":6496,"parent_record_offset":6468,"ancestor_record_offset":4828,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":696,"record_offset":6528,"parent_record_offset":6496,"ancestor_record_offset":4828,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":696,"record_offset":6404,"parent_record_offset":6380,"ancestor_record_offset":4828,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":696,"record_offset":6380,"parent_record_offset":6356,"ancestor_record_offset":4828,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_registry::ResourceRegistry::into_exports
// 记录形式: compiled_procedure; instances=1
// 源码行: 472
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":696,"record_offset":12212,"rva":12488768,"va":5381197888}
// 编译实例 RVAs: 0xbe9040
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f6670
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6237052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381197888,"raw_hex":"e884d4a0ff","size":5,"soft_fail":false,"target_va":5374961264,"va":5381198311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6116779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381197888,"raw_hex":"e855aaa2ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381198278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be22c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381197888,"raw_hex":"e86690ffff","size":5,"soft_fail":false,"target_va":5381169856,"va":5381198421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418c2b60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_registry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13474458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381197888,"raw_hex":"e89a9acd00","size":5,"soft_fail":false,"target_va":5394672480,"va":5381198017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":696,"record_offset":12936,"parent_record_offset":12212,"ancestor_record_offset":12212,"inlinee":7686,"source_line":2096,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":696,"record_offset":12908,"parent_record_offset":12212,"ancestor_record_offset":12212,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":696,"record_offset":12844,"parent_record_offset":12212,"ancestor_record_offset":12212,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":696,"record_offset":13328,"parent_record_offset":12212,"ancestor_record_offset":12212,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":696,"record_offset":12376,"parent_record_offset":12212,"ancestor_record_offset":12212,"inlinee":24852,"source_line":210,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":696,"record_offset":13516,"parent_record_offset":13492,"ancestor_record_offset":12212,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":696,"record_offset":13492,"parent_record_offset":13400,"ancestor_record_offset":12212,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":696,"record_offset":13428,"parent_record_offset":13400,"ancestor_record_offset":12212,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":696,"record_offset":13400,"parent_record_offset":13376,"ancestor_record_offset":12212,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":696,"record_offset":13376,"parent_record_offset":13352,"ancestor_record_offset":12212,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":696,"record_offset":12960,"parent_record_offset":12936,"ancestor_record_offset":12212,"inlinee":7687,"source_line":2122,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":696,"record_offset":13036,"parent_record_offset":13004,"ancestor_record_offset":12212,"inlinee":7691,"source_line":2100,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":696,"record_offset":13284,"parent_record_offset":13260,"ancestor_record_offset":12212,"inlinee":7696,"source_line":2156,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::process_loop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":696,"record_offset":13004,"parent_record_offset":12960,"ancestor_record_offset":12212,"inlinee":7689,"source_line":2176,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":696,"record_offset":12692,"parent_record_offset":12664,"ancestor_record_offset":12212,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":696,"record_offset":12752,"parent_record_offset":12720,"ancestor_record_offset":12212,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":696,"record_offset":13204,"parent_record_offset":13104,"ancestor_record_offset":12212,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":696,"record_offset":12872,"parent_record_offset":12844,"ancestor_record_offset":12212,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":696,"record_offset":13236,"parent_record_offset":12960,"ancestor_record_offset":12212,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":696,"record_offset":13460,"parent_record_offset":13428,"ancestor_record_offset":12212,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":696,"record_offset":12636,"parent_record_offset":12604,"ancestor_record_offset":12212,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":696,"record_offset":13260,"parent_record_offset":13236,"ancestor_record_offset":12212,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":696,"record_offset":12664,"parent_record_offset":12604,"ancestor_record_offset":12212,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":696,"record_offset":12720,"parent_record_offset":12664,"ancestor_record_offset":12212,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":696,"record_offset":12460,"parent_record_offset":12432,"ancestor_record_offset":12212,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::keys
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":696,"record_offset":12432,"parent_record_offset":12404,"ancestor_record_offset":12212,"inlinee":10779,"source_line":650,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":696,"record_offset":12604,"parent_record_offset":12516,"ancestor_record_offset":12212,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":696,"record_offset":12488,"parent_record_offset":12460,"ancestor_record_offset":12212,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":696,"record_offset":12576,"parent_record_offset":12516,"ancestor_record_offset":12212,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":696,"record_offset":12548,"parent_record_offset":12516,"ancestor_record_offset":12212,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":696,"record_offset":12516,"parent_record_offset":12488,"ancestor_record_offset":12212,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":696,"record_offset":12404,"parent_record_offset":12376,"ancestor_record_offset":12212,"inlinee":10778,"source_line":290,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_registry::impl$1::into_exports::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_registry.rs
//     evidence={"module":696,"record_offset":13064,"parent_record_offset":13036,"ancestor_record_offset":12212,"inlinee":39452,"source_line":511,"archive":"_pdb/B/modules/00696.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_registry::impl$1::into_exports::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 498
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x130C1
// 首条位置: {"module":1741,"inlinee":78017}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1741,"record_offset":11540,"parent_record_offset":11496,"ancestor_record_offset":11496,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01741.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1741,"record_offset":11636,"parent_record_offset":11496,"ancestor_record_offset":11496,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/01741.jsonl.gz"}
//   [pdb_inline_direct] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1741,"record_offset":11744,"parent_record_offset":11496,"ancestor_record_offset":11496,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01741.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1741,"record_offset":11772,"parent_record_offset":11744,"ancestor_record_offset":11496,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01741.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1741,"record_offset":11800,"parent_record_offset":11744,"ancestor_record_offset":11496,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01741.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1741,"record_offset":11568,"parent_record_offset":11540,"ancestor_record_offset":11496,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01741.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1741,"record_offset":11668,"parent_record_offset":11636,"ancestor_record_offset":11496,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/01741.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1741,"record_offset":11596,"parent_record_offset":11568,"ancestor_record_offset":11496,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01741.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1741,"record_offset":11700,"parent_record_offset":11668,"ancestor_record_offset":11496,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/01741.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_registry::impl$1::into_exports::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 511
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9A1C
// 首条位置: {"module":696,"inlinee":39452}
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
// 完整结构及原始类型记录: resource_registry.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::resource_registry::RegistryResource
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::resource_registry::RegistryResource>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::resource_registry::RegistryResource"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<rhapsody::rg::resource_registry::RegistryResource,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["rhapsody::rg::resource_registry::RegistryResource", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::rg::resource_registry::RegistryResource,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<rhapsody::rg::resource_registry::RegistryResource,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<rhapsody::rg::resource_registry::RegistryResource,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["rhapsody::rg::resource_registry::RegistryResource", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<rhapsody::rg::resource_registry::RegistryResource,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["rhapsody::rg::resource_registry::RegistryResource", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<rhapsody::rg::resource_registry::RegistryResource,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<rhapsody::rg::resource_registry::RegistryResource,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "slice2$<rhapsody::rg::resource_registry::RegistryResource>", "constructor": "slice2$", "arguments": ["rhapsody::rg::resource_registry::RegistryResource"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<slice2$<rhapsody::rg::resource_registry::RegistryResource> >", "constructor": "core::ptr::drop_in_place", "arguments": ["slice2$<rhapsody::rg::resource_registry::RegistryResource>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::resource_registry::ResourceRegistry
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 9; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::resource_registry::ResourceRegistry>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::resource_registry::ResourceRegistry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::resource_registry::ResourceReuseInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<rhapsody::raw::texture::TextureDesc,rhapsody::rg::resource_registry::ResourceReuseInfo,std::hash::random::RandomState>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["rhapsody::raw::texture::TextureDesc", "rhapsody::rg::resource_registry::ResourceReuseInfo", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::HashMap<rhapsody::raw::texture::TextureDesc,rhapsody::rg::resource_registry::ResourceReuseInfo,std::hash::random::RandomState> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::HashMap<rhapsody::raw::texture::TextureDesc,rhapsody::rg::resource_registry::ResourceReuseInfo,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<rhapsody::raw::buffer::BufferDesc,rhapsody::rg::resource_registry::ResourceReuseInfo>", "constructor": "tuple$", "arguments": ["rhapsody::raw::buffer::BufferDesc", "rhapsody::rg::resource_registry::ResourceReuseInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<rhapsody::raw::buffer::BufferDesc,rhapsody::raw::buffer::BufferDesc,rhapsody::rg::resource_registry::ResourceReuseInfo>", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["rhapsody::raw::buffer::BufferDesc", "rhapsody::raw::buffer::BufferDesc", "rhapsody::rg::resource_registry::ResourceReuseInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<rhapsody::raw::buffer::BufferDesc,rhapsody::rg::resource_registry::ResourceReuseInfo>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<rhapsody::raw::buffer::BufferDesc,rhapsody::raw::buffer::BufferDesc,rhapsody::rg::resource_registry::ResourceReuseInfo> >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<rhapsody::raw::buffer::BufferDesc,rhapsody::rg::resource_registry::ResourceReuseInfo>", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<rhapsody::raw::buffer::BufferDesc,rhapsody::raw::buffer::BufferDesc,rhapsody::rg::resource_registry::ResourceReuseInfo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<rhapsody::raw::texture::TextureDesc,rhapsody::rg::resource_registry::ResourceReuseInfo>", "constructor": "tuple$", "arguments": ["rhapsody::raw::texture::TextureDesc", "rhapsody::rg::resource_registry::ResourceReuseInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<rhapsody::raw::texture::TextureDesc,rhapsody::raw::texture::TextureDesc,rhapsody::rg::resource_registry::ResourceReuseInfo>", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["rhapsody::raw::texture::TextureDesc", "rhapsody::raw::texture::TextureDesc", "rhapsody::rg::resource_registry::ResourceReuseInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<rhapsody::raw::texture::TextureDesc,rhapsody::rg::resource_registry::ResourceReuseInfo>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<rhapsody::raw::texture::TextureDesc,rhapsody::raw::texture::TextureDesc,rhapsody::rg::resource_registry::ResourceReuseInfo> >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<rhapsody::raw::texture::TextureDesc,rhapsody::rg::resource_registry::ResourceReuseInfo>", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<rhapsody::raw::texture::TextureDesc,rhapsody::raw::texture::TextureDesc,rhapsody::rg::resource_registry::ResourceReuseInfo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: resource_registry.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: resource_registry.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>", "bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_audio::WaterAudioState>,bevy_ecs::event::reader::EventReader<country_core::resources::water::WaterDrawEvent>,bevy_ecs::event::reader::EventReader<country_core::resources::water::WaterEraseEvent>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_audio::WaterAudioState>", "bevy_ecs::event::reader::EventReader<country_core::resources::water::WaterDrawEvent>", "bevy_ecs::event::reader::EventReader<country_core::resources::water::WaterEraseEvent>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_audio::WaterAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::water::WaterDrawEvent>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::water::WaterEraseEvent>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::audio::water_audio::WaterAudioState> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::audio::water_audio::WaterAudioState> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::water_audio::WaterAudioState>

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_registry::ResourceRegistry::buffer
// 信息 ID: 00a217e0c303cec64c7b293e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::clear_rg_buffer::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources> @ 0x9049b4 -> 0xbe79d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::allocate_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0x15f8ff7 -> 0xbe79d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0x15f9a7f -> 0xbe79d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::fill_unsorted_instance_indirection::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0x15fa7a0 -> 0xbe79d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0x15faccf -> 0xbe79d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$1<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0x15fba2b -> 0xbe79d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::fill_sorted_instance_list::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0x15fbf53 -> 0xbe79d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32>,alloc::alloc::Global> @ 0x169a680 -> 0xbe79d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,alloc::alloc::Global> @ 0x169a87d -> 0xbe79d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,alloc::alloc::Global> @ 0x169a89f -> 0xbe79d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,alloc::alloc::Global> @ 0x169a8c1 -> 0xbe79d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args::closure_env$0,alloc::alloc::Global> @ 0x169a8e3 -> 0xbe79d0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::rg::binding::rg_binding_to_descriptor_binding; module=3095, inline_record=2516; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::binding::rg_binding_to_descriptor_binding; module=3095, inline_record=2916; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_registry::ResourceRegistry::image
// 信息 ID: 541bf5e38887a1580ab1f2c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::clear_rg_color_image::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>; module=503, inline_record=5092; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_registry::ResourceRegistry::image_from_raw_handle
// 信息 ID: d04b53061dc107b5559aa84c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::clear_rg_color_image::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources> @ 0x904a54 -> 0xbe7940 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::rg::pass_api::RenderPassApi::begin_render_pass; module=1680, inline_record=1316; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::binding::rg_binding_to_descriptor_binding; module=3095, inline_record=3316; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<ref$<rhapsody::rg::pass_api::RasterPassColorAttachment> >,rhapsody::rg::pass_api::impl$0::begin_render_pass::closure_env$0>; module=3314, inline_record=624; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_registry::ResourceRegistry::image_view
// 信息 ID: 9afc9336006d51aa49da52ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::pass_api::RenderPassApi::begin_render_pass @ 0x16f710d -> 0xbe7a60 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::pass_api::RenderPassApi::begin_render_pass @ 0x16f727d -> 0xbe7a60 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<ash::vk::definitions::DescriptorImageInfo,core::ops::range::Range<u32>,rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::DescriptorImageInfo,alloc::vec::impl$20::extend_trusted::closure_env$0<ash::vk::definitions::DescriptorImageInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<u32>,rhapsody::rg::binding::rg_binding_to_descriptor_binding::closure_env$2> > > > @ 0x18b47be -> 0xbe7a60 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::binding::rg_binding_to_descriptor_binding @ 0x205f31a -> 0xbe7a60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_registry::ResourceRegistry::into_exports
// 信息 ID: 01f19281a3e70005a3548155
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::execute_compiled<country_core::render::pass_builder::pass_resources::PassResources> @ 0xfaba1 -> 0xbe9040 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_registry::ResourceRegistry::populate_resources
// 信息 ID: b022337d37a4f3534c9bc92b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x163af4 -> 0xbe7ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0> @ 0x165827 -> 0xbe7ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0> @ 0x166fc6 -> 0xbe7ba0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_registry::ResourceRegistry::resource
// 信息 ID: ae3e4164344b59e8b52ba2d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,alloc::alloc::Global>,ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,rhapsody::rg::compiled::impl$1::record_batch::closure_env$2<country_core::render::pass_builder::pass_resources::PassResources>,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > > > > @ 0xf86f9 -> 0xbe78b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2c1e8d -> 0xbe78b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2c2726 -> 0xbe78b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2c27bf -> 0xbe78b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$1::record_batch::closure$1<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2c345a -> 0xbe78b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::add_pre_pass_barriers<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2c3df5 -> 0xbe78b0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::rg::pass_api::RenderPassApi::begin_render_pass; module=1680, inline_record=2332; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::pass_api::RenderPassApi::begin_render_pass; module=1680, inline_record=3208; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_registry::impl$1::into_exports::closure$0
// 信息 ID: c9cdf87beb24ea1ef227fcca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::rg::resource_registry::impl$1::into_exports", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::rg::resource_registry::impl$1::into_exports::closure_env$0 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<u16,tuple$<> >,hashbrown::map::impl$44::fold::closure_env$0<u16,tuple$<>,tuple$<>,hashbrown::map::impl$58::fold::closure_env$0<u16,tuple$<>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref$<u16>,tuple$<rhapsody::rg::resource::ResourceHandle,enum2$<rhapsody::rg::resource::RuntimeResource> >,tuple$<>,rhapsody::rg::resource_registry::impl$1::into_exports::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<rhapsody::rg::resource::ResourceHandle,enum2$<rhapsody::rg::resource::RuntimeResource> >,hashbrown::map::impl$82::extend::closure_env$0<rhapsody::rg::resource::ResourceHandle,enum2$<rhapsody::rg::resource::RuntimeResource>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::set::Iter<u16>,rhapsody::rg::resource_registry::impl$1::into_exports::closure_env$0> > > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::rg::resource_registry::impl$1::into_exports::closure_env$0 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<u16,tuple$<> >,hashbrown::map::impl$44::fold::closure_env$0<u16,tuple$<>,tuple$<>,hashbrown::map::impl$58::fold::closure_env$0<u16,tuple$<>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref$<u16>,tuple$<rhapsody::rg::resource::ResourceHandle,enum2$<rhapsody::rg::resource::RuntimeResource> >,tuple$<>,rhapsody::rg::resource_registry::impl$1::into_exports::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<rhapsody::rg::resource::ResourceHandle,enum2$<rhapsody::rg::resource::RuntimeResource> >,hashbrown::map::impl$82::extend::closure_env$0<rhapsody::rg::resource::ResourceHandle,enum2$<rhapsody::rg::resource::RuntimeResource>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::set::Iter<u16>,rhapsody::rg::resource_registry::impl$1::into_exports::closure_env$0> > > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::rg::resource_registry::impl$1::into_exports::closure_env$0 <- std::collections::hash::map::impl$87::from_iter<rhapsody::rg::resource::ResourceHandle,enum2$<rhapsody::rg::resource::RuntimeResource>,std::hash::random::RandomState,core::iter::adapters::map::Map<std::collections::hash::set::Iter<u16>,rhapsody::rg::resource_registry::impl$1::into_exports::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] hashbrown::raw::RawIterRange::fold_impl<tuple$<u16,tuple$<> >,hashbrown::map::impl$44::fold::closure_env$0<u16,tuple$<>,tuple$<>,hashbrown::map::impl$58::fold::closure_env$0<u16,tuple$<>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref$<u16>,tuple$<rhapsody::rg::resource::ResourceHandle,enum2$<rhapsody::rg::resource::RuntimeResource> >,tuple$<>,rhapsody::rg::resource_registry::impl$1::into_exports::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<rhapsody::rg::resource::ResourceHandle,enum2$<rhapsody::rg::resource::RuntimeResource> >,hashbrown::map::impl$82::extend::closure_env$0<rhapsody::rg::resource::ResourceHandle,enum2$<rhapsody::rg::resource::RuntimeResource>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::set::Iter<u16>,rhapsody::rg::resource_registry::impl$1::into_exports::closure_env$0> > > > > >,tuple$<> >; module=1741, inline_record=11496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_registry::impl$1::into_exports::closure$1
// 信息 ID: c5c073d0eae8906254c9315e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::rg::resource_registry::impl$1::into_exports", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::resource_registry::ResourceRegistry::into_exports; module=696, inline_record=13064; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::resource_registry::ResourceRegistry::into_exports; module=696, inline_record=13172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_registry::impl$2::fmt
// 信息 ID: 639b2b7e8fef56ccb8f3e494
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_registry::impl$5::default
// 信息 ID: ea7ed5b30f41c8fd4f3e6cb4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>; module=66, inline_record=26644; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0>; module=66, inline_record=40892; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0>; module=66, inline_record=48892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

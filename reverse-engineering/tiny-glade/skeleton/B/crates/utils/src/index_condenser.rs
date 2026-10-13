// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/utils/src/index_condenser.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: ddee69e15855c872b553aeb84f33947bb62b895508fb7b2ba947ec47fb33ff2e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: index_condenser.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 523, 540, 554, 1622
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::index_condenser::impl$6::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7934
// 首条位置: {"module":540,"inlinee":31028}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::index_condenser::Condex::last_known_index
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x74F0
// 首条位置: {"module":523,"inlinee":29936}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::index_condenser::impl$3::default
// 记录形式: inline_instance, inlinee_declaration; instances=22
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7959
// 首条位置: {"module":1622,"inlinee":31065}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1622,"record_offset":9116,"parent_record_offset":9088,"ancestor_record_offset":9088,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":9316,"parent_record_offset":9288,"ancestor_record_offset":9088,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":9368,"parent_record_offset":9344,"ancestor_record_offset":9088,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":9344,"parent_record_offset":9288,"ancestor_record_offset":9088,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1622,"record_offset":9392,"parent_record_offset":9368,"ancestor_record_offset":9088,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1622,"record_offset":9260,"parent_record_offset":9232,"ancestor_record_offset":9088,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":9416,"parent_record_offset":9392,"ancestor_record_offset":9088,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1622,"record_offset":9172,"parent_record_offset":9144,"ancestor_record_offset":9088,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1622,"record_offset":9288,"parent_record_offset":9232,"ancestor_record_offset":9088,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1622,"record_offset":9144,"parent_record_offset":9116,"ancestor_record_offset":9088,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1622,"record_offset":9232,"parent_record_offset":9200,"ancestor_record_offset":9088,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1622,"record_offset":9200,"parent_record_offset":9172,"ancestor_record_offset":9088,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01622.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::index_condenser::IndexCondenser::get
// 记录形式: inline_instance, inlinee_declaration; instances=26
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x74E3
// 首条位置: {"module":523,"inlinee":29923}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cell::Cell::set
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":523,"record_offset":9064,"parent_record_offset":8032,"ancestor_record_offset":8032,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":523,"record_offset":9036,"parent_record_offset":8032,"ancestor_record_offset":8032,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":523,"record_offset":8072,"parent_record_offset":8032,"ancestor_record_offset":8032,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::impl$41::eq
//     owner=private_source_ownership_candidate; count=23
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":523,"record_offset":8128,"parent_record_offset":8032,"ancestor_record_offset":8032,"inlinee":29925,"source_line":796,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":523,"record_offset":8156,"parent_record_offset":8032,"ancestor_record_offset":8032,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":523,"record_offset":9088,"parent_record_offset":9064,"ancestor_record_offset":8032,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":523,"record_offset":8792,"parent_record_offset":8768,"ancestor_record_offset":8032,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":523,"record_offset":8444,"parent_record_offset":8420,"ancestor_record_offset":8032,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":523,"record_offset":8528,"parent_record_offset":8504,"ancestor_record_offset":8032,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":523,"record_offset":8468,"parent_record_offset":8444,"ancestor_record_offset":8032,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":523,"record_offset":8860,"parent_record_offset":8312,"ancestor_record_offset":8032,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":523,"record_offset":9112,"parent_record_offset":9088,"ancestor_record_offset":8032,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":523,"record_offset":8676,"parent_record_offset":8648,"ancestor_record_offset":8032,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":523,"record_offset":8648,"parent_record_offset":8620,"ancestor_record_offset":8032,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":523,"record_offset":9136,"parent_record_offset":9112,"ancestor_record_offset":8032,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":523,"record_offset":8096,"parent_record_offset":8072,"ancestor_record_offset":8032,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$41::eq
//     owner=private_source_ownership_candidate; count=23
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":523,"record_offset":8816,"parent_record_offset":8792,"ancestor_record_offset":8032,"inlinee":29925,"source_line":796,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=23
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":523,"record_offset":8588,"parent_record_offset":8560,"ancestor_record_offset":8032,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=23
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":523,"record_offset":8620,"parent_record_offset":8588,"ancestor_record_offset":8032,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=23
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":523,"record_offset":8560,"parent_record_offset":8312,"ancestor_record_offset":8032,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=23
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":523,"record_offset":8420,"parent_record_offset":8312,"ancestor_record_offset":8032,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=23
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":523,"record_offset":8888,"parent_record_offset":8312,"ancestor_record_offset":8032,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=46
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":523,"record_offset":8504,"parent_record_offset":8312,"ancestor_record_offset":8032,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=23
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":523,"record_offset":8392,"parent_record_offset":8312,"ancestor_record_offset":8032,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=23
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":523,"record_offset":8768,"parent_record_offset":8744,"ancestor_record_offset":8032,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":1622,"record_offset":77876,"parent_record_offset":77852,"ancestor_record_offset":77728,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":523,"record_offset":8180,"parent_record_offset":8156,"ancestor_record_offset":8032,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=23
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":523,"record_offset":8204,"parent_record_offset":8180,"ancestor_record_offset":8032,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=23
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":523,"record_offset":8744,"parent_record_offset":8720,"ancestor_record_offset":8032,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=23
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":523,"record_offset":8236,"parent_record_offset":8204,"ancestor_record_offset":8032,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=23
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":523,"record_offset":8980,"parent_record_offset":8312,"ancestor_record_offset":8032,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=23
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":523,"record_offset":8288,"parent_record_offset":8264,"ancestor_record_offset":8032,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=23
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":523,"record_offset":8264,"parent_record_offset":8204,"ancestor_record_offset":8032,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=23
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":523,"record_offset":8312,"parent_record_offset":8288,"ancestor_record_offset":8032,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=23
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":523,"record_offset":8364,"parent_record_offset":8312,"ancestor_record_offset":8032,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=23
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":523,"record_offset":8720,"parent_record_offset":8312,"ancestor_record_offset":8032,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::index_condenser::IndexCondenser::insert
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x74EE
// 首条位置: {"module":523,"inlinee":29934}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":523,"record_offset":12540,"parent_record_offset":11616,"ancestor_record_offset":11616,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":523,"record_offset":11656,"parent_record_offset":11616,"ancestor_record_offset":11616,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":523,"record_offset":12716,"parent_record_offset":11616,"ancestor_record_offset":11616,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":523,"record_offset":12596,"parent_record_offset":12572,"ancestor_record_offset":11616,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":523,"record_offset":12644,"parent_record_offset":12620,"ancestor_record_offset":11616,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":523,"record_offset":12620,"parent_record_offset":12596,"ancestor_record_offset":11616,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":523,"record_offset":12572,"parent_record_offset":12540,"ancestor_record_offset":11616,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":523,"record_offset":12296,"parent_record_offset":12272,"ancestor_record_offset":11616,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":523,"record_offset":11948,"parent_record_offset":11924,"ancestor_record_offset":11616,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":523,"record_offset":12032,"parent_record_offset":12008,"ancestor_record_offset":11616,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":523,"record_offset":11972,"parent_record_offset":11948,"ancestor_record_offset":11616,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":523,"record_offset":12364,"parent_record_offset":11812,"ancestor_record_offset":11616,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":523,"record_offset":12180,"parent_record_offset":12152,"ancestor_record_offset":11616,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":523,"record_offset":12152,"parent_record_offset":12124,"ancestor_record_offset":11616,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":523,"record_offset":12684,"parent_record_offset":12540,"ancestor_record_offset":11616,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$41::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":523,"record_offset":12320,"parent_record_offset":12296,"ancestor_record_offset":11616,"inlinee":29925,"source_line":796,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":523,"record_offset":12092,"parent_record_offset":12064,"ancestor_record_offset":11616,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":523,"record_offset":12124,"parent_record_offset":12092,"ancestor_record_offset":11616,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":523,"record_offset":12064,"parent_record_offset":11812,"ancestor_record_offset":11616,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":523,"record_offset":11924,"parent_record_offset":11812,"ancestor_record_offset":11616,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":523,"record_offset":12392,"parent_record_offset":11812,"ancestor_record_offset":11616,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":523,"record_offset":12008,"parent_record_offset":11812,"ancestor_record_offset":11616,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":523,"record_offset":11896,"parent_record_offset":11812,"ancestor_record_offset":11616,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":523,"record_offset":12272,"parent_record_offset":12248,"ancestor_record_offset":11616,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":1622,"record_offset":72048,"parent_record_offset":72020,"ancestor_record_offset":71980,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":523,"record_offset":11680,"parent_record_offset":11656,"ancestor_record_offset":11616,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":523,"record_offset":11704,"parent_record_offset":11680,"ancestor_record_offset":11616,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":523,"record_offset":12248,"parent_record_offset":12224,"ancestor_record_offset":11616,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":523,"record_offset":11736,"parent_record_offset":11704,"ancestor_record_offset":11616,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":523,"record_offset":12484,"parent_record_offset":11812,"ancestor_record_offset":11616,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":523,"record_offset":11788,"parent_record_offset":11764,"ancestor_record_offset":11616,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":523,"record_offset":11764,"parent_record_offset":11704,"ancestor_record_offset":11616,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":523,"record_offset":11812,"parent_record_offset":11788,"ancestor_record_offset":11616,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":523,"record_offset":11868,"parent_record_offset":11812,"ancestor_record_offset":11616,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":523,"record_offset":12224,"parent_record_offset":11812,"ancestor_record_offset":11616,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00523.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::index_condenser::IndexCondenser::remove<country_core::render::nani::gpu_world::InstanceKey>
// 记录形式: compiled_procedure; instances=1
// 源码行: 145
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":540,"record_offset":1464,"rva":10592080,"va":5379301200}
// 编译实例 RVAs: 0xa19f50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0ed0
//     owner=unresolved_namespace; count=3
//     target_aliases=6 candidates; complete list: index_condenser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4624582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379301200,"raw_hex":"e83a6fb9ff","size":5,"soft_fail":false,"target_va":5374676688,"va":5379301265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b76a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: index_condenser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1427867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379301200,"raw_hex":"e89bc91500","size":5,"soft_fail":false,"target_va":5380729376,"va":5379301504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::swap_remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32239257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379301200,"raw_hex":"e899eeeb01","size":5,"soft_fail":false,"target_va":5411541017,"va":5379301755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32240034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379301200,"raw_hex":"e8a2f1eb01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379301737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::swap_remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":540,"record_offset":2856,"parent_record_offset":1464,"ancestor_record_offset":1464,"inlinee":5532,"source_line":1927,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":540,"record_offset":4136,"parent_record_offset":1464,"ancestor_record_offset":1464,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":540,"record_offset":3196,"parent_record_offset":1464,"ancestor_record_offset":1464,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":540,"record_offset":3252,"parent_record_offset":1464,"ancestor_record_offset":1464,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":540,"record_offset":3080,"parent_record_offset":1464,"ancestor_record_offset":1464,"inlinee":8292,"source_line":1244,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] utils::index_condenser::IndexCondenser::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/index_condenser.rs
//     evidence={"module":540,"record_offset":1668,"parent_record_offset":1464,"ancestor_record_offset":1464,"inlinee":29923,"source_line":96,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":540,"record_offset":2928,"parent_record_offset":2896,"ancestor_record_offset":1464,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":540,"record_offset":2976,"parent_record_offset":2952,"ancestor_record_offset":1464,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":540,"record_offset":2952,"parent_record_offset":2928,"ancestor_record_offset":1464,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":540,"record_offset":2896,"parent_record_offset":2856,"ancestor_record_offset":1464,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":540,"record_offset":3048,"parent_record_offset":2856,"ancestor_record_offset":1464,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":540,"record_offset":2764,"parent_record_offset":2740,"ancestor_record_offset":1464,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":540,"record_offset":2740,"parent_record_offset":1668,"ancestor_record_offset":1464,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":540,"record_offset":2464,"parent_record_offset":2440,"ancestor_record_offset":1464,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":540,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":1464,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":540,"record_offset":2200,"parent_record_offset":2176,"ancestor_record_offset":1464,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":540,"record_offset":3016,"parent_record_offset":2856,"ancestor_record_offset":1464,"inlinee":5303,"source_line":4520,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":540,"record_offset":2140,"parent_record_offset":2116,"ancestor_record_offset":1464,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":540,"record_offset":2532,"parent_record_offset":1976,"ancestor_record_offset":1464,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":540,"record_offset":2788,"parent_record_offset":2764,"ancestor_record_offset":1464,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":540,"record_offset":2348,"parent_record_offset":2320,"ancestor_record_offset":1464,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":540,"record_offset":2320,"parent_record_offset":2292,"ancestor_record_offset":1464,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":540,"record_offset":2712,"parent_record_offset":1668,"ancestor_record_offset":1464,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":540,"record_offset":2812,"parent_record_offset":2788,"ancestor_record_offset":1464,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":540,"record_offset":1716,"parent_record_offset":1668,"ancestor_record_offset":1464,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":540,"record_offset":1740,"parent_record_offset":1716,"ancestor_record_offset":1464,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$41::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":540,"record_offset":1772,"parent_record_offset":1668,"ancestor_record_offset":1464,"inlinee":29925,"source_line":796,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":540,"record_offset":2260,"parent_record_offset":2232,"ancestor_record_offset":1464,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":540,"record_offset":2292,"parent_record_offset":2260,"ancestor_record_offset":1464,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":540,"record_offset":2232,"parent_record_offset":1976,"ancestor_record_offset":1464,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":540,"record_offset":2092,"parent_record_offset":1976,"ancestor_record_offset":1464,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":540,"record_offset":2560,"parent_record_offset":1976,"ancestor_record_offset":1464,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":540,"record_offset":2176,"parent_record_offset":1976,"ancestor_record_offset":1464,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":540,"record_offset":2064,"parent_record_offset":1976,"ancestor_record_offset":1464,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":540,"record_offset":2440,"parent_record_offset":2416,"ancestor_record_offset":1464,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":540,"record_offset":1828,"parent_record_offset":1800,"ancestor_record_offset":1464,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":540,"record_offset":1856,"parent_record_offset":1828,"ancestor_record_offset":1464,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":540,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":1464,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":540,"record_offset":3276,"parent_record_offset":3252,"ancestor_record_offset":1464,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":540,"record_offset":3104,"parent_record_offset":3080,"ancestor_record_offset":1464,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":540,"record_offset":3128,"parent_record_offset":3104,"ancestor_record_offset":1464,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":540,"record_offset":2416,"parent_record_offset":2392,"ancestor_record_offset":1464,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":540,"record_offset":1892,"parent_record_offset":1856,"ancestor_record_offset":1464,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":540,"record_offset":2656,"parent_record_offset":1976,"ancestor_record_offset":1464,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":540,"record_offset":1948,"parent_record_offset":1920,"ancestor_record_offset":1464,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":540,"record_offset":1920,"parent_record_offset":1856,"ancestor_record_offset":1464,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":540,"record_offset":3360,"parent_record_offset":3300,"ancestor_record_offset":1464,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":540,"record_offset":1976,"parent_record_offset":1948,"ancestor_record_offset":1464,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":540,"record_offset":2036,"parent_record_offset":1976,"ancestor_record_offset":1464,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":540,"record_offset":2392,"parent_record_offset":1976,"ancestor_record_offset":1464,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":540,"record_offset":1800,"parent_record_offset":1668,"ancestor_record_offset":1464,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00540.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::index_condenser::IndexCondenser::clear
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 172
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x74EB
// 首条位置: {"module":523,"inlinee":29931}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":22276,"parent_record_offset":21768,"ancestor_record_offset":21768,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":554,"record_offset":21800,"parent_record_offset":21768,"ancestor_record_offset":21768,"inlinee":19928,"source_line":727,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":554,"record_offset":22204,"parent_record_offset":22180,"ancestor_record_offset":21768,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":21892,"parent_record_offset":21856,"ancestor_record_offset":21768,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":554,"record_offset":22180,"parent_record_offset":22156,"ancestor_record_offset":21768,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::impl$2::fill_tag
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":554,"record_offset":22156,"parent_record_offset":22096,"ancestor_record_offset":21768,"inlinee":6225,"source_line":79,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":554,"record_offset":21828,"parent_record_offset":21800,"ancestor_record_offset":21768,"inlinee":17339,"source_line":1009,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":21856,"parent_record_offset":21828,"ancestor_record_offset":21768,"inlinee":10622,"source_line":813,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":21976,"parent_record_offset":21948,"ancestor_record_offset":21768,"inlinee":10625,"source_line":807,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":22004,"parent_record_offset":21976,"ancestor_record_offset":21768,"inlinee":10626,"source_line":3020,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::ctrl_slice
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":22096,"parent_record_offset":22004,"ancestor_record_offset":21768,"inlinee":10627,"source_line":2482,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":22068,"parent_record_offset":22004,"ancestor_record_offset":21768,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::num_ctrl_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":22128,"parent_record_offset":22096,"ancestor_record_offset":21768,"inlinee":6148,"source_line":2504,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":22040,"parent_record_offset":22004,"ancestor_record_offset":21768,"inlinee":6127,"source_line":128,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::clear::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":21948,"parent_record_offset":21920,"ancestor_record_offset":21768,"inlinee":10624,"source_line":819,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::scopeguard::impl$3::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\scopeguard.rs
//     evidence={"module":554,"record_offset":21920,"parent_record_offset":21892,"ancestor_record_offset":21768,"inlinee":6123,"source_line":69,"archive":"_pdb/B/modules/00554.jsonl.gz"}
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
// 完整结构及原始类型记录: index_condenser.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::index_condenser::Condex
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "utils::index_condenser::Condex<country_core::render::nani::gpu_world::InstanceKey>", "constructor": "utils::index_condenser::Condex", "arguments": ["country_core::render::nani::gpu_world::InstanceKey"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<utils::index_condenser::Condex<country_core::render::nani::gpu_world::InstanceKey> >", "constructor": "core::fmt::impl$74::fmt", "arguments": ["utils::index_condenser::Condex<country_core::render::nani::gpu_world::InstanceKey>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::index_condenser::IndexCondenser
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "utils::index_condenser::IndexCondenser<country_core::render::nani::gpu_world::InstanceKey>", "constructor": "utils::index_condenser::IndexCondenser", "arguments": ["country_core::render::nani::gpu_world::InstanceKey"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<utils::index_condenser::IndexCondenser<country_core::render::nani::gpu_world::InstanceKey> >", "constructor": "core::ptr::drop_in_place", "arguments": ["utils::index_condenser::IndexCondenser<country_core::render::nani::gpu_world::InstanceKey>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::index_condenser::IndexCondenser::remove<country_core::render::nani::gpu_world::InstanceKey>", "constructor": "utils::index_condenser::IndexCondenser::remove", "arguments": ["country_core::render::nani::gpu_world::InstanceKey"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: index_condenser.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: index_condenser.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::index_condenser::Condex::last_known_index
// 信息 ID: 984a2cc1c6d0554884d4b856
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::extend_no_data; module=523, inline_record=15836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::index_condenser::IndexCondenser::clear
// 信息 ID: 344264a93ca35c4575323895
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::clearing::uninit_clearing; module=554, inline_record=21768; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::index_condenser::IndexCondenser::get
// 信息 ID: 6793333341d768119cd38b5e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 23 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_mesh; module=523, inline_record=8032; 不是独立调用边。
// [PDB 内联宿主] utils::index_condenser::IndexCondenser::remove<country_core::render::nani::gpu_world::InstanceKey>; module=540, inline_record=1668; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::UberInstanceData>; module=1622, inline_record=77728; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::SolidVertexColorInstanceData>; module=1622, inline_record=79676; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData>; module=1622, inline_record=81628; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::PlasterInstanceData>; module=1622, inline_record=83564; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>; module=1622, inline_record=85516; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::GothicWindowBricksInstanceData>; module=1622, inline_record=87464; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::index_condenser::IndexCondenser::insert
// 信息 ID: 8f6936e5786e2748fe9ae578
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::extend_no_data; module=523, inline_record=11616; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::create_instances<country_core::resources::placement_data::PlacementDataSsbo,country_core::systems::prefab::create_prefab_gpu_instances::closure_env$0>; module=1622, inline_record=71980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::index_condenser::IndexCondenser::remove<country_core::render::nani::gpu_world::InstanceKey>
// 信息 ID: c54cda393e28497af0533c1d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::index_condenser::IndexCondenser::remove -> ["country_core::render::nani::gpu_world::InstanceKey"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::GpuWorld::remove_instance @ 0x991f97 -> 0xa19f50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::index_condenser::impl$3::default
// 信息 ID: 184d929c118b6742d81c0cb4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 21 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::BalconyDoorInstanceData>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >; module=1622, inline_record=9088; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::UberInstanceData>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >; module=1622, inline_record=11584; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketCurvedHalftimber>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >; module=1622, inline_record=14168; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::StoneFloorInstanceData>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >; module=1622, inline_record=16664; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned_custom::closure_env$0<country_core::render::nani::instance_data::SmokeInstanceData,country_core::render::nani::cull_shaders::NaniCullAndBucketSmoke>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >; module=1622, inline_record=19228; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >; module=1622, inline_record=21744; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::SolidVertexColorInstanceData>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >; module=1622, inline_record=24244; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$4::gpu_world_subset_desc::owned::closure_env$0<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >; module=1622, inline_record=26764; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::index_condenser::impl$6::fmt
// 信息 ID: c90e8c7bf4f3b0086419994b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<utils::index_condenser::Condex<country_core::render::nani::gpu_world::InstanceKey> >; module=540, inline_record=452; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

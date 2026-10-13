// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/rg/resource_pool.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 901320120f85e03a51fdbf1dd293459f6f31f71aa0bf8bce6f114ae106b078c9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: resource_pool.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 31, 240, 695
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use gpu_allocator; // namespace candidate
//   package identities: [{"name":"gpu-allocator","source":"local_or_path","version":"0.26.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_pool::impl$1::default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F9C
// 首条位置: {"module":31,"inlinee":8092}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":10764,"parent_record_offset":9300,"ancestor_record_offset":9300,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":31,"record_offset":9336,"parent_record_offset":9300,"ancestor_record_offset":9300,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":31,"record_offset":9864,"parent_record_offset":9832,"ancestor_record_offset":9300,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":31,"record_offset":9924,"parent_record_offset":9892,"ancestor_record_offset":9300,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":31,"record_offset":9720,"parent_record_offset":9696,"ancestor_record_offset":9300,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":31,"record_offset":9892,"parent_record_offset":9832,"ancestor_record_offset":9300,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":31,"record_offset":9468,"parent_record_offset":9432,"ancestor_record_offset":9300,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":31,"record_offset":9584,"parent_record_offset":9556,"ancestor_record_offset":9300,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":10788,"parent_record_offset":10764,"ancestor_record_offset":9300,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":31,"record_offset":9672,"parent_record_offset":9556,"ancestor_record_offset":9300,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":9748,"parent_record_offset":9720,"ancestor_record_offset":9300,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":9696,"parent_record_offset":9672,"ancestor_record_offset":9300,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":9776,"parent_record_offset":9720,"ancestor_record_offset":9300,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":31,"record_offset":10836,"parent_record_offset":10812,"ancestor_record_offset":9300,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":31,"record_offset":10032,"parent_record_offset":9336,"ancestor_record_offset":9300,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":31,"record_offset":9384,"parent_record_offset":9360,"ancestor_record_offset":9300,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":31,"record_offset":9632,"parent_record_offset":9608,"ancestor_record_offset":9300,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":31,"record_offset":9496,"parent_record_offset":9468,"ancestor_record_offset":9300,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":31,"record_offset":9832,"parent_record_offset":9432,"ancestor_record_offset":9300,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":31,"record_offset":9360,"parent_record_offset":9336,"ancestor_record_offset":9300,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":31,"record_offset":9524,"parent_record_offset":9496,"ancestor_record_offset":9300,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":31,"record_offset":9556,"parent_record_offset":9524,"ancestor_record_offset":9300,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":31,"record_offset":9432,"parent_record_offset":9408,"ancestor_record_offset":9300,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":31,"record_offset":9408,"parent_record_offset":9384,"ancestor_record_offset":9300,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_pool::ResourcePool::take_texture
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":695,"record_offset":256,"rva":12479376,"va":5381188496}
// 编译实例 RVAs: 0xbe6b90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1418bcc30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13459568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381188496,"raw_hex":"e87060cd00","size":5,"soft_fail":false,"target_va":5394648112,"va":5381188539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":695,"record_offset":1656,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":6530,"source_line":2503,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":695,"record_offset":400,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":695,"record_offset":1728,"parent_record_offset":1688,"ancestor_record_offset":256,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":695,"record_offset":1776,"parent_record_offset":1752,"ancestor_record_offset":256,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":695,"record_offset":1752,"parent_record_offset":1728,"ancestor_record_offset":256,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":695,"record_offset":1688,"parent_record_offset":1656,"ancestor_record_offset":256,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$346::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":695,"record_offset":1292,"parent_record_offset":1220,"ancestor_record_offset":256,"inlinee":14690,"source_line":183,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$363::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":695,"record_offset":1320,"parent_record_offset":1220,"ancestor_record_offset":256,"inlinee":14692,"source_line":206,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$527::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":695,"record_offset":1348,"parent_record_offset":1220,"ancestor_record_offset":256,"inlinee":8804,"source_line":649,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":695,"record_offset":1376,"parent_record_offset":1220,"ancestor_record_offset":256,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":695,"record_offset":1400,"parent_record_offset":1376,"ancestor_record_offset":256,"inlinee":9478,"source_line":146,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":695,"record_offset":1192,"parent_record_offset":1164,"ancestor_record_offset":256,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":695,"record_offset":688,"parent_record_offset":664,"ancestor_record_offset":256,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":695,"record_offset":772,"parent_record_offset":748,"ancestor_record_offset":256,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":695,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":256,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":695,"record_offset":1480,"parent_record_offset":556,"ancestor_record_offset":256,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":695,"record_offset":924,"parent_record_offset":896,"ancestor_record_offset":256,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":695,"record_offset":896,"parent_record_offset":868,"ancestor_record_offset":256,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":695,"record_offset":1044,"parent_record_offset":1020,"ancestor_record_offset":256,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":695,"record_offset":832,"parent_record_offset":804,"ancestor_record_offset":256,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":695,"record_offset":868,"parent_record_offset":832,"ancestor_record_offset":256,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":695,"record_offset":804,"parent_record_offset":556,"ancestor_record_offset":256,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":695,"record_offset":664,"parent_record_offset":556,"ancestor_record_offset":256,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":695,"record_offset":1508,"parent_record_offset":556,"ancestor_record_offset":256,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":695,"record_offset":748,"parent_record_offset":556,"ancestor_record_offset":256,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":695,"record_offset":636,"parent_record_offset":556,"ancestor_record_offset":256,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":695,"record_offset":1164,"parent_record_offset":1136,"ancestor_record_offset":256,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":695,"record_offset":448,"parent_record_offset":424,"ancestor_record_offset":256,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":695,"record_offset":424,"parent_record_offset":400,"ancestor_record_offset":256,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":695,"record_offset":1136,"parent_record_offset":968,"ancestor_record_offset":256,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":695,"record_offset":480,"parent_record_offset":448,"ancestor_record_offset":256,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":1076,"parent_record_offset":992,"ancestor_record_offset":256,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":1020,"parent_record_offset":992,"ancestor_record_offset":256,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":1600,"parent_record_offset":556,"ancestor_record_offset":256,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":992,"parent_record_offset":968,"ancestor_record_offset":256,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":532,"parent_record_offset":508,"ancestor_record_offset":256,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":508,"parent_record_offset":448,"ancestor_record_offset":256,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":556,"parent_record_offset":532,"ancestor_record_offset":256,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":608,"parent_record_offset":556,"ancestor_record_offset":256,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":968,"parent_record_offset":556,"ancestor_record_offset":256,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":695,"record_offset":1264,"parent_record_offset":1220,"ancestor_record_offset":256,"inlinee":14688,"source_line":38,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$24::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":695,"record_offset":1432,"parent_record_offset":1220,"ancestor_record_offset":256,"inlinee":12549,"source_line":49,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$31::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":695,"record_offset":1220,"parent_record_offset":1192,"ancestor_record_offset":256,"inlinee":14686,"source_line":67,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_pool::ResourcePool::insert_texture
// 记录形式: compiled_procedure; instances=1
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":695,"record_offset":1824,"rva":12479856,"va":5381188976}
// 编译实例 RVAs: 0xbe6d70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6107930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381188976,"raw_hex":"e8e6cca2ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381189413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6108249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381188976,"raw_hex":"e8a7cba2ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381189748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381188976,"raw_hex":"e8d2b4ffff","size":5,"soft_fail":false,"target_va":5381170352,"va":5381189593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418ad6b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13395753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381188976,"raw_hex":"e82967cc00","size":5,"soft_fail":false,"target_va":5394585264,"va":5381189506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418bcc30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13459063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381188976,"raw_hex":"e8775ecd00","size":5,"soft_fail":false,"target_va":5394648112,"va":5381189044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30351199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381188976,"raw_hex":"e85f1fcf01","size":5,"soft_fail":false,"target_va":5411540992,"va":5381189788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5985878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381188976,"raw_hex":"e8aaa9a4ff","size":5,"soft_fail":false,"target_va":5375203792,"va":5381189665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":695,"record_offset":3244,"parent_record_offset":1824,"ancestor_record_offset":1824,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":695,"record_offset":3396,"parent_record_offset":1824,"ancestor_record_offset":1824,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":695,"record_offset":4220,"parent_record_offset":1824,"ancestor_record_offset":1824,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":695,"record_offset":3508,"parent_record_offset":1824,"ancestor_record_offset":1824,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":695,"record_offset":1988,"parent_record_offset":1824,"ancestor_record_offset":1824,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":695,"record_offset":3480,"parent_record_offset":1824,"ancestor_record_offset":1824,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":695,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":1824,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":695,"record_offset":3324,"parent_record_offset":3300,"ancestor_record_offset":1824,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":695,"record_offset":3964,"parent_record_offset":3940,"ancestor_record_offset":1824,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":695,"record_offset":3276,"parent_record_offset":3244,"ancestor_record_offset":1824,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":695,"record_offset":3940,"parent_record_offset":3852,"ancestor_record_offset":1824,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":695,"record_offset":4296,"parent_record_offset":4264,"ancestor_record_offset":1824,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":695,"record_offset":3880,"parent_record_offset":3852,"ancestor_record_offset":1824,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":695,"record_offset":3852,"parent_record_offset":3828,"ancestor_record_offset":1824,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":695,"record_offset":4344,"parent_record_offset":4320,"ancestor_record_offset":1824,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":695,"record_offset":4320,"parent_record_offset":4296,"ancestor_record_offset":1824,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":695,"record_offset":3828,"parent_record_offset":3804,"ancestor_record_offset":1824,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":695,"record_offset":3420,"parent_record_offset":3396,"ancestor_record_offset":1824,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":695,"record_offset":4264,"parent_record_offset":4220,"ancestor_record_offset":1824,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":695,"record_offset":3444,"parent_record_offset":3420,"ancestor_record_offset":1824,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":695,"record_offset":3572,"parent_record_offset":3540,"ancestor_record_offset":1824,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$346::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":695,"record_offset":2880,"parent_record_offset":2808,"ancestor_record_offset":1824,"inlinee":14690,"source_line":183,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$363::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":695,"record_offset":2908,"parent_record_offset":2808,"ancestor_record_offset":1824,"inlinee":14692,"source_line":206,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$527::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":695,"record_offset":2936,"parent_record_offset":2808,"ancestor_record_offset":1824,"inlinee":8804,"source_line":649,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":695,"record_offset":2964,"parent_record_offset":2808,"ancestor_record_offset":1824,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":695,"record_offset":2988,"parent_record_offset":2964,"ancestor_record_offset":1824,"inlinee":9478,"source_line":146,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":695,"record_offset":2780,"parent_record_offset":2752,"ancestor_record_offset":1824,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":695,"record_offset":2276,"parent_record_offset":2252,"ancestor_record_offset":1824,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":695,"record_offset":2360,"parent_record_offset":2336,"ancestor_record_offset":1824,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":695,"record_offset":2300,"parent_record_offset":2276,"ancestor_record_offset":1824,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":695,"record_offset":3068,"parent_record_offset":2144,"ancestor_record_offset":1824,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":695,"record_offset":3908,"parent_record_offset":3880,"ancestor_record_offset":1824,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":695,"record_offset":2512,"parent_record_offset":2484,"ancestor_record_offset":1824,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":695,"record_offset":2484,"parent_record_offset":2456,"ancestor_record_offset":1824,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":695,"record_offset":3540,"parent_record_offset":3508,"ancestor_record_offset":1824,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":695,"record_offset":2632,"parent_record_offset":2608,"ancestor_record_offset":1824,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":695,"record_offset":3352,"parent_record_offset":3324,"ancestor_record_offset":1824,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":695,"record_offset":4384,"parent_record_offset":4220,"ancestor_record_offset":1824,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":695,"record_offset":2420,"parent_record_offset":2392,"ancestor_record_offset":1824,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":695,"record_offset":2456,"parent_record_offset":2420,"ancestor_record_offset":1824,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":695,"record_offset":2392,"parent_record_offset":2144,"ancestor_record_offset":1824,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":695,"record_offset":2252,"parent_record_offset":2144,"ancestor_record_offset":1824,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":695,"record_offset":3096,"parent_record_offset":2144,"ancestor_record_offset":1824,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":695,"record_offset":2336,"parent_record_offset":2144,"ancestor_record_offset":1824,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":695,"record_offset":2224,"parent_record_offset":2144,"ancestor_record_offset":1824,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":695,"record_offset":2752,"parent_record_offset":2724,"ancestor_record_offset":1824,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":695,"record_offset":2036,"parent_record_offset":2012,"ancestor_record_offset":1824,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":695,"record_offset":2012,"parent_record_offset":1988,"ancestor_record_offset":1824,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":695,"record_offset":2724,"parent_record_offset":2556,"ancestor_record_offset":1824,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":695,"record_offset":2068,"parent_record_offset":2036,"ancestor_record_offset":1824,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":2664,"parent_record_offset":2580,"ancestor_record_offset":1824,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":2608,"parent_record_offset":2580,"ancestor_record_offset":1824,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":3188,"parent_record_offset":2144,"ancestor_record_offset":1824,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":2580,"parent_record_offset":2556,"ancestor_record_offset":1824,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":2120,"parent_record_offset":2096,"ancestor_record_offset":1824,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":2096,"parent_record_offset":2036,"ancestor_record_offset":1824,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":2144,"parent_record_offset":2120,"ancestor_record_offset":1824,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":2196,"parent_record_offset":2144,"ancestor_record_offset":1824,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":2556,"parent_record_offset":2144,"ancestor_record_offset":1824,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":695,"record_offset":2852,"parent_record_offset":2808,"ancestor_record_offset":1824,"inlinee":14688,"source_line":38,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$24::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":695,"record_offset":3020,"parent_record_offset":2808,"ancestor_record_offset":1824,"inlinee":12549,"source_line":49,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$31::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":695,"record_offset":2808,"parent_record_offset":2780,"ancestor_record_offset":1824,"inlinee":14686,"source_line":67,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_pool::ResourcePool::take_buffer
// 记录形式: compiled_procedure; instances=1
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":695,"record_offset":4588,"rva":12480992,"va":5381190112}
// 编译实例 RVAs: 0xbe71e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1418bce70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13458530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381190112,"raw_hex":"e8625ccd00","size":5,"soft_fail":false,"target_va":5394648688,"va":5381190153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":695,"record_offset":5832,"parent_record_offset":4588,"ancestor_record_offset":4588,"inlinee":6530,"source_line":2503,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":695,"record_offset":4732,"parent_record_offset":4588,"ancestor_record_offset":4588,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":695,"record_offset":5904,"parent_record_offset":5864,"ancestor_record_offset":4588,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":695,"record_offset":5952,"parent_record_offset":5928,"ancestor_record_offset":4588,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":695,"record_offset":5928,"parent_record_offset":5904,"ancestor_record_offset":4588,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":695,"record_offset":5864,"parent_record_offset":5832,"ancestor_record_offset":4588,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$278::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":695,"record_offset":5580,"parent_record_offset":5548,"ancestor_record_offset":4588,"inlinee":39433,"source_line":114,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":695,"record_offset":5520,"parent_record_offset":5492,"ancestor_record_offset":4588,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":695,"record_offset":5020,"parent_record_offset":4996,"ancestor_record_offset":4588,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":695,"record_offset":5104,"parent_record_offset":5080,"ancestor_record_offset":4588,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":695,"record_offset":5044,"parent_record_offset":5020,"ancestor_record_offset":4588,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":695,"record_offset":5656,"parent_record_offset":4888,"ancestor_record_offset":4588,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":695,"record_offset":5252,"parent_record_offset":5224,"ancestor_record_offset":4588,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":695,"record_offset":5224,"parent_record_offset":5196,"ancestor_record_offset":4588,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":695,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":4588,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] gpu_allocator::impl$7::eq
//     owner=confirmed_vendored_dependency; count=1
//     packages=[{"name":"gpu-allocator","version":"0.26.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/gpu-allocator/src/lib.rs
//     evidence={"module":695,"record_offset":5608,"parent_record_offset":5548,"ancestor_record_offset":4588,"inlinee":39435,"source_line":223,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":695,"record_offset":5164,"parent_record_offset":5136,"ancestor_record_offset":4588,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":695,"record_offset":5196,"parent_record_offset":5164,"ancestor_record_offset":4588,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":695,"record_offset":5136,"parent_record_offset":4888,"ancestor_record_offset":4588,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":695,"record_offset":4996,"parent_record_offset":4888,"ancestor_record_offset":4588,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":695,"record_offset":5684,"parent_record_offset":4888,"ancestor_record_offset":4588,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":695,"record_offset":5080,"parent_record_offset":4888,"ancestor_record_offset":4588,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":695,"record_offset":4968,"parent_record_offset":4888,"ancestor_record_offset":4588,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":695,"record_offset":5492,"parent_record_offset":5464,"ancestor_record_offset":4588,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":695,"record_offset":4780,"parent_record_offset":4756,"ancestor_record_offset":4588,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":695,"record_offset":4756,"parent_record_offset":4732,"ancestor_record_offset":4588,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":695,"record_offset":5464,"parent_record_offset":5296,"ancestor_record_offset":4588,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":695,"record_offset":4812,"parent_record_offset":4780,"ancestor_record_offset":4588,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":5404,"parent_record_offset":5320,"ancestor_record_offset":4588,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":5348,"parent_record_offset":5320,"ancestor_record_offset":4588,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":5776,"parent_record_offset":4888,"ancestor_record_offset":4588,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":5320,"parent_record_offset":5296,"ancestor_record_offset":4588,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":4864,"parent_record_offset":4840,"ancestor_record_offset":4588,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":4840,"parent_record_offset":4780,"ancestor_record_offset":4588,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":4888,"parent_record_offset":4864,"ancestor_record_offset":4588,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":4940,"parent_record_offset":4888,"ancestor_record_offset":4588,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":5296,"parent_record_offset":4888,"ancestor_record_offset":4588,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::impl$14::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":695,"record_offset":5548,"parent_record_offset":5520,"ancestor_record_offset":4588,"inlinee":39431,"source_line":66,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_pool::ResourcePool::insert_buffer
// 记录形式: compiled_procedure; instances=1
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":695,"record_offset":6000,"rva":12481312,"va":5381190432}
// 编译实例 RVAs: 0xbe7320
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6109245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381190432,"raw_hex":"e8c3c7a2ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381190728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6109529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381190432,"raw_hex":"e8a7c6a2ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381191028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381190432,"raw_hex":"e8c6afffff","size":5,"soft_fail":false,"target_va":5381170352,"va":5381190885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418ad9c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13395228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381190432,"raw_hex":"e81c65cc00","size":5,"soft_fail":false,"target_va":5394586048,"va":5381190815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418bce70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13458183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381190432,"raw_hex":"e8075bcd00","size":5,"soft_fail":false,"target_va":5394648688,"va":5381190500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30349919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381190432,"raw_hex":"e85f1acf01","size":5,"soft_fail":false,"target_va":5411540992,"va":5381191068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6834810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381190432,"raw_hex":"e886b597ff","size":5,"soft_fail":false,"target_va":5374356144,"va":5381190949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":695,"record_offset":7260,"parent_record_offset":6000,"ancestor_record_offset":6000,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":695,"record_offset":7412,"parent_record_offset":6000,"ancestor_record_offset":6000,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":695,"record_offset":8236,"parent_record_offset":6000,"ancestor_record_offset":6000,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":695,"record_offset":7524,"parent_record_offset":6000,"ancestor_record_offset":6000,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":695,"record_offset":6160,"parent_record_offset":6000,"ancestor_record_offset":6000,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":695,"record_offset":7496,"parent_record_offset":6000,"ancestor_record_offset":6000,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":695,"record_offset":7316,"parent_record_offset":7292,"ancestor_record_offset":6000,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":695,"record_offset":7340,"parent_record_offset":7316,"ancestor_record_offset":6000,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":695,"record_offset":7980,"parent_record_offset":7956,"ancestor_record_offset":6000,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":695,"record_offset":7292,"parent_record_offset":7260,"ancestor_record_offset":6000,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":695,"record_offset":7956,"parent_record_offset":7868,"ancestor_record_offset":6000,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":695,"record_offset":8312,"parent_record_offset":8280,"ancestor_record_offset":6000,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":695,"record_offset":7896,"parent_record_offset":7868,"ancestor_record_offset":6000,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":695,"record_offset":7868,"parent_record_offset":7844,"ancestor_record_offset":6000,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":695,"record_offset":8360,"parent_record_offset":8336,"ancestor_record_offset":6000,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":695,"record_offset":8336,"parent_record_offset":8312,"ancestor_record_offset":6000,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":695,"record_offset":7844,"parent_record_offset":7820,"ancestor_record_offset":6000,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":695,"record_offset":7436,"parent_record_offset":7412,"ancestor_record_offset":6000,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":695,"record_offset":8280,"parent_record_offset":8236,"ancestor_record_offset":6000,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":695,"record_offset":7460,"parent_record_offset":7436,"ancestor_record_offset":6000,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":695,"record_offset":7588,"parent_record_offset":7556,"ancestor_record_offset":6000,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$278::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":695,"record_offset":7008,"parent_record_offset":6976,"ancestor_record_offset":6000,"inlinee":39433,"source_line":114,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":695,"record_offset":6948,"parent_record_offset":6920,"ancestor_record_offset":6000,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":695,"record_offset":6448,"parent_record_offset":6424,"ancestor_record_offset":6000,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":695,"record_offset":6532,"parent_record_offset":6508,"ancestor_record_offset":6000,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":695,"record_offset":6472,"parent_record_offset":6448,"ancestor_record_offset":6000,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":695,"record_offset":7084,"parent_record_offset":6316,"ancestor_record_offset":6000,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":695,"record_offset":7924,"parent_record_offset":7896,"ancestor_record_offset":6000,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":695,"record_offset":6680,"parent_record_offset":6652,"ancestor_record_offset":6000,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":695,"record_offset":6652,"parent_record_offset":6624,"ancestor_record_offset":6000,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":695,"record_offset":7556,"parent_record_offset":7524,"ancestor_record_offset":6000,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":695,"record_offset":6800,"parent_record_offset":6776,"ancestor_record_offset":6000,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":695,"record_offset":7368,"parent_record_offset":7340,"ancestor_record_offset":6000,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":695,"record_offset":8400,"parent_record_offset":8236,"ancestor_record_offset":6000,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] gpu_allocator::impl$7::eq
//     owner=confirmed_vendored_dependency; count=1
//     packages=[{"name":"gpu-allocator","version":"0.26.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/gpu-allocator/src/lib.rs
//     evidence={"module":695,"record_offset":7036,"parent_record_offset":6976,"ancestor_record_offset":6000,"inlinee":39435,"source_line":223,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":695,"record_offset":6592,"parent_record_offset":6564,"ancestor_record_offset":6000,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":695,"record_offset":6624,"parent_record_offset":6592,"ancestor_record_offset":6000,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":695,"record_offset":6564,"parent_record_offset":6316,"ancestor_record_offset":6000,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":695,"record_offset":6424,"parent_record_offset":6316,"ancestor_record_offset":6000,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":695,"record_offset":7112,"parent_record_offset":6316,"ancestor_record_offset":6000,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":695,"record_offset":6508,"parent_record_offset":6316,"ancestor_record_offset":6000,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":695,"record_offset":6396,"parent_record_offset":6316,"ancestor_record_offset":6000,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":695,"record_offset":6920,"parent_record_offset":6892,"ancestor_record_offset":6000,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":695,"record_offset":6208,"parent_record_offset":6184,"ancestor_record_offset":6000,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":695,"record_offset":6184,"parent_record_offset":6160,"ancestor_record_offset":6000,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":695,"record_offset":6892,"parent_record_offset":6724,"ancestor_record_offset":6000,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":695,"record_offset":6240,"parent_record_offset":6208,"ancestor_record_offset":6000,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":6832,"parent_record_offset":6748,"ancestor_record_offset":6000,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":6776,"parent_record_offset":6748,"ancestor_record_offset":6000,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":7204,"parent_record_offset":6316,"ancestor_record_offset":6000,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":6748,"parent_record_offset":6724,"ancestor_record_offset":6000,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":6292,"parent_record_offset":6268,"ancestor_record_offset":6000,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":6268,"parent_record_offset":6208,"ancestor_record_offset":6000,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":6316,"parent_record_offset":6292,"ancestor_record_offset":6000,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":6368,"parent_record_offset":6316,"ancestor_record_offset":6000,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":695,"record_offset":6724,"parent_record_offset":6316,"ancestor_record_offset":6000,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::impl$14::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":695,"record_offset":6976,"parent_record_offset":6948,"ancestor_record_offset":6000,"inlinee":39431,"source_line":66,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_pool::ResourcePool::clear
// 记录形式: compiled_procedure; instances=1
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":695,"record_offset":8604,"rva":12482288,"va":5381191408}
// 编译实例 RVAs: 0xbe76f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1417e85c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12586690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381191408,"raw_hex":"e8c20ec000","size":5,"soft_fail":false,"target_va":5393778112,"va":5381191417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1417e8160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t12585557","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381191408,"raw_hex":"e9550ac000","size":5,"soft_fail":false,"target_va":5393776992,"va":5381191430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":695,"record_offset":8736,"parent_record_offset":8604,"ancestor_record_offset":8604,"inlinee":19928,"source_line":727,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":695,"record_offset":8756,"parent_record_offset":8736,"ancestor_record_offset":8604,"inlinee":17339,"source_line":1009,"archive":"_pdb/B/modules/00695.jsonl.gz"}
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
// 完整结构及原始类型记录: resource_pool.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::resource_pool::ResourcePool
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 8; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::resource_pool::ResourcePool>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::resource_pool::ResourcePool"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: resource_pool.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: resource_pool.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_pool::ResourcePool::clear
// 信息 ID: 6931cb124d7710567c560e19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x4dd6f7 -> 0xbe76f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render @ 0x4e8cb0 -> 0xbe76f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_pool::ResourcePool::insert_buffer
// 信息 ID: 97323221fc6bfc34c046ed88
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_pool::ResourcePool::insert_texture
// 信息 ID: b9cf563ef65e69bc993dd012
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_pool::ResourcePool::take_buffer
// 信息 ID: 08979c866323b1e9456025a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_registry::ResourceRegistry::populate_resources @ 0xbe846d -> 0xbe71e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_pool::ResourcePool::take_texture
// 信息 ID: 6010779406d3c6c101806ccc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_registry::ResourceRegistry::populate_resources @ 0xbe83b4 -> 0xbe6b90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_pool::impl$1::default
// 信息 ID: aaf7198c90ca97f5da4a9140
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$8,tuple$<> >; module=31, inline_record=9300; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::run_game::run_game_thread; module=240, inline_record=72948; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
